mod scription;
use crate::config::Config;
pub use scription::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct InscriptionWithOutId<'a> {
    config: &'a Config,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct InscriptionWithId<'a> {
    config: &'a Config,
}
