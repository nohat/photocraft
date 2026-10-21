//! `cargo xtask docs-claims [--base REF] [--out PATH]`: the deterministic half of the PR-time
//! documentation review (`.github/workflows/docs-claims.yml`).
//!
//! It narrows a diff down to what an agent (or a person) has to read to tell whether the
//! progress docs still describe the code: the files changed against `--base` (default
//! `origin/main`), the scorecard rows whose evidence cites one of those files, the rows the diff
//! itself edits, the progress docs the diff edits, and the result of `docs-sync --check`. The
//! result is a Markdown brief (`--out`, default `target/docs-claims/context.md`).
//!
//! Nothing here judges prose. The agent does that, reading this brief plus the docs, and only
//! the agent's findings are posted; the brief keeps it from rereading the whole tree for every PR.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use crate::scorecard;

/// The prose progress docs the review covers (AGENTS.md §4).
pub const PROGRESS_DOCS: &[&str] = &[
    "ROADMAP.md",
    "docs/roadmap.md",
    "docs/gaps.md",
    "docs/target-app-parity.md",
    "docs/ui-parity.md",
    "docs/brush-parity.md",
    "docs/context-menu-parity.md",
    "docs/file-format-parity.md",
    "docs/hardware-parity.md",
    "docs/localization-parity.md",
];

fn git(root: &Path, args: &[&str]) -> Result<String, String> {
    let out = Command::new("git").current_dir(root).args(args).output().map_err(|e| format!("git {}: {e}", args.join(" ")))?;
    if !out.status.success() {
        return Err(format!("git {}: {}", args.join(" "), String::from_utf8_lossy(&out.stderr).trim()));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// The names a scorecard note uses to cite `path`: its file name, and for files under a crate,
/// the crate-relative tail too (`io/tests/psd_kerning.rs`).
pub fn citations(path: &str) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(name) = path.rsplit('/').next()
        && name.ends_with(".rs")
    {
        out.push(name.to_string());
    }
    if let Some(rest) = path.strip_prefix("crates/") {
        out.push(rest.to_string());
    }
    out.sort();
    out.dedup();
    out
}

/// Whether `note` cites `name` as a whole token (so `atomic.rs` doesn't match `nonatomic.rs`).
pub fn cites(note: &str, name: &str) -> bool {
    note.match_indices(name).any(|(i, _)| {
        let before = note.get(..i).and_then(|s| s.chars().last());
        let after = note.get(i + name.len()..).and_then(|s| s.chars().next());
        before.is_none_or(|c| !(c.is_alphanumeric() || c == '_' || c == '/')) && after.is_none_or(|c| !(c.is_alphanumeric() || c == '_'))
    })
}

/// Checklist ids whose `[[item]]` block a unified diff of `scorecard/*.toml` touches.
pub fn edited_ids(diff: &str) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    let mut current: Option<String> = None;
    for line in diff.lines() {
        let body = line.get(1..).unwrap_or("");
        if line.starts_with("@@") || line.starts_with("diff ") {
            current = None;
            // The hunk header's context often names the block's id line.
            if let Some(id) = line.split("id = \"").nth(1).and_then(|r| r.split('"').next()) {
                current = Some(id.to_string());
            }
            continue;
        }
        if let Some(id) = body.trim().strip_prefix("id = \"").and_then(|r| r.strip_suffix('"')) {
            current = Some(id.to_string());
        }
        if body.trim() == "[[item]]" {
            current = None;
        }
        if (line.starts_with('+') || line.starts_with('-'))
            && !line.starts_with("+++")
            && !line.starts_with("---")
            && let Some(id) = &current
        {
            ids.insert(id.clone());
        }
    }
    ids
}

pub fn run(root: &Path, rest: &[&str]) -> Result<(), String> {
    let mut base = "origin/main".to_string();
    let mut out = root.join("target/docs-claims/context.md");
    let mut args = rest.iter();
    while let Some(a) = args.next() {
        match *a {
            "--base" => base = args.next().ok_or("--base needs a ref")?.to_string(),
            "--out" => out = root.join(args.next().ok_or("--out needs a path")?),
            other => return Err(format!("docs-claims: unknown option `{other}`")),
        }
    }
    let range = format!("{base}...HEAD");
    let changed: Vec<String> = git(root, &["diff", "--name-only", &range])?.lines().map(str::to_string).collect();
    let toml_diff = git(root, &["diff", &range, "--", "scorecard/"])?;
    let inp = scorecard::gather(root)?;
    let items: Vec<&scorecard::Item> = inp.checklists.iter().flat_map(|c| &c.item).collect();

    let mut s = format!("# Documentation review brief\n\nDiff: `{range}`, {} file(s) changed.\n\n", changed.len());

    s.push_str("## Scorecard rows whose evidence cites a changed file\n\n");
    let mut cited = Vec::new();
    for item in &items {
        let note = item.note.as_deref().unwrap_or("");
        let hits: Vec<&String> = changed.iter().filter(|p| citations(p).iter().any(|c| cites(note, c))).collect();
        if !hits.is_empty() {
            cited.push(format!(
                "- **{}** ({}, released {}, checked {}): {}\n  - cites: {}\n  - note: {}\n",
                item.id,
                format!("{:?}", item.status).to_lowercase(),
                item.release_cell(),
                item.checked.as_ref().map_or_else(|| "?".into(), ToString::to_string),
                item.target,
                hits.iter().map(|h| format!("`{h}`")).collect::<Vec<_>>().join(", "),
                note
            ));
        }
    }
    if cited.is_empty() {
        s.push_str("None.\n\n");
    } else {
        s.push_str(&cited.join(""));
        s.push('\n');
    }

    s.push_str("## Scorecard rows this diff edits\n\n");
    let edited = edited_ids(&toml_diff);
    if edited.is_empty() {
        s.push_str("None.\n\n");
    } else {
        for id in &edited {
            let row = items.iter().find(|i| &i.id == id);
            let state =
                row.map_or_else(|| "removed".to_string(), |i| format!("now {}, released {}", format!("{:?}", i.status).to_lowercase(), i.release_cell()));
            s.push_str(&format!("- **{id}** ({state})\n"));
        }
        s.push_str("\n```diff\n");
        s.push_str(&toml_diff);
        s.push_str("```\n\n");
    }

    s.push_str("## Progress docs\n\n");
    for d in PROGRESS_DOCS {
        let mark = if changed.iter().any(|c| c == d) { "edited in this diff" } else { "unchanged" };
        s.push_str(&format!("- `{d}`: {mark}\n"));
    }

    s.push_str("\n## `cargo xtask docs-sync --check`\n\n");
    match crate::docs_sync::run(root, &["--check"]) {
        Ok(()) => s.push_str("Passes: every fact is current and no status claim contradicts the scorecard.\n"),
        Err(e) => s.push_str(&format!("Fails: {e}\n")),
    }

    s.push_str("\n## Changed files\n\n");
    for c in &changed {
        s.push_str(&format!("- `{c}`\n"));
    }

    if let Some(dir) = out.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("create {}: {e}", dir.display()))?;
    }
    std::fs::write(&out, &s).map_err(|e| format!("write {}: {e}", out.display()))?;
    println!("wrote {} ({} cited row(s), {} edited row(s))", out.display(), cited.len(), edited.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn citation_names() {
        assert_eq!(citations("crates/ui-egui/src/caf_ui.rs"), ["caf_ui.rs", "ui-egui/src/caf_ui.rs"]);
        assert!(citations("crates/io/tests/psd_kerning.rs").contains(&"io/tests/psd_kerning.rs".to_string()));
        assert!(citations("docs/gaps.md").is_empty());
    }

    #[test]
    fn cites_whole_tokens_only() {
        assert!(cites("Full-window workspace (ui-egui caf_ui.rs): …", "caf_ui.rs"));
        assert!(cites("edit_menu_cmds.rs:324", "edit_menu_cmds.rs"));
        assert!(!cites("nonatomic.rs", "atomic.rs"));
        assert!(!cites("format/src/atomic/tests.rs", "tests.rs"));
        assert!(!cites("atomic.rsx", "atomic.rs"));
    }

    #[test]
    fn finds_edited_rows() {
        let diff = "\
diff --git a/scorecard/tools.toml b/scorecard/tools.toml
@@ -210,7 +210,8 @@ target = \"…\"
 [[item]]
 id = \"TOOL-214-15\"
 target = \"Content-Aware Fill workspace\"
-status = \"partial\"
+status = \"done\"
+released = \"0.11.1\"
 issue = 214
@@ -300,3 +301,3 @@ id = \"TOOL-213-12\"
-note = \"old\"
+note = \"new\"
 [[item]]
 id = \"TOOL-999-1\"
";
        let ids: Vec<String> = edited_ids(diff).into_iter().collect();
        assert_eq!(ids, ["TOOL-213-12", "TOOL-214-15"]);
    }
}
