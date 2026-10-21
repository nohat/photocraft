# Roadmap

> **Last reviewed:** 2026-10-21 · **Last updated:** 2026-10-21 · **Change:** major (reassessment against 0.11.1 and `main` at 68348dc: Current focus replaced by five evidence-based milestones; the 0.7/0.8/0.9 sequence and the beta hour estimate archived as historical; alpha-gate evidence corrected) · **Target:** Adobe Photoshop 2026

- UI.5: Panels can detach, join other tab groups, split, resize and redock through shared docking transactions. Custom layouts persist with workspaces, respect workspace locking, and retain collapse flags for intact native groups.

- UI.3: Shared icons, accessible icon buttons and panel tabs come from the pinned `craft-ui` library. Tab overflow stays within narrow panels; document tiling uses bounded shared geometry. Palettes and document commands remain app-owned. Existing strict-Clippy boolean/range warnings are corrected.

- UI.4: Compact toolbars keep colour and mode controls visible while tool rows scroll. Long tool-options rows scroll horizontally and reveal keyboard focus; offscreen geometry and interaction regressions cover all themes and 1×/1.5×/2× display scales.


Forward-looking plan: milestones, the **Current focus** list, and what's next with estimates.
Where we stand today is in [`target-app-parity.md`](target-app-parity.md) (two numbers, by
dimension and area) and the ranked work list in [`gaps.md`](gaps.md); the one-page summary is
[`ROADMAP.md`](../ROADMAP.md). Measured numbers: [`scorecard.md`](scorecard.md)
(`cargo xtask scorecard`: performance budgets, corpus floors, per-area checklists, dead
preferences) and [`parity-checklist.md`](parity-checklist.md) (`cargo xtask parity`: menu wiring,
628/628, which says nothing about behaviour).

Status legend: ✅ done · 🟡 in progress · ⬜ not started. Hours are Opus 5.5 agent wall-clock hours
for one agent working sequentially (calibration in
[`target-app-parity.md`](target-app-parity.md#remaining-effort-and-calibration)).

**Stage: alpha.** Beta needs the evidence listed under [Current focus](#current-focus), not a
readiness percentage. The ~45% ready and ~1,100–1,800 h figures are the 2026-10-10 estimates,
kept as historical until they are recalculated ([reassessment](../ROADMAP.md#reassessment-2026-10-21)).

## Alpha gate

The core workflows a typical Photoshop professional runs every day, checked end to end on the main
platform (macOS, checked on 0.6.0; evidence updated 2026-10-21), including saving and reopening the work. Any **no**, or a **partial**
that blocks the workflow, would make PhotoCraft pre-alpha.

| Core workflow | Works end to end? | Evidence | Hours to pass |
|---|---|---|---:|
| Photo retouch: open JPEG/raw, heal/clone/remove, adjustment layers, export JPEG, save and reopen | yes | Spot Healing, Healing, Clone, Patch, Remove tools; 16 adjustment layers; raw for DNG/CR2/NEF/ARW/RAF; JPEG export; MCP workflow tests (`automation/tests/agent_tasks.rs`). Quality complaints (#1092, #2104) and no CR3 don't stop the workflow | 0 |
| Composite: layers, masks, selections, blend modes, smart objects; save as PSD and `.pcraft` and reopen | yes | PSD round trip ≥ 169/170, 308/309, 258/258 corpus files (our own export and import, not Photoshop's); `.pcraft` atomic save and crash recovery. Multi-layer Smart Object conversion (#2604) and mask targeting (#2458) are partial but have workarounds | 0 |
| Cut-out: select a subject, refine the edge, mask, export PNG with transparency | partial (not blocking) | Object/Quick Selection, Select Subject (classical), Select and Mask dialog with view modes and Refine Edge Brush, layer masks, PNG export. Select and Mask is a dialog, not the workspace; no Refine Hair (#2671) | 0 (depth: 80–140 h, G7) |
| Design: type, shapes, layer styles, artboards; export PNG/JPEG; save and reopen | partial (not blocking) | Point/paragraph type, shapes with stroke controls, all 10 layer effects, artboards, Export As. One macOS report of a 60 s freeze on choosing the Type tool (#2518, not reproduced); no type on a path | 0 (fix #2518: 2–6 h) |
| Paint with a pen tablet: pressure brushes, layers, save and reopen | partial (not blocking) | Pressure, tilt, rotation on macOS (AppKit monitor); every Brush Settings section. Lag with big brushes (#2619) and feel gaps ([brush-parity.md](brush-parity.md)) | 0 (feel: 100–180 h, G4) |
| Exchange PSDs with Photoshop users: open a client PSD, edit, send a PSD back | partial (not blocking for alpha; blocking for beta) | Photoshop-authored oracles: floor 188/258 render as Photoshop does (2026-10-14). Whether our exports reopen in Photoshop is not tested (FILE-216-4 missing); #1281 is open without enough detail to reproduce, and #2469 was closed as a duplicate, not as fixed | 0 for alpha; beta needs the acceptance evidence in Current focus 1 |

**Result: passes.** Every core workflow completes end to end and the work saves and reopens in
PhotoCraft; the partial rows are depth and fidelity gaps with workarounds, not blockers. The
historical ready-for-real-work estimate (~45%, 2026-10-10) was inside the alpha band
(~40–75%); it is pending recalculation.

## Milestones

| M | Status | Where we are |
|---|---|---|
| M0 Skeleton | ✅ | workspace, xtask (layers / wasm / ci / stats / corpus / parity), CI workflow |
| M1 Foundation | ✅ | geom, color (27 blend modes), raster (COW tiles, any depth), doc, ops, cms (ICC) |
| M2 PSD v1 | ✅ | photocraft-psd: 134/135 real files byte-exact round trip |
| M3 Viewer app | ✅ | egui shell (Pro / Studio / Classic themes), 13+ codecs, native and web (trunk) builds |
| M4 Native format + engine | ✅ | ~700 commands, `.pcraft` (incremental, autosave, crash recovery), CLI, persistent preferences |
| M5 GPU compositor | 🟡 | wgpu compositor drives the canvas, layer effects, vector masks, artboards, pattern fills and every clip case included (≤1/255 vs CPU); Multichannel documents fall back to the CPU |
| M6 Paint + select | 🟡 | brush engine, all selection tools, multi-layer selection, snapping + smart guides, free transform + warp; stylus pressure on Windows (WM_POINTER via winit), the web (Pointer Events, with tilt/twist), macOS (AppKit event monitor: pressure, tilt, rotation, eraser end) and Linux X11 (XInput2 raw valuators); on Linux Wayland a pen opens the window through Xwayland (native tablet-v2 still open, #79) |
| M7 Adjust + filters | 🟡 | 16 adjustment layers + destructive-only adjustments, 70+ filters incl. Blur Gallery, Actions record/replay, Fade |
| M8 PSD v2 | 🟡 | adjustments (incl. Selective Color, Color Lookup), fills, effects, patterns, text, shapes, smart objects, alpha channels; oracle floors 148/170 (io), 249/309 (psd-tools), 188/258 (Photoshop-authored), as enforced on `main` 2026-10-21 |
| M9 Text, vector, styles | 🟡 | type engine + Warp Text, shapes / pen / paths, all 10 effects on CPU and GPU (parity ≤1/255, 30/31 corpus effect files on the GPU) |
| M10 Smart features | 🟡 | classical Select Subject / Object, Content-Aware Fill workspace (#2376) and Content-Aware Scale, healing, auto-align / auto-blend; an embedded CNN face detector (`image.detectFaces`, 0.10.0, #4513); general ML backend not started |
| M11 Automation + formats | 🟡 | MCP (headless + live bridge), batch, Image Processor, prefs over MCP; DoD test passes (10 agent tasks over MCP, `automation/tests/agent_tasks.rs`); since 2026-10-07 the CLI rejects unknown flags and answers `<subcommand> --help`, batch runs report same-name outputs instead of overwriting them, MCP `command_batch` steps can start background jobs, and headless saves never flatten over the opened file; JP2 / DICOM / DPX / C2PA pending |
| M12 Pro parity | 🟡 | CMYK / Lab / Indexed / Bitmap / Duotone, ICC + soft proofing, channels + Quick Mask, smart-object stack modes, artboards, layer comps; print, HDR, timeline pending; Photomerge is implemented, with real-world quality validation still open |

Releases: 0.1.0 (2026-10-02), 0.2.0 (2026-10-05, first signed and notarized), 0.3.0 (10-07),
0.5.0 (10-08), 0.6.0 (10-10), 0.8.1 and 0.8.2 (10-12), 0.9.0 (10-14), 0.10.0 (10-17), 0.11.0 and
0.11.1 (10-20).

## Current focus

Proposed 2026-10-21, replacing the five-item list of 2026-10-10 (kept below). Each milestone
closes on evidence, not on an hour count, and every status we report carries an evidence date
and says whether the work is **released**, **merged** or **validated**. Pick from the top unless
an issue is assigned to you.

1. **Reliable document exchange and saving** ([G1](gaps.md#g1-psd-fidelity-and-photoshop-acceptance)).
   Closes when representative exports have been reopened in Photoshop, colour mode, bit depth and
   Smart Object cases are covered, and any unresolved destructive-save failures are listed
   explicitly. Link canonical, reproducible cases with their results rather than #1281 (open,
   too little detail) or #2469 (closed as a duplicate).
2. **Measured interactive performance** ([G2](gaps.md#g2-interactive-performance)). Closes on
   fresh measurements on named hardware for brushes, transforms, layer edits, large documents,
   background work and cancellation. The scorecard baseline is from 2026-10-05 and doesn't
   describe today's code in either direction.
3. **Remaining editing depth** ([G4](gaps.md#g4-brush-feel), [G6](gaps.md#g6-missing-tools),
   [G7](gaps.md#g7-workspaces-shown-as-parameter-dialogs), [G8](gaps.md#g8-type-depth)).
   Closes when Select and Mask, type on a path, brush behaviour, the remaining specialised
   workspaces and Art History Brush each have written residual requirements, and those are met.
4. **Hardware, localization and distribution acceptance** ([G12](gaps.md#g12-pen-tablets-and-display-hardware),
   [G13](gaps.md#g13-hindi-arabic-rtl-engine-messages-and-catalog-review),
   [G17](gaps.md#g17-platforms-and-distribution)). Closes on tablet tests on real devices,
   measured translation coverage, an RTL evaluation and verification of the signed artifacts.
5. **Beta acceptance.** Repeatable core workflows pass on the supported platforms, judged by
   severity-based blocker criteria (data loss, reproducibility, whether a supported workflow is
   affected) with the limitations documented. This replaces "bug backlog under 100": the
   open-issue count (1,748 on 2026-10-21, requests included) doesn't measure regressions or
   release blockers.

## What's next (historical, 2026-10-10)

The release sequence and estimates below were written for 0.6.0. Releases 0.7 to 0.11 shipped
without following it, and the beta total still counts work that is done, as does its
conversion to ~100–150 wall-clock hours at 12 agents in
[`target-app-parity.md`](target-app-parity.md#remaining-effort-and-calibration). Kept for the
record; no rebuilt estimate replaces it yet.

| Next milestone | Scope | Estimate |
|---|---|---:|
| 0.7 "files you can trust" | PSD that Photoshop trusts, JPEG Options, AVIF read, Photoshop PDF read | 200–320 h |
| 0.8 "fast enough" | Performance budgets; GPU tiling past the texture limit (#49); complex layout documents | 200–350 h |
| 0.9 "feels like Photoshop" | Brush and transform feel and the missing tools; transform context menu; floating panels; Color panel | 300–500 h |
| Beta | All of the above, type depth, Select and Mask / Content-Aware Fill workspaces, Hindi/Arabic/Vietnamese + RTL, bug backlog under 100 | 1,100–1,800 h total |
| Later, needs owner decisions | AI backend (#41), plug-in hosting (.8BF / UXP), scripting compatibility | 550–1,050 h |

The 2026-10-10 Current focus was: PSD that Photoshop trusts (150–250 h), performance budgets
(200–350 h), the user bug backlog (250–400 h), brush and transform feel (150–250 h) and 14
missing tools (60–100 h).

## Milestone definitions

Each milestone has a **definition of done (DoD)** and must leave `main` green on all Tier-1 platforms plus a wasm build.

| M | Name | Scope (key items) | DoD / acceptance |
|---|---|---|---|
| **M0** | Skeleton | Workspace, all crate stubs, lints, `xtask` (layers check, ci), CI matrix, testkit | `cargo test --workspace` green; `cargo check --target wasm32-unknown-unknown -p photocraft-engine -p photocraft-ui-egui` green; layering check passes |
| **M1** | Foundation | geom, color (formats, blend math for all 27 modes, sRGB/linear), raster (sparse COW tiles, U8/U16/F32), doc (full type model incl. CMYK/Lab/adjust/smart), ops (history) | Property tests (proptest) on tile COW and history; blend-mode reference tests against published formulas |
| **M2** | PSD v1 | `photocraft-psd`: header, resources, layer records, channel data (raw/RLE/ZIP/ZIP+pred), masks, groups (lsct), unicode names, unknown-block passthrough, merged image, PSB; writer | Round-trip byte-stability tests; synthetic PSD generator tests; fuzz target; ≥150 unit tests |
| **M3** | Viewer app | codecs (png/jpeg/tiff/webp/gif/bmp/tga/pnm/qoi/exr/hdr, all read+write), CPU compositor, egui shell (menu from registry, canvas pan/zoom, layers panel, history), open/save, native + web build | Opens PNG/JPEG/PSD, shows layers, toggles visibility, undo/redo, saves PNG/PSD; web build loads a file from the browser |
| **M4** | Native format + engine | `.pcraft` bundle, command registry with schemas, jobs/cancellation, snapshots via arc-swap, CLI (`convert`, `run`, `inspect`) | CLI parity tests: every GUI command also runs headless |
| **M5** | GPU compositor | wgpu planner backend, tile residency, parity tests vs CPU (all blend modes), viewport on GPU, mips | GPU vs CPU ≤1/255; pan/zoom 60 fps on a 100 MP / 20-layer document |
| **M6** | Paint + select | platform input (pen: macOS/Windows/Linux/web), brush engine v1, eraser, marquee/lasso/wand, move, free transform, crop | Brush latency <1 frame; selection-restricted paint tests |
| **M7** | Adjust + filters | 16 adjustment layers, first 30 filters, schema-generated dialogs + live preview, actions record/replay | Golden tests at 8/16/32f; action replay determinism |
| **M8** | PSD v2 | adjustment layers, fill layers, lfx2 effects (descriptor parser), text (EngineData) preserve+render, smart objects, vector masks | Composite-oracle pass rate ≥90% on corpus |
| **M9** | Text, vector, styles | parley text layers, shapes/pen/paths, all 10 layer effects on GPU | Visual goldens; PSD text round-trip |
| **M10** | Smart features | `ml` (ort native / ort-web on the web), Select Subject/Object/Sky, Remove BG, Remove tool, content-aware fill, healing, AI denoise, RAW develop | Quality benchmarks on a public dataset; timing budgets |
| **M11** | Automation + formats | MCP server, batch, scripting, remaining formats (JP2, DICOM, DPX…), C2PA | An agent completes 10 scripted edit tasks via MCP |
| **M12** | Pro parity | CMYK/Lab UI, print, HDR display/merge, Photomerge quality validation, timeline, layer comps, artboards, symmetry, neural filters | `xtask parity` ≥ 90% of Photoshop menu checklist |

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-21 | major | Reassessment against 0.11.1 and `main` at 68348dc: Current focus replaced by five evidence-based milestones; the 0.7/0.8/0.9 sequence and beta hours archived as historical; alpha-gate PSD evidence corrected (oracle floor 188/258, round trip 258/258, FILE-216-4 still missing, #2469 closed as duplicate); M8 floors and M10 (Content-Aware Fill workspace, face detector) updated; releases to 0.11.1 listed |
| 2026-10-11 | minor | Linked adaptive Unsharp Mask performance and quality evidence; milestones, alpha gate and estimates unchanged |
| 2026-10-10 | minor | Photomerge is implemented; real-world quality validation remains open |
| 2026-10-10 | minor | Alpha gate table (core-workflow gate from the progress-docs standard): passes, stage stays alpha |
| 2026-10-10 | major | Progress-docs standard: honest assessment and dated measurements moved to `target-app-parity.md`, Affinity notes to `file-format-parity.md`; new Current focus and estimates; `parity.md` renamed `parity-checklist.md` |
| 2026-10-08 | minor | Tools, Affinity import and performance measurements added |
| 2026-10-05 | major | Honest parity assessment added |
