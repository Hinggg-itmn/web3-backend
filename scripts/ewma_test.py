#!/usr/bin/env python3
# Thử EWMA trên gas.csv (do chương trình Rust ghi ra).
# Dùng: python3 ewma_test.py [gas.csv] [k]      (k = dự báo trước k block, mặc định 5)
#
# Cách làm: chia dữ liệu theo thời gian (70% đầu để chọn alpha, 30% sau để kiểm tra),
# dự báo giá trị ở block t+k từ dữ liệu đến hết block t, rồi đo MAE.
import csv
import sys

path = sys.argv[1] if len(sys.argv) > 1 else "data/gas.csv"
k = int(sys.argv[2]) if len(sys.argv) > 2 else 5

with open(path) as f:
    xs = [float(r["median_gas_price_gwei"]) for r in csv.DictReader(f)]
xs = [x for x in xs if x == x]  # bỏ NaN
if len(xs) < 30:
    sys.exit("cần ít nhất ~30 block để thử")

split = int(len(xs) * 0.7)


def ewma_forecasts(series, alpha):
    """Trả về danh sách s_t (giá trị EWMA sau mỗi quan sát)."""
    out, s = [], series[0]
    for x in series:
        s = alpha * x + (1 - alpha) * s
        out.append(s)
    return out


def mae(pred, real):
    return sum(abs(p - r) for p, r in zip(pred, real)) / len(real)


def eval_range(lo, hi, forecast_fn):
    preds, reals = [], []
    for t in range(lo, hi - k):
        preds.append(forecast_fn(t))
        reals.append(xs[t + k])
    return mae(preds, reals)


# Baseline 1: lấy giá trị block gần nhất. Baseline 2: trung bình trượt N block.
naive = lambda t: xs[t]
N = 10
moving_avg = lambda t: sum(xs[max(0, t - N + 1): t + 1]) / len(xs[max(0, t - N + 1): t + 1])

# Chọn alpha trên đoạn huấn luyện (quét lưới)
best_alpha, best_mae = None, float("inf")
for a in [i / 100 for i in range(5, 100, 5)]:
    s = ewma_forecasts(xs, a)
    m = eval_range(1, split, lambda t: s[t])
    if m < best_mae:
        best_alpha, best_mae = a, m

s = ewma_forecasts(xs, best_alpha)
print(f"k = {k} block, alpha tốt nhất trên đoạn huấn luyện = {best_alpha}")
print(f"MAE trên đoạn kiểm tra ({len(xs) - split} block cuối):")
print(f"  EWMA           : {eval_range(split, len(xs), lambda t: s[t]):.4f} gwei")
print(f"  Block gần nhất : {eval_range(split, len(xs), naive):.4f} gwei")
print(f"  Trung bình {N} block: {eval_range(split, len(xs), moving_avg):.4f} gwei")
