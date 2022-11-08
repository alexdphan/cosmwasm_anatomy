// The file src/msg.rs is where different types of messages and responses the smart contract can receive and return are defined.

// use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
}
// Defined in the file /src/msg.rs, the InstantiateMsg is received when an address tries to instantiate a contract on the blockchain through a MsgInstantiateContract message. 
// This message provides the contract with initial configuration and state.
// On CosmWasm, the upload of a contract's code and the instantiation of a contract are regarded as separate events, unlike on Ethereum.
// This message arrives embedded in the received MsgInstantiateContract message during instantiation. (in contract.rs)
// This is to allow a small set of vetted contract archetypes to exist as multiple instances sharing the same base code but be configured with different parameters (imagine one canonical ERC20, and multiple tokens that use its code).

// For our template contract, we will expect a contract creator to supply the initial state in a JSON formatted message as follows:

// {
//   "count": 100 
// }

// This message arrives embedded in the received MsgInstantiateContract message during instantiation.


// Defined in the file /src/msg.rs, an ExecuteMsg is received when an address tries to invoke one of the smart contract functions through a MsgExecuteContract message. 
// Unlike the InstantiateMsg, which was a single struct; the ExecuteMsg is an enumerator, which essentially holds a list of possible execute message structs with different names and attributes to account for the different types of functions that a smart contract can expose to a user. 
// The execute() function demultiplexes these different types of messages and forwards them to the appropriate message handler logic.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
// The line #[serde(rename_all = "snake_case")] performs a snake_case conversion (lowercase initials with an underscore between words) on the field names before serialization and deserialization. So, we'll have increment and reset instead of Increment and Reset when serializing and deserializing across JSON formatted messages.
pub enum ExecuteMsg {
  Increment {},
  Reset { count: i32 },
}
// The ExecuteMsg enum contains the different types of execute messages that our contract can understand.
// At this point, our template contract can accept the following two types of execute messages in JSON format, embedded in a MsgExecuteContract message:

// Increment
    // Any address can utilize the Increment function to increment the current count by 1.

// {
//   "increment": {}
// }

// Reset
    // The owner of the contract can reset the count to an arbitrary number. The check regarding whether a user is the contract owner is a part of the execution logic.

// {
//   "reset": {
//     "count": 5
//   }
// }


// #[cw_serde]
// #[derive(QueryResponses)]
// pub enum QueryMsg {
//     // GetCount returns the current count as a json-encoded number
//     #[returns(GetCountResponse)]
//     GetCount {},
// }

// // We define a custom struct for each query response
// #[cw_serde]
// pub struct GetCountResponse {
//     pub count: i32,
// }
