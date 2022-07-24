//! Generate CLI completions for your [`clap::Command`]s.
//!
//! Supported shells:
//!
//! - [nu][nushell]
//!
//! If you want support for other shells, please file an issue with links to
//! resources explaining the structure of your shell's completion files.
//!
//! See also:
//!
//! - [clap_complete]: first-party core completion generation support for
//!   - [bash]
//!   - [elvish]
//!   - [fish]
//!   - [pwsh]
//!   - [zsh]
//! - [clap_complete_fig]: first-party completion generation support for [fig]
//!
//! [bash]: https://www.gnu.org/software/bash/
//! [clap_complete]: https://lib.rs/crates/clap_complete
//! [clap_complete_fig]: https://lib.rs/crates/clap_complete_fig
//! [elvish]: https://elv.sh/
//! [fig]: https://fig.io/
//! [fish]: https://fishshell.com/
//! [nushell]: https://www.nushell.sh/
//! [pwsh]: https://docs.microsoft.com/en-us/powershell/
//! [zsh]: https://zsh.sourceforge.io/

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    elided_lifetimes_in_paths,
    trivial_casts,
    unused_allocation,
    trivial_numeric_casts,
    missing_debug_implementations
)]
#![cfg_attr(nightly, feature(non_exhaustive_omitted_patterns))]

#[cfg(not(feature = "std"))]
compile_error!("This crate requires the `std` feature to be enabled");

/// Completions for [nushell].
///
/// [nushell]: https://www.nushell.sh/
#[cfg(feature = "nu")]
pub mod nu;
