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

pub mod game;

#[cfg(test)]
mod tests;
