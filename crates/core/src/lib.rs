//! Lõi nghiệp vụ: error, kiểu dữ liệu, RPC client, thống kê.
//!
//! Quy tắc: crate này KHÔNG biết gì về HTTP server hay CLI.
//! Mọi I/O đi qua trait để test dễ.

pub mod error;
pub mod rpc;
pub mod stats;
pub mod types;

pub use error::{AppError, AppResult};
pub use rpc::{HttpRpc, MockRpc, RpcClient};
pub use stats::{ewma, median, moving_average};
pub use types::{Block, BlockSummary};