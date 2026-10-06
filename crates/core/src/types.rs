use serde::{Deserialize, Serialize};

/// Một block đã chuẩn hóa (không phụ thuộc chain cụ thể).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub number: u64,
    pub timestamp: u64,
    pub tx_count: usize,
    pub base_fee_gwei: Option<f64>,
    pub median_gas_price_gwei: Option<f64>,
}

/// Bản rút gọn để trả cho dashboard.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlockSummary {
    pub number: u64,
    pub tx_count: usize,
    pub median_gas_price_gwei: Option<f64>,
}