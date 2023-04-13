use super::foundry_interface::FOUNDRY_INTERFACE;

pub const EXAMPLES: &str = "Here as some examples of what I mean, the request is prefixed with 'Request:' and the response is prefixed with 'Response:'. The responses you send must be in the same format as the examples:

```
Request:
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

Request: !chat fork mainnet and fetch the WETH contract

Response:
##Start##
`!f https://mainnet.infura.io/v3/84842078b09946638c03157f83405213`
`!fe 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2 WETH`
`WETH weth = WETH(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2);`
##End##

Request: Deal me 100 ETH

Response:
##Start##
`vm.deal(address(this))`

Request: !chat can you fork mainnet

Response:
##Start##
`!f https://mainnet.infura.io/v3/84842078b09946638c03157f83405213`
##End##

```";

// Request: !chat can you fork mainnet with this api key 84842078b09946638c03157f83405213

// Response:
// ##Start##
// `!f https://mainnet.infura.io/v3/84842078b09946638c03157f83405213`
// ##End##

pub fn create_context_string_2(help_text: String, chisel_context: String) -> String {
    String::from("I want you to act as ChiselGPT, a translator between native language and Chisel commands. Chisel is a REPL for executing commands in a blockchain environment, inside Chisel it allows all valid solidity syntax and it also some special methods for controlling the environment. When I want you to translate a command for me I will prefix my command with: !chat. 
    
    Here is the chisel documentation for the other environment commands you can execute:
    ") + &help_text + 
    
    "
    
    
    Note that the documentation for some commands lets you use two potential options: !help | !h, preferably use the longer name in these situations; !help, as that is more readable.

    When commands are executed in Chisel they are executed within the context of the 'run' method inside a contract titled 'REPL'. These new commands that are executed are appended to the run function. This is important because you can call re-use variables inside the run function. Here is the entire context of the REPL contract and run method:" + &chisel_context + FOUNDRY_INTERFACE +

    "Here as some examples of some commands that I sent and requests received, note that requests start with '!chat' and I've added Response: to the ChiselGPT responses to show where the response starts. The responses you send must be in the same format as the examples:" + EXAMPLES + 

    "You must return the commands matching the following format: '##Start##' to start writing chisel commands, '##End##' to end the commands and individual commands are wrapped in backticks; `!clear`. The examples follow the formatting so refer to those. Only return the commands, do not return any other information, the returned commands will be passed directly back into Chisel. "
}

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

// !fetch 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2 WETH