use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Uint256};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum InstantiateMsg {
    
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Add {
        delta1:Uint256,
        delta2:Uint256,
        delta3:Uint256,
        delta4:Uint256,
        delta5:Uint256,
        delta6:Uint256,
        delta7:Uint256,
        delta8:Uint256,
        delta9:Uint256,
        delta10:Uint256,
    }, Add1 {
        delta1:Uint256,
        delta2:Uint256,
        delta3:Uint256,
        delta4:Uint256,
        delta5:Uint256,
        delta6:Uint256,
        delta7:Uint256,
        delta8:Uint256,
        delta9:Uint256,
        delta10:Uint256,
    },
    Add2 {
        delta1:Uint256,
        delta2:Uint256,
        delta3:Uint256,
        delta4:Uint256,
        delta5:Uint256,
        delta6:Uint256,
        delta7:Uint256,
        delta8:Uint256,
        delta9:Uint256,
        delta10:Uint256,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCounter {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}
