#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, AllBalanceResponse, BankQuery, Binary, Coin, Deps, Env, QueryRequest,
    StdResult, Uint128, Uint64,
};
use cw20::{BalanceResponse as Cw20BalanceResponse, Cw20QueryMsg};

use crate::state::{COMMUNITY, CONFIG, PROJECTSTATES};
use Interface::wefund::{BackerState, Config, ProjectState, QueryMsg};

// version info for migration info
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance { wallet } => to_binary(&query_balance(deps, _env, wallet)?),
        QueryMsg::GetConfig {} => to_binary(&query_getconfig(deps)?),
        QueryMsg::GetAllProject {} => to_binary(&query_allproject(deps)?),
        QueryMsg::GetProject { project_id } => to_binary(&query_project(deps, project_id)?),
        QueryMsg::GetBacker { project_id } => to_binary(&query_backer(deps, project_id)?),
        QueryMsg::GetCommunitymembers {} => to_binary(&query_communitymembers(deps)?),
    }
}

fn query_communitymembers(deps: Deps) -> StdResult<Vec<Addr>> {
    let community = COMMUNITY.load(deps.storage).unwrap();
    Ok(community)
}
fn query_balance(deps: Deps, _env: Env, wallet: String) -> StdResult<AllBalanceResponse> {
    // let uusd_denom = String::from("uusd");
    let mut balance: AllBalanceResponse =
        deps.querier
            .query(&QueryRequest::Bank(BankQuery::AllBalances {
                address: wallet.clone(),
            }))?;

    Ok(balance)
}
fn query_getconfig(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage).unwrap();
    Ok(config)
}
fn query_allproject(deps: Deps) -> StdResult<Vec<ProjectState>> {
    let all: StdResult<Vec<_>> = PROJECTSTATES
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .collect();
    let all = all.unwrap();

    let mut all_project: Vec<ProjectState> = Vec::new();
    for x in all {
        all_project.push(x.1);
    }
    Ok(all_project)
}
fn query_backer(deps: Deps, id: Uint64) -> StdResult<Vec<BackerState>> {
    let x = PROJECTSTATES.load(deps.storage, id.u64())?;
    Ok(x.backer_states)
}
fn query_project(deps: Deps, id: Uint64) -> StdResult<ProjectState> {
    let x = PROJECTSTATES.load(deps.storage, id.u64())?;
    Ok(x)
}
