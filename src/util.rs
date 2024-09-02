use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;
use regex::Regex;

pub fn parse_duration(input: &str) -> Result<Duration, String> {
    // 定义支持的时间单位及其对应的秒数
    let mut units: HashMap<&str, u64> = HashMap::new();
    units.insert("d", 86400); // 一天的秒数
    units.insert("h", 3600);  // 一小时的秒数
    units.insert("m", 60);    // 一分钟的秒数
    units.insert("s", 1);     // 一秒的秒数

    // 使用正则表达式解析字符串
    let re = Regex::new(r"(?P<value>\d+)(?P<unit>[dhms])").map_err(|e| e.to_string())?;
    let mut total_seconds = 0;

    for cap in re.captures_iter(input) {
        let value: u64 = cap["value"].parse().map_err(|_| "Invalid number".to_string())?;
        let unit = &cap["unit"];
        if let Some(&unit_seconds) = units.get(unit) {
            total_seconds += value * unit_seconds;
        } else {
            return Err(format!("Unknown time unit: {}", unit));
        }
    }

    Ok(Duration::from_secs(total_seconds))
}


pub fn verify_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
