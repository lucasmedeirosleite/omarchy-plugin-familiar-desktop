//! Explicit, reversible keyboard preference. Never rewrite input.lua or core files.
use crate::{
    Result,
    common::{self, Hypr, checked},
};
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
};

const BEGIN: &str = "\n-- BEGIN FAMILIAR CAPS LOCK\n";
const END: &str = "-- END FAMILIAR CAPS LOCK\n";

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
            state: state.join("omarchy/familiar-caps-lock"),
            generated: common::config_home()?.join("omarchy/familiar-input/caps-lock.lua"),
        })
    }
}

pub fn hook(mode: &str, manifest: &Path) -> Result<String> {
    let option = match mode {
        "normal" => "caps:capslock",
        "compose" => "compose:caps",
        _ => return Err("Choose normal, compose or reset".into()),
    };
    Ok(format!(
        "{BEGIN}-- mode: {mode}\ndo\n  local plugin = io.open({}, \"r\")\n  if plugin then\n    plugin:close()\n    local options = {{}}\n    for option in (hl.get_config(\"input.kb_options\") or \"\"):gmatch(\"[^,]+\") do\n      option = option:match(\"^%s*(.-)%s*$\")\n      if not option:find(\"caps\", 1, true) then\n        table.insert(options, option)\n      end\n    end\n    table.insert(options, \"{option}\")\n    hl.config({{ input = {{ kb_options = table.concat(options, \",\") }} }})\n  end\nend\n{END}",
        common::lua(&manifest.to_string_lossy())
    ))
}

// Only this stable, guarded include lives in the user's main configuration.
pub fn include_hook(paths: &Paths) -> String {
    format!(
        "\n-- BEGIN FAMILIAR INPUT\ndo\n  local plugin = io.open({}, 'r')\n  if plugin then\n    plugin:close()\n    local config = io.open({}, 'r')\n    if config then config:close(); dofile({}) end\n  end\nend\n-- END FAMILIAR INPUT\n",
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
            if !["normal", "compose"]
                .iter()
                .any(|mode| hook(mode, &paths.manifest).ok().as_ref() == Some(&value))
            {
                return Err(
                    "Familiar's separate keyboard config was edited; no file changed".into(),
                );
            }
            Ok(Some(value))
        }
    }
}

pub fn split_current(value: &str, paths: &Paths) -> Result<(String, String)> {
    if value.contains("-- BEGIN FAMILIAR INPUT") || value.contains("-- END FAMILIAR INPUT") {
        let clean = crate::config_file::strip_exact(
            value,
            "-- BEGIN FAMILIAR INPUT",
            "-- END FAMILIAR INPUT",
            &include_hook(paths),
        )?;
        let (_, legacy) = split(&clean, &paths.manifest)?;
        if legacy != "reset" {
            return Err("Duplicate legacy and separate keyboard hooks; no file changed".into());
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
            "Separate keyboard config changed externally; preserve it and review the backup".into(),
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
    let starts = text.matches("-- BEGIN FAMILIAR CAPS LOCK").count();
    let ends = text.matches("-- END FAMILIAR CAPS LOCK").count();
    if starts == 0 && ends == 0 {
        return Ok((text.into(), "reset".into()));
    }
    if starts == 1 && ends == 1 {
        for mode in ["normal", "compose"] {
            let block = hook(mode, manifest)?;
            if let Some(start) = text.find(&block) {
                return Ok((
                    format!("{}{}", &text[..start], &text[start + block.len()..]),
                    mode.into(),
                ));
            }
        }
    }
    Err("Familiar Caps Lock block was edited, damaged or belongs to another installation; no file changed".into())
}

fn reload(hypr: &mut impl Hypr) -> Result<()> {
    checked(hypr, &["reload"])?;
    let errors: Vec<String> = serde_json::from_str(&hypr.command(&["-j", "configerrors"])?)
        .map_err(|_| "Could not verify Hyprland configuration errors")?;
    if !errors.is_empty() {
        return Err(format!(
            "Hyprland configuration error: {}",
            common::clipped(&errors.join("; "))
        ));
    }
    Ok(())
}

pub fn change(mode: &str, paths: &Paths, hypr: &mut impl Hypr) -> Result<Value> {
    if !["normal", "compose", "reset", "status"].contains(&mode) {
        return Err("Usage: familiar-desktop caps-lock <normal|compose|reset|status>".into());
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
            json!({"state":"ok","mode":previous,"message":"Saved preference; per-device keyboard overrides still take precedence."}),
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
                "assert(hl and hl.config and hl.get_config, 'Familiar Caps Lock requires Hyprland Lua configuration')",
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
    let applied = reload(hypr).and_then(|()| {
        if mode == "reset" { return Ok(()); }
        let option = if mode == "normal" { "caps:capslock" } else { "compose:caps" };
        checked(hypr, &["eval", &format!("assert((',' .. (hl.get_config('input.kb_options') or '') .. ','):find({}, 1, true), 'Caps Lock override did not apply')", common::lua(&format!(",{option},")))])
    });
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
        json!({"state":"ok","mode":mode,"backup":backup,"message":if mode == "reset" { "Using your keyboard configuration again." } else { "Caps Lock preference applied. Per-device overrides still take precedence." }}),
    )
}

pub fn execute(args: &[String]) -> Result<Value> {
    if args.len() != 1 {
        return Err("Usage: familiar-desktop caps-lock <normal|compose|reset|status>".into());
    }
    change(&args[0], &Paths::system()?, &mut common::SystemHypr)
}
