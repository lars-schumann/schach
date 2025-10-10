#![deny(clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![feature(const_trait_impl, const_ops, const_convert, const_try)]
#[allow(clippy::wildcard_imports)]
pub mod game;

#[cfg(test)]
mod tests;
