#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, Env, StdResult,
    Uint128, QueryRequest, BankQuery,
    Coin, AllBalanceResponse,
};

use cw20::{ Cw20QueryMsg, BalanceResponse as Cw20BalanceResponse, TokenInfoResponse };

use Interface::staking::{ExecuteMsg, InstantiateMsg, QueryMsg, UserInfo, CardInfo, CardType};
use crate::state::{ OWNER, REWARD_TOKEN, USER_INFOS, CARD_INFOS, START_TIME};
use crate::util::{ get_reward };

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner{ } => {
            to_binary(&OWNER.load(deps.storage)?)
        },

        QueryMsg::GetTokenAddress{ } => {
            to_binary(&REWARD_TOKEN.load(deps.storage)?)
        },

        QueryMsg::GetStartTime{ } => {
            to_binary(&START_TIME.load(deps.storage)?)
        },

        QueryMsg::GetUserInfo{ wallet } => {
            let user_info = USER_INFOS.may_load(deps.storage, wallet.clone())?;
            match user_info {
                Some(info) => {
                    to_binary(&info)                    
                },
                None => {
                    to_binary(&UserInfo{
                        wallet: wallet,
                        amount: Uint128::zero(),
                        last_withdraw_time: Uint128::zero(),
                        reward_amount: Uint128::zero(),
                        last_reward_time: Uint128::zero(),
                        card_type: CardType::Other,
                        card_number: Uint128::zero(),
                    })
                }
            }
            // to_binary(&USER_INFOS.load(deps.storage, wallet)?)
        },

        QueryMsg::GetPendingRewards{ wallet } => {
            let user_info = USER_INFOS.load(deps.storage, wallet)?;
            let (reward, extra) = get_reward(
                user_info.amount,
                user_info.card_type,
                user_info.last_reward_time,
                Uint128::from(env.block.time.seconds() as u128)
            )?;
            to_binary(&reward)
        },

        QueryMsg::GetCardInfo{ } => {
            to_binary(&CARD_INFOS.load(deps.storage)?)
        }

    }
}