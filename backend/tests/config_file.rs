use familiar_desktop::config_file;
use std::{fs, os::unix::fs::PermissionsExt};

#[test]
fn compare_before_replace_preserves_edits_and_private_backup() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("hyprland.lua");
    let backups = temp.path().join("backups");
    fs::write(&path, "-- before  \n\n").unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o640)).unwrap();
    let before = config_file::read(&path).unwrap();
    fs::write(&path, "-- edited concurrently\n").unwrap();
    assert!(config_file::replace(&path, &before, "-- replacement", &backups).is_err());
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        "-- edited concurrently\n"
    );
    let expected = fs::read_to_string(&path).unwrap();
    let backup = config_file::replace(&path, &expected, "-- replacement", &backups)
        .unwrap()
        .unwrap();
    assert_eq!(fs::read_to_string(backup.clone()).unwrap(), expected);
    assert_eq!(
        fs::metadata(backup).unwrap().permissions().mode() & 0o777,
        0o600
    );
    assert_eq!(
        fs::metadata(path).unwrap().permissions().mode() & 0o777,
        0o640
    );
}
