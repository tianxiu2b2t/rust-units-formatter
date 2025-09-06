//
const UNITS: &[&str] = &[
    "iB",
    "KiB",
    "MiB",
    "GiB",
    "TiB",
    "PiB",
    "EiB",
    "ZiB",
    "YiB",
];

/// 将字节数格式化为带单位的字符串（如 KiB、MiB、GiB 等）。
///
/// # 参数
/// - `bytes`: 原始字节数（`usize`）。
/// - `round`: 可选的小数精度（默认保留 2 位，最大不超过 10）。
///
/// # 返回
/// 返回一个字符串，例如 `"1.23KiB"`、`"999iB"`、`"4.56MiB"`。
pub fn format_bytes(bytes: usize, round: Option<u8>) -> String {
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    let precision = round.unwrap_or(2).min(10) as usize;
    format!("{:.precision$}{}", size, UNITS[unit_index])
}