// wrap exe

use embed_nu::{
    nu_engine::get_full_help,
    nu_protocol::{
        ast::Call,
        engine::{Command, EngineState, Stack},
        Category, IntoPipelineData, ShellError, Signature, Type,
    },
    PipelineData,
};

#[derive(Clone)]
pub struct Wrap;

impl Command for Wrap {
    fn name(&self) -> &str {
        "wrap"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![(Type::Nothing, Type::String)])
            .category(Category::Custom("Nuka".to_owned()))
    }

    fn usage(&self) -> &str {
        "Various commands for wrapping files"
    }

    fn extra_usage(&self) -> &str {
        "You must use one of the following subcommands. Using this command as-is will only produce this help message."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        Ok(embed_nu::Value::string(
            get_full_help(
                &Wrap.signature(),
                &Wrap.examples(),
                engine_state,
                stack,
                self.is_parser_keyword(),
            ),
            call.head,
        )
        .into_pipeline_data())
    }
}
