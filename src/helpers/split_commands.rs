use yansi::Paint;

const START_TAG: &str = "##START##";
const END_TAG: &str = "##END##";

pub fn split_commands(input: &str) -> Vec<String> {
    println!("{}", Paint::green(input));

    let mut commands = Vec::new();

    // TODO: If Some(start) but no END, throw an error, this means the response size was too large, long-term we could query ChatGPT for the rest of the response and join them
    if let (Some(start), Some(end)) = (input.find(START_TAG), input.find(END_TAG)) {
        // Only look for commands if start is greater than the end
        if start < end {
            let stripped_response = &input[start + START_TAG.len()..end];
            let lines: Vec<&str> = stripped_response
                .split('\n')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();

            let mut nested_total = 0;
            let mut current_command = String::new();
            let mut within_interface = false;

            for line in lines {
                // May need to check for this later on, can throw an error if not equal to current: if line.starts_with("pragma") {}
                if !within_interface
                    && (line.starts_with("contract")
                        || line.starts_with("function")
                        || line.starts_with("constructor")
                        || line.starts_with("if")
                        || line.starts_with("else"))
                {
                    nested_total += 1;
                } else if line.starts_with("interface") {
                    within_interface = true;
                    nested_total += 1;
                }

                if nested_total > 0 {
                    current_command.push_str(line);
                    current_command.push_str("\n");

                    if line.ends_with("}") {
                        nested_total -= 1;

                        if nested_total == 0 {
                            commands.push(current_command.clone());
                            current_command.clear();
                            within_interface = false; // Reset this
                        }
                    }
                } else {
                    commands.push(line.to_string());
                }
            }
        }
    }

    commands
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::helpers::split_commands::split_commands;

    #[test]
    fn it_can_split_contract_string() {
        let input = "##START##
        contract Token {
          string public constant symbol = 'TKN';
        }
      ##END##";

        assert_eq!(
            split_commands(input),
            vec!["contract Token {\nstring public constant symbol = 'TKN';\n}\n"],
        );
    }

    #[test]
    fn it_can_split_function_string() {
        let input = "##START##
        function test() external {
          address owner = msg.sender;
        }
      ##END##";

        assert_eq!(
            split_commands(input),
            vec!["function test() external {\naddress owner = msg.sender;\n}\n"],
        );
    }

    #[test]
    fn it_can_split_multiple_solidity_single_lines() {
        let input = "##START##
        address owner = msg.sender;
        address token1;
        address token2;
      ##END##";

        assert_eq!(
            split_commands(input),
            vec![
                "address owner = msg.sender;",
                "address token1;",
                "address token2;"
            ],
        );
    }

    #[test]
    fn it_can_split_function_and_method() {
        let input = "##START##
      contract Token {
        string public constant symbol = 'TKN';
        string public constant name = 'TKN';
      }
      function test() external {
          address owner = msg.sender;
      }
    ##END##";

        assert_eq!(
            split_commands(input),
            vec![
                "contract Token {\nstring public constant symbol = 'TKN';\nstring public constant name = 'TKN';\n}\n",
                "function test() external {\naddress owner = msg.sender;\n}\n"
            ],
        );
    }

    #[test]
    fn it_can_split_single_lines_and_contract() {
        let input = "##START##
        !clear
        contract Token {
          string public constant symbol = 'TKN';
          string public constant name = 'TKN';
        }
        !clear
    ##END##";

        assert_eq!(
            split_commands(input),
            vec![
                "!clear", "contract Token {\nstring public constant symbol = 'TKN';\nstring public constant name = 'TKN';\n}\n",
                "!clear"
            ],
        );
    }

    #[test]
    fn it_can_split_contract_with_methods() {
        let input = "##START##
      contract LiquidityPool {
          address public token1;
      
          function addLiquidity(uint256 _amount1, uint256 _amount2) public {
            uint256 total = _amount + _amount2;
          }
      }
      ##END##";

        assert_eq!(
        split_commands(input),
        vec![
            "contract LiquidityPool {\naddress public token1;\nfunction addLiquidity(uint256 _amount1, uint256 _amount2) public {\nuint256 total = _amount + _amount2;\n}\n}\n"
        ],
    );
    }

    #[test]
    fn it_can_split_contract_and_interface() {
        let input = "##START##
      contract LiquidityPool {
          address public token1;
      
          function addLiquidity(uint256 _amount1, uint256 _amount2) public {
            uint256 total = _amount + _amount2;
          }
      }
      
      interface IERC20 {
          function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
      }
      ##END##";

        assert_eq!(
        split_commands(input),
        vec![
            "contract LiquidityPool {\naddress public token1;\nfunction addLiquidity(uint256 _amount1, uint256 _amount2) public {\nuint256 total = _amount + _amount2;\n}\n}\n", "interface IERC20 {\nfunction transferFrom(address sender, address recipient, uint256 amount) external returns (bool);\n}\n"
        ],
    );
    }

    #[test]
    fn it_can_split_contract_with_constructor_and_function() {
        let input = "##START##
      contract LiquidityPool {
          address public token1;
      
          constructor(address _token1) {
              token1 = _token1;
          }
      
          function getReserves() public view returns (uint256) {
              return (reserve1);
          }
      }
      ##END##";

        assert_eq!(split_commands(input), vec!["contract LiquidityPool {\naddress public token1;\nconstructor(address _token1) {\ntoken1 = _token1;\n}\nfunction getReserves() public view returns (uint256) {\nreturn (reserve1);\n}\n}\n"]);
    }

    #[test]
    fn it_can_split_contract_with_if_statements() {
        let input = "##START##
      contract LiquidityPool {
          function addLiquidity(uint256 _amount1, uint256 _amount2) public {
              if (reserve1 == 0 && reserve2 == 0) {
              } else {
              }
          }
      }
      ##END##";

        assert_eq!(split_commands(input), vec!["contract LiquidityPool {\nfunction addLiquidity(uint256 _amount1, uint256 _amount2) public {\nif (reserve1 == 0 && reserve2 == 0) {\n} else {\n}\n}\n}\n"]);
    }

    #[test]
    fn it_can_split_with_leading_interface() {
        let input = "##START##
        interface IERC20 {
        }

        contract LiquidityPool {
        }
    ##END##";

        assert_eq!(
            split_commands(input),
            vec!["interface IERC20 {\n}\n", "contract LiquidityPool {\n}\n"]
        );
    }
}
