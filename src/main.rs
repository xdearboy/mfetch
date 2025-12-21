mod config;
mod fetch;
mod theme;

use colored::*;
use fetch::{dmi, mem, spd};

fn get_bar(current: f64, total: f64) -> String {
    let width = 15;
    let percentage = (current / total).clamp(0.0, 1.0);
    let filled = (percentage * width as f64) as usize;
    let empty = width - filled;

    format!(
        "{}{} {:.1}%",
        "|".repeat(filled).green(),
        ".".repeat(empty).bright_black(),
        percentage * 100.0
    )
}

fn main() {
    let cfg = config::load();
    let theme = theme::get_theme(&cfg.theme);

    println!("\n  {} {}", (theme.primary)("󰏗"), (theme.primary)("mfetch"));
    println!("  {}", (theme.primary)("github.com/xdearboy/mfetch"));
    println!("  {}\n", "──────────────────────────────".bright_black());

    if let Some(info) = mem::get_info() {
        println!("  {}", (theme.primary)("  memory"));

        let total_kb = info.get("memtotal").map(|v| v.as_str()).unwrap_or("0");
        let avail_kb = info.get("memavailable").map(|v| v.as_str()).unwrap_or("0");
        let total_val = mem::kb_to_gb(total_kb);
        let avail_val = mem::kb_to_gb(avail_kb);
        let used_val = total_val - avail_val;

        println!(
            "    {:<12} {}",
            (theme.key)("total:"),
            (theme.val)(&format!("{:.2} gb", total_val))
        );
        println!(
            "    {:<12} {}",
            (theme.key)("available:"),
            (theme.val)(&format!("{:.2} gb", avail_val))
        );
        println!(
            "    {:<12} {}\n",
            (theme.key)("usage:"),
            get_bar(used_val, total_val)
        );
    }

    let spd_data = spd::get_data();
    let modules_opt = dmi::get_modules();

    if let Some(modules) = modules_opt {
        println!("  {}", (theme.primary)("󰬦  modules"));

        for (i, module) in modules.iter().enumerate() {
            let s_data = spd_data.as_ref().and_then(|v| v.get(i));

            let slot = module.get("locator").map(|s| s.as_str()).unwrap_or("-");
            let source_tag = if cfg.show_spd_source {
                (theme.dim)(if s_data.is_some() { " (spd)" } else { " (dmi)" })
            } else {
                "".normal()
            };

            println!(
                "\n    {} {}{}",
                "○".bright_black(),
                (theme.key)(&format!("slot {}", slot.to_lowercase())),
                source_tag
            );

            let mut maker = s_data
                .and_then(|s| s.get("module_maker"))
                .cloned()
                .unwrap_or_default();
            if maker.is_empty() || maker.to_lowercase().contains("unknown") {
                maker = module
                    .get("manufacturer")
                    .cloned()
                    .unwrap_or_else(|| "unknown".to_string());
            }

            println!(
                "      🏭 {:<10} {}",
                (theme.key)("maker:"),
                (theme.val)(&maker.to_lowercase())
            );

            if let Some(dram) = s_data.and_then(|s| s.get("dram_maker")) {
                if dram.to_lowercase() != maker.to_lowercase()
                    && !dram.to_lowercase().contains("unknown")
                {
                    println!(
                        "      💎 {:<10} {}",
                        (theme.key)("dram:"),
                        (theme.val)(&dram.to_lowercase())
                    );
                }
            }

            let part = module.get("part number").map(|s| s.as_str()).unwrap_or("-");
            println!(
                "      🏷️  {:<10} {}",
                (theme.key)("part:"),
                (theme.val)(&part.to_lowercase())
            );

            if let Some(t) = s_data.and_then(|s| s.get("timings")) {
                println!(
                    "      ⏱️  {:<10} {}",
                    (theme.key)("spd timings:"),
                    (theme.val)(t)
                );
            }

            let size = module.get("size").map(|s| s.as_str()).unwrap_or("-");
            let speed = module.get("speed").map(|s| s.as_str()).unwrap_or("-");
            let volt = module
                .get("configured voltage")
                .map(|s| s.as_str())
                .unwrap_or("-");

            println!(
                "      📦 {:<10} {}",
                (theme.key)("size:"),
                (theme.val)(&size.to_lowercase())
            );
            println!(
                "      ⚡ {:<10} {}",
                (theme.key)("speed:"),
                (theme.val)(&speed.to_lowercase())
            );
            println!(
                "      🔌 {:<10} {}",
                (theme.key)("voltage:"),
                (theme.val)(&volt.to_lowercase())
            );
        }
    }
    println!();
}
