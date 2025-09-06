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
    let sec = duration.as_secs_f64();
    let min = sec / 60.0;
    let h = min / 60.0;

    let precision = round.unwrap_or(2).min(10) as usize;

    if h > 0.0 {
        return format!("{:.precision$}h", h);
    } else if min > 0.0 {
        return format!("{:.precision$}min", min);
    } else if sec > 0.0 {
        return format!("{:.precision$}s", sec);
    } 

    let ms = duration.as_millis();
    let μs = duration.as_micros();
    let ns = duration.as_nanos();
    if ms > 0 {
        return format!("{:.precision$}ms", ms);
    } else if μs > 0 {
        return format!("{:.precision$}μs", μs);
    } else if ns > 0 {
        return format!("{:.precision$}ns", ns);
    }
    "0ns".to_string()
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