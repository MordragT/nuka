use embed_nu::{
    nu_protocol::{engine::Command, Category, Signature, SyntaxShape, Type},
    rusty_value::*,
};

use super::FetchResponse;
use crate::NukaValue;

// TODO inside nuka-core
// pub struct NukaDerivation {
//     name: String,
//     version: String,
//     // path to the source inside the store: <hash>.src
//     source: StorePath,
//     // path to their nuon objects: <hash>-name-version.drv
//     inputs: Vec<StorePath>,
//     // path to the builder <hash>.run
//     builder: StorePath,
// }

#[derive(Clone)]
pub struct Derivation;

impl Command for Derivation {
    fn name(&self) -> &str {
        &"derivation"
    }

    fn usage(&self) -> &str {
        &"Create a package."
    }

    // fn signature(&self) -> Signature {
    //     Signature::build(self.name())
    //         .input_output_types(vec![(Type::Nothing, Type::Any)])
    //         .required_named(
    //             "name",
    //             SyntaxShape::String,
    //             "The name of the created package.",
    //             None,
    //         )
    //         .required_named(
    //             "version",
    //             SyntaxShape::String,
    //             "The version of the created package.",
    //             None,
    //         )
    //         .required_named(
    //             "source",
    //             FetchResponse::shape(),
    //             "The source data of the package.",
    //             None,
    //         )
    //         .required_named(
    //             "inputs",
    //             SyntaxShape::List(Box::new(SyntaxShape::ImportPattern)),
    //             "Other depending packages.",
    //             None,
    //         )
    //         .required_named(
    //             "build",
    //             SyntaxShape::Closure(None),
    //             "The build function to execute.",
    //             None,
    //         )
    //         .named(
    //             "licenses",
    //             SyntaxShape::List(Box::new(SyntaxShape::String)),
    //             "The licenses of the package.",
    //             None,
    //         )
    //         .named(
    //             "platforms",
    //             SyntaxShape::List(Box::new(SyntaxShape::String)),
    //             "The platforms this package supports.",
    //             None,
    //         )
    //         .category(Category::Custom("Nuka".to_owned()))
    // }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![(Type::Nothing, Type::Any)])
            .required("arguments", Arguments::shape(), "The arguments needed.")
            .category(Category::Custom("Nuka".to_owned()))
    }

    fn run(
        &self,
        engine_state: &embed_nu::nu_protocol::engine::EngineState,
        stack: &mut embed_nu::nu_protocol::engine::Stack,
        call: &embed_nu::nu_protocol::ast::Call,
        input: embed_nu::PipelineData,
    ) -> Result<embed_nu::PipelineData, embed_nu::nu_protocol::ShellError> {
        todo!()
    }
}

#[derive(RustyValue)]
struct Arguments {
    name: String,
    version: String,
    source: FetchResponse,
    // derivation functions of depending packages
    inputs: Vec<embed_nu::Value>,
    build_fn: embed_nu::Value,
    licenses: Vec<String>,
    platforms: Vec<String>,
}

impl NukaValue for Arguments {
    fn kind() -> Type {
        Type::Record(vec![
            ("name".to_owned(), Type::String),
            ("version".to_owned(), Type::String),
            ("source".to_owned(), FetchResponse::kind()),
            ("inputs".to_owned(), Type::List(Box::new(Type::Any))),
            ("build_fn".to_owned(), Type::Closure),
            ("licenses".to_owned(), Type::List(Box::new(Type::String))),
            ("platforms".to_owned(), Type::List(Box::new(Type::String))),
        ])
    }
}
