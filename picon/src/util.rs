use chrono::{FixedOffset, TimeZone, Utc};

pub fn timestamp() -> i64 {
    Utc::now().timestamp()
}

#[allow(dead_code)]
pub fn time_from_utc_seconds(sec: i64) -> String {
    let time = FixedOffset::east_opt(0)
        .unwrap()
        .timestamp_opt(sec, 0)
        .unwrap();
    format!("{}", time.format("%Y-%m-%d %H:%M"))
}

pub fn pretty_price(price: f64) -> String {
    match price {
        p if p < 0.000_01 => format!("{:.6}", p),
        p if p < 0.000_1 => format!("{:.5}", p),
        p if p < 0.001 => format!("{:.4}", p),
        p if p < 0.01 => format!("{:.3}", p),
        p if p > 10_000. => format!("{:.0}", p),
        _ => format!("{:.2}", price),
    }
}
