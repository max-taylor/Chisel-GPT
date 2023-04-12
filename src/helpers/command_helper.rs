// my_module.rs

use std::borrow::Cow;

use chisel::{
    prelude::COMMAND_LEADER,
    solidity_helper::{self, SolidityHelper},
};
use rustyline::Helper;
use yansi::{Color, Style};

pub struct CommandHelper {
    solidity_helper: SolidityHelper,
}

const MAX_ANSI_LEN: usize = 9;

impl CommandHelper {
    pub fn new() -> Self {
        Self {
            solidity_helper: SolidityHelper::default(),
        }
    }

    /// Highlights a solidity source string
    pub fn highlight(self, input: &str) -> Cow<str> {
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
            // SolidityHelper::paint_unchecked(cmd, style, &mut out);

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
}

impl std::ops::Deref for CommandHelper {
    type Target = SolidityHelper;

    fn deref(&self) -> &Self::Target {
        &self.solidity_helper
    }
}

impl std::ops::DerefMut for CommandHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.solidity_helper
    }
}
