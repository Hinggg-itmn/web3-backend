#!/usr/bin/env python3
# RPC giả (chỉ để thử demo khi chưa có khóa API thật). Dữ liệu là NGẪU NHIÊN, không phải Ethereum thật.
# Chạy:  python3 mock_rpc.py          (lắng nghe http://127.0.0.1:8545)
# Rồi:   export RPC_URL=http://127.0.0.1:8545 && cargo run --release
import json
import random
from http.server import BaseHTTPRequestHandler, HTTPServer

LATEST = 20_000_000
random.seed(1)


def gwei(x):
    return hex(int(x * 1e9))


def make_block(n):
    rnd = random.Random(n)  # cùng block luôn ra cùng dữ liệu
    base = 20 + 8 * ((n % 200) / 200) + rnd.uniform(-1, 1)  # đường trôi chậm + nhiễu
    txs = [{"gasPrice": gwei(base + rnd.uniform(0.1, 6))} for _ in range(rnd.randint(50, 200))]
    return {
        "number": hex(n),
        "timestamp": hex(1_700_000_000 + (n - LATEST) * 12),
        "baseFeePerGas": gwei(base),
        "transactions": txs,
    }


class Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        req = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
        if req["method"] == "eth_blockNumber":
            result = hex(LATEST)
        elif req["method"] == "eth_getBlockByNumber":
            result = make_block(int(req["params"][0], 16))
        else:
            result = None
        body = json.dumps({"jsonrpc": "2.0", "id": req["id"], "result": result}).encode()
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, *args):
        pass


if __name__ == "__main__":
    print("RPC giả đang chạy ở http://127.0.0.1:8545 (Ctrl+C để dừng)")
    HTTPServer(("127.0.0.1", 8545), Handler).serve_forever()
