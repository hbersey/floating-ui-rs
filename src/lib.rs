#[macro_use]
extern crate static_assertions;

mod floating_ui;
pub use floating_ui::*;

pub mod utils_;

mod error;
pub use error::{Error,Result};
