use std::fs;
use std::os::unix::fs::{ FileTypeExt, MetadataExt, PermissionsExt };
use std::path::Path;
use users::{ get_user_by_uid, get_group_by_gid };
use chrono::{ DateTime, Local };
use std::time::SystemTime;
use std::io::{ self, Write };
use crossterm::{ cursor, execute, terminal };

pub fn ls(args: &[String]) {
    let mut a_flag = false;
    let mut l_flag = false;
    let mut f_flag = false;
    let mut err = "".to_string();

    for arg in args {
        if arg.starts_with('-') && arg.len() > 1 {
            for c in arg.chars().skip(1) {
                match c {
                    'a' => { a_flag = true; }
                    'l' => { l_flag = true; }
                    'F' => { f_flag = true; }
                    _ => {err = format!("ls: invalid option -- {}", c)} 
                }
            }
        }
    }

    if err != "".to_string(){
        println!("{}",err);
        return
    }

    let path_str = args
        .iter()
        .find(|arg| !arg.starts_with('-'))
        .map(|s| s.as_str())
        .unwrap_or("."); // Default to current directory
    let path = Path::new(path_str);

    let metadata = match fs::symlink_metadata(path) {
        Ok(meta) => meta,
        Err(e) => {
            eprintln!("0-shell: ls: {}: {}\r", path_str, e);
            return; // Exit if metadata cannot be retrieved
        }
    };

    if metadata.is_file() || metadata.is_symlink() {
        if l_flag {
            // Print detailed info for a single file
            let max_links = 1;
            let max_user = 8;
            let max_group = 8;
            let max_size = metadata.len().to_string().len();
            print_long_entry(
                path_str,
                path,
                &metadata,
                f_flag,
                max_links,
                max_user,
                max_group,
                max_size,
                0,
                0
            );
        } else {
            // Print simple file name with optional indicator
            let indicator = if f_flag {
                get_indicator(&metadata).to_string()
            } else {
                String::new()
            };
            print!("{}{}\r\n", escape_filename(path_str), indicator);
            let _ = io::stdout().flush();
        }
        return; // Done with single file
    }

    let entries = match fs::read_dir(path) {
        Ok(read) => read.flatten().collect::<Vec<_>>(),
        Err(e) => {
            eprintln!("0-shell: ls: {}: {}\r", path_str, e);
            return;
        }
    };

    let mut filtered_entries: Vec<_> = entries
        .into_iter()
        .filter(|entry| {
            let name = entry.file_name().into_string().unwrap_or_default();
            a_flag || !name.starts_with('.') // Keep hidden files only if -a
        })
        .collect();

    filtered_entries.sort_by(|a, b| {
        let name_a = a.file_name().into_string().unwrap_or_default();
        let name_b = b.file_name().into_string().unwrap_or_default();
        let sort_a = name_a.trim_start_matches('.');
        let sort_b = name_b.trim_start_matches('.');
        sort_a.cmp(sort_b) // Sort alphabetically ignoring leading '.'
    });

    if filtered_entries.is_empty() && !a_flag {
        return; // Nothing to display
    }

    let max_links = filtered_entries
        .iter()
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.nlink().to_string().len())
        .max()
        .unwrap_or(1);

    let max_user = filtered_entries
        .iter()
        .filter_map(|e| e.metadata().ok())
        .map(|m| {
            let uid = m.uid();
            get_user_by_uid(uid)
                .map(|u| u.name().to_string_lossy().len())
                .unwrap_or_else(|| uid.to_string().len())
        })
        .max()
        .unwrap_or(1);

    let max_group = filtered_entries
        .iter()
        .filter_map(|e| e.metadata().ok())
        .map(|m| {
            let gid = m.gid();
            get_group_by_gid(gid)
                .map(|g| g.name().to_string_lossy().len())
                .unwrap_or_else(|| gid.to_string().len())
        })
        .max()
        .unwrap_or(1);

    let max_major = filtered_entries
        .iter()
        .filter_map(|e| e.metadata().ok())
        .filter(|m| m.file_type().is_char_device() || m.file_type().is_block_device())
        .map(|m| {
            let major = (m.rdev() >> 8) & 0xfff;
            major.to_string().len()
        })
        .max()
        .unwrap_or(0);

    let max_minor = filtered_entries
        .iter()
        .filter_map(|e| e.metadata().ok())
        .filter(|m| m.file_type().is_char_device() || m.file_type().is_block_device())
        .map(|m| {
            let minor = m.rdev() & 0xff;
            minor.to_string().len()
        })
        .max()
        .unwrap_or(0);

    let device_col_width = if max_major > 0 { max_major + 2 + max_minor } else { 0 };

    let max_file_size = filtered_entries
        .iter()
        .filter_map(|e| e.metadata().ok())
        .filter(|m| !m.file_type().is_char_device() && !m.file_type().is_block_device())
        .map(|m| m.len().to_string().len())
        .max()
        .unwrap_or(0);

    let max_size = std::cmp::max(device_col_width, max_file_size);

    if l_flag {
        let mut total_blocks = 0;
        for entry in &filtered_entries {
            if let Ok(meta) = entry.metadata() {
                total_blocks += meta.blocks();
            }
        }

        // Include '.' and '..' if -a flag
        if a_flag {
            if let Ok(meta) = fs::symlink_metadata(path) {
                total_blocks += meta.blocks();
            }
            let parent_path = if path_str == "." || path_str == "./" {
                Path::new("..")
            } else {
                Path::new(path_str).parent().unwrap_or(Path::new(".."))
            };
            if let Ok(meta) = fs::symlink_metadata(parent_path) {
                total_blocks += meta.blocks();
            }
        }

        let total_output = format!("total {}\r\n", total_blocks / 2);
        let _ = io::stdout().write_all(total_output.as_bytes());
        let _ = io::stdout().flush();

        if a_flag {
            // Print '.' and '..' entries
            if let Ok(meta) = fs::symlink_metadata(path) {
                print_long_entry(
                    ".",
                    path,
                    &meta,
                    f_flag,
                    max_links,
                    max_user,
                    max_group,
                    max_size,
                    max_major,
                    max_minor
                );
            }
            let parent_path = if path_str == "." || path_str == "./" {
                Path::new("..")
            } else {
                Path::new(path_str).parent().unwrap_or(Path::new(".."))
            };
            if let Ok(meta) = fs::symlink_metadata(parent_path) {
                print_long_entry(
                    "..",
                    parent_path,
                    &meta,
                    f_flag,
                    max_links,
                    max_user,
                    max_group,
                    max_size,
                    max_major,
                    max_minor
                );
            }
        }

        // Print all filtered entries
        for entry in &filtered_entries {
            let file_name = entry.file_name().into_string().unwrap_or_default();
            if let Ok(metadata) = fs::symlink_metadata(entry.path()) {
                print_long_entry(
                    &file_name,
                    &entry.path(),
                    &metadata,
                    f_flag,
                    max_links,
                    max_user,
                    max_group,
                    max_size,
                    max_major,
                    max_minor
                );
            }
        }
    } 
    else {
        if a_flag {
            println!(".{width:<width$}..", width = 18);
        }

        if filtered_entries.is_empty() {
            return;
        }

        // Compute column width for terminal display
        let max_name_len = filtered_entries
            .iter()
            .map(|e| {
                let name = e.file_name().into_string().unwrap_or_default();
                let display_name = escape_filename(&name);
                let indicator_len = if f_flag {
                    if let Ok(metadata) = e.metadata() {
                        if get_indicator(&metadata).is_empty() { 0 } else { 1 }
                    } else { 0 }
                } else { 0 };
                display_name.len() + indicator_len
            })
            .max()
            .unwrap_or(1);

        let col_width = max_name_len + 2;
        let terminal_width = terminal
            ::size()
            .map(|(w, _)| w as usize)
            .unwrap_or(80);
        let cols = std::cmp::max(1, terminal_width / col_width);

        let mut row = String::new();
        let mut col_count = 0;

        // Print entries row by row
        for entry in &filtered_entries {
            let file_name = entry.file_name().into_string().unwrap_or_default();
            let display_name = escape_filename(&file_name);
            let indicator = if f_flag {
                if let Ok(metadata) = entry.metadata() {
                    get_indicator(&metadata).to_string()
                } else { String::new() }
            } else { String::new() };

            let formatted = format!("{}{}", display_name, indicator);

            if col_count < cols - 1 {
                row.push_str(&format!("{:<width$}", formatted, width = col_width));
                col_count += 1;
            } else {
                row.push_str(&formatted);
                print!("{}\r\n", row);
                let _ = io::stdout().flush();
                row.clear();
                col_count = 0;
            }
        }

        if !row.is_empty() {
            print!("{}\r\n", row);
            let _ = io::stdout().flush();
        }
    }

    let _ = execute!(io::stdout(), cursor::MoveToColumn(0));
}


pub fn escape_filename(name: &str) -> String {
    if name.contains('\n') {
        let mut result = format!("'{}'", name.replace('\n', "'$'\\n''"));
        if result.ends_with("''") {
            result.truncate(result.len() - 2);
        }
        result
    } else {
        name.to_string()
    }
}

fn get_indicator(metadata: &fs::Metadata) -> &str {
    let mode = metadata.permissions().mode();
    if metadata.is_dir() {
        "/"
    } else if metadata.file_type().is_symlink() {
        ""
    } else if (mode & 0o111) != 0 {
        "@"
    } else {
        ""
    }
}

fn format_permissions(mode: u32, is_dir: bool) -> String {
    let mut p = String::with_capacity(10);

    p.push(if is_dir { 'd' } else { '-' });

    let flags = [0o400, 0o200, 0o100, 0o040, 0o020, 0o010, 0o004, 0o002, 0o001];

    for (i, flag) in flags.iter().enumerate() {
        if (mode & flag) != 0 {
            p.push(match i % 3 {
                0 => 'r',
                1 => 'w',
                _ => 'x',
            });
        } else {
            p.push('-');
        }
    }

    p
}

fn print_long_entry(
    name: &str,
    path: &Path,
    meta: &fs::Metadata,
    f_flag: bool,
    max_links: usize,
    max_user: usize,
    max_group: usize,
    max_size: usize,
    max_major: usize,
    max_minor: usize
) {
    let modified = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let datetime: DateTime<Local> = modified.into();
    let now = Local::now();
    let duration = now.signed_duration_since(datetime);

    let file_type = meta.file_type();
    let file_type_char = if file_type.is_dir() {
        'd'
    } else if file_type.is_symlink() {
        'l'
    } else if file_type.is_char_device() {
        'c'
    } else if file_type.is_block_device() {
        'b'
    } else if file_type.is_fifo() {
        'p'
    } else if file_type.is_socket() {
        's'
    } else {
        '-'
    };

    let permissions = format_permissions(meta.permissions().mode(), meta.is_dir());

    let permissions_fixed = format!("{}{}", file_type_char, &permissions[1..]);

    let nlink = meta.nlink();
    let uid = meta.uid();
    let gid = meta.gid();
    let size = if meta.file_type().is_char_device() || meta.file_type().is_block_device() {
        let major = (meta.rdev() >> 8) & 0xfff;
        let minor = meta.rdev() & 0xff;
        let maj_w = if max_major > 0 { max_major } else { major.to_string().len() };
        let min_w = if max_minor > 0 { max_minor } else { minor.to_string().len() };
        format!("{:>w1$}, {:>w2$}", major, minor, w1 = maj_w, w2 = min_w)
    } else {
        meta.len().to_string()
    };

    let user = get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or(uid.to_string());

    let group = get_group_by_gid(gid)
        .map(|g| g.name().to_string_lossy().into_owned())
        .unwrap_or(gid.to_string());

    let time_str = if duration.num_days() > 181 || duration.num_days() < 0 {
        datetime.format("%b %e  %Y").to_string()
    } else {
        datetime.format("%b %e %H:%M").to_string()
    };

    let display_name = escape_filename(name);

    let symlink_target = if meta.file_type().is_symlink() {
        match fs::read_link(path) {
            Ok(target) => format!(" -> {}", target.display()),
            Err(_) => String::new(),
        }
    } else {
        String::new()
    };

    let indicator = if f_flag {
        if meta.file_type().is_symlink() {
            if let Ok(target) = fs::read_link(path) {
                let target_path = if target.is_absolute() {
                    target
                } else {
                    path.parent().unwrap_or(Path::new(".")).join(&target)
                };
                if let Ok(target_meta) = fs::metadata(&target_path) {
                    get_indicator(&target_meta).to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        } else {
            get_indicator(meta).to_string()
        }
    } else {
        String::new()
    };

    let perms_col = permissions_fixed;
    let links_col = format!("{:>width$}", nlink, width = max_links);
    let user_col = format!("{:<width$}", user, width = max_user);
    let group_col = format!("{:<width$}", group, width = max_group);
    let size_col = format!("{:>width$}", size, width = max_size);

    let output = format!(
        "{} {} {} {} {} {} {}{}{}\r\n",
        perms_col,
        links_col,
        user_col,
        group_col,
        size_col,
        time_str,
        display_name,
        symlink_target,
        indicator
    );

    let _ = io::stdout().write_all(output.as_bytes());
    let _ = io::stdout().flush();
}
