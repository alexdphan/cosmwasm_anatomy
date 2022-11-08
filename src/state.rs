// The file src/state.rs defines how the smart contract state data is represented and the way it will be stored.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr, // The type Addr represents a human-readable Bech32 address with a wasm prefix.
} // hold both count and owner

// derive attribute is used to auto-implement traits
    // Serialize: provides serialization
    // Deserialize: provides deserialization
    // Clone: makes our struct copyable
    // Debug: enables our struct to be printed to string
    // PartialEq: gives us equality comparison
    // JsonSchema: auto-generates a JSON schema for us

// Serialize and Deserialize are used to convert our struct to and from JSON 
// Data can only be persisted as a raw byte array, so any notion of structure or data-type must be expressed as a pair of serializing and deserializing functions. For instance, objects must be stored as bytes, so you must supply both the function that encodes the object into bytes to save it on the blockchain, and the function that decodes the bytes back into the data-types that your contract logic can understand. The choice of byte representation is up to you, so long as it provides a clean, bi-directional mapping.

pub const STATE: Item<State> = Item::new("state"); // "state" is used as a prefix


