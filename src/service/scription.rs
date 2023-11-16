use crate::config::Config;
use rand::prelude::*;
use std::{collections::HashSet, time::Duration};
use tokio::time::sleep;
use tracing::info;

use ethers::{
    prelude::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::LocalWallet,
};
use ethers_core::{
    abi::{Address, Bytes},
    types::{BlockId, TransactionRequest, U256},
};

use super::{InscriptionWithId, InscriptionWithOutId};

#[allow(dead_code)]
impl<'a> InscriptionWithId<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }
    pub async fn mint(&self) {
        info!("***** 开始铭刻 *****");

        let mut rng = rand::thread_rng();
        let mut generated_ids: HashSet<u64> = HashSet::new();

        let provider = get_http(self.config.network_info.https.clone()).await;
        let mut nonce = get_initial_nonce(
            &provider,
            self.config.account_info.address.parse::<Address>().unwrap(),
        )
        .await;
        info!("***** 获取初始nonce成功:{:?} *****", nonce);
        let signer = self
            .config
            .account_info
            .private_key
            .parse::<LocalWallet>()
            .unwrap();

        let provider_with_signer = SignerMiddleware::new_with_provider_chain(&provider, signer)
            .await
            .unwrap();

        let id = gen_id(
            &mut rng,
            &mut generated_ids,
            self.config.token_info.amt,
            self.config.token_info.total,
        )
        .await;

        let data = format!(
            "data:,{{\"p\":\"{}\",\"op\":\"mint\",\"tick\":\"{}\",\"id\":\"{}\",\"amt\":\"{}\"}}",
            self.config.token_info.protocol,
            self.config.token_info.tick,
            id,
            self.config.token_info.amt
        );

        for i in 0..self.config.mint_info.amount {
            let tx = TransactionRequest::new()
                .to(self.config.account_info.address.parse::<Address>().unwrap())
                .value(0)
                .data(Bytes::from(data.clone()))
                .nonce(nonce);

            let pending_tx = provider_with_signer.send_transaction(tx, None).await;

            match pending_tx {
                Ok(tx) => {
                    info!(
                        "***** 第 {} 次铭刻成功: {:?}, nonce:{:?} *****",
                        i + 1,
                        tx.tx_hash(),
                        nonce
                    );
                    nonce += U256::from(1);
                    sleep(Duration::from_millis(100)).await;
                }
                Err(e) => {
                    info!(
                        "***** 第 {} 次铭刻失败 {:?}, nonce:{:?} *****",
                        i + 1,
                        e,
                        nonce
                    );
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }
}

#[allow(dead_code)]
impl<'a> InscriptionWithOutId<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub async fn mint(&self) {
        info!("***** 开始铭刻 *****");
        let provider = get_http(self.config.network_info.https.clone()).await;
        let mut nonce = get_initial_nonce(
            &provider,
            self.config.account_info.address.parse::<Address>().unwrap(),
        )
        .await;
        info!("***** 获取初始nonce成功:{:?} *****", nonce);
        let signer = self
            .config
            .account_info
            .private_key
            .parse::<LocalWallet>()
            .unwrap();

        let provider_with_signer = SignerMiddleware::new_with_provider_chain(&provider, signer)
            .await
            .unwrap();

        let data = format!(
            "data:,{{\"p\":\"{}\",\"op\":\"mint\",\"tick\":\"{}\",\"amt\":\"{}\"}}",
            self.config.token_info.protocol,
            self.config.token_info.tick,
            self.config.token_info.amt
        );

        for i in 0..self.config.mint_info.amount {
            let gas_price_before = provider.get_gas_price().await.unwrap();
            let gas_price_u64 = gas_price_before.as_u64();
            let gas_price_u64 = (gas_price_u64 as f64 * 1.1) as u64;
            let gas_price_after = U256::from(gas_price_u64);

            let tx = TransactionRequest::new()
                .to(self.config.account_info.address.parse::<Address>().unwrap())
                .value(0)
                .data(Bytes::from(data.clone()))
                .nonce(nonce)
                .gas_price(gas_price_after);

            let pending_tx = provider_with_signer.send_transaction(tx, None).await;

            match pending_tx {
                Ok(tx) => {
                    info!(
                        "***** 第 {} 次铭刻成功: {:?}, nonce:{:?} *****",
                        i + 1,
                        tx.tx_hash(),
                        nonce
                    );
                    nonce += U256::from(1);
                    // sleep(Duration::from_millis(100)).await;
                }
                Err(e) => {
                    info!(
                        "***** 第 {} 次铭刻失败 {:?}, nonce:{} *****",
                        i + 1,
                        e,
                        nonce
                    );
                    sleep(Duration::from_millis(200)).await;
                }
            }
        }
    }
}

// 获取初始化nonce
async fn get_initial_nonce(http_provider: &Provider<Http>, address: Address) -> U256 {
    info!("***** 获取初始化nonce *****");
    let b = http_provider.get_block_number().await.unwrap();
    let nonce = http_provider
        .get_transaction_count(address, Some(BlockId::from(b)))
        .await
        .unwrap();
    nonce
}

// 获取http端点
async fn get_http(eth_rpc: String) -> Provider<Http> {
    Provider::<Http>::try_from(eth_rpc).unwrap()
}

async fn gen_id(
    rng: &mut impl Rng,
    generated_numbers: &mut HashSet<u64>,
    amt: u64,
    total: u64,
) -> u64 {
    let mut random_id;
    loop {
        let size = total / amt;
        random_id = rng.gen_range(1..=size);
        if generated_numbers.insert(random_id) {
            break;
        }
    }
    random_id
}
