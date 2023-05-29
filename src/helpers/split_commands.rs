const START_TAG: &str = "##START##";
const END_TAG: &str = "##END##";

type Complete<T> = T;

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
            // if index != current_lines.len() {
            //     // TODO: better fix for this
            //     for line in current_lines.iter().skip(index + 1) {
            //         dbg!(line);
            //     }
            // }
            return Response::Complete(current_command);
        }
    }

    Response::Incomplete(current_command)
}

pub fn split_commands(input: &str) -> Vec<String> {
    let mut commands = Vec::new();
    let mut current_command = String::new();
    let mut open_brackets = 0;
    let mut in_multiline_construct = false;

    if let (Some(start), Some(end)) = (input.find(START_TAG), input.find(END_TAG)) {
        if start < end {
            let stripped_response = &input[start + START_TAG.len()..end];
            let lines: Vec<&str> = stripped_response
                .split('\n')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();

            for line in lines {
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

                current_command.push_str(line);
                current_command.push('\n');

                if open_brackets == 0 && !in_multiline_construct {
                    commands.push(current_command.trim().to_string());
                    current_command.clear();
                }
            }
        }
    }

    commands
}

// #[cfg(test)]
// mod tests {
//     use std::vec;

//     use crate::helpers::split_commands::split_commands;

//     #[test]
//     fn it_can_split_contract_string() {
//         let input = "##START##
//         contract Token {
//           string public constant symbol = 'TKN';
//         }
//       ##END##";

//         assert_eq!(
//             split_commands(input),
//             vec!["contract Token {\nstring public constant symbol = 'TKN';\n}"],
//         );
//     }

//     #[test]
//     fn it_can_split_function_string() {
//         let input = "##START##
//         function test() external {
//           address owner = msg.sender;
//         }
//       ##END##";

//         assert_eq!(
//             split_commands(input),
//             vec!["function test() external {\naddress owner = msg.sender;\n}"],
//         );
//     }

//     #[test]
//     fn it_can_split_multiple_solidity_single_lines() {
//         let input = "##START##
//         address owner = msg.sender;
//         address token1;
//         address token2;
//       ##END##";

//         assert_eq!(
//             split_commands(input),
//             vec![
//                 "address owner = msg.sender;",
//                 "address token1;",
//                 "address token2;"
//             ],
//         );
//     }

//     #[test]
//     fn it_can_split_function_and_method() {
//         let input = "##START##
//       contract Token {
//         string public constant symbol = 'TKN';
//         string public constant name = 'TKN';
//       }
//       function test() external {
//           address owner = msg.sender;
//       }
//     ##END##";

//         assert_eq!(
//             split_commands(input),
//             vec![
//                 "contract Token {\nstring public constant symbol = 'TKN';\nstring public constant name = 'TKN';\n}",
//                 "function test() external {\naddress owner = msg.sender;\n}"
//             ],
//         );
//     }

//     #[test]
//     fn it_can_split_single_lines_and_contract() {
//         let input = "##START##
//         !clear
//         contract Token {
//           string public constant symbol = 'TKN';
//           string public constant name = 'TKN';
//         }
//         !clear
//     ##END##";

//         assert_eq!(
//             split_commands(input),
//             vec![
//                 "!clear", "contract Token {\nstring public constant symbol = 'TKN';\nstring public constant name = 'TKN';\n}",
//                 "!clear"
//             ],
//         );
//     }

//     #[test]
//     fn it_can_split_contract_with_methods() {
//         let input = "##START##
//       contract LiquidityPool {
//           address public token1;

//           function addLiquidity(uint256 _amount1, uint256 _amount2) public {
//             uint256 total = _amount + _amount2;
//           }
//       }
//       ##END##";

//         assert_eq!(
//         split_commands(input),
//         vec![
//             "contract LiquidityPool {\naddress public token1;\nfunction addLiquidity(uint256 _amount1, uint256 _amount2) public {\nuint256 total = _amount + _amount2;\n}\n}"
//         ],
//     );
//     }

//     #[test]
//     fn it_can_split_contract_and_interface() {
//         let input = "##START##
//       contract LiquidityPool {
//           address public token1;

//           function addLiquidity(uint256 _amount1, uint256 _amount2) public {
//             uint256 total = _amount + _amount2;
//           }
//       }

//       interface IERC20 {
//           function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
//       }
//       ##END##";

//         assert_eq!(
//         split_commands(input),
//         vec![
//             "contract LiquidityPool {\naddress public token1;\nfunction addLiquidity(uint256 _amount1, uint256 _amount2) public {\nuint256 total = _amount + _amount2;\n}\n}", "interface IERC20 {\nfunction transferFrom(address sender, address recipient, uint256 amount) external returns (bool);\n}"
//         ],
//     );
//     }

//     #[test]
//     fn it_can_split_contract_with_constructor_and_function() {
//         let input = "##START##
//       contract LiquidityPool {
//           address public token1;

//           constructor(address _token1) {
//               token1 = _token1;
//           }

//           function getReserves() public view returns (uint256) {
//               return (reserve1);
//           }
//       }
//       ##END##";

//         assert_eq!(split_commands(input), vec!["contract LiquidityPool {\naddress public token1;\nconstructor(address _token1) {\ntoken1 = _token1;\n}\nfunction getReserves() public view returns (uint256) {\nreturn (reserve1);\n}\n}"]);
//     }

//     #[test]
//     fn it_can_split_contract_with_if_statements() {
//         let input = "##START##
//       contract LiquidityPool {
//           function addLiquidity(uint256 _amount1, uint256 _amount2) public {
//               if (reserve1 == 0 && reserve2 == 0) {
//               } else {
//               }
//           }
//       }
//       ##END##";

//         assert_eq!(split_commands(input), vec!["contract LiquidityPool {\nfunction addLiquidity(uint256 _amount1, uint256 _amount2) public {\nif (reserve1 == 0 && reserve2 == 0) {\n} else {\n}\n}\n}"]);
//     }

//     #[test]
//     fn it_can_split_with_leading_interface() {
//         let input = "##START##
//         interface IERC20 {
//         }

//         contract LiquidityPool {
//         }
//     ##END##";

//         assert_eq!(
//             split_commands(input),
//             vec!["interface IERC20 {\n}", "contract LiquidityPool {\n}"]
//         );
//     }

//     #[test]
//     fn it_can_split_contract_with_assembly() {
//         let input = "##START##
//       contract BitShifter {
//         function shiftLeft(uint256 input) public pure returns (uint256) {
//         uint256 result;
//         assembly {
//         result := shl(2, input)
//         result := shl(2, result)
//         }
//         return result;
//         }
//       }
//       ##END##";

//         assert_eq!(split_commands(input).len(), 1);
//     }

//     #[test]
//     fn it_can_split_multiline_construct() {
//         let input = "##START##
//       VRFExample vrf = new VRFExample(
//         0x0,
//         0x0,
//         0x0,
//         0x0
//       );
//       ##END##";

//         assert_eq!(split_commands(input).len(), 1);
//     }
// }
