// The file src/contract.rs contains the main smart contract logic and is where the functions instantiate(), execute() and query() are implemented.

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, CountResponse};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cosmowasm_anatomy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
//An overview of function parameters:

  //"deps" allows us to perform storage related actions, validate addresses and query other smart contracts
  //"_env" is mainly used to access details about the current state of the blockchain (i.e., block height, time, chain id) 
  //"info" provides access to the message metadata (i.e., sender address, the amount and type of funds)
  //"msg" is the MsgInstantiateContract payload, which comprises the data received from the contract creator in JSON format that conforms to the InstantiateMsg struct

//Introduce a new variable named `state` of type `State`
    let state = State {
        //the value for count in the received message is assigned to the variable `count` of the `State` struct
        count: msg.count,
        //the sender address of the MsgInstantiateContract is assigned to the variable `owner` of the `State` struct
        owner: info.sender.clone(),
    };
    //Store the contract name and version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    //Store the initial state of the contract
    STATE.save(deps.storage, &state)?;

    //Form and return an Ok(Response)
    //The attributes will be included in the JSON formatted response message
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

// Defined in the file /src/contract.rs, the execute() function uses Rust's pattern matching to route the received ExecuteMsg to the appropriate handling logic, by either routing to the function try_increment() or try_reset() depending on the type of message received.
// basically, the execute() function is a router that routes the received message to the appropriate function
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
    ExecuteMsg::Increment {} => try_increment(deps),
    ExecuteMsg::Reset { count } => try_reset(deps, info, count),
  }
}

// It is quite straightforward to follow the logic of try_increment(). We acquire a mutable reference to the storage to update the item located at key "state", made accessible through the STATE convenience function defined in the src/state.rs. 
// We then update the present state's count by returning an Ok result with the new state. Finally, we terminate the contract's execution with an acknowledgement of success by returning an Ok result with the default Response.
pub fn try_increment(deps: DepsMut) -> Result<Response, ContractError> { // state is mutable and definied intially in state.rs
  STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
    state.count += 1;
    Ok(state)
  })?;

  Ok(Response::new().add_attribute("method", "try_increment"))
}
// In this example, the default Response is used for simplicity. However, the Response can be manually created to provide the following information:
    // messages: A list of messages. This is how smart contracts execute other smart contract functions or use native modules.
    // attributes: A list of key-value pairs to define emitted SDK events that can be subscribed to and parsed by clients.
    // events: Extra, custom events separate from the main wasm one. These will have wasm- prepended to the type. These can be used by explorers and applications to report important events or state changes that occurred during the execution.
    // data: additional data that the contract returns to the client.

// The logic for try_reset() is very similar to increment — except this time, we first check that the message sender is permitted to invoke the reset function. 
// Please, observe the use of ContractError::Unauthorized {} to return an error if the sender is not the owner of the contract. Custom error messages can be defined in the file /src/error.rs.
pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
  STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
    if info.sender != state.owner {
      return Err(ContractError::Unauthorized {});
    }
    state.count = count;
    Ok(state)
  })?;
  Ok(Response::new().add_attribute("method", "reset"))
}

// Query Logic
// The logic for query() is similar to that of execute(), except the fact that the query() function is called without the need of making a transaction by the end-user. 
// Therefore, the argument info can be omitted in the query() function signature as there is no message information present to be processed.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
  match msg {
    // Match and route the query message to the appropriate handler
    QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
    // Return the response in byte-array format
  }
}
fn query_count(deps: Deps) -> StdResult<CountResponse> {
  let state = STATE.load(deps.storage)?;
  // Load the current contract state
  Ok(CountResponse { count: state.count })
  // Form and return a CountResponse
}



// pub mod execute {
//     use super::*;

//     pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
//         STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
//             state.count += 1;
//             Ok(state)
//         })?;

//         Ok(Response::new().add_attribute("action", "increment"))
//     }

//     pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
//         STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
//             if info.sender != state.owner {
//                 return Err(ContractError::Unauthorized {});
//             }
//             state.count = count;
//             Ok(state)
//         })?;
//         Ok(Response::new().add_attribute("action", "reset"))
//     }
// }

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
//     match msg {
//         QueryMsg::GetCount {} => to_binary(&query::count(deps)?),
//     }
// }

// pub mod query {
//     use super::*;

//     pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
//         let state = STATE.load(deps.storage)?;
//         Ok(GetCountResponse { count: state.count })
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
//     use cosmwasm_std::{coins, from_binary};

//     #[test]
//     fn proper_initialization() {
//         let mut deps = mock_dependencies();

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(1000, "earth"));

//         // we can just call .unwrap() to assert this was a success
//         let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
//         assert_eq!(0, res.messages.len());

//         // it worked, let's query the state
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: GetCountResponse = from_binary(&res).unwrap();
//         assert_eq!(17, value.count);
//     }

//     #[test]
//     fn increment() {
//         let mut deps = mock_dependencies();

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // beneficiary can release it
//         let info = mock_info("anyone", &coins(2, "token"));
//         let msg = ExecuteMsg::Increment {};
//         let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // should increase counter by 1
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: GetCountResponse = from_binary(&res).unwrap();
//         assert_eq!(18, value.count);
//     }

//     #[test]
//     fn reset() {
//         let mut deps = mock_dependencies();

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // beneficiary can release it
//         let unauth_info = mock_info("anyone", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
//         match res {
//             Err(ContractError::Unauthorized {}) => {}
//             _ => panic!("Must return unauthorized error"),
//         }

//         // only the original creator can reset the counter
//         let auth_info = mock_info("creator", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

//         // should now be 5
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: GetCountResponse = from_binary(&res).unwrap();
//         assert_eq!(5, value.count);
//     }
// }
