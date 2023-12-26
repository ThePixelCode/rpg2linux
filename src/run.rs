#[derive(thiserror::Error, Debug)]
pub enum RunErrors {
    #[error("Failed to run the game!, the error was {0}")]
    ProcessError(#[from] std::io::Error),
    #[error("Game exited unsuccessfully, exit status was {0}")]
    RunError(i32),
}

type Result<T> = std::result::Result<T, RunErrors>;

pub fn run_game<P>(
    game_data: &crate::GameData<P>,
    logger: &mut crate::logger::Logger,
    args: Vec<String>,
) -> Result<()>
where
    P: AsRef<std::path::Path>,
{
    logger.log("creating proccess...");
    let mut run = std::process::Command::new(game_data.path.as_ref().join("nw"))
        .args(args)
        .current_dir(game_data.path.as_ref().display().to_string())
        .spawn()?;
    logger.log("process created waiting for exit...");
    let exit_status = run.wait()?;
    if !exit_status.success() {
        return Err(RunErrors::RunError(exit_status.code().unwrap_or(128)));
    }
    Ok(())
}
