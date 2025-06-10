use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;
use regex::Regex;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let base_dir = std::env::current_dir()?;
    println!("Scanning: {}", base_dir.display());

    let mut moved_from: HashSet<PathBuf> = HashSet::new();

    for entry in WalkDir::new(&base_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_type().is_file()
                && e.path()
                    .extension()
                    .map_or(false, |ext| matches!(ext.to_str().unwrap_or("").to_lowercase().as_str(), "mp3" | "flac"))
        })
    {
        let path = entry.path();
        let parent_dir = path.parent().unwrap().to_path_buf();

        let artist = get_ffprobe_tag(path, "artist");
        let album = get_ffprobe_tag(path, "album");

        if let (Some(artist), Some(album)) = (artist, album) {
            let artist_clean = sanitize(&artist);
            let album_clean = sanitize(&album);

            let relative_target_dir = Path::new(&artist_clean).join(&album_clean);
            let absolute_target_dir = base_dir.join(&relative_target_dir);

            let filename = path.file_name().unwrap();
            let expected_path = absolute_target_dir.join(filename);

            // Skip if file is already in the right place
            if fs::canonicalize(&expected_path).ok() == fs::canonicalize(path).ok() {
                continue;
            }

            fs::create_dir_all(&absolute_target_dir)?;
            fs::rename(path, &expected_path)?;
            println!("→ Moved: {}", expected_path.display());

            moved_from.insert(parent_dir);
        }
    }

    // Remove empty folders
    for folder in moved_from {
        if folder == base_dir {
            continue; // Never delete base dir
        }
        if folder.read_dir()?.next().is_none() {
            fs::remove_dir(&folder)?;
            println!("🗑 Deleted empty folder: {}", folder.display());
        }
    }

    println!("\nDone.");
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
    let cleaned = line.trim();
    if cleaned.is_empty() {
        None
    } else {
        Some(cleaned.to_string())
    }
}

fn sanitize(name: &str) -> String {
    let re = Regex::new(r#"[:*?"<>|\\/\r\n]"#).unwrap();
    re.replace_all(name, "_").to_string()
}
