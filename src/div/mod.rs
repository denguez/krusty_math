pub mod division;
pub mod euclid;
pub mod bezout;
pub mod diophantine;
pub mod base;
pub mod modular;
pub mod prime;
pub mod cli;
pub mod module;

pub use module::init;
pub use cli::*;
pub use modular::{
    InverseMod,
    RestMod,
    ChineseAlg,
    FermatResult
};





