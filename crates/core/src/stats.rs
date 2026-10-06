/// Trung vị của một mảng (sắp xếp tại chỗ). Trả None nếu rỗng.
pub fn median(v: &mut [f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = v.len();
    Some(if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    })
}

/// Trung bình trượt N phần tử gần nhất, tính tại vị trí t.
pub fn moving_average(series: &[f64], t: usize, n: usize) -> f64 {
    let lo = t.saturating_sub(n - 1);
    let slice = &series[lo..=t];
    slice.iter().sum::<f64>() / slice.len() as f64
}

/// Chuỗi giá trị EWMA sau mỗi quan sát.
pub fn ewma(series: &[f64], alpha: f64) -> Vec<f64> {
    if series.is_empty() {
        return vec![];
    }
    let mut out = Vec::with_capacity(series.len());
    let mut s = series[0];
    for &x in series {
        s = alpha * x + (1.0 - alpha) * s;
        out.push(s);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_le() {
        let mut v = vec![3.0, 1.0, 2.0];
        assert_eq!(median(&mut v), Some(2.0));
    }

    #[test]
    fn median_chan() {
        let mut v = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(median(&mut v), Some(2.5));
    }

    #[test]
    fn median_rong() {
        let mut v: Vec<f64> = vec![];
        assert_eq!(median(&mut v), None);
    }

    #[test]
    fn ewma_khop_tay() {
        // alpha = 1.0 → s_t = x_t
        let s = ewma(&[10.0, 20.0, 30.0], 1.0);
        assert_eq!(s, vec![10.0, 20.0, 30.0]);

        // alpha = 0.5, s0 = 10 → s1 = 0.5*20 + 0.5*10 = 15, s2 = 0.5*30 + 0.5*15 = 22.5
        let s = ewma(&[10.0, 20.0, 30.0], 0.5);
        assert_eq!(s, vec![10.0, 15.0, 22.5]);
    }

    #[test]
    fn moving_average_dung() {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(moving_average(&v, 2, 3), 2.0); // (1+2+3)/3
        assert_eq!(moving_average(&v, 4, 3), 4.0); // (3+4+5)/3
    }
}