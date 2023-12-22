// fetch url
// fetch git
// fetch tar
// fetch zip
// fetch path

use embed_nu::{
    nu_engine::get_full_help,
    nu_protocol::{
        ast::Call,
        engine::{Command, EngineState, Stack},
        Category, IntoPipelineData, ShellError, Signature, Type,
    },
    rusty_value::*,
    PipelineData,
};
use std::path::PathBuf;

pub use git::FetchGit;

use crate::NukaValue;

mod git;

#[derive(RustyValue)]
pub struct FetchResponse {
    path: PathBuf,
    hash: u64,
}

impl NukaValue for FetchResponse {
    fn kind() -> Type {
        Type::Record(vec![
            ("path".to_owned(), Type::String),
            ("hash".to_owned(), Type::Number),
        ])
    }
}

#[derive(Clone)]
pub struct Fetch;

impl Command for Fetch {
    fn name(&self) -> &str {
        "fetch"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![(Type::Nothing, Type::String)])
            .category(Category::Network)
    }

    fn usage(&self) -> &str {
        "Various commands for fetching data."
    }

    fn extra_usage(&self) -> &str {
        "You must use one of the following subcommands. Using this command as-is will only produce this help message."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec![
            "network", "fetch", "pull", "request", "download", "curl", "wget",
        ]
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
                &Fetch.signature(),
                &Fetch.examples(),
                engine_state,
                stack,
                self.is_parser_keyword(),
            ),
            call.head,
        )
        .into_pipeline_data())
    }
}
