use anyhow::*;
use clitool::CliTool;
use move_readme::MoveReadmeTool;

#[tokio::main]
async fn main() -> Result<()> {
    MoveReadmeTool::execute_main().await
}
