use std::collections::HashMap;
use std::process::Command;

pub fn get_modules() -> Option<Vec<HashMap<String, String>>> {
    let output = Command::new("dmidecode")
        .args(["--type", "17"])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_modules(&stdout)
}

pub fn parse_modules(raw: &str) -> Option<Vec<HashMap<String, String>>> {
    let mut modules = Vec::new();

    for block in raw.split("\nHandle ") {
        if block.contains("Size:") && !block.contains("No Module Installed") {
            let mut map = HashMap::new();
            for line in block.lines() {
                if let Some((k, v)) = line.split_once(':') {
                    map.insert(k.trim().to_lowercase(), v.trim().to_string());
                }
            }
            modules.push(map);
        }
    }
    if modules.is_empty() {
        None
    } else {
        Some(modules)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        assert!(parse_modules("").is_none());
    }

    #[test]
    fn test_parse_no_module() {
        let sample = "Handle 0x0001, DMI type 17, 40 bytes
Memory Device
\tArray Handle: 0x0000
\tError Information Handle: Not Provided
\tTotal Width: Unknown
\tData Width: Unknown
\tSize: No Module Installed
";
        assert!(parse_modules(sample).is_none());
    }

    #[test]
    fn test_parse_one_module() {
        let sample = "Handle 0x0010, DMI type 17, 92 bytes
Memory Device
\tArray Handle: 0x0000
\tError Information Handle: Not Provided
\tTotal Width: 64 bits
\tData Width: 64 bits
\tSize: 16 GB
\tForm Factor: SODIMM
\tSet: None
\tLocator: DIMM 0
\tBank Locator: P0 CHANNEL A
\tType: DDR5
\tType Detail: Synchronous
\tSpeed: 5600 MT/s
\tManufacturer: Samsung
\tSerial Number: 12345678
\tAsset Tag: Not Specified
\tPart Number: M425R2GA3BB0-CWM
\tRank: 2
\tConfigured Memory Speed: 5200 MT/s
\tMinimum Voltage: 1.1 V
\tMaximum Voltage: 1.1 V
\tConfigured Voltage: 1.1 V
";
        let modules = parse_modules(sample).unwrap();
        assert_eq!(modules.len(), 1);
        let m = &modules[0];
        assert_eq!(m.get("locator").unwrap(), "DIMM 0");
        assert_eq!(m.get("size").unwrap(), "16 GB");
        assert_eq!(m.get("type").unwrap(), "DDR5");
        assert_eq!(m.get("speed").unwrap(), "5600 MT/s");
        assert_eq!(m.get("manufacturer").unwrap(), "Samsung");
        assert_eq!(m.get("part number").unwrap(), "M425R2GA3BB0-CWM");
        assert_eq!(m.get("rank").unwrap(), "2");
        assert_eq!(m.get("configured voltage").unwrap(), "1.1 V");
    }

    #[test]
    fn test_parse_two_modules() {
        let sample = "Handle 0x0010, DMI type 17, 92 bytes
Memory Device
\tArray Handle: 0x0000
\tTotal Width: 64 bits
\tData Width: 64 bits
\tSize: 8 GB
\tLocator: DIMM 0
\tManufacturer: Crucial

Handle 0x0011, DMI type 17, 92 bytes
Memory Device
\tArray Handle: 0x0000
\tTotal Width: 64 bits
\tData Width: 64 bits
\tSize: 8 GB
\tLocator: DIMM 1
\tManufacturer: Crucial
";
        let modules = parse_modules(sample).unwrap();
        assert_eq!(modules.len(), 2);
        assert_eq!(modules[0].get("locator").unwrap(), "DIMM 0");
        assert_eq!(modules[1].get("locator").unwrap(), "DIMM 1");
    }
}
