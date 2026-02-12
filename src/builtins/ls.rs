use std::fs;
use std::os::unix::fs::{ MetadataExt, PermissionsExt };
use std::path::Path;
use users::{ get_user_by_uid, get_group_by_gid };
use chrono::{ DateTime, Local };
use std::time::SystemTime;
use std::io::{ self, Write };

pub fn ls(args: &[String]) {
    let mut a_flag = false;
    let mut l_flag = false;
    let mut f_flag = false;

    for arg in args {
        if arg.starts_with('-') && arg.len() > 1 {
            for c in arg.chars().skip(1) {
                match c {
                    'a' => {
                        a_flag = true;
                    }
                    'l' => {
                        l_flag = true;
                    }
                    'F' => {
                        f_flag = true;
                    }
                    _ => {} // Ignore unknown flags
                }
            }
        }
    }

    let path_str = args
        .iter()
        .find(|arg| !arg.starts_with('-'))
        .map(|s| s.as_str())
        .unwrap_or(".");

    let path = Path::new(path_str);

    let entries = match fs::read_dir(path) {
        Ok(read) => read.flatten().collect::<Vec<_>>(),
        Err(e) => {
            eprintln!("0-shell: ls: {}: {}", path_str, e);
            return;
        }
    };

    // Filter based on -a flag
    let mut filtered_entries: Vec<_> = entries
        .into_iter()
        .filter(|entry| {
            let name = entry.file_name().into_string().unwrap_or_default();
            a_flag || !name.starts_with('.')
        })
        .collect();

    filtered_entries.sort_by_key(|e| e.file_name());

    if filtered_entries.is_empty() {
        return;
    }

    if l_flag {
        // Calculate column widths for long format
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

        let max_size = filtered_entries
            .iter()
            .filter_map(|e| e.metadata().ok())
            .map(|m| m.len().to_string().len())
            .max()
            .unwrap_or(1);

        // Print total blocks
        let mut total_blocks = 0;
        for entry in &filtered_entries {
            if let Ok(meta) = entry.metadata() {
                total_blocks += meta.blocks();
            }
        }
        let total_output = format!("total {}\r\n", total_blocks / 2);
        let _ = io::stdout().write_all(total_output.as_bytes());
        let _ = io::stdout().flush();

        // Print entries
        for entry in &filtered_entries {
            let file_name = entry.file_name().into_string().unwrap_or_default();
            if let Ok(metadata) = entry.metadata() {
                print_long_entry(
                    &file_name,
                    &metadata,
                    f_flag,
                    max_links,
                    max_user,
                    max_group,
                    max_size
                );
            }
        }
    } else {
        // Short format - column-based
        let mut row = String::new();
        let col_width = 20;

        for entry in &filtered_entries {
            let file_name = entry.file_name().into_string().unwrap_or_default();
            let indicator = if f_flag {
                if let Ok(metadata) = entry.metadata() {
                    get_indicator(&metadata).to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            let formatted = format!("{}{}", file_name, indicator);
            row.push_str(&format!("{:<width$}", formatted, width = col_width));

            if row.len() > 80 {
                println!("{}", row.trim_end());
                row.clear();
            }
        }

        if !row.is_empty() {
            println!("{}", row.trim_end());
        }
    }
}

fn get_indicator(metadata: &fs::Metadata) -> &str {
    let mode = metadata.permissions().mode();
    if metadata.is_dir() {
        "/"
    } else if metadata.file_type().is_symlink() {
        "@"
    } else if (mode & 0o111) != 0 {
        "*"
    } else {
        ""
    }
}

fn format_permissions(mode: u32, is_dir: bool) -> String {
    let mut p = String::with_capacity(10);

    p.push(if is_dir { 'd' } else { '-' });

    let flags = [
        0o400,
        0o200,
        0o100, // user
        0o040,
        0o020,
        0o010, // group
        0o004,
        0o002,
        0o001, // others
    ];

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
    meta: &fs::Metadata,
    f_flag: bool,
    max_links: usize,
    max_user: usize,
    max_group: usize,
    max_size: usize
) {
    let modified = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let datetime: DateTime<Local> = modified.into();
    let now = Local::now();
    let duration = now.signed_duration_since(datetime);

    let permissions = format_permissions(meta.permissions().mode(), meta.is_dir());
    let nlink = meta.nlink();
    let uid = meta.uid();
    let gid = meta.gid();
    let size = meta.len();

    let user = get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or(uid.to_string());

    let group = get_group_by_gid(gid)
        .map(|g| g.name().to_string_lossy().into_owned())
        .unwrap_or(gid.to_string());

    let time_str = if duration.num_days() > 180 {
        datetime.format("%b %e  %Y").to_string()
    } else {
        datetime.format("%b %e %H:%M").to_string()
    };

    let indicator = if f_flag { get_indicator(meta) } else { "" };

    // Format each column with proper alignment
    let perms_col = format!("{}", permissions);
    let links_col = format!("{:>width$}", nlink, width = max_links);
    let user_col = format!("{:<width$}", user, width = max_user);
    let group_col = format!("{:<width$}", group, width = max_group);
    let size_col = format!("{:>width$}", size, width = max_size);

    let output = format!(
        "{} {} {} {} {} {} {}{}\r\n",
        perms_col,
        links_col,
        user_col,
        group_col,
        size_col,
        time_str,
        name,
        indicator
    );

    let _ = io::stdout().write_all(output.as_bytes());
    let _ = io::stdout().flush();
}
