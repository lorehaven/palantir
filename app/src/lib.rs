#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![deny(missing_debug_implementations)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::redundant_pub_crate)]
#![allow(clippy::similar_names)]
#![allow(clippy::struct_field_names)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::needless_pass_by_value)]
#![cfg_attr(test, deny(rust_2018_idioms))]

mod pages;

pub mod components;
pub mod shell;
pub mod utils;
pub mod web_app;
