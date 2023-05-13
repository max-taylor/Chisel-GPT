use std::vec;

use async_openai::types::CreateChatCompletionStreamResponse;
use chisel::prelude::{ChiselDispatcher, DispatchResult};

use crate::helpers::{
    dispatch::log_dispatch_result,
    split_commands::{format_current_line, process_lines, Response},
};

use super::context::{END_TAG, START_TAG};

type CommandResult = Result<(), Box<dyn std::error::Error>>;

pub enum CurrentCommandState {
    Complete(Vec<String>),
    Incomplete(Vec<String>),
}

pub struct CommandResponse {
    pub parsed_commands: Vec<String>,
    pub current_command_state: CurrentCommandState,
    pub current_line: String,
    pub within_command_response: bool,
}

impl CommandResponse {
    pub fn new() -> Self {
        Self {
            parsed_commands: vec![],
            within_command_response: false,
            current_line: String::new(),
            current_command_state: CurrentCommandState::Incomplete(vec![]),
        }
    }

    pub async fn try_send_current_command(
        &mut self,
        dispatcher: &mut ChiselDispatcher,
    ) -> CommandResult {
        if let CurrentCommandState::Complete(lines) = &self.current_command_state {
            let command = lines.join("\n").trim().to_string();

            let dispatch_result = dispatcher.dispatch(&command).await;
            log_dispatch_result(&dispatch_result);

            match dispatch_result {
                DispatchResult::Success(_) | DispatchResult::CommandSuccess(_) => {
                    self.parsed_commands.push(command);
                    self.current_command_state = CurrentCommandState::Incomplete(vec![]);
                }
                // TODO: Retry logic here
                _ => {
                    dbg!("command failed");
                }
            }

            return Ok(());
        }

        Ok(())
    }

    // pub fn handle_complete_line()

    pub fn handle_stream(&mut self, next_stream_response: &String) -> (&mut Self, bool) {
        if let CurrentCommandState::Incomplete(lines) = &mut self.current_command_state {
            if !self.within_command_response && !next_stream_response.contains(START_TAG) {
                return (self, false);
            }

            if next_stream_response.contains(END_TAG) {
                // TODO: State for when completed
                self.within_command_response = false;
                self.current_command_state = CurrentCommandState::Complete(lines.clone());

                return (self, false);
            }

            let current_line = format_current_line(&self.current_line, next_stream_response);

            if let Response::Incomplete(line) = current_line {
                self.current_line = line;

                return (self, false);
            }

            self.current_line = String::new();

            lines.push(next_stream_response.clone());

            let parsed_command = process_lines(lines);

            if let Response::Complete(_) = parsed_command {
                self.current_command_state = CurrentCommandState::Complete(lines.clone());

                return (self, true);
            }

            return (self, false);
        } else {
            panic!("Bad path reached");
            // DispatchResult::Failure(None)
        }
    }
}
