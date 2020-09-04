#![deny(
    // The following are allowed by default lints according to
    // https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html
    anonymous_parameters,
    bare_trait_objects,
    // box_pointers,
    elided_lifetimes_in_paths,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    // unreachable_pub, // This lint conflicts with clippy::redundant_pub_crate
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    // unused_results,
    variant_size_differences,

    // Treat warnings as errors
    warnings,

    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]
#![allow(
    clippy::module_name_repetitions, // repeation of module name in a struct name is not big deal
    clippy::implicit_return, // This is rust style
    clippy::panic,
)]

//! Traits for datenlord.
pub mod arithmetic;
pub mod conversion;
