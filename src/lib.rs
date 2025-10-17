#![deny(clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![feature(
    const_trait_impl,
    const_ops,
    const_convert,
    const_try,
    const_index,
    iter_collect_into
)]

pub mod board;
pub mod coord;
pub mod game;
pub mod player;
pub mod piece;
pub mod mov;

#[cfg(test)]
mod tests;
