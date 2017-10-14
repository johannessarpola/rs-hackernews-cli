#![allow(unused_imports)]

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
        match self.theme {
            Theme::Light => {
                // todo
            }
            Theme::Dark => {
                 // todo
            }
            Theme::Green => {
                // todo
            }
            Theme::Disabled => {
                // no theming
            }
        }
    }

    pub fn reset_colors(&self) {
        // todo
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