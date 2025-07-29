use rayon::prelude::*;
use std::fs;
use std::io::{self, Write};
use std::path::{Path};
use std::process::Command;
use walkdir::WalkDir;
use colored::*;

fn user_prompt(prompt: &str) -> bool {
  print!("{} [y/N] :", prompt);
  io::stdout().flush().unwrap();
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

fn convert_flac_to_mp3(input_path: &Path, output_path: &Path) -> bool {
  let filename = input_path.file_name().unwrap().to_str().expect("").green();
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
      "-loglevel",
      "error",
      output_path.to_str().unwrap(),
    ])
    .status()
    .expect("Failed to execute ffmpeg");

  if status.success() {
    println!("Converted: {}", filename);
    true
  } else {
    eprintln!("Failed to convert: {}", input_path.display());
    false
  }
}

fn main() {
  let delete_originals = user_prompt("Delete original files?");
  let input_dir = Path::new(".");

  WalkDir::new(input_dir)
    .into_iter()
    .par_bridge()
    .filter_map(Result::ok)
    .filter(|e| e.path().extension().map_or(false, |ext| ext == "flac"))
    .for_each(|entry| {
      let new_path = entry.path().with_extension("mp3");
      let filename = new_path.file_name().unwrap().to_str().expect("").yellow();

      if new_path.exists() {
        println!("{} already exists (skipping)", filename);
        return;
      }

      if convert_flac_to_mp3(entry.path(), &new_path) && delete_originals {
        if let Err(e) = fs::remove_file(entry.path()) {
          eprintln!("Failed to delete original: {} ({})", entry.path().display(), e);
        } else {
          println!("Deleted original: {}", entry.path().display());
        }
      }
    });
}
