use tokio::process::Command;

pub async fn exec_async(command: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    let (command, args) = parts.split_at(1);

    let child = Command::new(command[0]).args(args).spawn()?;

    let output = child.wait_with_output().await?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into())
    }
}
