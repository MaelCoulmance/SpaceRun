use super::*;


#[derive(Resource, Copy, Clone, Debug, PartialEq, Eq)]
pub struct BackgroundLocker(pub bool);