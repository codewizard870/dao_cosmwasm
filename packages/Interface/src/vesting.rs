use cosmwasm_std::{Addr, Uint128, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetConfig {
        admin: String,
    },
    AddProject {
        project_id: Uint64,
        admin: String,
        token_addr: String,
        vesting_params: Vec<VestingParameter>,
        start_time: Uint128,
    },
    StartRelease {
        project_id: Uint64,
        start_time: Uint128,
    },
    SetProjectInfo {
        project_id: Uint64,
        project_info: ProjectInfo,
    },
    SetProjectConfig {
        project_id: Uint64,
        admin: String,
        token_addr: String,
        start_time: Uint128,
    },
    SetVestingParameters {
        project_id: Uint64,
        params: Vec<VestingParameter>,
    },
    AddUser {
        project_id: Uint64,
        stage: Uint128,
        wallet: Addr,
        amount: Uint128,
    },
    SetUsers {
        project_id: Uint64,
        stage: Uint128,
        user_infos: Vec<UserInfo>,
    },
    ClaimPendingTokens {
        project_id: Uint64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig { project_id: Uint64 },
    GetPendingTokens { project_id: Uint64, wallet: String },
    GetUserInfo { project_id: Uint64, wallet: String },
    GetBalance { project_id: Uint64, wallet: String },
    GetProjectInfo { project_id: Uint64 },
    GetAllProjectInfo {},
    GetOwner {},
}

//------------Config---------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub token_addr: String,
    pub start_time: Uint128,
}

//------------Vesting parameter---------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Copy)]
pub struct VestingParameter {
    pub soon: Uint128,
    pub after: Uint128,
    pub period: Uint128,
}

//-------------Token holder-------------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserInfo {
    pub wallet_address: Addr,     //investor wallet address
    pub total_amount: Uint128,    //WFD token total amount that the investor buys.
    pub released_amount: Uint128, //released WFD token amount of totalAmount
    pub pending_amount: Uint128,  //token amount that investor can claim
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProjectInfo {
    pub project_id: Uint64,
    pub config: Config,
    pub vest_param: Vec<VestingParameter>,
    pub users: Vec<Vec<UserInfo>>,
    pub total: Vec<Uint128>,
}
