use termion::{color, style};
use std::fmt::{Debug, Display};
use std::fmt;

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
        // todo handle both rows so that there does not need to be module on cli.rs
        match self.theme {
            Theme::Light => {
                self.fg.lgrey();
                self.bg.lwhite();
            }
            Theme::Dark => {
                self.fg.lwhite();
                self.bg.lgrey();
            }
            Theme::Green => {
                self.bg.rgb(1, 2, 4); // todo different themes
                self.fg.rgb(1, 2, 2);
            }
            Theme::Disabled => {
                // no theming
            }
        }
    }

    pub fn reset_colors(&self) {
        self.bg.reset();
        self.fg.reset();
    }
}

#[derive(Debug)]
pub enum Theme {
    Light,
    Dark,
    Green,
    Disabled,
}

impl Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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