use std::path::{Path, PathBuf};

use nu_plugin::{JsonSerializer, LabeledError, Plugin};
use nu_protocol::{PluginExample, PluginSignature, Signature, SyntaxShape, Value, Category};
use nuka_core::{Store, NUKA_STORE_PATH};

struct NukaExecPlugin {
    store_path: PathBuf,
}

impl NukaExecPlugin {
    pub fn new<P: AsRef<Path>>(search_path: P) -> Self {
        Self {
            store_path: search_path.as_ref().to_owned(),
        }
    }
}

impl Plugin for NukaExecPlugin {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature {
            sig: Signature::build("nuka exec")
                .usage("Tries to execute an external command not compiled within nuka.")
                .required("command", SyntaxShape::String, "the command to execute")
                .allows_unknown_args()
                .category(Category::System),
            examples: vec![PluginExample {
                example: "nuka exec ps aux".to_owned(),
                description: "Execute external 'ps aux' tool".to_owned(),
                result: None,
            }],
        }]
    }

    fn run(
        &mut self,
        name: &str,
        call: &nu_plugin::EvaluatedCall,
        input: &Value,
    ) -> Result<Value, nu_plugin::LabeledError> {
        match name {
            "nuka exec" => {
                let store = Store::open(&self.store_path).unwrap();

                eprintln!("Value: {input:?}");

                todo!()
            } 
            _ => Err(LabeledError {
                label: "Plugin call with wrong name signature".to_owned(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".to_owned(),
                span: Some(call.head),
            })
        }
    }
}

fn main() {
    nu_plugin::serve_plugin(&mut NukaExecPlugin::new(NUKA_STORE_PATH), JsonSerializer);
}
