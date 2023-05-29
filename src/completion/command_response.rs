use std::vec;

use chisel::prelude::{ChiselDispatcher, DispatchResult};

use crate::helpers::{
    dispatch::log_dispatch_result,
    split_commands::{format_current_line, process_lines, Response},
};

use super::context::{END_TAG, START_TAG};

type CommandResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq)]
pub enum Command {
    Pending(String),
    Errored(String),
    Complete(String),
}

/**
 * CommandResponse struct holds the state for a single command response, this is primarily so that it can be retried
 */
pub struct CommandResponse {
    pub current_line: String,
    pub current_command_lines: Vec<String>,
    pub within_command_response: bool,
    pub commands: Vec<Command>,
}

impl CommandResponse {
    pub fn new() -> Self {
        Self {
            within_command_response: false,
            current_line: String::new(),
            current_command_lines: vec![],
            commands: vec![],
        }
    }

    pub async fn resolve_errored_commands(&mut self) {
        for command in self.commands.iter_mut() {
            match command {
                Command::Errored(command_text) => {
                    // TODO: Tell ChatGPT you done fucked up
                }
                _ => {}
            }
        }
    }

    pub async fn try_send_pending_commands(
        &mut self,
        dispatcher: &mut ChiselDispatcher,
    ) -> CommandResult {
        for command in self.commands.iter_mut() {
            match command {
                Command::Pending(command_text) => {
                    let dispatch_result = dispatcher.dispatch(&command_text).await;
                    log_dispatch_result(&dispatch_result);

                    match dispatch_result {
                        DispatchResult::Success(_) | DispatchResult::CommandSuccess(_) => {
                            *command = Command::Complete(command_text.clone());

                            self.current_command_lines = vec![];
                        }
                        _ => {
                            eprintln!("Ingredient failed");

                            *command = Command::Errored(command_text.clone());
                        }
                    }
                }
                // Ignore the command if it has successfully completed
                _ => {}
            }
        }

        Ok(())
    }

    /// This method takes in the next stream response and appends it to the pending command state
    ///
    /// # Parameters
    ///
    /// * `next_stream_response` - The next response from the stream
    ///
    /// # Returns
    ///
    /// * `bool` - True if the command is complete, false if the command is incomplete
    pub fn handle_next_stream_value(&mut self, next_stream_response: &String) -> bool {
        let current_line = format_current_line(&self.current_line, next_stream_response);

        if let Response::Incomplete(line) = current_line {
            self.current_line = line;

            return false;
        } else if let Response::Complete(line) = current_line {
            self.current_line = String::new();

            if line.contains(START_TAG) {
                self.within_command_response = true;

                return false;
            } else if line.contains(END_TAG) {
                self.within_command_response = false;

                return false;
            } else if !self.within_command_response {
                return false;
            }

            self.current_command_lines.push(line.clone());

            let parsed_command = process_lines(&mut self.current_command_lines);

            if let Response::Complete(mut command) = parsed_command {
                command.push_str("\n");

                self.commands.push(Command::Pending(command));
                self.current_command_lines = vec![];

                return true;
            }

            return false;
        } else {
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::completion::command_response::Command;

    use super::CommandResponse;

    fn from_line_array(line_array: Vec<&str>) -> CommandResponse {
        let mut command_response = CommandResponse::new();

        for line in line_array {
            command_response.handle_next_stream_value(&line.to_string());
        }

        command_response
    }

    #[test]
    fn it_can_handle_simple_stream() {
        let items = vec![
            "##START##\n",
            "contract Token {\n",
            "string public constant symbol = 'TKN';\n",
            "}\n",
            "##END##\n",
        ];

        let command_response = from_line_array(items);

        assert_eq!(command_response.commands.len(), 1);
        assert_eq!(
            command_response.commands[0],
            Command::Pending(
                "contract Token {\nstring public constant symbol = 'TKN';\n}\n".to_string()
            )
        );
        assert_eq!(command_response.current_command_lines.len(), 0);
        assert_eq!(command_response.current_line.len(), 0);
    }

    #[test]
    fn it_can_split_function_string() {
        let items = vec![
            "##START##\n",
            "function test() external {\n",
            "address owner = msg.sender;\n",
            "}\n",
            "##END##\n",
        ];

        let command_response = from_line_array(items);

        assert_eq!(
            command_response.commands[0],
            Command::Pending(
                "function test() external {\naddress owner = msg.sender;\n}\n".to_string()
            )
        );
    }

    #[test]
    fn it_can_split_multiple_solidity_single_lines() {
        let items = vec![
            "##START##\n",
            "address owner = msg.sender;\n",
            "address token1;\n",
            "address token2;\n",
            "##END##\n",
        ];

        let command_response = from_line_array(items);

        assert_eq!(command_response.commands.len(), 3);
        assert_eq!(
            command_response.commands[0],
            Command::Pending("address owner = msg.sender;\n".to_string())
        );
        assert_eq!(
            command_response.commands[1],
            Command::Pending("address token1;\n".to_string())
        );
        assert_eq!(
            command_response.commands[2],
            Command::Pending("address token2;\n".to_string())
        );
    }

    #[test]
    fn it_can_split_function_and_method() {
        let items = vec![
            "##START##\n",
            "contract Token {\n",
            "string public constant symbol = 'TKN';\n",
            "string public constant name = 'TKN';\n",
            "}\n",
            "function test() external {\n",
            "address owner = msg.sender;\n",
            "}\n",
            "##END##\n",
        ];

        let command_response = from_line_array(items);

        assert_eq!(command_response.commands.len(), 2);
        assert_eq!(
            command_response.commands[0],
            Command::Pending("contract Token {\nstring public constant symbol = 'TKN';\nstring public constant name = 'TKN';\n}\n".to_string())
        );
        assert_eq!(
            command_response.commands[1],
            Command::Pending(
                "function test() external {\naddress owner = msg.sender;\n}\n".to_string()
            )
        );
    }

    #[test]
    fn it_can_split_contract_with_methods() {
        let items = vec![
            "##START##\n",
            "contract LiquidityPool {\n",
            "address public token1;\n",
            "function addLiquidity(uint256 _amount1, uint256 _amount2) public {\n",
            "uint256 total = _amount + _amount2;\n",
            "}\n",
            "}\n",
            "##END##\n",
        ];

        let command_response = from_line_array(items);

        assert_eq!(command_response.commands.len(), 1);
        assert_eq!(
            command_response.commands[0],
            Command::Pending("contract LiquidityPool {\naddress public token1;\nfunction addLiquidity(uint256 _amount1, uint256 _amount2) public {\nuint256 total = _amount + _amount2;\n}\n}\n".to_string())
        );
    }

    #[test]
    fn it_can_split_contract_and_interface() {
        let items = vec![
            "##START##\n",
            "contract LiquidityPool {\n",
            "address public token1;\n",
            "function addLiquidity(uint256 _amount1, uint256 _amount2) public {\n",
            "uint256 total = _amount + _amount2;\n",
            "}\n",
            "}\n",
            "interface IERC20 {\n",
            "function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);\n",
            "}\n",
            "##END##\n",
        ];

        let command_response = from_line_array(items);

        assert_eq!(command_response.commands.len(), 2);
        assert_eq!(
            command_response.commands[0],
            Command::Pending("contract LiquidityPool {\naddress public token1;\nfunction addLiquidity(uint256 _amount1, uint256 _amount2) public {\nuint256 total = _amount + _amount2;\n}\n}\n".to_string())
        );
        assert_eq!(
            command_response.commands[1],
            Command::Pending("interface IERC20 {\nfunction transferFrom(address sender, address recipient, uint256 amount) external returns (bool);\n}\n".to_string())
        );
    }

    #[test]
    fn it_can_split_contract_with_constructor_and_function() {
        let items = vec![
            "##START##\n",
            "contract LiquidityPool {\n",
            "address public token1;\n",
            "constructor(address _token1) {\n",
            "token1 = _token1;\n",
            "}\n",
            "function getReserves() public view returns (uint256) {\n",
            "return (reserve1);\n",
            "}\n",
            "}\n",
            "##END##\n",
        ];

        let command_response = from_line_array(items);

        assert_eq!(command_response.commands.len(), 1);
        assert_eq!(
            command_response.commands[0],
            Command::Pending("contract LiquidityPool {\naddress public token1;\nconstructor(address _token1) {\ntoken1 = _token1;\n}\nfunction getReserves() public view returns (uint256) {\nreturn (reserve1);\n}\n}\n".to_string())
        );
    }
    #[test]
    fn it_can_split_contract_with_if_statements() {
        let items = vec![
            "##START##\n",
            "contract LiquidityPool {\n",
            "function addLiquidity(uint256 _amount1, uint256 _amount2) public {\n",
            "if (reserve1 == 0 && reserve2 == 0) {\n",
            "}\n",
            "else {\n",
            "}\n",
            "}\n",
            "}\n",
            "##END##\n",
        ];

        let command_response = from_line_array(items);

        assert_eq!(command_response.commands.len(), 1);
        assert_eq!(
            command_response.commands[0],
            Command::Pending("contract LiquidityPool {\nfunction addLiquidity(uint256 _amount1, uint256 _amount2) public {\nif (reserve1 == 0 && reserve2 == 0) {\n}\nelse {\n}\n}\n}\n".to_string())
        );
    }

    #[test]
    fn it_can_split_with_leading_interface() {
        let items = vec![
            "##START##\n",
            "interface IERC20 {\n",
            "}\n",
            "contract LiquidityPool {\n",
            "}\n",
            "##END##\n",
        ];

        let command_response = from_line_array(items);

        assert_eq!(command_response.commands.len(), 2);
        assert_eq!(
            command_response.commands[0],
            Command::Pending("interface IERC20 {\n}\n".to_string())
        );
        assert_eq!(
            command_response.commands[1],
            Command::Pending("contract LiquidityPool {\n}\n".to_string())
        );
    }

    #[test]
    fn it_can_split_contract_with_assembly() {
        let items = vec![
            "##START##\n",
            "contract BitShifter {\n",
            "function shiftLeft(uint256 input) public pure returns (uint256) {\n",
            "uint256 result;\n",
            "assembly {\n",
            "result := shl(2, input)\n",
            "result := shl(2, result)\n",
            "}\n",
            "return result;\n",
            "}\n",
            "}\n",
            "##END##\n",
        ];

        let command_response = from_line_array(items);

        assert_eq!(command_response.commands.len(), 1);
        assert_eq!(
            command_response.commands[0],
            Command::Pending("contract BitShifter {\nfunction shiftLeft(uint256 input) public pure returns (uint256) {\nuint256 result;\nassembly {\nresult := shl(2, input)\nresult := shl(2, result)\n}\nreturn result;\n}\n}\n".to_string())
        );
    }

    // #[test]
    // fn it_can_split_multiline_construct() {
    //     let items = vec![
    //         "##START##\n",
    //         "VRFExample vrf = new VRFExample(\n",
    //         "0x0,\n",
    //         "0x0,\n",
    //         "0x0,\n",
    //         "0x0\n",
    //         ");\n",
    //         "##END##\n",
    //     ];

    //     let command_response = from_line_array(items);

    //     assert_eq!(command_response.commands.len(), 1);
    //     assert_eq!(
    //         command_response.commands[0],
    //         Command::Pending(
    //             "VRFExample vrf = new VRFExample(\n0x0,\n0x0,\n0x0,\n0x0\n);\n".to_string()
    //         )
    //     );
    // }
}
