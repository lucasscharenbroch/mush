// Need a `lib.rs` file to make these modules accessible to
// the integration tests.

mod io;
mod object;
mod refs;
mod revision; // plural to avoid name collision with `ref` keyword
mod index;
mod config;
pub mod hash;
pub mod cli;

const SEMANTIC_VERSION: &'static str = "1.0";
const PROGRAM_NAME: &'static str = "mush";
const PROGRAM_DESCRIPTION: &'static str = "A minimalist git clone";
