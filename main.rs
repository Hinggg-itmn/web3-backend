// Demo: lấy N block Ethereum gần nhất qua JSON-RPC, tính gas price trung vị mỗi block,
// in ra màn hình và ghi vào gas.csv.
//
// Chạy:
//   export RPC_URL="https://.../v2/KHOA_CUA_BAN"     (đừng dán khóa vào code)
//   cargo run --release            # mặc định 100 block
//   cargo run --release -- 300     # lấy 300 block

use serde_json::{json, Value};
use std::fs::File;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

/// Gọi một phương thức JSON-RPC, tự thử lại khi gặp lỗi tạm thời (ví dụ 429).
fn rpc(url: &str, method: &str, params: Value) -> Result<Value, String> {
    let body = json!({ "jsonrpc": "2.0", "id": 1, "method": method, "params": params });
    let mut wait_ms = 500;
    for attempt in 1..=5 {
        match ureq::post(url).send_json(body.clone()) {
            Ok(resp) => {
                let v: Value = resp.into_json().map_err(|e| e.to_string())?;
                if let Some(err) = v.get("error") {
                    return Err(format!("RPC báo lỗi: {}", err));
                }
                return Ok(v["result"].clone());
            }
            Err(ureq::Error::Status(code, _)) if code == 429 || code >= 500 => {
                eprintln!("  lần {}: HTTP {}, chờ {} ms rồi thử lại", attempt, code, wait_ms);
                sleep(Duration::from_millis(wait_ms));
                wait_ms *= 2; // lùi dần (backoff)
            }
            Err(e) => return Err(e.to_string()),
        }
    }
    Err("thử lại 5 lần vẫn lỗi".to_string())
}

/// Đổi chuỗi hex như "0x1a2b" thành u128.
fn hex_to_u128(s: &str) -> Option<u128> {
    u128::from_str_radix(s.trim_start_matches("0x"), 16).ok()
}

fn median(v: &mut Vec<f64>) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    Some(if n % 2 == 1 { v[n / 2] } else { (v[n / 2 - 1] + v[n / 2]) / 2.0 })
}

fn main() -> Result<(), String> {
    let url = std::env::var("RPC_URL").map_err(|_| "thiếu biến môi trường RPC_URL".to_string())?;
    let count: u64 = std::env::args()
        .nth(1)
        .map(|s| s.parse().expect("số block phải là số nguyên"))
        .unwrap_or(100);

    let latest_hex = rpc(&url, "eth_blockNumber", json!([]))?;
    let latest = hex_to_u128(latest_hex.as_str().ok_or("kết quả không phải chuỗi")?)
        .ok_or("không đọc được số block")? as u64;
    eprintln!("block mới nhất: {}", latest);

    let mut file = File::create("gas.csv").map_err(|e| e.to_string())?;
    writeln!(file, "block,timestamp,tx_count,base_fee_gwei,median_gas_price_gwei")
        .map_err(|e| e.to_string())?;
    println!("block,timestamp,tx_count,base_fee_gwei,median_gas_price_gwei");

    let start = latest.saturating_sub(count - 1);
    for n in start..=latest {
        // true = lấy đầy đủ nội dung từng giao dịch (cần để đọc gasPrice)
        let block = rpc(&url, "eth_getBlockByNumber", json!([format!("0x{:x}", n), true]))?;

        let timestamp = block["timestamp"].as_str().and_then(hex_to_u128).unwrap_or(0);
        let base_fee_gwei = block["baseFeePerGas"]
            .as_str()
            .and_then(hex_to_u128)
            .map(|w| w as f64 / 1e9)
            .unwrap_or(f64::NAN);

        let txs = block["transactions"].as_array().cloned().unwrap_or_default();
        let mut prices: Vec<f64> = txs
            .iter()
            .filter_map(|t| t["gasPrice"].as_str().and_then(hex_to_u128))
            .map(|w| w as f64 / 1e9) // wei -> gwei
            .collect();
        let med = median(&mut prices).unwrap_or(f64::NAN);

        let line = format!("{},{},{},{:.4},{:.4}", n, timestamp, txs.len(), base_fee_gwei, med);
        println!("{}", line);
        writeln!(file, "{}", line).map_err(|e| e.to_string())?;

        sleep(Duration::from_millis(60)); // nghỉ nhẹ để không chạm giới hạn tốc độ
    }
    eprintln!("đã ghi gas.csv");
    Ok(())
}
