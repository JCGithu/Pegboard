use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;
use regex::Regex;

fn main() -> io::Result<()> {
    let base_dir = std::env::current_dir()?;
    println!("Scanning: {}", base_dir.display());

    for entry in WalkDir::new(&base_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "mp3"))
    {
        let path = entry.path();
        println!("Processing: {}", path.display());

        let artist = get_ffprobe_tag(path, "artist");
        let album = get_ffprobe_tag(path, "album");

        if let (Some(artist), Some(album)) = (artist, album) {
            let artist_clean = sanitize(&artist);
            let album_clean = sanitize(&album);

            let target_dir = base_dir.join(&artist_clean).join(&album_clean);
            fs::create_dir_all(&target_dir)?;

            let filename = path.file_name().unwrap();
            let target_path = target_dir.join(filename);

            fs::rename(path, &target_path)?;
            println!("→ Moved to {}", target_path.display());
        } else {
            println!("× Skipped: Missing artist/album tags");
        }
    }

    Ok(())
}

fn get_ffprobe_tag(file: &Path, tag: &str) -> Option<String> {
    let output = Command::new("ffprobe")
        .arg("-v").arg("quiet")
        .arg("-show_entries").arg(format!("format_tags={}", tag))
        .arg("-of").arg("default=nw=1:nk=1")
        .arg(file)
        .output()
        .ok()?;

    let line = output.stdout.lines().next()?.ok()?;
    if line.trim().is_empty() {
        None
    } else {
        Some(line.trim().to_string())
    }
}

fn sanitize(name: &str) -> String {
    let re = Regex::new(r#"[:*?"<>|\\/\r\n]"#).unwrap();
    re.replace_all(name, "_").to_string()
}
