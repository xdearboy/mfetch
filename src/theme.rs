use colored::*;

pub struct Theme {
    pub primary: Box<dyn Fn(&str) -> ColoredString>,
    pub key: Box<dyn Fn(&str) -> ColoredString>,
    pub val: Box<dyn Fn(&str) -> ColoredString>,
    pub dim: Box<dyn Fn(&str) -> ColoredString>,
}

pub fn get_theme(name: &str) -> Theme {
    match name.to_lowercase().as_str() {
        "catppuccin" => Theme {
            primary: Box::new(|s| s.truecolor(137, 180, 250).bold()),
            key: Box::new(|s| s.truecolor(203, 166, 247)),
            val: Box::new(|s| s.truecolor(166, 227, 161).bold()),
            dim: Box::new(|s| s.truecolor(108, 112, 134).italic()),
        },
        "tokyonight" => Theme {
            primary: Box::new(|s| s.truecolor(122, 162, 247).bold()),
            key: Box::new(|s| s.truecolor(187, 154, 247)),
            val: Box::new(|s| s.truecolor(158, 206, 106).bold()),
            dim: Box::new(|s| s.truecolor(86, 95, 137).italic()),
        },
        "rosepine" => Theme {
            primary: Box::new(|s| s.truecolor(235, 188, 186).bold()),
            key: Box::new(|s| s.truecolor(196, 167, 231)),
            val: Box::new(|s| s.truecolor(49, 116, 143).bold()),
            dim: Box::new(|s| s.truecolor(110, 106, 134).italic()),
        },
        "gruvbox" => Theme {
            primary: Box::new(|s| s.truecolor(216, 166, 87).bold()),
            key: Box::new(|s| s.truecolor(234, 105, 98)),
            val: Box::new(|s| s.truecolor(169, 182, 101).bold()),
            dim: Box::new(|s| s.truecolor(146, 131, 116).italic()),
        },
        "nord" => Theme {
            primary: Box::new(|s| s.truecolor(143, 188, 187).bold()),
            key: Box::new(|s| s.truecolor(129, 161, 193)),
            val: Box::new(|s| s.truecolor(163, 190, 140).bold()),
            dim: Box::new(|s| s.truecolor(76, 86, 106).italic()),
        },
        "dracula" => Theme {
            primary: Box::new(|s| s.truecolor(189, 147, 249).bold()),
            key: Box::new(|s| s.truecolor(255, 121, 198)),
            val: Box::new(|s| s.truecolor(80, 250, 123).bold()),
            dim: Box::new(|s| s.truecolor(98, 114, 164).italic()),
        },
        "everforest" => Theme {
            primary: Box::new(|s| s.truecolor(167, 192, 128).bold()),
            key: Box::new(|s| s.truecolor(214, 153, 182)),
            val: Box::new(|s| s.truecolor(131, 192, 146).bold()),
            dim: Box::new(|s| s.truecolor(122, 132, 122).italic()),
        },
        "neon" => Theme {
            primary: Box::new(|s| s.truecolor(255, 0, 255).bold()),
            key: Box::new(|s| s.truecolor(0, 255, 255)),
            val: Box::new(|s| s.truecolor(255, 255, 0).bold()),
            dim: Box::new(|s| s.truecolor(100, 100, 100).italic()),
        },
        _ => Theme {
            primary: Box::new(|s| s.cyan().bold()),
            key: Box::new(|s| s.magenta()),
            val: Box::new(|s| s.green().bold()),
            dim: Box::new(|s| s.bright_black().italic()),
        },
    }
}
