// use cosmwasm_schema::write_api;

// use my_first_cosmwasm_contract::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// fn main() {
//     write_api! {
//         instantiate: InstantiateMsg,
//         execute: ExecuteMsg,
//         query: QueryMsg,
//     }
// }

// examples/schema.rs

use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

// changed the name from my_first_cosmwasm_contract to cosmwasm_anaotmy
use cosmwasm_anatomy::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use cosmwasm_anatomy::state::State;

fn main() {
  let mut out_dir = current_dir().unwrap();
  out_dir.push("schema");
  create_dir_all(&out_dir).unwrap();
  remove_schemas(&out_dir).unwrap();

  export_schema(&schema_for!(InstantiateMsg), &out_dir);
  export_schema(&schema_for!(ExecuteMsg), &out_dir);
  export_schema(&schema_for!(QueryMsg), &out_dir);
  export_schema(&schema_for!(State), &out_dir);
  export_schema(&schema_for!(CountResponse), &out_dir);
}