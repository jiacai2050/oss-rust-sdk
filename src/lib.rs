#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate log;

pub mod async_object;
pub mod async_service;
pub mod errors;
pub mod object;
pub mod oss;
pub mod prelude;
pub mod service;

mod auth;
mod utils;
