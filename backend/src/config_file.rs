//! Surgical edits to user-owned configuration. Never restore an old whole-file
//! backup over subsequent user edits.
use crate::{Result, common};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

pub fn read(path: &Path) -> Result<String> {
    let metadata = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if !metadata.file_type().is_file() {
        return Err("Configuration must be a regular file, not a symlink; no file changed".into());
    }
    String::from_utf8(common::bounded_file(path, common::FILE_LIMIT)?).map_err(|e| e.to_string())
}

pub fn replace(
    path: &Path,
    expected: &str,
    replacement: &str,
    backups: &Path,
) -> Result<Option<PathBuf>> {
    if read(path)? != expected {
        return Err(
            "Configuration changed externally; no file changed. Retry after reviewing your edits"
                .into(),
        );
    }
    if expected == replacement {
        return Ok(None);
    }
    let permissions = fs::metadata(path).map_err(|e| e.to_string())?.permissions();
    fs::create_dir_all(backups).map_err(|e| e.to_string())?;
    let mut backup = tempfile::Builder::new()
        .prefix("before-")
        .suffix(".lua")
        .tempfile_in(backups)
        .map_err(|e| e.to_string())?;
    backup
        .write_all(expected.as_bytes())
        .map_err(|e| e.to_string())?;
    backup.as_file().sync_all().map_err(|e| e.to_string())?;
    let (_, backup_path) = backup.keep().map_err(|e| e.to_string())?;
    let mut temp =
        tempfile::NamedTempFile::new_in(path.parent().ok_or("Missing config directory")?)
            .map_err(|e| e.to_string())?;
    temp.write_all(replacement.as_bytes())
        .map_err(|e| e.to_string())?;
    temp.as_file()
        .set_permissions(permissions)
        .map_err(|e| e.to_string())?;
    temp.as_file().sync_all().map_err(|e| e.to_string())?;
    if read(path)? != expected {
        return Err("Configuration changed externally; no file changed".into());
    }
    temp.persist(path).map_err(|e| e.to_string())?;
    Ok(Some(backup_path))
}

pub fn strip_exact(text: &str, begin: &str, end: &str, block: &str) -> Result<String> {
    let starts = text.matches(begin).count();
    let ends = text.matches(end).count();
    if starts == 0 && ends == 0 {
        return Ok(text.into());
    }
    if starts == 1
        && ends == 1
        && let Some(offset) = text.find(block)
    {
        return Ok(format!(
            "{}{}",
            &text[..offset],
            &text[offset + block.len()..]
        ));
    }
    Err(
        "Familiar hook was edited, damaged or belongs to another installation; no file changed"
            .into(),
    )
}
