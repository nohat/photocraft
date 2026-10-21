//! `cargo xtask docs-sync [--check]`: keep the numbers and checklist statuses quoted in the prose
//! progress docs equal to their sources.
//!
//! **Facts.** A doc embeds a measured value as `<!-- fact:KEY -->VALUE<!-- /fact -->`; this
//! command rewrites VALUE from the same inputs as `docs/scorecard.md` (corpus floors, checklist
//! counts and statuses, the performance baseline, the prefs audit, menu wiring), from
//! `cargo xtask i18n-coverage`, and from `scorecard/measured.toml`, the one place for numbers
//! someone measured by hand (each with a date and a source). `cargo xtask docs-sync --facts`
//! lists every key. An unknown key is an error.
//!
//! **Claims.** Prose that states a status for a scorecard checklist id ("TOOL-213-12 missing",
//! a parity-table row `| … | done | TOOL-213-12 … |`) must agree with `scorecard/*.toml`.
//! Revision histories, progress logs and sections whose heading says they are historical are
//! exempt, since they record what was true then.
//!
//! `--check` fails when a fact would change or a claim disagrees; CI runs it.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::scorecard::{self, ItemStatus};

const MEASURED: &str = "scorecard/measured.toml";
const OPEN: &str = "<!-- fact:";
const CLOSE: &str = "<!-- /fact -->";
/// Generated files are kept current by their own commands.
const GENERATED: &[&str] = &["docs/scorecard.md", "docs/parity-checklist.md"];
/// A heading containing one of these starts a section whose statements are dated records.
const EXEMPT_HEADINGS: &[&str] = &["revision history", "progress log", "historical", "earlier assessment", "kept for the record"];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Measured {
    #[serde(default)]
    fact: Vec<MeasuredFact>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct MeasuredFact {
    key: String,
    value: String,
    measured: toml::value::Datetime,
    source: String,
}

fn status_word(s: ItemStatus) -> &'static str {
    match s {
        ItemStatus::Done => "done",
        ItemStatus::Partial => "partial",
        ItemStatus::Missing => "missing",
    }
}

fn thousands(n: usize) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// `part / whole` as a percentage with one decimal (`99.3%`; whole numbers drop it: `100%`,
/// `0%`). A share short of the whole never rounds up to `100%`.
fn percent(part: usize, whole: usize) -> String {
    if whole == 0 {
        return "–".into();
    }
    let mut p = (1000.0 * part as f64 / whole as f64).round() / 10.0;
    if part < whole && p >= 100.0 {
        p = 99.9;
    }
    if p.fract() == 0.0 { format!("{p:.0}%") } else { format!("{p:.1}%") }
}

/// `part / whole` as a whole percentage (`73%`), for corpus pass rates.
fn percent0(part: usize, whole: usize) -> String {
    if whole == 0 { "–".into() } else { format!("{:.0}%", 100.0 * part as f64 / whole as f64) }
}

/// Every fact key and its current value.
pub fn facts(root: &Path) -> Result<BTreeMap<String, String>, String> {
    let inp = scorecard::gather(root)?;
    let mut f = BTreeMap::new();
    let mut put = |k: String, v: String| {
        f.insert(k, v);
    };
    for c in &inp.corpora {
        let slug = c.dir.rsplit('/').next().unwrap_or(&c.dir).to_string();
        let (pass, rt) = (c.pass_floor.unwrap_or(0) as usize, c.roundtrip_floor.unwrap_or(0) as usize);
        put(format!("corpus.{slug}.pass"), pass.to_string());
        put(format!("corpus.{slug}.roundtrip"), rt.to_string());
        if let Some(n) = c.files {
            let n = n as usize;
            put(format!("corpus.{slug}.files"), n.to_string());
            put(format!("corpus.{slug}.pass_of"), format!("{pass}/{n}"));
            put(format!("corpus.{slug}.pass_pct"), percent0(pass, n));
            put(format!("corpus.{slug}.roundtrip_of"), format!("{rt}/{n}"));
        }
    }
    for c in &inp.checklists {
        let (d, p, m) = c.counts();
        put(format!("checklist.{}", c.area), format!("{d} done, {p} partial, {m} missing"));
        put(format!("checklist.{}.short", c.area), format!("{d} / {p} / {m}"));
        for i in &c.item {
            put(format!("status.{}", i.id), status_word(i.status).into());
            put(format!("released.{}", i.id), i.release_cell());
        }
    }
    let outcomes = crate::perf::outcomes_from_baseline(&inp.budgets, inp.baseline.as_ref());
    let (pass, over, _, _) = crate::perf::counts(&outcomes);
    put("perf.met".into(), format!("{pass} of {}", pass + over));
    let base = inp.baseline.as_ref().and_then(|b| b.get("modes")?.get("full")?.get("date")?.as_str()).map(|d| d.get(..10).unwrap_or(d).to_string());
    put("perf.baseline".into(), base.unwrap_or_else(|| "not run".into()));
    put("prefs.unread".into(), format!("{} of {}", inp.prefs.1.len(), inp.prefs.0));
    if let Some(line) = &inp.parity {
        let nums: Vec<&str> = line.split_whitespace().filter(|w| w.bytes().all(|b| b.is_ascii_digit() || b == b',')).collect();
        if let [live, total, ..] = nums.as_slice() {
            put("menu.live".into(), format!("{live}/{total}"));
        }
    }
    let cov = crate::i18n_coverage::coverage(root)?;
    put("i18n.keys".into(), thousands(cov.keys));
    for (code, n) in &cov.languages {
        put(format!("i18n.{code}"), percent(*n, cov.keys));
        put(format!("i18n.{code}.of"), format!("{} / {}", thousands(*n), thousands(cov.keys)));
    }
    let path = root.join(MEASURED);
    let text = std::fs::read_to_string(&path).map_err(|e| format!("read {MEASURED}: {e}"))?;
    let measured: Measured = toml::from_str(&text).map_err(|e| format!("{MEASURED}: {e}"))?;
    for m in measured.fact {
        if m.source.trim().is_empty() || m.value.contains("-->") {
            return Err(format!("{MEASURED}: {}: needs a source and a value without `-->`", m.key));
        }
        if f.contains_key(&m.key) {
            return Err(format!("{MEASURED}: {} is computed from the tree; don't set it by hand", m.key));
        }
        f.insert(format!("{}.measured", m.key), m.measured.to_string());
        f.insert(m.key, m.value);
    }
    Ok(f)
}

/// `text` with every fact marker's value replaced. Errors name unknown keys and unclosed markers.
pub fn fill(name: &str, text: &str, facts: &BTreeMap<String, String>) -> Result<String, String> {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find(OPEN) {
        // Inside an inline code span (an odd number of backticks earlier on the line) the marker
        // is an example, not a fact.
        let line_start = format!("{out}{}", rest.get(..start).unwrap_or("")).rfind('\n').map_or(0, |i| i + 1);
        let so_far = format!("{out}{}", rest.get(..start).unwrap_or(""));
        if so_far.get(line_start..).is_some_and(|l| l.matches('`').count() % 2 == 1) {
            out.push_str(rest.get(..start + OPEN.len()).unwrap_or(""));
            rest = rest.get(start + OPEN.len()..).unwrap_or("");
            continue;
        }
        let after = rest.get(start + OPEN.len()..).unwrap_or("");
        let Some(key_end) = after.find("-->") else {
            return Err(format!("{name}: unterminated `{OPEN}` marker"));
        };
        let key = after.get(..key_end).unwrap_or("").trim();
        let body = after.get(key_end + 3..).unwrap_or("");
        let Some(close) = body.find(CLOSE) else {
            return Err(format!("{name}: `{OPEN}{key} -->` has no `{CLOSE}`"));
        };
        if body.get(..close).is_some_and(|v| v.contains('\n') || v.contains(OPEN)) {
            return Err(format!("{name}: fact `{key}` must open and close on one line"));
        }
        let Some(value) = facts.get(key) else {
            return Err(format!("{name}: unknown fact `{key}` (cargo xtask docs-sync --facts lists them)"));
        };
        out.push_str(rest.get(..start).unwrap_or(""));
        out.push_str(&format!("{OPEN}{key} -->{value}{CLOSE}"));
        rest = body.get(close + CLOSE.len()..).unwrap_or("");
    }
    out.push_str(rest);
    Ok(out)
}

/// A checklist id: upper-case letters, then two dash-separated numbers (`TOOL-213-12`).
fn ids_in(line: &str) -> Vec<(usize, &str)> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let starts = b.get(i).is_some_and(u8::is_ascii_uppercase) && (i == 0 || b.get(i - 1).is_some_and(|c| !c.is_ascii_alphanumeric() && *c != b'-'));
        if !starts {
            i += 1;
            continue;
        }
        let mut j = i;
        while b.get(j).is_some_and(u8::is_ascii_uppercase) {
            j += 1;
        }
        let mut groups = 0;
        while b.get(j) == Some(&b'-') && b.get(j + 1).is_some_and(u8::is_ascii_digit) {
            j += 1;
            while b.get(j).is_some_and(u8::is_ascii_digit) {
                j += 1;
            }
            groups += 1;
        }
        let ends = b.get(j).is_none_or(|c| !c.is_ascii_alphanumeric() && *c != b'-');
        if groups == 2
            && ends
            && let Some(id) = line.get(i..j)
        {
            out.push((i, id));
        }
        i = j.max(i + 1);
    }
    out
}

fn as_status(w: &str) -> Option<ItemStatus> {
    match w {
        "done" => Some(ItemStatus::Done),
        "partial" => Some(ItemStatus::Partial),
        "missing" => Some(ItemStatus::Missing),
        _ => None,
    }
}

/// The status a line states for `id`, if any: `ID done`, `ID is/stays/now/still partial`,
/// `ID moved to done`, or a table row whose status cell is exactly a status word and whose next
/// cell starts with the id (the parity docs' `| behaviour | status | evidence |` rows).
fn stated(line: &str, at: usize, id: &str) -> Option<ItemStatus> {
    let after = line.get(at + id.len()..).unwrap_or("");
    let clause: String = after.chars().take_while(|c| !".;,|()[]:—".contains(*c)).collect::<String>().to_lowercase();
    let mut words = clause.split_whitespace().peekable();
    while words.peek().is_some_and(|w| ["is", "stays", "now", "still", "marked", "moved", "to", "as", "row"].contains(w)) {
        words.next();
    }
    if let Some(s) = words.next().and_then(as_status) {
        return Some(s);
    }
    if line.trim_start().starts_with('|') {
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        for (k, cell) in cells.iter().enumerate() {
            if cell.starts_with(id) && cell.get(id.len()..).is_some_and(|r| r.is_empty() || r.starts_with([',', ' ', ';'])) {
                return k.checked_sub(1).and_then(|p| cells.get(p)).and_then(|c| as_status(c));
            }
        }
    }
    None
}

/// Disagreements between the statuses `text` states for checklist ids and `statuses`.
pub fn check_claims(name: &str, text: &str, statuses: &BTreeMap<String, ItemStatus>) -> Vec<String> {
    let mut problems = Vec::new();
    let mut exempt_level: Option<usize> = None;
    let mut in_code = false;
    for (n, line) in text.lines().enumerate() {
        if line.trim_start().starts_with("```") {
            in_code = !in_code;
            continue;
        }
        if in_code {
            continue;
        }
        let level = line.bytes().take_while(|b| *b == b'#').count();
        if level > 0 && line.as_bytes().get(level) == Some(&b' ') {
            if exempt_level.is_some_and(|l| level <= l) {
                exempt_level = None;
            }
            let h = line.to_lowercase();
            if exempt_level.is_none() && EXEMPT_HEADINGS.iter().any(|e| h.contains(e)) {
                exempt_level = Some(level);
            }
            continue;
        }
        if exempt_level.is_some() || line.starts_with('>') {
            continue;
        }
        for (at, id) in ids_in(line) {
            let (Some(claim), Some(actual)) = (stated(line, at, id), statuses.get(id)) else { continue };
            if claim != *actual {
                problems.push(format!("{name}:{}: says {id} is {}, scorecard/*.toml says {}", n + 1, status_word(claim), status_word(*actual)));
            }
        }
    }
    problems
}

/// The prose docs this command maintains: `ROADMAP.md`, `AGENTS.md` and `docs/*.md`, minus
/// generated files.
fn docs(root: &Path) -> Vec<(String, PathBuf)> {
    let mut out = vec![("ROADMAP.md".to_string(), root.join("ROADMAP.md")), ("AGENTS.md".to_string(), root.join("AGENTS.md"))];
    if let Ok(rd) = std::fs::read_dir(root.join("docs")) {
        let mut more: Vec<(String, PathBuf)> = rd
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "md"))
            .filter_map(|p| Some((format!("docs/{}", p.file_name()?.to_string_lossy()), p)))
            .filter(|(n, _)| !GENERATED.contains(&n.as_str()))
            .collect();
        more.sort();
        out.extend(more);
    }
    out.into_iter().filter(|(_, p)| p.exists()).collect()
}

pub fn run(root: &Path, rest: &[&str]) -> Result<(), String> {
    let check = rest.contains(&"--check");
    let list = rest.contains(&"--facts");
    if let Some(bad) = rest.iter().find(|a| !["--check", "--facts"].contains(*a)) {
        return Err(format!("docs-sync: unknown option `{bad}`"));
    }
    let facts = facts(root)?;
    if list {
        for (k, v) in &facts {
            println!("{k} = {v}");
        }
        return Ok(());
    }
    let statuses: BTreeMap<String, ItemStatus> = scorecard::gather(root)?.checklists.iter().flat_map(|c| &c.item).map(|i| (i.id.clone(), i.status)).collect();
    let (mut stale, mut problems, mut markers) = (Vec::new(), Vec::new(), 0);
    for (name, path) in docs(root) {
        let text = std::fs::read_to_string(&path).map_err(|e| format!("read {name}: {e}"))?.replace("\r\n", "\n");
        markers += text.matches(OPEN).count();
        let filled = fill(&name, &text, &facts)?;
        problems.extend(check_claims(&name, &filled, &statuses));
        if filled != text {
            if check {
                stale.push(name);
            } else {
                std::fs::write(&path, &filled).map_err(|e| format!("write {name}: {e}"))?;
                println!("updated {name}");
            }
        }
    }
    for p in &problems {
        eprintln!("{p}");
    }
    if !stale.is_empty() {
        return Err(format!("facts are stale in {}: run `cargo xtask docs-sync` and commit the result", stale.join(", ")));
    }
    if !problems.is_empty() {
        return Err(format!("{} status claim(s) disagree with scorecard/*.toml: fix the prose, or the row if the prose is right", problems.len()));
    }
    println!("docs-sync: {markers} facts current, no status claim disagrees with the scorecard");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facts_of(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn fills_markers_and_rejects_bad_ones() {
        let f = facts_of(&[("a", "188"), ("b", "24 of 171")]);
        let t = "floor <!-- fact:a -->133<!-- /fact --> and <!-- fact:b --><!-- /fact -->.\n";
        assert_eq!(fill("t", t, &f).unwrap(), "floor <!-- fact:a -->188<!-- /fact --> and <!-- fact:b -->24 of 171<!-- /fact -->.\n");
        assert!(fill("t", "<!-- fact:zz -->1<!-- /fact -->", &f).unwrap_err().contains("unknown fact `zz`"));
        assert!(fill("t", "<!-- fact:a -->1", &f).is_err());
        assert!(fill("t", "<!-- fact:a -->1\n<!-- /fact -->", &f).is_err());
        assert_eq!(fill("t", "no markers", &f).unwrap(), "no markers");
        let example = "quote it as `<!-- fact:KEY -->VALUE<!-- /fact -->` and <!-- fact:a -->1<!-- /fact -->\n";
        assert_eq!(fill("t", example, &f).unwrap(), example.replace(">1<", ">188<"), "markers in code spans are examples");
    }

    #[test]
    fn percentages_never_round_up_to_whole() {
        assert_eq!(percent(1918, 1918), "100%");
        assert_eq!(percent(1904, 1918), "99.3%");
        assert_eq!(percent(19_999, 20_000), "99.9%");
        assert_eq!(percent(0, 1918), "0%");
        assert_eq!(percent0(188, 258), "73%");
        assert_eq!(thousands(1918), "1,918");
        assert_eq!(thousands(12), "12");
    }

    #[test]
    fn finds_ids() {
        let ids: Vec<&str> = ids_in("TOOL-213-12, (UI-217-5) X-1 AB-1-2-3 #215 FILE-215-11.").into_iter().map(|(_, id)| id).collect();
        assert_eq!(ids, ["TOOL-213-12", "UI-217-5", "FILE-215-11"]);
    }

    #[test]
    fn checks_stated_statuses() {
        let st: BTreeMap<String, ItemStatus> = [("TOOL-213-12".to_string(), ItemStatus::Done), ("UI-217-5".to_string(), ItemStatus::Partial)].into();
        let doc = "\
# Doc
- no image file as a brush tip (TOOL-213-12 missing)
- UI-217-5 stays partial; TOOL-213-12, #3279
| Brush tip from an image file | missing | TOOL-213-12 |
| Panels | missing (restore) | UI-217-5 partial, #1828 |
> **Change:** TOOL-213-12 missing in the header line
## Revision history
| 2026-10-10 | TOOL-213-12 missing |
## Later
TOOL-213-12 is done
";
        let p = check_claims("d.md", doc, &st);
        assert_eq!(p, ["d.md:2: says TOOL-213-12 is missing, scorecard/*.toml says done", "d.md:4: says TOOL-213-12 is missing, scorecard/*.toml says done"]);
    }

    #[test]
    fn the_tree_is_in_sync() {
        let root = crate::root();
        run(&root, &["--check"]).unwrap_or_else(|e| panic!("{e}"));
    }
}
