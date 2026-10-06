//! Explicit, reversible trackpad preference. Never rewrite input.lua or core files.
use crate::{
    Result,
    common::{self, Hypr, checked},
};
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
};

const BEGIN: &str = "\n-- BEGIN FAMILIAR TRACKPAD MODE\n";
const END: &str = "-- END FAMILIAR TRACKPAD MODE\n";

pub struct Paths {
    pub config: PathBuf,
    pub manifest: PathBuf,
    pub state: PathBuf,
    pub generated: PathBuf,
}

impl Paths {
    pub fn system() -> Result<Self> {
        let binary = std::env::current_exe().map_err(|e| e.to_string())?;
        let source = binary
            .parent()
            .and_then(Path::parent)
            .ok_or("Install the helper in the plugin bin directory")?;
        let state = std::env::var_os("XDG_STATE_HOME")
            .map(PathBuf::from)
            .unwrap_or(common::home()?.join(".local/state"));
        Ok(Self {
            config: common::config_home()?.join("hypr/hyprland.lua"),
            manifest: source.join("manifest.json"),
            state: state.join("omarchy/familiar-gestures"),
            generated: common::config_home()?.join("omarchy/familiar-trackpad/gestures.lua"),
        })
    }
}

pub fn hook(mode: &str, manifest: &Path) -> Result<String> {
    if !["all", "workspace", "desktop"].contains(&mode) {
        return Err("Choose all, workspace, desktop or reset".into());
    }
    let helper = manifest
        .parent()
        .ok_or("Missing plugin directory")?
        .join("bin/familiar-desktop");
    let command = common::shell_quote(&helper.to_string_lossy());
    let mut body = String::new();
    if mode == "all" || mode == "workspace" {
        body.push_str(
            "    hl.gesture({ fingers = 3, direction = 'horizontal', action = 'workspace' })\n",
        );
    }
    if mode == "all" || mode == "desktop" {
        body.push_str(&format!(
            "    hl.gesture({{ fingers = 4, direction = 'down', action = hl.dsp.exec_cmd({}) }})\n    hl.gesture({{ fingers = 4, direction = 'up', action = hl.dsp.exec_cmd({}) }})\n",
            common::lua(&format!("{command} desktop show")),
            common::lua(&format!("{command} desktop restore"))
        ));
    }
    Ok(format!(
        "{BEGIN}-- mode: {mode}\ndo\n  local plugin = io.open({}, 'r')\n  if plugin then\n    plugin:close()\n{body}  end\nend\n{END}",
        common::lua(&manifest.to_string_lossy())
    ))
}

// Only this stable, guarded include lives in the user's main configuration.
pub fn include_hook(paths: &Paths) -> String {
    format!(
        "\n-- BEGIN FAMILIAR TRACKPAD PREFERENCE\ndo\n  local plugin = io.open({}, 'r')\n  if plugin then\n    plugin:close()\n    local config = io.open({}, 'r')\n    if config then config:close(); dofile({}) end\n  end\nend\n-- END FAMILIAR TRACKPAD PREFERENCE\n",
        common::lua(&paths.manifest.to_string_lossy()),
        common::lua(&paths.generated.to_string_lossy()),
        common::lua(&paths.generated.to_string_lossy())
    )
}

fn generated_content(paths: &Paths) -> Result<Option<String>> {
    match fs::symlink_metadata(&paths.generated) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.to_string()),
        Ok(_) => {
            let value = text(&paths.generated)?;
            if !["all", "workspace", "desktop"]
                .iter()
                .any(|mode| hook(mode, &paths.manifest).is_ok_and(|expected| value == expected))
            {
                return Err(
                    "Familiar's separate trackpad config was edited; no file changed".into(),
                );
            }
            Ok(Some(value))
        }
    }
}

pub fn split_current(value: &str, paths: &Paths) -> Result<(String, String)> {
    if value.contains("-- BEGIN FAMILIAR TRACKPAD PREFERENCE")
        || value.contains("-- END FAMILIAR TRACKPAD PREFERENCE")
    {
        let clean = crate::config_file::strip_exact(
            value,
            "-- BEGIN FAMILIAR TRACKPAD PREFERENCE",
            "-- END FAMILIAR TRACKPAD PREFERENCE",
            &include_hook(paths),
        )?;
        let (_, legacy) = split(&clean, &paths.manifest)?;
        if legacy != "reset" {
            return Err("Duplicate legacy and separate trackpad hooks; no file changed".into());
        }
        let mode = match generated_content(paths)? {
            Some(content) => split(&content, &paths.manifest)?.1,
            None => "reset".into(),
        };
        return Ok((clean, mode));
    }
    split(value, &paths.manifest)
}

fn restore_generated(paths: &Paths, expected: &str, before: &Option<String>) -> Result<()> {
    if text(&paths.generated)? != expected {
        return Err(
            "Separate trackpad config changed externally; preserve it and review the backup".into(),
        );
    }
    match before {
        Some(value) => common::atomic(&paths.generated, value.as_bytes()),
        None => fs::remove_file(&paths.generated).map_err(|e| e.to_string()),
    }
}

fn text(path: &Path) -> Result<String> {
    // Refuse symlink replacement (including broken links) and non-regular files.
    let metadata = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if !metadata.file_type().is_file() {
        return Err(
            "Hyprland config must be a regular file, not a symlink; no file changed".into(),
        );
    }
    String::from_utf8(common::bounded_file(path, common::FILE_LIMIT)?).map_err(|e| e.to_string())
}

pub fn split(text: &str, manifest: &Path) -> Result<(String, String)> {
    let starts = text.matches("-- BEGIN FAMILIAR TRACKPAD MODE").count();
    let ends = text.matches("-- END FAMILIAR TRACKPAD MODE").count();
    if starts == 0 && ends == 0 {
        return Ok((text.into(), "reset".into()));
    }
    if starts == 1 && ends == 1 {
        for mode in ["all", "workspace", "desktop"] {
            let block = hook(mode, manifest)?;
            if let Some(start) = text.find(&block) {
                return Ok((
                    format!("{}{}", &text[..start], &text[start + block.len()..]),
                    mode.into(),
                ));
            }
        }
    }

    Err("Familiar Trackpad block was edited, damaged or belongs to another installation; no file changed".into())
}

fn reload(hypr: &mut impl Hypr) -> Result<()> {
    checked(hypr, &["reload"])?;
    let errors: Vec<String> = serde_json::from_str(&hypr.command(&["-j", "configerrors"])?)
        .map_err(|_| "Could not verify Hyprland configuration errors")?;
    // Hyprland 0.56.2 / Hyprutils 0.14 serialises an empty error string as [""].
    // Ignore blank entries only; malformed responses and real diagnostics still fail.
    let errors: Vec<&str> = errors
        .iter()
        .map(|error| error.trim())
        .filter(|error| !error.is_empty())
        .collect();
    if !errors.is_empty() {
        return Err(format!(
            "Hyprland configuration error: {}",
            common::clipped(&errors.join("; "))
        ));
    }
    Ok(())
}

pub fn change(mode: &str, paths: &Paths, hypr: &mut impl Hypr) -> Result<Value> {
    if !["all", "workspace", "desktop", "reset", "status"].contains(&mode) {
        return Err("Usage: familiar-desktop gestures <all|workspace|desktop|reset|status>".into());
    }
    let _lock = if mode == "status" {
        None
    } else {
        Some(common::lock(&paths.state.join("config.lock"))?)
    };
    let before = text(&paths.config)?;
    let (clean, previous) = split_current(&before, paths)?;
    if mode == "status" {
        return Ok(
            json!({"state":"ok","mode":previous,"message":"Opt-in gestures. Existing gesture conflicts cause rollback; choose a separate group if needed."}),
        );
    }
    let after = if mode == "reset" {
        clean
    } else {
        if !paths.manifest.is_file() {
            return Err("Familiar manifest is missing; no file changed".into());
        }
        // Refuse unsupported compositor APIs before touching the user's config.
        checked(
            hypr,
            &[
                "eval",
                "assert(hl and hl.gesture and hl.dsp and hl.dsp.exec_cmd, 'Familiar Trackpad requires Hyprland Lua gesture support')",
            ],
        )?;
        format!("{}{}", clean, include_hook(paths))
    };
    let generated_before = generated_content(paths)?;
    let generated_after = if mode == "reset" {
        None
    } else {
        Some(hook(mode, &paths.manifest)?)
    };
    if let Some(content) = &generated_after {
        common::atomic(&paths.generated, content.as_bytes())?;
    }
    let backup = match crate::config_file::replace(&paths.config, &before, &after, &paths.state) {
        Ok(backup) => backup,
        Err(error) => {
            if let Some(content) = &generated_after {
                restore_generated(paths, content, &generated_before)?;
            }
            return Err(error);
        }
    };
    let applied = reload(hypr);
    if let Err(error) = applied {
        if text(&paths.config)? != after {
            return Err(format!(
                "{error}. Config changed externally; preserve your edits and recover from {backup:?}"
            ));
        }
        if let Some(content) = &generated_after {
            restore_generated(paths, content, &generated_before)?;
        }
        crate::config_file::replace(&paths.config, &after, &before, &paths.state)?;
        let recovery = reload(hypr);
        return Err(format!(
            "{error}. Previous configuration restored on disk. Recovery reload: {}",
            recovery.err().unwrap_or_else(|| "ok".into())
        ));
    }
    if mode == "reset"
        && let Some(content) = &generated_before
    {
        restore_generated(paths, content, &None)?;
    }
    Ok(
        json!({"state":"ok","mode":mode,"backup":backup,"message":if mode == "reset" { "Familiar gesture preferences removed; your configuration is restored." } else { "Trackpad gestures enabled. Three fingers switch workspaces; four down shows desktop and four up restores, according to the selected group." }}),
    )
}

pub fn execute(args: &[String]) -> Result<Value> {
    if args.len() != 1 {
        return Err("Usage: familiar-desktop gestures <all|workspace|desktop|reset|status>".into());
    }
    change(&args[0], &Paths::system()?, &mut common::SystemHypr)
}
