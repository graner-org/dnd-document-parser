#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]
pub mod parsers {
    pub mod spells;
}

pub mod models {
    pub mod common;
    pub mod creatures;
    pub mod items;
    pub mod spells;
}

pub mod utils {
    pub mod compare;
    pub mod error;
    pub mod traits;
}
