use chrono::Local;
use colored::*;
use fern::Dispatch;
use log::{LevelFilter};

pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    let log_file_path = "output.log";

    // Initialize the logger
    Dispatch::new()
        .format(|out, message, record| {
            let formatted_message = match record.level() {
                log::Level::Error => message.to_string().red().to_string(),
                log::Level::Warn => message.to_string().yellow().to_string(),
                log::Level::Info => message.to_string().green().to_string(),
                log::Level::Debug => message.to_string().cyan().to_string(),
                log::Level::Trace => message.to_string().dimmed().to_string(),
            };
            
            out.finish(format_args!(
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                formatted_message
            ))
        })
        .level(LevelFilter::Debug)  // Adjust log level as needed
        .chain(std::io::stdout())
        .chain(fern::log_file(log_file_path)?)  // Handle potential errors
        .apply()?;
    
    // Inform the user where the log file is located
    println!("Logging initialized. Log file can be found at: {}", log_file_path);
    Ok(())
}

