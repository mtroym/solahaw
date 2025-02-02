use borsh::{BorshDeserialize, BorshSerialize};
use hex;
use serde::Serialize;
use sha2;
use sha2::Digest;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
};
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use std::{collections::HashMap, fs::File, str::FromStr, time::Duration};
use tracing::{info, warn};
use tracing_subscriber::filter::EnvFilter;
// Anchor IDL 结构定义
mod anchor_idl;
use anchor_idl::AnchorIdl;

const METEORA_POOL_PROGRAM_ID: &str = "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB";

// 账户数据解析结果
#[derive(Debug, Serialize)]
struct AccountSnapshot {
    pubkey: String,
    account_type: String,
    data: serde_json::Value,
}

// 加载Anchor IDL
async fn load_anchor_idl(path: &str) -> Result<AnchorIdl, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let idl: AnchorIdl = serde_json::from_str(&content)?;
    Ok(idl)
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize)]
struct Bootstrapping {
    activation_point: u64,
    whitelisted_vault: Pubkey,
    pool_creator: Pubkey,
    activation_type: u8,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize)]
struct PoolFees {
    trade_fee_numerator: u64,
    trade_fee_denominator: u64,
    protocol_trade_fee_numerator: u64,
    protocol_trade_fee_denominator: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize)]
enum PoolType {
    Permissioned {},
    Permissionless {},
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize)]
enum CurveType {
    ConstantProduct {},
    Stable {
        amp: u64,
        token_multiplier: TokenMultiplier,
        depeg: Depeg,
        last_amp_updated_timestamp: u64,
    },
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize)]
struct TokenMultiplier {
    token_a_multiplier: u64,
    token_b_multiplier: u64,
    precision_factor: u8,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize)]
struct Depeg {
    base_virtual_price: u64,
    base_cache_updated: u64,
    depeg_type: DepegType,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize)]
enum DepegType {
    None,
    Marinade,
    Lido,
    SplStake,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize)]
struct PartnerInfo {
    fee_numerator: u64,
    partner_authority: Pubkey,
    pending_fee_a: u64,
    pending_fee_b: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize)]
struct Padding {
    padding0: [u8; 6],
    padding1: [u64; 21],
    padding2: [u64; 21],
}

// 在Pool结构体中添加必要字段
#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Pool {
    lp_mint: Pubkey,
    token_a_mint: Pubkey,
    token_b_mint: Pubkey,
    a_vault: Pubkey,
    b_vault: Pubkey,
    a_vault_lp: Pubkey,
    b_vault_lp: Pubkey,
    a_vault_lp_bump: u8,
    enabled: bool,
    protocol_token_a_fee: Pubkey,
    protocol_token_b_fee: Pubkey,
    fee_last_updated_at: u64,
    padding0: [u8; 24],
    fees: PoolFees,
    pool_type: PoolType,
    stake: Pubkey,
    total_locked_lp: u64,
    bootstrapping: Bootstrapping,
    partner_info: PartnerInfo,
    padding: Padding,
    curve_type: CurveType,
}

// 改进错误处理
fn parse_meteora_pool(
    idl: &AnchorIdl,
    data: &[u8],
    address: &str,
) -> Result<AccountSnapshot, Box<dyn std::error::Error>> {
    let discriminator = &data[..8];
    if let Some(account_def) = idl.accounts.iter().find(|a| {
        let preimage = format!("account:{}", a.name);
        let mut hash = sha2::Sha256::digest(preimage.as_bytes());
        hash[8..].fill(0);
        &hash[..8] == discriminator
    }) {
        match account_def.name.as_str() {
            "Pool" => {
                let pool_data = Pool::deserialize(&mut &data[8..])?;
                Ok(AccountSnapshot {
                    pubkey: address.to_string(),
                    account_type: "Pool".to_string(),
                    data: serde_json::json!(
                        {
                            "enabled": pool_data.enabled,
                            "lp_mint": pool_data.lp_mint.to_string(),
                            "token_a_mint": pool_data.token_a_mint.to_string(),
                            "token_b_mint": pool_data.token_b_mint.to_string(),
                            "a_vault": pool_data.a_vault.to_string(),
                            "b_vault": pool_data.b_vault.to_string(),
                            "a_vault_lp": pool_data.a_vault_lp.to_string(),
                            "b_vault_lp": pool_data.b_vault_lp.to_string(),
                            "a_vault_lp_bump": pool_data.a_vault_lp_bump.to_string(),
                            "enabled": pool_data.enabled.to_string(),
                            "protocol_token_a_fee": pool_data.protocol_token_a_fee.to_string(),
                            "protocol_token_b_fee": pool_data.protocol_token_b_fee.to_string(),
                            "pool_type": pool_data.pool_type,
                            "curve_type": pool_data.curve_type,
                            "stake": pool_data.stake.to_string(),
                            "total_locked_lp": pool_data.total_locked_lp.to_string(),
                            // "bootstrapping": pool_data.bootstrapping,
                            // "partner_info": pool_data.partner_info,
                            // "padding": pool_data.padding,
                        }
                    ),
                })
            }
            "LockEscrow" => Ok(AccountSnapshot {
                pubkey: address.to_string(),
                account_type: "LockEscrow".to_string(),
                data: serde_json::json!({"discriminator": hex::encode(discriminator)}),
            }),
            other_name => {
                warn!(
                    "address: {}, Unhandled account type: {} (discriminator: {})",
                    address,
                    other_name,
                    hex::encode(discriminator),
                );
                Ok(AccountSnapshot {
                    pubkey: address.to_string(),
                    account_type: other_name.to_string(),
                    data: serde_json::json!({"discriminator": hex::encode(discriminator)}),
                })
            }
        }
    } else {
        Ok(AccountSnapshot {
            pubkey: address.to_string(),
            account_type: "Unknown".to_string(),
            data: serde_json::json!({"discriminator": hex::encode(discriminator)}),
        })
    }
}

fn save_snapshot(
    snapshot: &HashMap<String, AccountSnapshot>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("snapshots/meteora_snapshot.json")?;
    serde_json::to_writer_pretty(file, snapshot)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用 RUST_LOG 环境变量配置日志级别 (支持更灵活的过滤规则)
    let filter = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    tracing_subscriber::fmt().with_env_filter(filter).init();

    let idl = load_anchor_idl("idls/meteora_pool.json").await?;
    let client = RpcClient::new_with_timeout_and_commitment(
        "https://solana-rpc.publicnode.com".to_string(),
        Duration::from_secs(60),
        CommitmentConfig::confirmed(),
    );

    let program_id = Pubkey::from_str(METEORA_POOL_PROGRAM_ID)?;
    let accounts = client
        .get_program_accounts_with_config(
            &program_id,
            RpcProgramAccountsConfig {
                filters: Some(vec![]),
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    data_slice: None,
                    commitment: Some(CommitmentConfig::confirmed()),
                    min_context_slot: None,
                },
                with_context: Some(true),
            },
        )
        .await?;

    info!("Found {} pools", accounts.len());

    let mut snapshot = HashMap::new();
    for (pubkey, account) in accounts.iter() {
        let pubkey_str = pubkey.to_string(); // 账户地址部分
        info!(
            "Parsing account data (len={}): {}",
            account.data.len(),
            pubkey_str
        );
        let parsed = parse_meteora_pool(&idl, &account.data, &pubkey_str)?;
        if parsed.account_type == "Pool" {
            snapshot.insert(pubkey_str, parsed);
        }
    }

    save_snapshot(&snapshot)?;
    info!("成功保存 {} 个账户的快照", snapshot.len());
    Ok(())
}
