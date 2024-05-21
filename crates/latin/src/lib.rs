pub mod types_lat;

#[derive(Debug)]
pub struct Latin;

impl Language for Latin {
    // internDict   _ = latinDict
    // composition  _ = Just $ compDesc
    // paradigms    _ = foldr insertCommand emptyC commands
    // wordGuesser  _ = silly_guesser
}
