use std::process::Command;

pub struct GpuInfo {
    pub name: String,
    pub vram_total: String,
    pub vram_used: String,
}

pub fn get_nvidia() -> Option<Vec<GpuInfo>> {
    let output = Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,memory.total,memory.used",
            "--format=csv,noheader,nounits",
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut gpus = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if parts.len() >= 3 {
            let total_mib: f64 = parts[1].parse().unwrap_or(0.0);
            let used_mib: f64 = parts[2].parse().unwrap_or(0.0);
            gpus.push(GpuInfo {
                name: parts[0].to_string(),
                vram_total: format!("{:.0} mib", total_mib),
                vram_used: format!("{:.0} mib", used_mib),
            });
        }
    }

    if gpus.is_empty() {
        None
    } else {
        Some(gpus)
    }
}
