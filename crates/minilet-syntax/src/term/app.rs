use super::{Term, Tuple};

#[derive(Debug)]
pub struct App {
    pub receiver: Term,
    pub arg: Tuple,
}
