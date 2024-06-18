use std::str;
use tokio::process::Command;
use tokio::runtime::Runtime;

pub fn exec_async(command: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    let (command, args) = parts.split_at(1);

    let rt = Runtime::new().unwrap(); // Create a new Tokio runtime

    rt.block_on(async {
        // Use the runtime to run the async task
        let output = Command::new(command[0])
            .args(args)
            .output()
            .await
            .expect("Failed to run command");

        // Convert the output to a string
        let stdout = str::from_utf8(&output.stdout)?.trim().to_string();

        Ok(stdout)
    })
}
