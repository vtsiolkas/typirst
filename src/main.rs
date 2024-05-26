use color_eyre::Result;
use dirs::data_dir;
use simplelog::*;
use std::fs::{create_dir_all, File};
use std::path::PathBuf;
use typirst::App;

mod errors;
use typirst::tui;

fn main() -> Result<()> {
    errors::install_hooks()?;

    // Setup logging
    // Determine the XDG data directory
    let mut log_file_path = data_dir().unwrap_or_else(|| PathBuf::from("."));
    log_file_path.push("typirst");
    create_dir_all(&log_file_path).unwrap();
    log_file_path.push("app.log");

    let log_file = File::create(log_file_path).unwrap();

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        log_file,
    )])
    .unwrap();

    let mut terminal = tui::init()?;
    App::new().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
