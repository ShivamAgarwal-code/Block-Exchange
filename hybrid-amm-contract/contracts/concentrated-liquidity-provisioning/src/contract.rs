pub use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cosmwasm_storage::{
    singleton, singleton_read, ReadonlySingleton, Singleton, 
};
use cosmwasm_std::Storage;

//use cosmwasm_std::Storage;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

// Define the state struct to store the liquidity and price range information
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct State {
    pub token_reserve: u64,
    pub base_reserve: u64,
    pub price_range_min: u64,
    pub price_range_max: u64,
}

// Define the contract message struct for depositing tokens
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct DepositMsg {
    pub tokens: u64,
}

// Define the contract message struct for withdrawing tokens
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct WithdrawMsg {
    pub tokens: u64,
}

// Define the contract message struct for querying the current state
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum QueryMsg {
    State {},
}

// Implement the contract logic
pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: DepositMsg,
) -> StdResult<Response> {
    // Read the current state from the contract storage
    let mut state = get_state(deps.storage)?;

    // Calculate the amount of base tokens to add based on the current reserves and the deposit amount
    let base_tokens = (msg.tokens * state.base_reserve) / state.token_reserve;

    // Update the reserves and the state
    state.token_reserve += msg.tokens;
    state.base_reserve += base_tokens;

    // Save the updated state to the contract storage
    set_state(deps.storage, &state)?;

    // Return a successful response
    Ok(Response::new().add_attributes(vec![("action", "deposit")]))
}

pub fn handle_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: WithdrawMsg,
) -> StdResult<Response> {
    // Read the current state from the contract storage
    let mut state = get_state(deps.storage)?;

    // Calculate the amount of tokens to withdraw based on the current reserves and the withdrawal amount
    let tokens = (msg.tokens * state.token_reserve) / state.base_reserve;

    // Update the reserves and the state
    state.token_reserve -= tokens;
    state.base_reserve -= msg.tokens;

    // Save the updated state to the contract storage
    set_state(deps.storage, &state)?;

    // Return a successful response
    Ok(Response::new().add_attributes(vec![("action", "withdraw")]))
}

pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::State {} => to_binary(&get_state(deps.storage)?),
    }
}

// Helper functions for reading and updating the contract state

fn get_state(storage: &dyn Storage) -> StdResult<State> {
    singleton_read(storage,&[2]).load().or_else(|_| {
        let initial_state = State {
            token_reserve: 0,
            base_reserve:0,
            price_range_min: 0,
            price_range_max: 0,
        };

        //set_state(&mut dyn storage, &initial_state)?;
        Ok(initial_state)
    })
}

fn set_state(storage: &mut dyn Storage, state: &State) -> StdResult<()> {
    singleton(storage,&[2]).save(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
    };

    // Helper function to initialize the contract
    fn instantiate_contract(
        deps: &mut Extern<MockStorage, MockApi, MockQuerier>,
        owner: HumanAddr,
        token_address: HumanAddr,
        fee_rate: Decimal,
    ) {
        let msg = HandleMsg::UpdateConfig {
            fee_rate: fee_rate.clone(),
        };

        let env = mock_env();
        let info = mock_info(owner.clone(), &[]);
        let res = handle(deps, env, info, msg).unwrap();

        assert_eq!(0, res.messages.len());

        let expected_config = Config {
            owner: deps.api.canonical_address(&owner).unwrap(),
            token_address: token_address.clone(),
            fee_rate: fee_rate.clone(),
        };

        let config = get_config(&deps.storage).unwrap();
        assert_eq!(expected_config, config);
    }

    #[test]
    fn test_provide_liquidity() {
        let mut deps = mock_dependencies(&[]);

        // Initialize contract
        let owner = HumanAddr::from("owner");
        let token_address = HumanAddr::from("token");
        let fee_rate = Decimal::percent(0.3);
        instantiate_contract(&mut deps, owner.clone(), token_address.clone(), fee_rate.clone());

        // Set up reserves
        let reserves = vec![
            Reserve {
                asset: Cw20Coin {
                    address: HumanAddr::from("asset1"),
                    amount: Uint128::new(100),
                },
                concentration: Decimal::percent(50),
            },
            Reserve {
                asset: Cw20Coin {
                    address: HumanAddr::from("asset2"),
                    amount: Uint128::new(200),
                },
                concentration: Decimal::percent(50),
            },
        ];

        set_reserves(&mut deps.storage, &reserves).unwrap();

        // Provide liquidity
        let assets = vec![
            Cw20Coin {
                address: HumanAddr::from("asset3"),
                amount: Uint128::new(300),
            },
            Cw20Coin {
                address: HumanAddr::from("asset4"),
                amount: Uint128::new(400),
            },
        ];

        let env = mock_env();
        let info = mock_info(owner.clone(), &[]);
        let msg = HandleMsg::ProvideLiquidity { assets };

        let res = handle(&mut deps, env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Check updated reserves
        let updated_reserves = get_reserves(&deps.storage).unwrap();

        assert_eq!(reserves.len() + assets.len(), updated_reserves.len());

        // Check LP token minting
        let config = get_config(&deps.storage).unwrap();
        let lp_token_address = config.token_address;
        let lp_token = Cw20Coin {
            address: lp_token_address,
            amount: Uint128::new(1000),
        };

        let mint_msg = Cw20HandleMsg::Mint {
            recipient: owner.clone(),
            amount: lp_token.amount,
        };

        let expected_msg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: lp_token.address.clone(),
            msg: to_binary(&mint_msg).unwrap(),
            send: vec![],
        });
        assert_eq!(1, res.messages.len());
        assert_eq!(expected_msg, res.messages[0]);

        // Check LP token balance of the sender
        let query_msg = QueryMsg::Balance {
            address: owner.clone(),
        };

        let query_response = query(&deps, mock_env(), query_msg).unwrap();
        let balance: Balance = from_binary(&query_response).unwrap();

        assert_eq!(lp_token.amount, balance.amount);
        assert_eq!(lp_token.address, balance.address);
    }

    #[test]
    fn test_withdraw_liquidity() {
        let mut deps = mock_dependencies(&[]);

        // Initialize contract
        let owner = HumanAddr::from("owner");
        let token_address = HumanAddr::from("token");
        let fee_rate = Decimal::percent(0.3);
        instantiate_contract(&mut deps, owner.clone(), token_address.clone(), fee_rate.clone());

        // Set up reserves
        let reserves = vec![
            Reserve {
                asset: Cw20Coin {
                    address: HumanAddr::from("asset1"),
                    amount: Uint128::new(100),
                },
                concentration: Decimal::percent(50),
            },
            Reserve {
                asset: Cw20Coin {
                    address: HumanAddr::from("asset2"),
                    amount: Uint128::new(200),
                },
                concentration: Decimal::percent(50),
            },
        ];

        set_reserves(&mut deps.storage, &reserves).unwrap();

        // Provide liquidity
        let assets = vec![
            Cw20Coin {
                address: HumanAddr::from("asset3"),
                amount: Uint128::new(300),
            },
            Cw20Coin {
                address: HumanAddr::from("asset4"),
                amount: Uint128::new(400),
            },
        ];

        let env = mock_env();
        let info = mock_info(owner.clone(), &[]);
        let msg = HandleMsg::ProvideLiquidity { assets };

        let res = handle(&mut deps, env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Withdraw liquidity
        let amount = Uint128::new(500);

        let env = mock_env();
        let info = mock_info(owner.clone(), &[]);
        let msg = HandleMsg::WithdrawLiquidity { amount };

        let res = handle(&mut deps, env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Check updated reserves
        let updated_reserves = get_reserves(&deps.storage).unwrap();

        assert_eq!(reserves.len(), updated_reserves.len());

        // Check LP token burning
        let config = get_config(&deps.storage).unwrap();
        let lp_token_address = config.token_address;
        let burn_msg = Cw20HandleMsg::Burn { amount };

        let expected_msg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: lp_token_address.clone(),
            msg: to_binary(&burn_msg).unwrap(),
            send: vec![],
        });

        assert_eq!(1, res.messages.len());
        assert_eq!(expected_msg, res.messages[0]);

        // Check LP token balance of the sender
        let query_msg = QueryMsg::Balance {
            address: owner.clone(),
        };

        let query_response = query(&deps, mock_env(), query_msg).unwrap();
        let balance: Balance = from_binary(&query_response).unwrap();

        assert_eq!(lp_token_amount.checked_sub(amount).unwrap(), balance.amount);
        assert_eq!(lp_token_address, balance.address);
    }

}