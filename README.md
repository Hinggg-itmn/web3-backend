# web3-backend

Backend cho dashboard Web3: lấy dữ liệu on-chain, tính chỉ số, cung cấp JSON API.

## Cấu trúc
- `crates/core` — error, types, RPC client, logic thuần (có test)
- `crates/cli`  — công cụ dòng lệnh để debug nhanh
- `scripts/`    — mock RPC + thuật toán Python (EWMA…)

## Chạy dev
    python3 scripts/mock_rpc.py &          # RPC giả ở :8545
    RPC_URL=http://127.0.0.1:8545 cargo run -p cli -- 20

## Test
    cargo test --workspace