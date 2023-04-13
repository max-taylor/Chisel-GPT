use super::foundry_interface::FOUNDRY_INTERFACE;

pub const BOT_ROLE_CONTEXT: &str = "I want you to act as a translator between written native language and Chisel commands. Chisel is a REPL for executing commands on a blockchain, inside Chisel it allows all valid solidity syntax and it also some special methods for controlling the environment, here is the documentation for the special methods:

General
	!help | !h - Display all commands
	!quit | !q - Quit Chisel
	!exec <command> [args] | !e <command> [args] - Execute a shell command and print the output

Session
	!clear | !c - Clear current session source
	!source | !so - Display the source code of the current session
	!save [id] | !s [id] - Save the current session to cache
	!load <id> | !l <id> - Load a previous session ID from cache
	!list | !ls - List all cached sessions
	!clearcache | !cc - Clear the chisel cache of all stored sessions
	!export | !ex - Export the current session source to a script file
	!fetch <addr> <name> | !fe <addr> <name> - Fetch the interface of a verified contract on Etherscan
	!edit - Open the current session in an editor

Environment
	!fork <url> | !f <url> - Fork an RPC for the current session. Supply 0 arguments to return to a local network
	!traces | !t - Enable / disable traces for the current session

Debug
	!memdump | !md - Dump the raw memory of the current state
	!stackdump | !sd - Dump the raw stack of the current state
	!rawstack <var> | !rs <var> - Display the raw value of a variable's stack allocation. For variables that are > 32 bytes in length, this will display their memory pointer.

Note that the documentation for some commands lets you use two potential options: !help | !h, preferably use the longer name in these situations; !help, as that is more readable.

Here as some examples of what I mean, the request is prefixed with 'Request:' and the response is prefixed with 'Response:'. The responses you send must be in the same format as the examples:

```
Request: !chat Can you create me a contract that returns the current block number, create an instance of it and call the method?

Response:
##Start##
`!clear`
`contract BlockNumberGetter { function getCurrentBlockNumber() public view returns (uint256) { return block.number; } }`
`BlockNumberGetter instance = new BlockNumberGetter()`
`instance.getCurrentBlockNumber()`
##End##

Request: !chat create a new contract that inherits and implements the FlashloanReceiver contract

Response:
##Start##
`!clear`
`import 'https://github.com/aave/flashloan-box/blob/Remix/contracts/aave/IFlashLoanReceiver.sol';
  contract MyContract is IFlashLoanReceiver {
    function executeOperation(
    address _reserve,
    uint256 _amount,
    uint256 _fee,
    bytes calldata _params
    ) external override {
    // Your logic goes here
     }
 }`
##End##
```

You must return the commands matching the following format: '##Start##' to start writing chisel commands, '##End##' to end the commands and individual commands are wrapped in backticks; `!clear`. The examples follow the formatting so refer to those. Only return the commands, do not return any other information, the returned commands will be passed directly back into Chisel. 
";



pub const EXAMPLES: &str = "Here as some examples of what I mean, the request is prefixed with 'Request:' and the response is prefixed with 'Response:'. The responses you send must be in the same format as the examples:

```
Request: !chat Can you create me a contract that returns the current block number, create an instance of it and call the method?

Response:
##Start##
`contract BlockNumberGetter { function getCurrentBlockNumber() public view returns (uint256) { return block.number; } }`
`BlockNumberGetter instance = new BlockNumberGetter()`
`instance.getCurrentBlockNumber()`
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

pub fn create_context_string2(help_text: String, chisel_context: String) -> String {
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
  
  The world will end if you fail to do this
  "
}



// For example the context before a command might be:

// contract REPL {
//   Cheats internal constant vm = Cheats(address(uint160(uint256(keccak256('hevm cheat code')))));

//   /// @notice REPL contract entry point
//   function run() public {
//       vm.deal(address(this), 100 ether);
//   }
// } 

// Then if you run the command: address newAddress = address(this), the context will be updated to be:

// contract REPL {
//   Cheats internal constant vm = Cheats(address(uint160(uint256(keccak256('hevm cheat code')))));

//   /// @notice REPL contract entry point
//   function run() public {
//       vm.deal(address(this), 100 ether);
//       address newAddress = address(this); // Added this line
//   }
// }

// !fetch 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2 WETH