use wezterm_dynamic::{FromDynamic, ToDynamic};

#[derive(Debug, Default, Clone, ToDynamic, PartialEq, Eq, FromDynamic)]
pub enum WindowLevel {
    AlwaysOnBottom = -1,
    #[default]
    Normal = 0,
    AlwaysOnTop = 3,
}

#[derive(Debug, Default, Clone, ToDynamic, PartialEq, Eq, FromDynamic)]
pub enum Win32WindowAppearance {
    #[default]
    System,
    Light,
    Dark,
}