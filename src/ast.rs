#[derive (Debug)]
pub enum Rule {
    Substitution(String, Vec<String>),
    Error,
}
