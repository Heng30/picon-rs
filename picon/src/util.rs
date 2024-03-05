use chrono::{FixedOffset, TimeZone, Utc};

pub fn timestamp() -> i64 {
    Utc::now().timestamp()
}

pub fn timelapse(s: i64) -> i64 {
    timestamp() - s
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

pub fn short_time(s: i64) -> String {
    match s {
        s if s > 3600 * 24 => format!("{}d", s / (3600 * 24)),
        s if s > 3600 => format!("{}h", s / 3600),
        s if s > 60 => format!("{}m", s / 60),
        _ => format!("{s}s"),
    }
}

pub fn pretty_precent(p: f64) -> String {
    match p {
        p if p >= 100. => format!("{:.0}%", p),
        _ => format!("{:.2}%", p),
    }
}

pub fn format_number_with_commas(number_str: &str) -> String {
    if number_str.is_empty() {
        return String::default();
    }

    let chars: Vec<char> = number_str.chars().collect();
    let decimal_index = chars.iter().position(|&c| c == '.').unwrap_or(chars.len());

    let left_part = &mut chars[0..decimal_index]
        .iter()
        .rev()
        .copied()
        .collect::<Vec<char>>();

    let right_part = &number_str[decimal_index..];

    let mut chs = vec![];
    for (i, ch) in left_part.iter().enumerate() {
        chs.push(*ch);
        if (i + 1) % 3 == 0 {
            chs.push(',');
        }
    }

    if chs[chs.len() - 1] == ',' {
        chs.pop();
    }

    format!("{}{}", chs.iter().rev().collect::<String>(), right_part)
}
