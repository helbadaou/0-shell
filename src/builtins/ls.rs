use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;
use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, Local};

// ===== COLORS =====
const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";
const CYAN: &str = "\x1b[36m";
const RED: &str = "\x1b[31m";


pub fn ls(args: &[String]) {
    let mut show_all = false; // -a
    let mut long = false; // -l
    let mut classify = false; // -F
    let mut path = ".";

    // ===== parse args =====
    for arg in args {
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                match c {
                    'a' => show_all = true,
                    'l' => long = true,
                    'F' => classify = true,
                    _ => {
                        println!("ls: invalid option '{}'",c);
                        return;
                    }
                }
            }
        } else {
            path = arg;
        }
    }

    // ===== read directory =====
    let mut entries: Vec<_> = match fs::read_dir(path) {
        Ok(e) => e.flatten().collect(),
        Err(e) => {
            eprintln!("ls: {}", e);
            return;
        }
    };

    // ===== sort alphabetically =====
    entries.sort_by_key(|e| e.file_name());

    // ===== calculate total (like bash) =====
    if long {
        let mut total_blocks: u64 = 0;

        // add . and .. only with -a
        if show_all {
            if let Ok(meta) = fs::metadata(Path::new(path)) {
                total_blocks += meta.blocks();
            }
            if let Ok(meta) = fs::metadata(Path::new(path).join("..")) {
                total_blocks += meta.blocks();
            }
        }

        for entry in &entries {
            let name = entry.file_name().to_string_lossy().to_string();
            if !show_all && name.starts_with('.') {
                continue;
            }
            if let Ok(meta) = entry.metadata() {
                total_blocks += meta.blocks();
            }
        }

        // st_blocks are 512B → bash prints 1K blocks
        println!("total {}", total_blocks / 2);
    }

    // ===== helper to print one entry =====
    let print_entry = |name: &str, meta: &fs::Metadata| {
        let mode = meta.permissions().mode();
        let file_type = if meta.is_dir() { 'd' } else { '-' };

        let perms = format!(
            "{}{}{}{}{}{}{}{}{}",
            if mode & 0o400 != 0 { 'r' } else { '-' },
            if mode & 0o200 != 0 { 'w' } else { '-' },
            if mode & 0o100 != 0 { 'x' } else { '-' },
            if mode & 0o040 != 0 { 'r' } else { '-' },
            if mode & 0o020 != 0 { 'w' } else { '-' },
            if mode & 0o010 != 0 { 'x' } else { '-' },
            if mode & 0o004 != 0 { 'r' } else { '-' },
            if mode & 0o002 != 0 { 'w' } else { '-' },
            if mode & 0o001 != 0 { 'x' } else { '-' },
        );

        let links = meta.nlink();

        let user = users::get_user_by_uid(meta.uid())
            .map(|u| u.name().to_string_lossy().to_string())
            .unwrap_or(meta.uid().to_string());

        let group = users::get_group_by_gid(meta.gid())
            .map(|g| g.name().to_string_lossy().to_string())
            .unwrap_or(meta.gid().to_string());

        let size = meta.len();

        let mtime = meta.mtime();
        let system_time = UNIX_EPOCH + Duration::from_secs(mtime as u64);
        let datetime: DateTime<Local> = system_time.into();
        let date = datetime.format("%b %d %H:%M");

        let is_symlink = meta.file_type().is_symlink();
        let is_dir = meta.is_dir();
        let is_exec = mode & 0o111 != 0;

        // ----- build display -----
        let colored = if is_symlink {
            let mut link_name = name.to_string();

            if classify {
                link_name.push('@');
            }

            if let Ok(target) = fs::read_link(Path::new(path).join(name)) {
                let target_str = target.to_string_lossy();

                let target_colored = if target.is_dir() {
                    format!("{BLUE}{target_str}{RESET}")
                } else {
                    format!("{target_str}")
                };

                format!("{CYAN}{link_name}{RESET} -> {target_colored}")
            } else {
                format!("{CYAN}{link_name}{RESET} -> {RED}(broken){RESET}")
            }
        } else {
            // ----- classify -----
            let mut display = name.to_string();
            if classify {
                if is_dir {
                    display.push('/');
                } else if is_exec {
                    display.push('*');
                }
            }

            // ----- color -----
            if is_dir {
                format!("{BLUE}{display}{RESET}")
            } else if is_exec {
                format!("{GREEN}{display}{RESET}")
            } else {
                display
            }
        };

        if long {
            println!(
                "{}{} {:>2} {:<8} {:<8} {:>6} {} {}",
                file_type, perms, links, user, group, size, date, colored
            );
        } else {
            print!("{}  ", colored);
        }
    };

    // ===== print . and .. first =====
    if show_all {
        if let Ok(meta) = fs::metadata(Path::new(path)) {
            print_entry(".", &meta);
        }
        if let Ok(meta) = fs::metadata(Path::new(path).join("..")) {
            print_entry("..", &meta);
        }
    }

    // ===== print directory entries =====
    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();
        if !show_all && name.starts_with('.') {
            continue;
        }
        if let Ok(meta) = entry.metadata() {
            print_entry(&name, &meta);
        }
    }

    if !long {
        println!();
    }
}
