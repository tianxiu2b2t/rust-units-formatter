const UNITS: &[&str] = &[
    "",
    "K",
    "M",
    "B",
    "T",
    "P",
    "E",
    "Z",
    "Y",
];

/// 将 `usize` 格式化为带单位的简洁字符串（如 K、M、B 等）。
///
/// # 参数
/// - `number`: 要格式化的整数值。
/// - `round`: 可选的小数精度（默认保留 2 位，最大不超过 10）。
///
/// # 返回
/// 返回一个字符串，例如 `"1.23K"`、`"999"`、`"4.56M"`。
pub fn format_usize(number: usize, round: Option<u8>) -> String {
    let mut number = number as f64;
    let mut unit_index = 0;

    while number >= 1000.0 && unit_index < UNITS.len() - 1 {
        number /= 1000.0;
        unit_index += 1;
    }

    let p = round.unwrap_or(2).min(10) as usize;
    format!("{:.p$}{}", number, UNITS[unit_index])
}