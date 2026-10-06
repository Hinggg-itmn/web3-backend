use clap::Parser;
use app_core::{AppResult, HttpRpc, RpcClient};
use std::fs::File;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "web3cli", about = "Lấy block và ghi gas.csv (dùng để debug)")]
struct Cli {
    /// Số block gần nhất cần lấy
    #[arg(default_value_t = 100)]
    count: u64,

    /// File CSV output
    #[arg(short, long, default_value = "data/gas.csv")]
    out: String,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    if let Err(e) = run() {
        eprintln!("Lỗi: {e}");
        std::process::exit(1);
    }
}

fn run() -> AppResult<()> {
    let cli = Cli::parse();
    let rpc = HttpRpc::from_env()?;

    let latest = rpc.block_number()?;
    tracing::info!(latest, "block mới nhất");

    std::fs::create_dir_all("data").ok();
    let mut file = File::create(&cli.out)
        .map_err(|e| app_core::AppError::WriteFile {
            path: cli.out.clone().into(),
            source: e,
        })?;
    writeln!(file, "block,timestamp,tx_count,base_fee_gwei,median_gas_price_gwei")
        .map_err(|e| app_core::AppError::WriteFile {
            path: cli.out.clone().into(),
            source: e,
        })?;

    let start = latest.saturating_sub(cli.count - 1);
    for n in start..=latest {
        let b = rpc.block(n)?;
        let line = format!(
            "{},{},{},{:.4},{:.4}",
            b.number,
            b.timestamp,
            b.tx_count,
            b.base_fee_gwei.unwrap_or(f64::NAN),
            b.median_gas_price_gwei.unwrap_or(f64::NAN),
        );
        println!("{}", line);
        writeln!(file, "{}", line).ok();
        sleep(Duration::from_millis(60));
    }

    tracing::info!(file = %cli.out, "đã ghi xong");
    Ok(())
}