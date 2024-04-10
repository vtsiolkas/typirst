use dirs::config_dir;
use serde_json::{from_str, to_string_pretty};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

fn save_typing_stats(stats: &HashMap<String, TypingStat>) -> std::io::Result<()> {
    let mut stats_path: PathBuf = config_dir().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Could not find configuration directory",
    ))?;
    stats_path.push("myapp"); // Replace "myapp" with your application's name
    stats_path.push("typing_stats.json");

    std::fs::create_dir_all(&stats_path.parent().unwrap())?;

    let stats_data = to_string_pretty(stats).expect("Failed to serialize typing stats");
    let mut file = File::create(stats_path)?;
    file.write_all(stats_data.as_bytes())?;

    Ok(())
}

fn load_typing_stats() -> std::io::Result<HashMap<String, TypingStat>> {
    let mut stats_path: PathBuf = config_dir().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Could not find configuration directory",
    ))?;
    stats_path.push("myapp"); // Replace "myapp" with your application's name
    stats_path.push("typing_stats.json");

    let mut file = File::open(stats_path)?;
    let mut stats_data = String::new();
    file.read_to_string(&mut stats_data)?;

    let stats: HashMap<String, TypingStat> =
        from_str(&stats_data).expect("Failed to deserialize typing stats");
    Ok(stats)
}
