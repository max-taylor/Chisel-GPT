pub enum Response {
    Complete(String),
    Incomplete(String),
}

impl Response {
    pub fn is_complete(&self) -> bool {
        match self {
            Response::Complete(_) => true,
            Response::Incomplete(_) => false,
        }
    }
}

impl Into<String> for Response {
    fn into(self) -> String {
        match self {
            Response::Complete(s) => s,
            Response::Incomplete(s) => s,
        }
    }
}

pub fn format_current_line(current_line: &str, next_value: &str) -> Response {
    let updated_line = format!("{}{}", current_line, next_value);

    if next_value.contains("\n") {
        Response::Complete(updated_line)
    } else {
        Response::Incomplete(updated_line)
    }
}

pub fn process_lines(current_lines: &mut Vec<String>) -> Response {
    let current_command = current_lines.join("").trim().to_string();

    let mut open_brackets = 0;
    let mut in_multiline_construct = false;

    for (index, line) in current_lines.iter().enumerate() {
        if line.starts_with("pragma") {
            continue;
        }

        open_brackets += line.chars().filter(|&c| c == '{').count();
        open_brackets -= line.chars().filter(|&c| c == '}').count();

        if line.contains("= new") && line.ends_with("(") {
            in_multiline_construct = true;
        } else if in_multiline_construct && line.ends_with(");") {
            in_multiline_construct = false;
        }

        if open_brackets == 0 && !in_multiline_construct {
            if index != current_lines.len() {
                // panic!("Index is not at the end of the lines");
                // dbg!("Not at end", index, current_lines.len());
            }
            return Response::Complete(current_command);
        }
    }

    Response::Incomplete(current_command)
}
