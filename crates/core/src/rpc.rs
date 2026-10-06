use crate::error::{AppError, AppResult};
use crate::types::Block;
use serde_json::{json, Value};
use std::thread::sleep;
use std::time::Duration;

/// Mọi nguồn dữ liệu on-chain phải implement trait này.
/// Nhờ vậy test có thể dùng MockRpc, không cần mạng.
pub trait RpcClient {
    fn block_number(&self) -> AppResult<u64>;
    fn block(&self, n: u64) -> AppResult<Block>;
}

// ---------- Helpers ----------

pub fn hex_to_u128(s: &str) -> Option<u128> {
    let body = s.trim_start_matches("0x");
    if body.is_empty() {
        return Some(0);  // "0x" hoặc "" đều coi là 0
    }
    u128::from_str_radix(body, 16).ok()
}

fn wei_to_gwei(w: u128) -> f64 {
    w as f64 / 1e9
}

// ---------- HTTP client ----------

pub struct HttpRpc {
    pub url: String,
}

impl HttpRpc {
    pub fn from_env() -> AppResult<Self> {
        let url = std::env::var("RPC_URL")
            .map_err(|_| AppError::Config("thiếu biến môi trường RPC_URL".into()))?;
        Ok(Self { url })
    }

    fn call(&self, method: &str, params: Value) -> AppResult<Value> {
        let body = json!({ "jsonrpc": "2.0", "id": 1, "method": method, "params": params });
        let mut wait_ms = 500u64;

        for attempt in 1..=5 {
            match ureq::post(&self.url).send_json(body.clone()) {
                Ok(resp) => {
                    let v: Value = resp
                        .into_json()
                        .map_err(|e| AppError::Network(e.to_string()))?;
                    if let Some(err) = v.get("error") {
                        return Err(AppError::Rpc(err.to_string()));
                    }
                    return Ok(v["result"].clone());
                }
                Err(ureq::Error::Status(code, _)) if code == 429 || code >= 500 => {
                    tracing::warn!(attempt, code, wait_ms, "RPC tạm lỗi, thử lại");
                    sleep(Duration::from_millis(wait_ms));
                    wait_ms *= 2;
                }
                Err(e) => return Err(AppError::Network(e.to_string())),
            }
        }
        Err(AppError::Network("thử lại 5 lần vẫn lỗi".into()))
    }
}

impl RpcClient for HttpRpc {
    fn block_number(&self) -> AppResult<u64> {
        let v = self.call("eth_blockNumber", json!([]))?;
        let s = v.as_str().ok_or_else(|| AppError::Rpc("blockNumber không phải chuỗi".into()))?;
        Ok(hex_to_u128(s).ok_or_else(|| AppError::Rpc("hex sai định dạng".into()))? as u64)
    }

    fn block(&self, n: u64) -> AppResult<Block> {
        let v = self.call(
            "eth_getBlockByNumber",
            json!([format!("0x{:x}", n), true]),
        )?;

        let timestamp = v["timestamp"].as_str().and_then(hex_to_u128).unwrap_or(0) as u64;

        let base_fee_gwei = v["baseFeePerGas"]
            .as_str()
            .and_then(hex_to_u128)
            .map(wei_to_gwei);

        let txs = v["transactions"].as_array().cloned().unwrap_or_default();
        let mut prices: Vec<f64> = txs
            .iter()
            .filter_map(|t| t["gasPrice"].as_str().and_then(hex_to_u128))
            .map(wei_to_gwei)
            .collect();

        Ok(Block {
            number: n,
            timestamp,
            tx_count: txs.len(),
            base_fee_gwei,
            median_gas_price_gwei: crate::stats::median(&mut prices),
        })
    }
}

// ---------- Mock client (cho test / demo offline) ----------

pub struct MockRpc {
    pub blocks: Vec<Block>,
}

impl MockRpc {
    pub fn with_blocks(blocks: Vec<Block>) -> Self {
        Self { blocks }
    }

    /// Sinh n block giả để test nhanh.
    pub fn synthetic(n: u64) -> Self {
        let blocks = (0..n)
            .map(|i| Block {
                number: i,
                timestamp: 1_700_000_000 + i * 12,
                tx_count: 100 + (i as usize % 50),
                base_fee_gwei: Some(20.0 + (i as f64 % 10.0)),
                median_gas_price_gwei: Some(22.0 + (i as f64 % 8.0)),
            })
            .collect();
        Self { blocks }
    }
}

impl RpcClient for MockRpc {
    fn block_number(&self) -> AppResult<u64> {
        Ok(self.blocks.last().map(|b| b.number).unwrap_or(0))
    }

    fn block(&self, n: u64) -> AppResult<Block> {
        self.blocks
            .iter()
            .find(|b| b.number == n)
            .cloned()
            .ok_or_else(|| AppError::Rpc(format!("mock không có block {n}")))
    }
}

// ---------- Tests ----------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_to_u128_hoat_dong() {
        assert_eq!(hex_to_u128("0x1a"), Some(26));
        assert_eq!(hex_to_u128("0x"), Some(0));
        assert_eq!(hex_to_u128("xyz"), None);
    }

    #[test]
    fn mock_rpc_tra_dung_block() {
        let rpc = MockRpc::synthetic(10);
        assert_eq!(rpc.block_number().unwrap(), 9);
        let b = rpc.block(3).unwrap();
        assert_eq!(b.number, 3);
        assert!(rpc.block(999).is_err());
    }
}