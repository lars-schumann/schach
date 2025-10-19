#![deny(clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(incomplete_features)]
#![feature(
    const_trait_impl,
    const_ops,
    const_convert,
    const_try,
    const_index,
    iter_collect_into,
    ascii_char,
    ascii_char_variants,
    generic_const_exprs
)]

pub mod board;
pub mod coord;
pub mod fen;
pub mod game;
pub mod mov;
pub mod piece;
pub mod player;

#[cfg(test)]
mod tests;
