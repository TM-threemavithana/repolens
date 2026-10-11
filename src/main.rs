use std::{env, fs, io, path::Path};

fn main() -> io::Result<()> {
    // 1. Collect target directory from command-line arguments (default to ".")
    let directory = env::args().nth(1).unwrap_or_else(|| ".".to_string());

    // 2. Borrow as a filesystem path
    let path = Path::new(&directory);

    // 3. Mutable counters for file statistics
    let mut files = 0u64;
    let mut folders = 0u64;
    let mut total_bytes = 0u64;

    // 4. Iterate over direct entries in the directory
    for item in fs::read_dir(path)? {
        let entry = item?;
        let file_type = entry.file_type()?;

        if file_type.is_file() {
            files += 1;
            total_bytes += entry.metadata()?.len();
        } else if file_type.is_dir() {
            folders += 1;
        }
    }

    // 5. Output summary
    println!("Repository: {}", path.display());
    println!("Files: {files}");
    println!("Folders: {folders}");
    println!("Total file bytes: {total_bytes}");

    // TODO [Learner Exercise]:
    // Add an output line that displays "Total entries: <count>" where count is (files + folders).
    // Consider whether you need a new variable or can compute it directly in the format string.

    Ok(())
}
