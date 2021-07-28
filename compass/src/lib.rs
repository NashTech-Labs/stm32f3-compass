#![no_std]
/// Package config contains the implementation of Code for Compass in initialization file.
mod config {
    pub mod initialization;
}
/// Making functions & enum of initialization-file public for main to use.
pub use config::initialization::*;
