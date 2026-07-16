mod config;
mod fetch;
mod theme;

use colored::*;
use fetch::{cpu, dmi, gpu, mem, pressure, spd, system};

fn bar(current: f64, total: f64) -> String {
    let width = 15;
    let pct = (current / total).clamp(0.0, 1.0);
    let filled = (pct * width as f64) as usize;

    format!(
        "{}{} {:.1}%",
        "|".repeat(filled).green(),
        ".".repeat(width - filled).bright_black(),
        pct * 100.0
    )
}

fn kv(theme: &theme::Theme, key: &str, val: &str) {
    println!(
        "    {:<12} {}",
        (theme.key)(&format!("{}:", key)),
        (theme.val)(val)
    );
}

fn section(title: &str, theme: &theme::Theme) {
    println!("  {}", (theme.primary)(title));
}

fn print_system(cfg: &config::MfetchConfig, theme: &theme::Theme) {
    if !cfg.show_system {
        return;
    }
    if let Some(sys) = system::get() {
        section("󰇄  system", theme);
        kv(theme, "host", &sys.hostname);
        kv(theme, "os", &sys.os);
        kv(theme, "kernel", &sys.kernel);
        kv(theme, "uptime", &sys.uptime);
        println!();
    }
}

fn print_cpu(cfg: &config::MfetchConfig, theme: &theme::Theme) {
    if !cfg.show_cpu {
        return;
    }
    if let Some(info) = cpu::get() {
        section("󰻠  cpu", theme);
        kv(theme, "model", &info.model);
        kv(theme, "cores", &info.cores.to_string());
        kv(theme, "threads", &info.threads.to_string());
        println!();
    }
}

fn print_mem(info: &std::collections::HashMap<String, String>, theme: &theme::Theme) {
    let total_kb = info.get("memtotal").map(|v| v.as_str()).unwrap_or("0");
    let avail_kb = info.get("memavailable").map(|v| v.as_str()).unwrap_or("0");
    let total = mem::kb_to_gb(total_kb);
    let avail = mem::kb_to_gb(avail_kb);
    let used = total - avail;

    section("  memory", theme);
    kv(theme, "total", &format!("{:.2} gb", total));
    kv(theme, "available", &format!("{:.2} gb", avail));
    println!("    {:<12} {}\n", (theme.key)("usage:"), bar(used, total));
}

fn print_swap(
    info: &std::collections::HashMap<String, String>,
    cfg: &config::MfetchConfig,
    theme: &theme::Theme,
) {
    if !cfg.show_swap {
        return;
    }
    if let Some((total, free)) = mem::get_swap(info) {
        let used = total - free;
        let pct = if total > 0.0 {
            (used / total) * 100.0
        } else {
            0.0
        };

        section("󰓡  swap", theme);
        kv(theme, "total", &format!("{:.2} gb", total));
        kv(theme, "used", &format!("{:.2} gb ({:.1}%)", used, pct));
        kv(theme, "free", &format!("{:.2} gb", free));
        println!();
    }
}

fn print_hugepages(
    info: &std::collections::HashMap<String, String>,
    cfg: &config::MfetchConfig,
    theme: &theme::Theme,
) {
    if !cfg.show_hugepages {
        return;
    }
    if let Some(hp) = mem::get_hugepages(info) {
        let used = hp.total - hp.free;
        let size_mb = hp.size_kb / 1024.0;

        section("󰌽  hugepages", theme);
        kv(theme, "size", &format!("{:.0} mb", size_mb));
        kv(theme, "total", &format!("{:.0}", hp.total));
        kv(theme, "used", &format!("{:.0}", used));
        kv(theme, "free", &format!("{:.0}", hp.free));
        println!();
    }
}

fn print_pressure(cfg: &config::MfetchConfig, theme: &theme::Theme) {
    if !cfg.show_pressure {
        return;
    }
    if let Some(p) = pressure::get() {
        section("󱗨  mem pressure", theme);
        kv(theme, "avg10", &format!("{:.2}%", p.avg10));
        kv(theme, "avg60", &format!("{:.2}%", p.avg60));
        kv(theme, "avg300", &format!("{:.2}%", p.avg300));
        println!();
    }
}

fn print_module(
    i: usize,
    module: &std::collections::HashMap<String, String>,
    spd_data: &Option<Vec<std::collections::HashMap<String, String>>>,
    cfg: &config::MfetchConfig,
    theme: &theme::Theme,
    debug: bool,
) {
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
        if dram.to_lowercase() != maker.to_lowercase() && !dram.to_lowercase().contains("unknown") {
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
            (theme.key)("timings:"),
            (theme.val)(t)
        );
    }

    if let Some(ranks) = s_data.and_then(|s| s.get("ranks")) {
        println!(
            "      ≡ {:<10} {}",
            (theme.key)("ranks:"),
            (theme.val)(&ranks.to_lowercase())
        );
    }

    if let Some(mem_type) = s_data.and_then(|s| s.get("mem_type")) {
        println!(
            "      ⬡ {:<10} {}",
            (theme.key)("type:"),
            (theme.val)(&mem_type.to_lowercase())
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

    if debug {
        println!(
            "      {} {:<10} {} {} keys",
            (theme.dim)("debug:"),
            (theme.dim)("dmi"),
            (theme.dim)(&module.len().to_string()),
            (theme.dim)("")
        );
        if let Some(s) = s_data {
            println!(
                "      {} {:<10} {} {} keys",
                (theme.dim)("debug:"),
                (theme.dim)("spd"),
                (theme.dim)(&s.len().to_string()),
                (theme.dim)("")
            );
        }
    }
}

fn print_modules(cfg: &config::MfetchConfig, theme: &theme::Theme, debug: bool) {
    if !cfg.show_modules {
        return;
    }
    let spd_data = spd::get_data();
    let modules_opt = dmi::get_modules();

    if let Some(modules) = modules_opt {
        section("󰬦  modules", theme);

        for (i, module) in modules.iter().enumerate() {
            print_module(i, module, &spd_data, cfg, theme, debug);
        }
        println!();
    } else if debug {
        println!(
            "  {}\n",
            (theme.dim)("no modules: dmidecode not available or no dimms installed")
        );
    }
}

fn print_gpu(cfg: &config::MfetchConfig, theme: &theme::Theme) {
    if !cfg.show_gpu {
        return;
    }
    if let Some(gpus) = gpu::get_nvidia() {
        section("󰾲  gpu", theme);
        for g in &gpus {
            kv(theme, "name", &g.name);
            kv(theme, "vram total", &g.vram_total);
            kv(theme, "vram used", &g.vram_used);
            println!();
        }
    }
}

fn main() {
    let args = config::parse_args();
    let cfg = config::load();
    let theme = theme::get_theme(&cfg.theme);

    println!("\n  {} {}", (theme.primary)("󰏗"), (theme.primary)("mfetch"));
    println!("  {}", (theme.primary)(env!("CARGO_PKG_REPOSITORY")));
    println!("  {}\n", "──────────────────────────────".bright_black());

    print_system(&cfg, &theme);
    print_cpu(&cfg, &theme);

    if let Some(info) = mem::get_info() {
        print_mem(&info, &theme);
        print_swap(&info, &cfg, &theme);
        print_hugepages(&info, &cfg, &theme);
    } else if args.debug {
        eprintln!("warning: failed to read /proc/meminfo");
    }

    print_pressure(&cfg, &theme);
    print_modules(&cfg, &theme, args.debug);
    print_gpu(&cfg, &theme);

    if args.debug {
        println!(
            "  {} config: theme={} system={} cpu={} spd={} swap={} hp={} pressure={} modules={} gpu={}",
            (theme.dim)("debug:"),
            cfg.theme,
            cfg.show_system,
            cfg.show_cpu,
            cfg.show_spd_source,
            cfg.show_swap,
            cfg.show_hugepages,
            cfg.show_pressure,
            cfg.show_modules,
            cfg.show_gpu,
        );
    }
    println!();
}
