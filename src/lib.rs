extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

mod errors;
pub use errors::*;

pub mod mediatypes;
pub mod v1;
