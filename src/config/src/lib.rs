#![crate_name = "amethyst_config"]
#![crate_type = "lib"]
#![doc(html_logo_url = "http://tinyurl.com/hgsb45k")]

//! Loading configuration (.yaml/.yml) files into a structure for easy usage
//!
//! # Basic usage:
//! ```rust
//! #[macro_use]
//! extern crate amethyst_config;
//!
//! use amethyst_config::Element;
//! use std::path::Path;
//!
//! config!(struct Config {
//!     pub amount: i32 = 50,
//! });
//!
//! fn main() {
//!     let config = Config::default();
//!     assert_eq!(config.amount, 50);
//! }
//! ```
//!
//! `Config` is the name of the rust struct that will be generated by the macro. It can be anything
//! as long as it would be a valid struct in its context. e.g. no other structs by the same name.
//!
//! The inner fields of `Config` can be summed up as:
//!
//! ```ignore rust
//! name: type = default,
//! ```
//!
//! The field name will be looked up when attempting to load from a .yml/.yaml file. If it is
//! found then the value will be converted from a yaml type to a rust type and assigned to the
//! field.
//!
//! In the case that the value is either the wrong type from the field's or simply cannot be
//! found in the file, the field will be defaulted to `default`.
//!
//!
//! In addition to basic types, any struct created through the `config!` macro will automatically
//! implement the [`Element`](trait.Element.html) trait. Meaning you can nest configuration structs
//! inside of eachother as such:
//!
//! ```rust
//! # #[macro_use] extern crate amethyst_config;
//! # use amethyst_config::Element;
//! # use std::path::Path;
//! config!(struct NestedConfig {
//!     pub some_field: [i64; 3] = [1, 2, 3],
//! });
//!
//! config!(struct Config {
//!     pub nested: NestedConfig = NestedConfig::default(),
//! });
//! # fn main() { }
//! ```
//!
//! # External .yml/.yaml files
//! In the event that a config is getting too long, you can define it in the .yml/.yaml file as
//! "extern"
//!
//! example:
//!
//! ```yaml
//! display: "extern"
//! ```
//!
//! This works similarly to rust's module system. It will first search for "\\display\\config.yml"
//! in the current context. If it cannot find it, then it will look for "\\display.yml". If it
//! cannot find either of these, then the value will be defaulted in addition to `display` being
//! overwritten if you called `write_file()`.
//!
//! # Enums
//! When `config!` is used on an enum type, it automatically implements the Element trait. However,
//! it does not provide possibilities for data holding enums, only a simple options list enum.
//!
//! ```rust
//! # #[macro_use] extern crate amethyst_config;
//! # use amethyst_config::Element;
//! # use std::path::Path;
//! config!(enum EnumName {
//!     Option1,
//!     Option2,
//! });
//!
//! config!(struct Config {
//!     pub field: EnumName = EnumName::Option2,
//! });
//!
//! fn main() {
//!     let config = Config::default();
//!     assert_eq!(config.field, EnumName::Option2);
//! }
//! ```
//!
//! # Documentation/Commenting
//! Normally when constructing a config you would want to have a small description as to what the
//! fields will be used for. And possibly defaults.
//!
//! ```rust
//! # #[macro_use] extern crate amethyst_config;
//! # use amethyst_config::Element;
//! # use std::path::Path;
//! config!(
//!     struct Config {
//!         /// Width and height of the window on initialization. Defaults to 1024x768.
//!         pub dimensions: [u16; 2] = [1024, 768],
//!     }
//! );
//! # fn main() { }
//! ```
//!
//! If the macro has problems expanding, then you may want to check whether you have the
//! documentation on the line before the field and that you have te `pub` identifier before the
//! field name.

extern crate yaml_rust;

use std::path::Path;
pub use yaml_rust::Yaml;

#[macro_use]
mod definitions;
mod yaml;

pub use yaml::{Element, to_string};
pub use definitions::{ConfigMeta, ConfigError};

config!(struct LoggingConfig {
    pub file_path: String = "new_project.log".to_string(),
    pub output_level: String = "warn".to_string(),
    pub logging_level: String = "debug".to_string(),
});

config!(
    struct DisplayConfig {
        pub title: String = "Amethyst game".to_string(),
        pub brightness: f64 = 1.0,
        pub fullscreen: bool = false,
        pub dimensions: (u16, u16) = (1024, 768),
        pub min_dimensions: Option<(u16, u16)> = None,
        pub max_dimensions: Option<(u16, u16)> = None,
        pub vsync: bool = true,
        pub multisampling: u16 = 0,
        pub visibility: bool = true,
    }
);

config!(struct Config {
    /// Configuration for display and graphics
    pub display: DisplayConfig = DisplayConfig::default(),

    /// Configuration for output
    pub logging: LoggingConfig = LoggingConfig::default(),
});
