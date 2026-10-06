//! Explicit native input preferences. No input daemon, global key swap or sudo.
use crate::{
    Result,
    common::{self, Hypr, checked},
    window_mode::Paths,
};
use serde_json::{Value, json};
use std::{fs, path::Path};

fn valid_kind(kind: &str) -> Result<()> {
    if ["resize", "command"].contains(&kind) {
        Ok(())
    } else {
        Err("Choose resize or command".into())
    }
}

pub fn paths(kind: &str) -> Result<Paths> {
    valid_kind(kind)?;
    let mut p = Paths::system()?;
    p.state = p.state.with_file_name(format!("familiar-{kind}"));
    p.generated = common::config_home()?.join(format!("omarchy/familiar-input/{kind}.lua"));
    Ok(p)
}

pub fn hook(kind: &str, manifest: &Path) -> Result<String> {
    valid_kind(kind)?;
    let body = if kind == "resize" {
        "hl.config({ general = { resize_on_border = true, extend_border_grab_area = 15, hover_icon_on_border = true } })\n"
    } else {
        include_str!("command_shortcuts.lua")
    };
    Ok(format!(
        "-- Familiar {kind}; managed from Input/Windows settings.\nlocal plugin=io.open({}, 'r')\nif not plugin then return end\nplugin:close()\n{body}",
        common::lua(&manifest.to_string_lossy())
    ))
}

fn markers(kind: &str) -> (String, String) {
    (
        format!("-- BEGIN FAMILIAR INPUT {kind}"),
        format!("-- END FAMILIAR INPUT {kind}"),
    )
}

pub fn include(kind: &str, p: &Paths) -> String {
    let (begin, end) = markers(kind);
    format!(
        "\n{begin}\ndo\n  local plugin=io.open({}, 'r')\n  if plugin then\n    plugin:close()\n    local file=io.open({}, 'r')\n    if file then file:close(); dofile({}) end\n  end\nend\n{end}\n",
        common::lua(&p.manifest.to_string_lossy()),
        common::lua(&p.generated.to_string_lossy()),
        common::lua(&p.generated.to_string_lossy())
    )
}

fn text(path: &Path) -> Result<String> {
    let m = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if !m.file_type().is_file() {
        return Err("Refusing a symlink or non-regular configuration file".into());
    }
    String::from_utf8(common::bounded_file(path, common::FILE_LIMIT)?).map_err(|e| e.to_string())
}

fn generated(kind: &str, p: &Paths) -> Result<Option<String>> {
    match fs::symlink_metadata(&p.generated) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.to_string()),
        Ok(_) => {
            let s = text(&p.generated)?;
            if s != hook(kind, &p.manifest)? {
                return Err("Familiar input config was edited; no files changed".into());
            }
            Ok(Some(s))
        }
    }
}

fn restore(p: &Paths, expected: &str, previous: &Option<String>) -> Result<()> {
    if text(&p.generated)? != expected {
        return Err("Input config changed externally; preserve it and review the backup".into());
    }
    match previous {
        Some(s) => common::atomic(&p.generated, s.as_bytes()),
        None => fs::remove_file(&p.generated).map_err(|e| e.to_string()),
    }
}

fn reload(h: &mut impl Hypr) -> Result<()> {
    checked(h, &["reload"])?;
    let errors: Vec<String> = serde_json::from_str(&h.command(&["-j", "configerrors"])?)
        .map_err(|_| "Cannot verify configuration errors")?;
    if errors.iter().any(|e| !e.trim().is_empty()) {
        return Err(format!(
            "Hyprland configuration error: {}",
            common::clipped(&errors.join("; "))
        ));
    }
    Ok(())
}

pub fn change(kind: &str, mode: &str, p: &Paths, h: &mut impl Hypr) -> Result<Value> {
    valid_kind(kind)?;
    if !["enable", "reset", "status"].contains(&mode) {
        return Err("Choose enable, reset or status".into());
    }
    let _lock = if mode == "status" {
        None
    } else {
        Some(common::lock(&p.state.join("config.lock"))?)
    };
    let before = text(&p.config)?;
    let (begin, end) = markers(kind);
    let owned = include(kind, p);
    let has_include = before.contains(&begin) || before.contains(&end);
    let clean = if has_include {
        crate::config_file::strip_exact(&before, &begin, &end, &owned)?
    } else {
        before.clone()
    };
    let old = generated(kind, p)?;
    if mode == "status" {
        return Ok(
            json!({"state":"ok","mode":if has_include && old.is_some(){"enable"}else{"reset"},"message":"Changes apply only when explicitly selected."}),
        );
    }
    if mode == "enable" {
        if !p.manifest.is_file() {
            return Err("Familiar manifest is missing".into());
        }
        let probe = if kind == "resize" {
            "assert(hl and hl.config, 'Native border resizing requires Hyprland Lua')"
        } else {
            "assert(hl and hl.bind and hl.unbind and hl.get_active_window and hl.dsp and hl.dsp.send_shortcut, 'Command shortcuts require Hyprland Lua')"
        };
        checked(h, &["eval", probe])?;
    }
    let new = if mode == "enable" {
        Some(hook(kind, &p.manifest)?)
    } else {
        None
    };
    if let Some(s) = &new {
        common::atomic(&p.generated, s.as_bytes())?;
    }
    let after = if mode == "enable" {
        format!("{clean}{owned}")
    } else {
        clean
    };
    let backup = match crate::config_file::replace(&p.config, &before, &after, &p.state) {
        Ok(b) => b,
        Err(e) => {
            if let Some(s) = &new {
                restore(p, s, &old)?;
            }
            return Err(e);
        }
    };
    if let Err(e) = reload(h) {
        if text(&p.config)? != after {
            return Err(format!(
                "{e}; configuration changed externally; review {backup:?}"
            ));
        }
        if let Some(s) = &new {
            restore(p, s, &old)?;
        }
        crate::config_file::replace(&p.config, &after, &before, &p.state)?;
        let recovery = reload(h);
        return Err(format!(
            "{e}. Previous configuration restored. Recovery reload: {}",
            recovery.err().unwrap_or_else(|| "ok".into())
        ));
    }
    if mode == "reset"
        && let Some(s) = &old
    {
        restore(p, s, &None)?;
    }
    Ok(
        json!({"state":"ok","mode":mode,"backup":backup,"message":if mode=="reset" {"Your original configuration is active again."} else if kind=="resize" {"Drag window borders or corners to resize; no modifier key required."} else {"Command editing shortcuts enabled. The listed Super bindings are replaced until reset."}}),
    )
}

pub fn execute(args: &[String]) -> Result<Value> {
    if args.len() != 2 {
        return Err("Usage: input-preference <resize|command> <enable|reset|status>".into());
    }
    change(
        &args[0],
        &args[1],
        &paths(&args[0])?,
        &mut common::SystemHypr,
    )
}
