use embed_nu::{rusty_value::*, CommandGroupConfig, Context, NewEmpty, PipelineData};
use nuka_nu::commands::*;

fn main() {
    let mut ctx = Context::builder()
        .with_command_groups(CommandGroupConfig::default().all_groups(true))
        .unwrap()
        .add_command(FetchGit)
        .unwrap()
        .add_command(Fetch)
        .unwrap()
        .add_parent_env_vars()
        .build()
        .unwrap();

    ctx.eval_raw(
        r#"
      def main [] {
          echo "Hello World from this script"
          fetch git "https://github.com/Mordragt/comoji"
      }        
  "#,
        PipelineData::empty(),
    )
    .unwrap();

    let pipeline = ctx.call_fn("main", [] as [String; 0]).unwrap();
    ctx.print_pipeline(pipeline).unwrap();
}
