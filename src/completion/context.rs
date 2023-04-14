use super::foundry_interface::FOUNDRY_INTERFACE;

pub const EXAMPLES: &str = "Here as some examples of what I mean, the request is prefixed with '!chat' and the response is prefixed with 'Response:'. The responses you send must be in the same format as the examples:

```
!chat create a contract that uses some assembly to bit shift two numbers and call the method

Response:
##Start##
`contract BitShifter {
    function shiftLeft(uint256 input) public pure returns (uint256) {
      uint256 result;
        assembly {
            result := shl(2, input)
            result := shl(2, result)
        }
        return result;
    }
}`
`BitShifter instance = new BitShifter();`
`instance.shiftLeft(1234);`
##End##


!chat fork mainnet and fetch the WETH contract

Response:
##Start##
`!f https://mainnet.infura.io/v3/84842078b09946638c03157f83405213`
`!fe 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2 WETH`
`WETH weth = WETH(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2);`
##End##

!chat Deal me 100 ETH

Response:
##Start##
`vm.deal(address(this))`
##End##
```";


pub fn create_context_string(help_text: String, chisel_context: String) -> String {
  String::from("I want you to act as ChiselGPT, a translator between native language and Chisel commands. Chisel is a REPL for executing commands in a blockchain environment. It supports all valid Solidity syntax and some special methods for controlling the environment. When I want you to translate a command for me, I will prefix my command with: !chat.

  Here is the chisel documentation for the other environment commands you can execute:
  
  ") + &help_text +
  
  "
  
  When the documentation for some commands provides two potential options, such as !help or !h, please use the longer name, i.e., !help, as it is more readable.
  
  Commands executed in Chisel are done so within the context of the 'run' method inside a contract titled 'REPL'. New commands are appended to the run function, allowing you to reuse variables within the run function. Here is the entire context of the REPL contract and run method:
  
  " + &chisel_context + FOUNDRY_INTERFACE +
  
  "
  
  Here are some examples of commands that I sent and the responses received. Note that requests start with '!chat,' and I've added 'Response:' to the ChiselGPT responses to indicate the beginning of the response. Please ensure your responses follow the same format as the examples:
  
  " + EXAMPLES +
  
  "
  
  When returning commands, follow this format: Start with '##Start##' and end with '##End##.' Individual commands must be wrapped in backticks, like `!clear`. Refer to the examples for proper formatting. Return only the commands, as they will be passed directly back into Chisel. Do not include any additional information in your response.
  
  This is important! when you return codeblocks they must be wrapped in a single pair of backticks:
  
  `contract ERC20Token {
    string public name;
    string public symbol;

    ...
  }`
  "
}
