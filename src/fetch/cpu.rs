use std::collections::HashMap;
use std::fs;

pub struct CpuInfo {
    pub model: String,
    pub cores: usize,
    pub threads: usize,
}

pub fn get() -> Option<CpuInfo> {
    let raw = fs::read_to_string("/proc/cpuinfo").ok()?;
    parse(&raw)
}

pub fn parse(raw: &str) -> Option<CpuInfo> {
    let mut model = String::new();
    let mut threads = 0usize;
    let mut siblings = 0usize;
    let mut cpu_cores = 0usize;
    let mut physical = HashMap::new();

    for line in raw.lines() {
        if let Some((k, v)) = line.split_once(':') {
            match k.trim() {
                "model name" => {
                    if model.is_empty() {
                        model = v.trim().to_string();
                    }
                }
                "processor" => {
                    threads += 1;
                }
                "physical id" => {
                    physical.insert(v.trim().to_string(), true);
                }
                "siblings" if siblings == 0 => {
                    siblings = v.trim().parse().unwrap_or(0);
                }
                "cpu cores" if cpu_cores == 0 => {
                    cpu_cores = v.trim().parse().unwrap_or(0);
                }
                "siblings" | "cpu cores" => {}
                _ => {}
            }
        }
    }

    if model.is_empty() {
        return None;
    }

    let phys_count = physical.len();
    let cores = if phys_count > 0 && cpu_cores > 0 {
        phys_count * cpu_cores
    } else if phys_count > 0 {
        threads.max(siblings).checked_div(phys_count).unwrap_or(0)
    } else {
        threads
    };

    Some(CpuInfo {
        model,
        cores,
        threads,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpuinfo_parse() {
        let sample = "processor	: 0
vendor_id	: GenuineIntel
cpu family	: 6
model		: 186
model name	: Intel(R) Core(TM) Ultra 7 155H
stepping	: 2
microcode	: 0x4125
cpu MHz		: 400.000
cache size	: 24576 KB
physical id	: 0
siblings	: 10
core id		: 0
cpu cores	: 5

processor	: 1
model name	: Intel(R) Core(TM) Ultra 7 155H
physical id	: 0
siblings	: 10
core id		: 1
cpu cores	: 5
";
        let cpu = parse(sample).unwrap();
        assert!(cpu.model.contains("Ultra 7"));
        assert_eq!(cpu.cores, 5);
        assert_eq!(cpu.threads, 2);
    }

    #[test]
    fn test_empty() {
        assert!(parse("").is_none());
    }
}
