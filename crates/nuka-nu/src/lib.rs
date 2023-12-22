use embed_nu::{
    nu_protocol::{SyntaxShape, Type},
    rusty_value::RustyValue,
};

pub mod commands;
pub mod context;

pub trait NukaValue: RustyValue {
    fn kind() -> Type;
    fn shape() -> SyntaxShape {
        Self::kind().to_shape()
    }
}
