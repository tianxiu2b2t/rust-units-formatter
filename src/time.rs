use std::time::Duration;

pub const TIME_UNITS: [(&str, f64); 6] = [
    ("ns", 1.0),
    ("μs", 1e3),
    ("ms", 1e3),
    ("s", 1e3),
    ("min", 60.0),
    ("h", 60.0),
];

/// 将 `Duration` 格式化为带单位的字符串，自动选择最合适的单位。
///
/// # 参数
/// - `duration`: 要格式化的时间长度。
/// - `round`: 可选的小数精度（默认保留 2 位，最大不超过 10）。
///
/// # 返回
/// 返回一个字符串，例如 `"1.50min"`、`"123.45ms"` 等。
pub fn format_duration(duration: Duration, round: Option<u8>) -> String {
    let p = round.unwrap_or(2).min(10) as usize;
    let ns = duration.as_nanos();
    if ns < 1_000 {
        return format!("{:.p$}ns", ns);
    }
    let μs = duration.as_micros();
    if μs < 1_000 {
        return format!("{:.p$}μs", μs);
    }
    let ms = duration.as_millis();
    if ms < 1_000 {
        return format!("{:.p$}ms", ms);
    }
    let sec = duration.as_secs();
    if sec < 60 {
        return format!("{:.p$}s", sec);
    }
    let sec = duration.as_secs_f64();
    let min = sec / 60.0;
    if min < 60.0 {
        return format!("{:.p$}min", min);
    }
    let h = min / 60.0;
    format!("{:.p$}h", h)

}

/// 将 `Duration` 格式化为更人类友好的字符串，按小时、分钟、秒显示。
///
/// # 参数
/// - `duration`: 要格式化的时间长度。
///
/// # 返回
/// 返回一个字符串，例如 `"01h 30m 45s"`、`"05m 12s"`、`"00s 123ms"` 等。
pub fn format_better_duration(duration: Duration) -> String {
    let sec = duration.as_secs();
    let min = sec / 60;
    let h = min / 60;

    if h > 0 {
        return format!("{:02}h {:02}m {:02}s", h, min, sec);
    } else if min > 0 {
        return format!("{:02}m {:02}s", min, sec);
    } 
    let ms = duration.subsec_millis();
    format!("{:02}s {:03}ms", sec, ms)

}