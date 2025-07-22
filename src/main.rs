use colored::*;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

fn main() {
    println!("{}", "📦 mfetch".green().bold());
    println!(
        "{}",
        "memory-focused system info tool\nby d3v\n".bright_black()
    );

    if let Some(mem_info) = get_mem_info() {
        println!("{}", "💾 memory".cyan().bold());

        if let Some(total_kb) = mem_info.get("memtotal") {
            let gb = kb_to_gb(total_kb);
            println!("  {}: {:.2} gb", "total".yellow(), gb);
        }

        if let Some(avail_kb) = mem_info.get("memavailable") {
            let gb = kb_to_gb(avail_kb);
            println!("  {}: {:.2} gb\n", "available".yellow(), gb);
        }
    } else {
        println!("{}", "could not retrieve memory info\n".red());
    }

    if let Some(modules) = get_dmi_info() {
        println!("{}", "📗 modules".cyan().bold());
        for module in modules {
            println!(
                "  {}: {}",
                "🧠 slot".magenta(),
                module.get("locator").unwrap_or(&"-".to_string())
            );
            println!(
                "  {}: {}",
                "📦 size".magenta(),
                module.get("size").unwrap_or(&"-".to_string())
            );
            println!(
                "  {}: {}",
                "⚡ speed".magenta(),
                module.get("speed").unwrap_or(&"-".to_string())
            );
            println!(
                "  {}: {}",
                "🔠 type".magenta(),
                module.get("type").unwrap_or(&"-".to_string())
            );
            println!(
                "  {}: {}",
                "📍 bank".magenta(),
                module.get("bank locator").unwrap_or(&"-".to_string())
            );
            println!(
                "  {}: {}",
                "✅ ecc".magenta(),
                module
                    .get("error correction type")
                    .unwrap_or(&"-".to_string())
            );
            println!(
                "  {}: {}",
                "🔌 voltage".magenta(),
                module.get("configured voltage").unwrap_or(&"-".to_string())
            );
            println!();
        }
    } else {
        println!(
            "{}",
            "could not retrieve dmi memory module info (need sudo?)".red()
        );
    }
}

fn kb_to_gb(kb_str: &str) -> f64 {
    let kb = kb_str
        .split_whitespace()
        .next()
        .unwrap_or("0")
        .parse::<f64>()
        .unwrap_or(0.0);
    kb / 1024.0 / 1024.0
}

fn get_mem_info() -> Option<HashMap<String, String>> {
    let meminfo = fs::read_to_string("/proc/meminfo").ok()?;
    let mut info = HashMap::new();

    for line in meminfo.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_lowercase();
            let value = parts[1].trim().to_lowercase();
            if key == "memtotal" || key == "memavailable" {
                info.insert(key, value);
            }
        }
    }

    Some(info)
}

fn get_dmi_info() -> Option<Vec<HashMap<String, String>>> {
    let output = Command::new("dmidecode")
        .args(["--type", "17"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut modules = Vec::new();
    let mut current_block = String::new();

    for line in stdout.lines() {
        if line.trim().is_empty() {
            if current_block.contains("Size") && current_block.contains("Speed") {
                let parsed = parse_dmi_block(&current_block);
                modules.push(parsed);
            }
            current_block.clear();
        } else {
            current_block.push_str(line);
            current_block.push('\n');
        }
    }

    Some(modules)
}

fn parse_dmi_block(block: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for line in block.lines() {
        let trimmed = line.trim();
        if let Some((key, value)) = trimmed.split_once(':') {
            let key = key.trim().to_lowercase();
            let value = value.trim().to_lowercase();
            if [
                "size",
                "speed",
                "type",
                "manufacturer",
                "configured voltage",
                "bank locator",
                "locator",
                "error correction type",
            ]
            .contains(&key.as_str())
            {
                result.insert(key, value);
            }
        }
    }
    result
}
