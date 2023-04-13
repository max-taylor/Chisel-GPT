// ! Had to copy most of solidity_helper from foundry to be able to modify the highlight method so that it returns a green color for commands prefixed with '!chat'. Rust makes it very difficult to modify struct's outside of scope, tried dynamic dispatching but that didn't work

use std::borrow::Cow;

use chisel::{
    prelude::{COMMAND_LEADER, PROMPT_ARROW},
    solidity_helper::SolidityHelper,
};
use rustyline::{
    completion::Completer,
    highlight::Highlighter,
    hint::Hinter,
    validate::{ValidationContext, ValidationResult, Validator},
    Helper,
};
use solang_parser::{
    lexer::{Lexer, LexicalError, Token},
    pt,
};
use yansi::{Color, Paint, Style};

/// The default pre-allocation for solang parsed comments
const DEFAULT_COMMENTS: usize = 5;

/// The maximum length of an ANSI prefix + suffix characters using [SolidityHelper].
///
/// * 5 - prefix:
///   * 2 - start: `\x1B[`
///   * 2 - fg: `3<fg_code>`
///   * 1 - end: `m`
/// * 4 - suffix: `\x1B[0m`
const MAX_ANSI_LEN: usize = 9;

/// A rustyline helper for Solidity code
#[derive(Clone, Debug, Default)]
pub struct CommandHelper {
    /// Whether the dispatcher has errored.
    pub errored: bool,
}

impl CommandHelper {
    /// Create a new CommandHelper.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the errored field.
    pub fn set_errored(&mut self, errored: bool) -> &mut Self {
        self.errored = errored;
        self
    }

    /// Highlights a solidity source string
    pub fn highlight(input: &str) -> Cow<str> {
        if input.starts_with("!chat") {
            let (cmd, rest) = match input.split_once(' ') {
                Some((cmd, rest)) => (cmd, Some(rest)),
                None => (input, None),
            };
            let cmd = cmd.strip_prefix(COMMAND_LEADER).unwrap_or(cmd);

            let mut out = String::with_capacity(input.len() + MAX_ANSI_LEN);

            // cmd
            out.push(COMMAND_LEADER);

            let style = Style::new(Color::Green);

            if style == Style::default() {
                out.push_str(cmd);
            } else {
                let _ = style.fmt_prefix(&mut out);
                out.push_str(cmd);
                let _ = style.fmt_suffix(&mut out);
            }
            // CommandHelper::paint_unchecked(cmd, style, &mut out);

            // rest
            match rest {
                Some(rest) if !rest.is_empty() => {
                    out.push(' ');
                    out.push_str(rest);
                }
                _ => {}
            }

            Cow::Owned(out)
        } else {
            SolidityHelper::highlight(input)
        }
    }

    /// Validate that a source snippet is closed (i.e., all braces and parenthesis are matched).
    fn validate_closed(input: &str) -> ValidationResult {
        if Self::skip_input(input) {
            let msg = Paint::red("\nInput must not start with `.<number>`");
            return ValidationResult::Invalid(Some(msg.to_string()));
        }

        let mut bracket_depth = 0usize;
        let mut paren_depth = 0usize;
        let mut brace_depth = 0usize;
        let mut comments = Vec::with_capacity(DEFAULT_COMMENTS);
        // returns on any encountered error, so allocate for just one
        let mut errors = Vec::with_capacity(1);
        for res in Lexer::new(input, 0, &mut comments, &mut errors) {
            match res {
                Err(err) => match err {
                    LexicalError::EndOfFileInComment(_)
                    | LexicalError::EndofFileInHex(_)
                    | LexicalError::EndOfFileInString(_) => return ValidationResult::Incomplete,
                    _ => return ValidationResult::Valid(None),
                },
                Ok((_, token, _)) => match token {
                    Token::OpenBracket => {
                        bracket_depth += 1;
                    }
                    Token::OpenCurlyBrace => {
                        brace_depth += 1;
                    }
                    Token::OpenParenthesis => {
                        paren_depth += 1;
                    }
                    Token::CloseBracket => {
                        bracket_depth = bracket_depth.saturating_sub(1);
                    }
                    Token::CloseCurlyBrace => {
                        brace_depth = brace_depth.saturating_sub(1);
                    }
                    Token::CloseParenthesis => {
                        paren_depth = paren_depth.saturating_sub(1);
                    }
                    _ => {}
                },
            }
        }
        if (bracket_depth | brace_depth | paren_depth) == 0 {
            ValidationResult::Valid(None)
        } else {
            ValidationResult::Incomplete
        }
    }

    /// Formats `input` with `style` into `out`, without checking `style.wrapping` or
    /// `Paint::is_enabled`
    #[inline]
    fn paint_unchecked(string: &str, style: Style, out: &mut String) {
        if style == Style::default() {
            out.push_str(string);
        } else {
            let _ = style.fmt_prefix(out);
            out.push_str(string);
            let _ = style.fmt_suffix(out);
        }
    }

    #[inline]
    fn paint_unchecked_owned(string: &str, style: Style) -> String {
        let mut out = String::with_capacity(MAX_ANSI_LEN + string.len());
        Self::paint_unchecked(string, style, &mut out);
        out
    }

    /// Whether to skip parsing this input due to known errors or panics
    #[inline]
    fn skip_input(input: &str) -> bool {
        // input.startsWith(/\.[0-9]/)
        let mut chars = input.chars();
        chars.next() == Some('.') && chars.next().map(|c| c.is_ascii_digit()).unwrap_or_default()
    }
}

impl Highlighter for CommandHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Self::highlight(line)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        pos == line.len()
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> Cow<'b, str> {
        if !Paint::is_enabled() {
            return Cow::Borrowed(prompt);
        }

        let mut out = prompt.to_string();

        // `^(\(ID: .*?\) )? ➜ `
        if prompt.starts_with("(ID: ") {
            let id_end = prompt.find(')').unwrap();
            let id_span = 5..id_end;
            let id = &prompt[id_span.clone()];
            out.replace_range(
                id_span,
                &Self::paint_unchecked_owned(id, Color::Yellow.style()),
            );
            out.replace_range(
                1..=2,
                &Self::paint_unchecked_owned("ID", Color::Cyan.style()),
            );
        }

        if let Some(i) = out.find(PROMPT_ARROW) {
            let style = if self.errored {
                Color::Red.style()
            } else {
                Color::Green.style()
            };

            let mut arrow = String::with_capacity(MAX_ANSI_LEN + 4);

            let _ = style.fmt_prefix(&mut arrow);
            arrow.push(PROMPT_ARROW);
            let _ = style.fmt_suffix(&mut arrow);

            out.replace_range(i..=i + 2, &arrow);
        }

        Cow::Owned(out)
    }
}

impl Validator for CommandHelper {
    fn validate(&self, ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        Ok(Self::validate_closed(ctx.input()))
    }
}

impl Completer for CommandHelper {
    type Candidate = String;
}

impl Hinter for CommandHelper {
    type Hint = String;
}

impl Helper for CommandHelper {}
