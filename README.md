# web3demo

Demo nhỏ: lấy N block Ethereum gần nhất qua JSON-RPC, tính gas price trung vị mỗi block, ghi `gas.csv`,
rồi thử EWMA bằng Python.

## Chạy thử (không cần khóa API)
```bash
python3 mock_rpc.py &                      # RPC giả, dữ liệu ngẫu nhiên
export RPC_URL=http://127.0.0.1:8545
cargo run --release -- 120                 # ghi gas.csv
python3 ewma_test.py gas.csv 5             # MAE của EWMA so với 2 baseline
```

## Chạy với dữ liệu thật
```bash
export RPC_URL="https://.../KHOA_CUA_BAN"  # KHÔNG dán khóa vào code, KHÔNG commit file .env
cargo run --release -- 100
```

Cần Rust bản mới (khoảng 1.85 trở lên): `rustup update`.
