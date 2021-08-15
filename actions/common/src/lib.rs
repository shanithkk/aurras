mod types;
pub use types::{Context, Trigger};
mod mock;

#[cfg(test)]
#[macro_use]
extern crate derive_new;


#[cfg(feature = "mock_containers")]
pub use mock::mock_containers;