use std::process::Command;
use walkdir::WalkDir;
use std::io::{self};
use std::env;

mod utils;

fn add_logging() -> bool {
  let args: Vec<String> = env::args().collect();
  args.iter().any(|arg| arg == "--logging" || arg == "-l")
}

fn main() -> io::Result<()> {
  let base_dir = std::env::current_dir()?;
  let overwrite = utils::user_prompt("Overwrite existing files?");
  let logging = add_logging();

  println!("Scanning: {}", base_dir.display());

  WalkDir::new(base_dir)
  .into_iter()
  .filter_map(Result::ok)
  .filter(|e| e.path().extension().map_or(false, |ext| matches!(ext.to_str().unwrap_or("").to_lowercase().as_str(), "rar" | "zip" | "7z" )))
  .for_each(|entry| {
    let new_path = entry.path().with_extension("");
    let filename = new_path.file_name().unwrap().to_string_lossy();
    let fileformat = format!("-o{}", filename);

    let mut args = vec![
      "e",
      "-y",
      &fileformat,
      entry.path().to_str().unwrap(),
    ];

    if !logging {
      args.insert(1, "-bb0");
      args.insert(2, "-bso0");
      args.insert(3, "-bsp0");
    };

    if overwrite {
      args.push("-aoa");
    } else {
      args.push("-aos");
    }

    let status =Command::new("7z")
      .args(&args)
      .status()
      .expect("Failed to run 7z");

    if status.success() {
      println!("Extracted: {}", filename);
    } else {
      eprintln!("Failed to extract: {}", entry.path().display());
    }
  });

  println!("\nDone.");
  Ok(())
}