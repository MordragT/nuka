use std::path::PathBuf;

use embed_nu::{
    nu_protocol::{engine::Command, Category, IntoPipelineData, Signature, SyntaxShape, Type},
    CallExt, IntoValue,
};

use super::FetchResponse;

#[derive(Clone)]
pub struct FetchGit;

impl Command for FetchGit {
    fn name(&self) -> &str {
        &"fetch git"
    }

    fn usage(&self) -> &str {
        &"Fetch data from git repositories"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![(Type::String, Type::Any)])
            .required(
                "uri",
                SyntaxShape::String,
                "An uniform resource identifier pointing to the git repository.",
            )
            .category(Category::Network)
    }

    fn run(
        &self,
        engine_state: &embed_nu::nu_protocol::engine::EngineState,
        stack: &mut embed_nu::nu_protocol::engine::Stack,
        call: &embed_nu::nu_protocol::ast::Call,
        input: embed_nu::PipelineData,
    ) -> Result<embed_nu::PipelineData, embed_nu::nu_protocol::ShellError> {
        let span = call.head;
        let uri: String = call.req(engine_state, stack, 0)?;

        // TODO clone git

        Ok(FetchResponse {
            path: PathBuf::new(),
            hash: 0,
        }
        .into_value()
        .into_pipeline_data())
    }
}
