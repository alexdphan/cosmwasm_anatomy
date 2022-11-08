// The file src/msg.rs is where different types of messages and responses the smart contract can receive and return are defined.

use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
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

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}
