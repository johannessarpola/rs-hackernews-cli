use termion::{color, style};

pub struct Foreground;
pub struct Background;

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

    pub fn reset(&self) {
        print!("{}", color::Bg(color::Reset));
    }
}
impl Foreground {
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