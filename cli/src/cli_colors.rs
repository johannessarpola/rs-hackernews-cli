use termion::{color, style};

pub struct Foreground;
pub struct Background;


pub struct CliColoring {
    theme: Theme,
    fg: Foreground,
    bg: Background,
}

impl CliColoring {
    pub fn new(theme: Theme) -> CliColoring {
        CliColoring {
            theme: theme,
            fg: Foreground,
            bg: Background,
        }
    }

    pub fn zebra_coloring(&self) {
        match self.theme {
            ref Light => {
                self.bg.lgrey();
                self.fg.lwhite();
            }
            ref Dark => {
                // todo
                self.bg.lwhite();
                self.fg.lgrey();
            }
            ref Green => {
                self.bg.rgb(5, 5, 5); // todo different themes
                self.fg.rgb(1, 1, 1);
            }
        }
    }

    pub fn reset_colors(&self) {
        self.bg.reset();
        self.fg.reset();
    }
}

pub enum Theme {
    Light,
    Dark,
    Green,
}


impl Background {
    pub fn lgrey(&self) {
        print!("{}", color::Bg(color::Rgb(4, 4, 4)));
    }
    pub fn lgreen(&self) {
        print!("{}", color::Bg(color::LightGreen));
    }
    pub fn rgb(&self, r: u8, g: u8, b: u8) {
        print!("{}", color::Fg(color::Rgb(r, b, g)));
    }

    pub fn lwhite(&self) {
        print!("{}", color::Bg(color::LightWhite));
    }

    pub fn reset(&self) {
        print!("{}", color::Bg(color::Reset));
    }
}
impl Foreground {
    pub fn lgrey(&self) {
        print!("{}", color::Fg(color::Rgb(4, 4, 4)));
    }

    pub fn red(&self) {
        print!("{}", color::Fg(color::Red));
    }

    pub fn green(&self) {
        print!("{}", color::Fg(color::Green));
    }

    pub fn lgreen(&self) {
        print!("{}", color::Fg(color::LightGreen));
    }

    pub fn lwhite(&self) {
        print!("{}", color::Fg(color::LightWhite));
    }

    pub fn blue(&self) {
        print!("{}", color::Fg(color::Blue));
    }

    pub fn rgb(&self, r: u8, g: u8, b: u8) {
        print!("{}", color::Fg(color::Rgb(r, b, g)));
    }

    pub fn reset(&self) {
        print!("{}", color::Fg(color::Reset));
    }
}