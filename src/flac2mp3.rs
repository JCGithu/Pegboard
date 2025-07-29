use rayon::prelude::*;
use std::fs;
use std::path::{Path};
use std::process::Command;
use walkdir::WalkDir;

fn convert_flac_to_mp3(input_path: &Path, output_path: &Path) -> bool {
  let status = Command::new("ffmpeg")
    .args(&[
      "-i",
      input_path.to_str().unwrap(),
      "-ab",
      "320k",
      "-map_metadata",
      "0",
      "-id3v2_version",
      "3",
      output_path.to_str().unwrap(),
    ])
    .status()
    .expect("Failed to execute ffmpeg");

  if status.success() {
    println!("Converted: {}", input_path.display());
    true
  } else {
    eprintln!("Failed to convert: {}", input_path.display());
    false
  }
}

fn main() {
  let input_dir = Path::new(".");

  WalkDir::new(input_dir)
    .into_iter()
    .par_bridge()
    .filter_map(Result::ok)
    .filter(|e| e.path().extension().map_or(false, |ext| ext == "flac"))
    .for_each(|entry| {
      let new_path = entry.path().with_extension("mp3");

      if new_path.exists() {
        println!("Skipping (already exists): {}", new_path.display());
        return;
      }

      if convert_flac_to_mp3(entry.path(), &new_path) {
        if let Err(e) = fs::remove_file(entry.path()) {
          eprintln!("Failed to delete original: {} ({})", entry.path().display(), e);
        } else {
          println!("Deleted original: {}", entry.path().display());
        }
      }
    });
}
