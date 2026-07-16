use std::fs;

#[derive(Debug)]
pub struct MemPressure {
    pub avg10: f64,
    pub avg60: f64,
    pub avg300: f64,
}

pub fn get() -> Option<MemPressure> {
    let raw = fs::read_to_string("/proc/pressure/memory").ok()?;
    parse(&raw)
}

pub fn parse(raw: &str) -> Option<MemPressure> {
    let mut avg10 = 0.0;
    let mut avg60 = 0.0;
    let mut avg300 = 0.0;

    for line in raw.lines() {
        if let Some(rest) = line.strip_prefix("some ") {
            for part in rest.split_whitespace() {
                if let Some(val) = part.strip_prefix("avg10=") {
                    avg10 = val.parse().unwrap_or(0.0);
                } else if let Some(val) = part.strip_prefix("avg60=") {
                    avg60 = val.parse().unwrap_or(0.0);
                } else if let Some(val) = part.strip_prefix("avg300=") {
                    avg300 = val.parse().unwrap_or(0.0);
                }
            }
            break;
        }
    }

    Some(MemPressure {
        avg10,
        avg60,
        avg300,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure_parse() {
        let sample = "some avg10=1.50 avg60=0.75 avg300=0.25 total=424242
full avg10=0.10 avg60=0.05 avg300=0.01 total=12345
";
        let p = parse(sample).unwrap();
        assert!((p.avg10 - 1.50).abs() < 0.001);
        assert!((p.avg60 - 0.75).abs() < 0.001);
        assert!((p.avg300 - 0.25).abs() < 0.001);
    }
}
