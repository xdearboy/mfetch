use std::collections::HashMap;
use std::fs;

pub fn get_info() -> Option<HashMap<String, String>> {
    let meminfo = fs::read_to_string("/proc/meminfo").ok()?;
    Some(parse_meminfo(&meminfo))
}

pub fn parse_meminfo(raw: &str) -> HashMap<String, String> {
    let mut info = HashMap::new();
    for line in raw.lines() {
        if let Some((k, v)) = line.split_once(':') {
            info.insert(k.trim().to_lowercase(), v.trim().to_string());
        }
    }
    info
}

pub fn kb_to_gb(kb_str: &str) -> f64 {
    let kb = kb_str
        .split_whitespace()
        .next()
        .unwrap_or("0")
        .parse::<f64>()
        .unwrap_or(0.0);
    kb / 1024.0 / 1024.0
}

pub fn get_swap(info: &HashMap<String, String>) -> Option<(f64, f64)> {
    let total = info.get("swaptotal").map(|s| kb_to_gb(s))?;
    let free = info.get("swapfree").map(|s| kb_to_gb(s))?;
    if total > 0.0 {
        Some((total, free))
    } else {
        None
    }
}

pub struct HugePageInfo {
    pub total: f64,
    pub free: f64,
    pub size_kb: f64,
}

pub fn get_hugepages(info: &HashMap<String, String>) -> Option<HugePageInfo> {
    let total: f64 = info
        .get("hugetlb")
        .and_then(|s| s.split_whitespace().next())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);
    let free = info
        .get("hugepages_free")
        .map(|s| s.parse().unwrap_or(0.0))
        .unwrap_or(0.0);
    let size_kb = info
        .get("hugepagesize")
        .and_then(|s| s.split_whitespace().next())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);

    if total > 0.0 && size_kb > 0.0 {
        let total_pages = total / size_kb;
        Some(HugePageInfo {
            total: total_pages,
            free,
            size_kb,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kb_to_gb() {
        let val = kb_to_gb("16384000 kB");
        assert!((val - 15.625).abs() < 0.01);
    }

    #[test]
    fn test_kb_to_gb_empty() {
        assert_eq!(kb_to_gb(""), 0.0);
    }

    #[test]
    fn test_kb_to_gb_garbage() {
        assert_eq!(kb_to_gb("nope"), 0.0);
    }

    #[test]
    fn test_parse_meminfo() {
        let sample = "MemTotal:       16384000 kB
MemFree:          8200000 kB
MemAvailable:    12000000 kB
SwapTotal:        2048000 kB
SwapFree:         2048000 kB
";
        let info = parse_meminfo(sample);
        assert_eq!(info.get("memtotal").unwrap(), "16384000 kB");
        assert_eq!(info.get("memfree").unwrap(), "8200000 kB");
        assert_eq!(info.get("memavailable").unwrap(), "12000000 kB");
    }

    #[test]
    fn test_get_swap() {
        let sample = "SwapTotal: 2048000 kB\nSwapFree: 1024000 kB\n";
        let info = parse_meminfo(sample);
        let (total, free) = get_swap(&info).unwrap();
        assert!((total - 1.953125).abs() < 0.001);
        assert!((free - 0.9765625).abs() < 0.001);
    }

    #[test]
    fn test_get_swap_none_when_zero() {
        let sample = "SwapTotal: 0 kB\nSwapFree: 0 kB\n";
        let info = parse_meminfo(sample);
        assert!(get_swap(&info).is_none());
    }

    #[test]
    fn test_hugepages_none() {
        let info = HashMap::new();
        assert!(get_hugepages(&info).is_none());
    }

    #[test]
    fn test_hugepages_parse() {
        let sample = "HugePages_Total:      64
HugePages_Free:       64
HugePages_Rsvd:        0
HugePages_Surp:        0
Hugepagesize:       2048 kB
Hugetlb:          131072 kB
";
        let info = parse_meminfo(sample);
        let hp = get_hugepages(&info).unwrap();
        assert_eq!(hp.total as u64, 64);
        assert_eq!(hp.free as u64, 64);
        assert_eq!(hp.size_kb as u64, 2048);
    }
}
