use std::path::PathBuf;

#[tauri::command]
pub async fn reveal_file_or_directory(path: String) -> Result<(), String> {
    let path = PathBuf::from(&path);

    if !path.exists() {
        return Err("Path does not exist".into());
    }

    let is_file = path.is_file();

    #[cfg(target_os = "windows")]
    {
        let mut cmd = std::process::Command::new("explorer");

        if is_file {
            cmd.arg("/select,").arg(&path);
        } else {
            cmd.arg(&path);
        }

        cmd.spawn().map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        let mut cmd = std::process::Command::new("open");

        if is_file {
            cmd.arg("-R").arg(&path); // reveal
        } else {
            cmd.arg(&path);
        }

        cmd.spawn().map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        let dir = if is_file {
            path.parent().unwrap_or(Path::new("/"))
        } else {
            &path
        };

        std::process::Command::new("xdg-open")
            .arg(dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
