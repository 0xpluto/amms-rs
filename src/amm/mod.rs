pub mod erc_4626;
pub mod factory;
pub mod uniswap_v2;
pub mod uniswap_v3;

use std::{collections::BTreeMap, sync::Arc};

use async_trait::async_trait;
use ethers::{
    providers::Middleware,
    types::{Log, H160, H256, U256},
};
use num_bigfloat::BigFloat;
use serde::{Deserialize, Serialize};

use crate::errors::{AMMError, ArithmeticError, EventLogError, SwapSimulationError};

use self::{erc_4626::ERC4626Vault, uniswap_v2::UniswapV2Pool, uniswap_v3::UniswapV3Pool};

#[async_trait]
pub trait AutomatedMarketMaker {
    fn address(&self) -> H160;
    async fn sync<M: Middleware>(&mut self, middleware: Arc<M>) -> Result<(), AMMError<M>>;
    fn sync_on_event_signatures(&self) -> Vec<H256>;
    fn tokens(&self) -> Vec<H160>;
    fn calculate_price(&self, base_token: H160) -> Result<f64, ArithmeticError>;
    fn sync_from_log(&mut self, log: Log) -> Result<(), EventLogError>;
    async fn populate_data<M: Middleware>(
        &mut self,
        block_number: Option<u64>,
        middleware: Arc<M>,
    ) -> Result<(), AMMError<M>>;
    fn sync_from_storage(&mut self, diff: &'_ BTreeMap<H256, H256>) -> Option<()>;

    fn reserves(&self) -> BTreeMap<H256, H256>;

    fn simulate_swap(&self, token_in: H160, amount_in: U256) -> Result<U256, SwapSimulationError>;
    fn simulate_swap_mut(
        &mut self,
        token_in: H160,
        amount_in: U256,
    ) -> Result<U256, SwapSimulationError>;
    fn gradient(&self, token_in: H160, amount: U256) -> Result<BigFloat, SwapSimulationError>;
    fn get_token_out(&self, token_in: H160) -> H160;
    fn opp_token(&self, token: H160) -> Option<H160>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AMM {
    UniswapV2Pool(UniswapV2Pool),
    UniswapV3Pool(UniswapV3Pool),
    ERC4626Vault(ERC4626Vault),
}

#[async_trait]
impl AutomatedMarketMaker for AMM {
    fn address(&self) -> H160 {
        match self {
            AMM::UniswapV2Pool(pool) => pool.address,
            AMM::UniswapV3Pool(pool) => pool.address,
            AMM::ERC4626Vault(vault) => vault.vault_token,
        }
    }

    async fn sync<M: Middleware>(&mut self, middleware: Arc<M>) -> Result<(), AMMError<M>> {
        match self {
            AMM::UniswapV2Pool(pool) => pool.sync(middleware).await,
            AMM::UniswapV3Pool(pool) => pool.sync(middleware).await,
            AMM::ERC4626Vault(vault) => vault.sync(middleware).await,
        }
    }

    fn sync_on_event_signatures(&self) -> Vec<H256> {
        match self {
            AMM::UniswapV2Pool(pool) => pool.sync_on_event_signatures(),
            AMM::UniswapV3Pool(pool) => pool.sync_on_event_signatures(),
            AMM::ERC4626Vault(vault) => vault.sync_on_event_signatures(),
        }
    }

    fn sync_from_log(&mut self, log: Log) -> Result<(), EventLogError> {
        match self {
            AMM::UniswapV2Pool(pool) => pool.sync_from_log(log),
            AMM::UniswapV3Pool(pool) => pool.sync_from_log(log),
            AMM::ERC4626Vault(vault) => vault.sync_from_log(log),
        }
    }

    fn sync_from_storage(&mut self, diff: &'_ BTreeMap<H256, H256>) -> Option<()> {
        match self {
            AMM::UniswapV2Pool(pool) => pool.sync_from_storage(diff),
            AMM::UniswapV3Pool(pool) => pool.sync_from_storage(diff),
            AMM::ERC4626Vault(vault) => vault.sync_from_storage(diff),
        }
    }

    fn reserves(&self) -> BTreeMap<H256, H256> {
        match self {
            AMM::UniswapV2Pool(pool) => pool.reserves(),
            AMM::UniswapV3Pool(pool) => pool.reserves(),
            AMM::ERC4626Vault(vault) => vault.reserves(),
        }
    }

    fn simulate_swap(&self, token_in: H160, amount_in: U256) -> Result<U256, SwapSimulationError> {
        match self {
            AMM::UniswapV2Pool(pool) => pool.simulate_swap(token_in, amount_in),
            AMM::UniswapV3Pool(pool) => pool.simulate_swap(token_in, amount_in),
            AMM::ERC4626Vault(vault) => vault.simulate_swap(token_in, amount_in),
        }
    }

    fn simulate_swap_mut(
        &mut self,
        token_in: H160,
        amount_in: U256,
    ) -> Result<U256, SwapSimulationError> {
        match self {
            AMM::UniswapV2Pool(pool) => pool.simulate_swap_mut(token_in, amount_in),
            AMM::UniswapV3Pool(pool) => pool.simulate_swap_mut(token_in, amount_in),
            AMM::ERC4626Vault(vault) => vault.simulate_swap_mut(token_in, amount_in),
        }
    }

    fn gradient(&self, token_in: H160, amount_in: U256) -> Result<BigFloat, SwapSimulationError> {
        match self {
            AMM::UniswapV2Pool(pool) => pool.gradient(token_in, amount_in),
            AMM::UniswapV3Pool(pool) => pool.gradient(token_in, amount_in),
            AMM::ERC4626Vault(vault) => vault.gradient(token_in, amount_in),
        }
    }

    fn get_token_out(&self, token_in: H160) -> H160 {
        match self {
            AMM::UniswapV2Pool(pool) => pool.get_token_out(token_in),
            AMM::UniswapV3Pool(pool) => pool.get_token_out(token_in),
            AMM::ERC4626Vault(vault) => vault.get_token_out(token_in),
        }
    }

    fn opp_token(&self, token_in: H160) -> Option<H160> {
        match self {
            AMM::UniswapV2Pool(pool) => pool.opp_token(token_in),
            AMM::UniswapV3Pool(pool) => pool.opp_token(token_in),
            AMM::ERC4626Vault(vault) => vault.opp_token(token_in),
        }
    }

    async fn populate_data<M: Middleware>(
        &mut self,
        block_number: Option<u64>,
        middleware: Arc<M>,
    ) -> Result<(), AMMError<M>> {
        match self {
            AMM::UniswapV2Pool(pool) => pool.populate_data(None, middleware).await,
            AMM::UniswapV3Pool(pool) => pool.populate_data(block_number, middleware).await,
            AMM::ERC4626Vault(vault) => vault.populate_data(None, middleware).await,
        }
    }

    fn tokens(&self) -> Vec<H160> {
        match self {
            AMM::UniswapV2Pool(pool) => pool.tokens(),
            AMM::UniswapV3Pool(pool) => pool.tokens(),
            AMM::ERC4626Vault(vault) => vault.tokens(),
        }
    }

    fn calculate_price(&self, base_token: H160) -> Result<f64, ArithmeticError> {
        match self {
            AMM::UniswapV2Pool(pool) => pool.calculate_price(base_token),
            AMM::UniswapV3Pool(pool) => pool.calculate_price(base_token),
            AMM::ERC4626Vault(vault) => vault.calculate_price(base_token),
        }
    }
}

impl PartialEq for AMM {
    fn eq(&self, other: &Self) -> bool {
        self.address() == other.address()
    }
}
