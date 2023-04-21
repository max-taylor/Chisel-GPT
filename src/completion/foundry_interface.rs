// Can only fit a subset of methods from this interface, due to its size

pub const FOUNDRY_INTERFACE: &str = "
The session imports the Cheats interface from import forge-std/Vm.sol, here is the interface:

interface Cheats {
// Sets block.timestamp
function warp(uint256 newTimestamp) external;
// Sets block.height
function roll(uint256 newHeight) external;
// Sets block.basefee
function fee(uint256 newBasefee) external;
// Sets block.chainid
function chainId(uint256 newChainId) external;
// Sets tx.gasprice
function txGasPrice(uint256 newGasPrice) external;
// Sets the nonce of an account; must be higher than the current nonce of the account
function setNonce(address account, uint64 newNonce) external;
// Sets all calls' msg.sender to be the input address, until `stopPrank` is called
function startPrank(address msgSender) external;
// Resets subsequent calls' msg.sender to be `address(this)`
function stopPrank() external;
// Sets an address' balance
function deal(address account, uint256 newBalance) external;
}

Note that you can call methods on this interface using the global value 'Cheats internal constant vm', i.e; vm.deal(...)

";

// // The entire interface
// const FOUNDRY_INTERFACE_ENTIRE: &str = "
// The session imports the Cheats interface from import forge-std/Vm.sol, here is the interface:

// interface Cheats {
//   // Sets block.timestamp
//   function warp(uint256 newTimestamp) external;
//   // Sets block.height
//   function roll(uint256 newHeight) external;
//   // Sets block.basefee
//   function fee(uint256 newBasefee) external;
//   // Sets block.difficulty
//   function difficulty(uint256 newDifficulty) external;
//   // Sets block.chainid
//   function chainId(uint256 newChainId) external;
//   // Sets tx.gasprice
//   function txGasPrice(uint256 newGasPrice) external;
//   // Stores a value to an address' storage slot.
//   function store(address target, bytes32 slot, bytes32 value) external;
//   // Sets the nonce of an account; must be higher than the current nonce of the account
//   function setNonce(address account, uint64 newNonce) external;
//   // Sets the *next* call's msg.sender to be the input address
//   function prank(address msgSender) external;
//   // Sets all subsequent calls' msg.sender to be the input address until `stopPrank` is called
//   function startPrank(address msgSender) external;
//   // Sets the *next* call's msg.sender to be the input address, and the tx.origin to be the second input
//   function prank(address msgSender, address txOrigin) external;
//   // Sets all subsequent calls' msg.sender to be the input address until `stopPrank` is called, and the tx.origin to be the second input
//   function startPrank(address msgSender, address txOrigin) external;
//   // Resets subsequent calls' msg.sender to be `address(this)`
//   function stopPrank() external;
//   // Sets an address' balance
//   function deal(address account, uint256 newBalance) external;
//   // Sets an address' code
//   function etch(address target, bytes calldata newRuntimeBytecode) external;
//   // Expects an error on next call
//   function expectRevert(bytes calldata revertData) external;
//   function expectRevert(bytes4 revertData) external;
//   function expectRevert() external;

//   // Prepare an expected log with all four checks enabled.
//   // Call this function, then emit an event, then call a function. Internally after the call, we check if
//   // logs were emitted in the expected order with the expected topics and data.
//   // Second form also checks supplied address against emitting contract.
//   function expectEmit() external;
//   function expectEmit(address emitter) external;

//   // Prepare an expected log with (bool checkTopic1, bool checkTopic2, bool checkTopic3, bool checkData).
//   // Call this function, then emit an event, then call a function. Internally after the call, we check if
//   // logs were emitted in the expected order with the expected topics and data (as specified by the booleans).
//   // Second form also checks supplied address against emitting contract.
//   function expectEmit(bool checkTopic1, bool checkTopic2, bool checkTopic3, bool checkData) external;
//   function expectEmit(bool checkTopic1, bool checkTopic2, bool checkTopic3, bool checkData, address emitter)
//       external;

//   // Mocks a call to an address, returning specified data.
//   // Calldata can either be strict or a partial match, e.g. if you only
//   // pass a Solidity selector to the expected calldata, then the entire Solidity
//   // function will be mocked.
//   function mockCall(address callee, bytes calldata data, bytes calldata returnData) external;
//   // Mocks a call to an address with a specific msg.value, returning specified data.
//   // Calldata match takes precedence over msg.value in case of ambiguity.
//   function mockCall(address callee, uint256 msgValue, bytes calldata data, bytes calldata returnData) external;
//   // Reverts a call to an address with specified revert data.
//   function mockCallRevert(address callee, bytes calldata data, bytes calldata revertData) external;
//   // Reverts a call to an address with a specific msg.value, with specified revert data.
//   function mockCallRevert(address callee, uint256 msgValue, bytes calldata data, bytes calldata revertData)
//       external;
//   // Clears all mocked calls
//   function clearMockedCalls() external;
//   // Expects a call to an address with the specified calldata.
//   // Calldata can either be a strict or a partial match
//   function expectCall(address callee, bytes calldata data) external;
//   // Expects a call to an address with the specified msg.value and calldata
//   function expectCall(address callee, uint256 msgValue, bytes calldata data) external;
//   // Expect a call to an address with the specified msg.value, gas, and calldata.
//   function expectCall(address callee, uint256 msgValue, uint64 gas, bytes calldata data) external;
//   // Expect a call to an address with the specified msg.value and calldata, and a *minimum* amount of gas.
//   function expectCallMinGas(address callee, uint256 msgValue, uint64 minGas, bytes calldata data) external;
//   // Only allows memory writes to offsets [0x00, 0x60) ∪ [min, max) in the current subcontext. If any other
//   // memory is written to, the test will fail. Can be called multiple times to add more ranges to the set.
//   function expectSafeMemory(uint64 min, uint64 max) external;
//   // Only allows memory writes to offsets [0x00, 0x60) ∪ [min, max) in the next created subcontext.
//   // If any other memory is written to, the test will fail. Can be called multiple times to add more ranges
//   // to the set.
//   function expectSafeMemoryCall(uint64 min, uint64 max) external;
//   // Sets block.coinbase
//   function coinbase(address newCoinbase) external;
//   // Snapshot the current state of the evm.
//   // Returns the id of the snapshot that was created.
//   // To revert a snapshot use `revertTo`
//   function snapshot() external returns (uint256 snapshotId);
//   // Revert the state of the EVM to a previous snapshot
//   // Takes the snapshot id to revert to.
//   // This deletes the snapshot and all snapshots taken after the given snapshot id.
//   function revertTo(uint256 snapshotId) external returns (bool success);
//   // Creates a new fork with the given endpoint and block and returns the identifier of the fork
//   function createFork(string calldata urlOrAlias, uint256 blockNumber) external returns (uint256 forkId);
//   // Creates a new fork with the given endpoint and the _latest_ block and returns the identifier of the fork
//   function createFork(string calldata urlOrAlias) external returns (uint256 forkId);
//   // Creates a new fork with the given endpoint and at the block the given transaction was mined in, replays all transaction mined in the block before the transaction,
//   // and returns the identifier of the fork
//   function createFork(string calldata urlOrAlias, bytes32 txHash) external returns (uint256 forkId);
//   // Creates _and_ also selects a new fork with the given endpoint and block and returns the identifier of the fork
//   function createSelectFork(string calldata urlOrAlias, uint256 blockNumber) external returns (uint256 forkId);
//   // Creates _and_ also selects new fork with the given endpoint and at the block the given transaction was mined in, replays all transaction mined in the block before
//   // the transaction, returns the identifier of the fork
//   function createSelectFork(string calldata urlOrAlias, bytes32 txHash) external returns (uint256 forkId);
//   // Creates _and_ also selects a new fork with the given endpoint and the latest block and returns the identifier of the fork
//   function createSelectFork(string calldata urlOrAlias) external returns (uint256 forkId);
//   // Takes a fork identifier created by `createFork` and sets the corresponding forked state as active.
//   function selectFork(uint256 forkId) external;
//   /// Returns the identifier of the currently active fork. Reverts if no fork is currently active.
//   function activeFork() external view returns (uint256 forkId);
//   // Updates the currently active fork to given block number
//   // This is similar to `roll` but for the currently active fork
//   function rollFork(uint256 blockNumber) external;
//   // Updates the currently active fork to given transaction
//   // this will `rollFork` with the number of the block the transaction was mined in and replays all transaction mined before it in the block
//   function rollFork(bytes32 txHash) external;
//   // Updates the given fork to given block number
//   function rollFork(uint256 forkId, uint256 blockNumber) external;
//   // Updates the given fork to block number of the given transaction and replays all transaction mined before it in the block
//   function rollFork(uint256 forkId, bytes32 txHash) external;
//   // Marks that the account(s) should use persistent storage across fork swaps in a multifork setup
//   // Meaning, changes made to the state of this account will be kept when switching forks
//   function makePersistent(address account) external;
//   function makePersistent(address account0, address account1) external;
//   function makePersistent(address account0, address account1, address account2) external;
//   function makePersistent(address[] calldata accounts) external;
//   // Revokes persistent status from the address, previously added via `makePersistent`
//   function revokePersistent(address account) external;
//   function revokePersistent(address[] calldata accounts) external;
//   // Returns true if the account is marked as persistent
//   function isPersistent(address account) external view returns (bool persistent);
//   // In forking mode, explicitly grant the given address cheatcode access
//   function allowCheatcodes(address account) external;
//   // Fetches the given transaction from the active fork and executes it on the current state
//   function transact(bytes32 txHash) external;
//   // Fetches the given transaction from the given fork and executes it on the current state
//   function transact(uint256 forkId, bytes32 txHash) external;
// }

// Note that you can call methods on this interface using the global value 'Cheats internal constant vm', i.e; vm.deal(...)

// ";
