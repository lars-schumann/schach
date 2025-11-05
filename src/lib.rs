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
    const_cmp,
    const_result_trait_fn,
    const_default,
    const_clone,
    derive_const,
    iter_collect_into,
    ascii_char,
    ascii_char_variants,
    result_option_map_or_default,
    stmt_expr_attributes
)]
#![no_std]
extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;

pub mod board;
pub mod coord;
pub mod game;
pub mod mov;
pub mod move_gen;
pub mod notation;
pub mod piece;
pub mod player;
pub mod testing;
