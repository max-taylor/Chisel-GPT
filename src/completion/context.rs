use super::foundry_interface::FOUNDRY_INTERFACE;

pub fn create_context_string(help_text: String, chisel_context: String) -> String {
    String::from("
  This prompt is designed to help you convert natural language text into Chisel commands and/or Solidity code. Follow the guidelines provided below and use the examples as a reference for your conversions:

  1. Use '##START##' to mark the start of commands and '##END##' to mark the end of the commands.
  2. Chisel commands are accessed with the exclamation mark prefix.
  3. When writing Solidity code, it is appended to the Chisel session source code.

  Here is the documentation for Chisel commands: ") +
    &help_text + "

  When you write Solidity code, it is appended to the Chisel session source code as follows:

    - If you write a function block, it is added into the REPL contract code.
    - If you write single line Solidity code, it is added to the run() method inside the REPL contract.

  You can use variables inside the REPL contract or variables declared inside the run() method of that contract, ensure when prompted you prioritize those variables instead of creating new ones. When you create new variables the names must be unique, so you must ensure that another variable inside the REPL contract or inside the run() method of that contract doesn't exist.
  
  And here is the current Chisel session source code: " +
    &chisel_context + 
    &FOUNDRY_INTERFACE +
  "

  Please provide a clear and concise output to perform the intended action in a blockchain environment using Chisel and Solidity.

  Examples of expected output:

  1. Input: !chat Create a new contract called 'Token' with a symbol 'TKN', total supply of 1000000, and 18 decimals. Then, reset the current Chisel session.
     Output: ##START##
     !clear
     contract Token { string public constant symbol = 'TKN'; uint256 public constant totalSupply = 1000000 * 10**18; uint8 public constant decimals = 18; }
     !clear
     ##END##

  2. Input: !chat Write a function to set a new value for a given variable.
     Output: ##START##function setValue(uint256 _newValue) public { value = _newValue; }##END##

  3. Input: !chat Initialize a variable 'owner' with the address deploying the contract.
     Output: ##START##
     address public owner;
     constructor() { owner = msg.sender; }
     ##END##

  4. Input: !chat Reset the current Chisel session and show Chisel documentation.
     Output: ##START##
     !clear
     !help
     ##END##

  5. Input: !chat create a contract that uses some assembly to bit shift two numbers and call the method
     Output:
     ##START##
     contract BitShifter {
         function shiftLeft(uint256 input) public pure returns (uint256) {
           uint256 result;
             assembly {
                 result := shl(2, input)
                 result := shl(2, result)
             }
             return result;
         }
     }
     BitShifter instance = new BitShifter();
     instance.shiftLeft(1234);
     ##END##

  6. Input: !chat fork mainnet and fetch the WETH contract
     Output:
     ##START##
     !f https://mainnet.infura.io/v3/84842078b09946638c03157f83405213
     !fe 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2 WETH
     WETH weth = WETH(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2);
     ##END##

  7. Input: !chat Deal me 100 ETH
    Output:
    ##START##
    vm.deal(address(this))
    ##END##

  8. Input: !chat using the local variable myERC20, transfer some tokens
    Output:
    ##START##
    myERC20.transfer(address(this), 50);
    ##END##
  
  Remember it is extremely important you use '##START##' to mark the start of commands and '##END##' to mark the end of the commands.
  "
}
