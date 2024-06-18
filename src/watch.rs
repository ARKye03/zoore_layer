use std::path::Path;

use notify::{RecursiveMode, Result, Watcher};

pub async fn watch_styles() -> Result<()> {
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(|res| match res {
        Ok(event) => println!("event: {:?}", event),
        Err(e) => println!("watch error: {:?}", e),
    })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("src/styles"), RecursiveMode::Recursive)?;
    println!("Watching for changes in src/styles");
    Ok(())
}
