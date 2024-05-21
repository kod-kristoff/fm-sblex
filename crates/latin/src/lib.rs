pub mod types_lat;
use std::fmt;

use fm_sblex::frontend::Language;

#[derive(Debug)]
pub struct Latin;

impl Language for Latin {
    fn name(&self) -> &'static str {
        "latin"
    }
    fn morphology_header(&self) -> &'static str {
        concat!(
            "FM-LATIN ",
            env!("CARGO_PKG_VERSION"),
            "\n © L. Borin, M. Forsberg, 2010, under GNU LGPL 3.0 or CC-SA 2.5 Generic",
            "\n includes FM 3.1 (M. Forsberg & A. Ranta, 2013, under GNU GPL)"
        )
    }
    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
    // internDict   _ = latinDict
    // composition  _ = Just $ compDesc
    // paradigms    _ = foldr insertCommand emptyC commands
    // wordGuesser  _ = silly_guesser
}

impl fmt::Display for Latin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Latin")
    }
}
