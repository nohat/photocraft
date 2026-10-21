# PhotoCraft roadmap

**Stage: alpha** · next: beta, once the [beta acceptance](#remaining-milestones) evidence exists. The readiness percentages and hours on this page are the 2026-10-10 estimates, kept as historical until they are recalculated.

> **Last reviewed:** 2026-10-21 · **Last updated:** 2026-10-21 · **Change:** major (dated reassessment against 0.11.1 and `main` at 68348dc: delivered features credited, aggregate estimates relabelled historical, milestones replaced by evidence-based deliverables) · **Target:** Adobe Photoshop 2026

PhotoCraft's one-page summary of how close it is to Adobe Photoshop 2026 (27.11.0, measured from
the installed app on 2026-10-10). The full re-measure was at version 0.6.0; the
[reassessment of 2026-10-21](#reassessment-2026-10-21) checks its claims against 0.11.1. Detail: [target-app parity](docs/target-app-parity.md)
(methodology, weights, evidence), [gaps](docs/gaps.md) (the ranked work list),
[roadmap](docs/roadmap.md) (milestones, current focus), [scorecard](docs/scorecard.md) and
[menu checklist](docs/parity-checklist.md) (both generated and measured),
[architecture](docs/architecture.md).

Hours are Opus 5.5 agent wall-clock hours for one agent working sequentially, calibrated from this
repository's 1,004 merged PRs (see [calibration](docs/target-app-parity.md#remaining-effort-and-calibration)).
About 80% of the work parallelizes.

## Reassessment (2026-10-21)

An audit of releases, source, tests and documentation, comparing these planning documents with
the 0.11.1 release (2026-10-20) and `main` at 68348dc. It is not a new benchmark or usability
evaluation, so it changes what we claim as delivered and how much weight the old numbers can
carry, and it does not produce new percentages.

The short version: feature completeness has moved further than this page said, and release
confidence still needs evidence. The list of missing major features is a lot shorter than the
old milestones suggest. The most important validation requirement, our exports reopening
correctly in Photoshop, is still open (scorecard FILE-216-4 is missing), and internal save and
reopen success does not stand in for it.

**Delivered, and previously listed as missing** (all in 0.11.1):

| Was claimed | What shipped | What stays open |
|---|---|---|
| 12–15 tools missing, depending on the document | The toolbox has 67 of 68 tools; Art History Brush is the only one missing ([G6](docs/gaps.md#g6-missing-tools)) | Art History Brush and specific behavioural gaps in existing tools |
| Content-Aware Fill is a generic dialog | A full workspace: Sampling Brush, Lasso edits to the fill area, live preview, rotation/scale/mirror, output to a new layer, repeated Apply (#2376, TOOL-214-15) | Fill fidelity and interaction gaps we can demonstrate |
| No image-file brush tips, no per-control response curves | Both shipped in 0.10.0 (#3279, #3294; TOOL-213-11, TOOL-213-12) | Pressure, smoothing, latency and tablet feel ([G4](docs/gaps.md#g4-brush-feel)) |
| Panel groups can't float | Panels float and regroup inside the app window through the shared docking; Brushes joined it in 0.10.0 (#4603) | Panels as separate OS windows; restoring floating groups at launch is not verified (UI-217-5 partial) |
| Lossy DNG is unsupported | JPEG-compressed DNG (compression 34892) decodes since 0.10.0 (#5112) | CR3, `.xmp` sidecars, camera coverage and the other RAW gaps, each tracked on its own ([G14](docs/gaps.md#g14-camera-raw-depth)) |
| Vietnamese is absent | Merged 2026-10-12 (#2636) with locale selection; 1,918 of 1,918 UI keys translated (measured 2026-10-21) | Native-speaker review; Arabic, Hindi and RTL are unaffected |
| Action recording uses document-local ids; Batch has no subfolders | Layer selections record by name, Apply Layer Comp by the comp's name (#7181); Batch and Image Processor take subfolders (#3753) | Broader action portability (AUTO-219-1 partial); Batch progress, cancel and error log (AUTO-219-5 partial) |
| Only classical image analysis | A local CNN face detector behind `image.detectFaces` shipped in 0.10.0 (#4513) | A general ML backend, Generative Fill/Expand and full Face-Aware Liquify are not established ([G15](docs/gaps.md#g15-ai-and-generative-features)) |

**Released versus merged.** Some progress is on `main` and has not reached users yet, so we
publish the two separately:

| Area | 0.11.1 | `main` (68348dc) | Note |
|---|---|---|---|
| PSD Smart Filter mappings (FILE-215-2) | 14 filters plus a Camera Raw subset | 41 filters plus Hue/Saturation, Perspective Warp, Filter Gallery (Chrome) and the Camera Raw subset | A mapping does not prove that Photoshop accepts the file |
| Large-file fixtures (FILE-216-2) | partial | generated 20 MP, 150-layer, ~2 GB PSD and 4.38 GB PSB tests (#5107) | The largest cases are opt-in and are not run in ordinary CI |
| Type checklist | 4 done, 3 partial, 2 missing | 5 done, 3 partial, 1 missing | The Type area percentage has not been re-graded from this |

**Numbers we no longer stand behind as current:**

- **Readiness percentages** (45%, 43%, 61%) rest on evidence tables that contain superseded
  feature claims. They are the estimates of 2026-10-10, pending recalculation; we don't have
  enough evidence to put a new percentage in their place.
- **Photoshop-oracle counts.** Older text quotes 133 or 145 passing; the enforced floor is 188 of
  258 (`crates/io/tests/corpus.rs`). The scorecard showed "258/256" for round trips because a doc
  comment still said 256 files; the pinned corpus has 258 (`xtask/photoshop-corpus.sha256`).
- **Performance.** The scorecard's "3 of 25 budgets met" is from the 2026-10-05 baseline, taken
  before a lot of optimisation work. It can't describe today's performance, and individual PR
  speedups don't show that the remaining budgets pass either. A fresh run is needed.
- **Stability.** Documents quote 579 or 953 open issues; a query on 2026-10-21 returned 1,748,
  excluding pull requests. Those include feature requests and other reports, and a raw backlog
  count does not measure the rate of regressions or how many release blockers are unresolved.
- **Effort and elapsed time.** The 1,100–1,800 agent-hour beta estimate still includes work that
  is done, and so does its conversion to roughly 100–150 wall-clock hours at 12 agents. Neither
  has been rebuilt from the remaining work and the new findings.
- **Photoshop acceptance reports.** #2469 was closed as a duplicate, which is not proof of a
  fix. #1281 is still open and doesn't have enough detail to pin down the failure. The roadmap
  should point at canonical, reproducible cases and their validation results instead.

## Headline numbers (historical, 2026-10-10)

Kept for the record; see the [reassessment](#reassessment-2026-10-21) for what each still means.

| | Value | Kind |
|---|---|---|
| **Feature breadth** | **~76%** | partly measured on 2026-10-10: menus 628/628, tools 56/68, panels 30/35, formats 13+4 of 31. Tools are <!-- fact:tools.inventory -->67 of 68<!-- /fact --> on <!-- fact:tools.inventory.measured -->2026-10-21<!-- /fact -->; the weighted number is not recalculated |
| **Ready for real work** | **~45%** (40–50%) | historical estimate, pending recalculation |
| Mainstream practitioner | ~43% (38–48%) | historical estimate, pending recalculation ([method](docs/target-app-parity.md#mainstream-practitioner-43)) |
| Essentials user | ~61% (55–67%) | historical estimate, pending recalculation ([method](docs/target-app-parity.md#essentials-user-61)) |
| Remaining to **beta** | ~1,100–1,800 h | historical; includes completed work, not rebuilt |
| Remaining to **full parity** | ~2,300–4,100 h | historical; AI and plug-ins need owner decisions |

### Readiness by audience (historical, 2026-10-10)

| Audience | Ready % | Opus 5.5 agent-hours to ~95% | Work that dominates |
|---|---:|---:|---|
| Full target (ready for real work) | ~45% (additive weighted sum: 46%) | 2,300–4,100 | Everything below, plus AI tier, plug-in/scripting ecosystem, video, localization, hardware |
| Mainstream practitioner | ~43% | 1,700–2,800 | Depth in the 11 weekly areas (1,180–2,000), performance budgets, stability on real machines, PSD exchange with Photoshop |
| Essentials user | ~61% | 400–700 | Startup and GPU robustness, brush and transform responsiveness, Type tool, discoverability (flyouts, commit buttons), AVIF/PDF open |

Hours calibrated as in [Remaining effort](docs/target-app-parity.md#remaining-effort-and-calibration) (2–9 h per checklist
item, 8–20 h per format or subsystem, from the repo's 1,004 merged PRs); each tier is a subset of
the one above. About 80% parallelizes (independent crates and command modules); performance
architecture and the PSD writer are the serial parts.

**Why alpha.** Core workflows (open, layer, select, paint, adjust, filter, type, save) work end to
end, and breadth is high. What keeps us in alpha is missing evidence: no test opens our exports
in Photoshop (FILE-216-4), the performance numbers date from 2026-10-05, and nobody has sorted
the open reports (<!-- fact:issues.open -->1,748<!-- /fact --> on <!-- fact:issues.open.measured -->2026-10-21<!-- /fact -->, requests included) by severity and data loss. Beta needs
the [beta acceptance](#remaining-milestones) evidence, not a percentage. The [alpha gate](docs/roadmap.md#alpha-gate) passes: all six core workflows
(retouch, composite, cut-out, design, paint, PSD exchange) complete end to end and save and reopen;
four are partial in depth, none blocked.

## By dimension (historical, 2026-10-10)

Several evidence cells behind these rows are superseded (see the
[reassessment](#reassessment-2026-10-21)); the percentages are not recalculated.

| Dimension | % ready | Hours to full | Doc |
|---|---:|---:|---|
| Features (depth) | 55% (breadth 76%) | 600–1,000 | [target-app-parity](docs/target-app-parity.md#feature-areas) |
| UI/UX fidelity (tools, handles, modifiers, brush feel, panels) | 40% | 350–600 | [ui-parity](docs/ui-parity.md), [brush-parity](docs/brush-parity.md), [context-menu-parity](docs/context-menu-parity.md) |
| File formats | 60% (presence ~80%) | 300–500 | [file-format-parity](docs/file-format-parity.md) |
| Hardware (GPU, pen, displays) | 50% | 80–150 | [hardware-parity](docs/hardware-parity.md) |
| Localization | 55% | 80–140 | [localization-parity](docs/localization-parity.md) |
| Performance | 25% (3/25 budgets met on the 2026-10-05 baseline) | 200–350 | [scorecard](docs/scorecard.md#performance) |
| Stability | 45% | 150–250 | [gaps](docs/gaps.md#g3-user-reported-bug-backlog) |
| Platforms | 80% | 30–60 | [scorecard](docs/scorecard.md#distribution) |
| Ecosystem / plug-ins / scripting | 15% | 250–450 | [gaps](docs/gaps.md#g16-plug-in-compatibility) |
| AI features | 10% | 300–600 | [gaps](docs/gaps.md#g15-ai-and-generative-features) |

## Features (historical, 2026-10-10)

| Area | % ready | Hours |
|---|---:|---:|
| Layers, masks, blend modes, smart objects, styles | 65% | 150–250 |
| Selections, Select and Mask, channels | 55% | 120–200 |
| Painting and brush engine | 55% | 150–250 |
| Retouching (healing, clone, patch, remove, content-aware) | 50% | 100–180 |
| Transform, warp, puppet, liquify | 55% | 100–180 |
| Adjustments and filters | 70% | 120–200 |
| Type | 40% | 120–200 |
| Vector shapes and paths | 55% | 80–140 |
| Colour management and modes | 70% | 60–100 |
| Camera Raw and computational photo | 45% | 120–200 |
| Automation, actions, scripting | 45% | 120–200 |
| File I/O | 60% | 300–500 |
| Print, export, Save for Web | 50% | 60–100 |
| Video / timeline, data-driven graphics | 30% | 100–200 |
| Workspace, panels, preferences | 40% | 150–250 |
| AI / generative | 10% | 300–600 |

Rows and evidence: [target-app-parity › Feature areas](docs/target-app-parity.md#feature-areas).

## Languages

Measured with `cargo xtask i18n-coverage` (<!-- fact:i18n.keys -->1,918<!-- /fact --> source keys) and kept current by `cargo xtask docs-sync`.
`partial` everywhere because engine messages are still English and no catalog has a recorded
native-speaker review. Detail: [localization-parity](docs/localization-parity.md).

| Language | Code | UI strings | Status |
|---|---|---:|---|
| English | `en` | <!-- fact:i18n.en -->100%<!-- /fact --> | full |
| Simplified Chinese | `zh-hans` | <!-- fact:i18n.zh-hans -->99.3%<!-- /fact --> | partial |
| Spanish | `es` | <!-- fact:i18n.es -->100%<!-- /fact --> | partial |
| Hindi | `hi` | 0% | none |
| Arabic | `ar` | 0% (RTL shell only) | none |
| French | `fr` | <!-- fact:i18n.fr -->99.3%<!-- /fact --> | partial |
| Portuguese (Brazil) | `pt-br` | <!-- fact:i18n.pt-br -->100%<!-- /fact --> | partial |
| Indonesian | `id` | <!-- fact:i18n.id -->99.3%<!-- /fact --> | partial |
| Japanese | `ja` | <!-- fact:i18n.ja -->100%<!-- /fact --> | partial |
| German | `de` | <!-- fact:i18n.de -->100%<!-- /fact --> | partial |
| Korean | `ko` | <!-- fact:i18n.ko -->99.3%<!-- /fact --> | partial |
| Vietnamese | `vi` | <!-- fact:i18n.vi -->100%<!-- /fact --> | partial (merged 2026-10-12, #2636; in 0.11.1) |

Also shipped: Traditional Chinese <!-- fact:i18n.zh-hant -->100%<!-- /fact -->, Russian <!-- fact:i18n.ru -->100%<!-- /fact -->, Ukrainian <!-- fact:i18n.uk -->100%<!-- /fact -->, Czech <!-- fact:i18n.cs -->100%<!-- /fact -->, Polish <!-- fact:i18n.pl -->99.3%<!-- /fact -->, Greek <!-- fact:i18n.el -->100%<!-- /fact -->, Dutch <!-- fact:i18n.nl -->100%<!-- /fact -->, Italian <!-- fact:i18n.it -->100%<!-- /fact -->, Hungarian <!-- fact:i18n.hu -->100%<!-- /fact -->, Serbian <!-- fact:i18n.sr -->100%<!-- /fact -->, Turkish <!-- fact:i18n.tr -->100%<!-- /fact -->.

## Remaining milestones

Proposed 2026-10-21, replacing the old ranked list (which ended with "the 12 missing tools" and
"bug backlog under 100"). Hours are left out until an estimate is rebuilt from the remaining
work. Every status we publish carries an evidence date and says whether it is **released**,
**merged** or **validated**.

| Priority | Milestone | Evidence required to close it |
|---|---|---|
| 1 | Reliable document exchange and saving | Representative exports reopened in Photoshop; colour mode, bit depth and Smart Object cases covered; any unresolved destructive-save failures listed explicitly |
| 2 | Measured interactive performance | Fresh measurements on named hardware for brushes, transforms, layer edits, large documents, background work and cancellation |
| 3 | Remaining editing depth | Written residual requirements for Select and Mask, type on a path, brush behaviour, the specialised workspaces and the one remaining tool |
| 4 | Hardware, localization and distribution acceptance | Tablet tests on real devices; measured translation coverage; an RTL evaluation; verification of the signed artifacts |
| 5 | Beta acceptance | Repeatable core workflows passing on the supported platforms, with severity-based blocker criteria (data loss, reproducibility, supported workflows) and documented limitations |

Detail: [docs/roadmap.md](docs/roadmap.md).

## Progress log

| Date | Entry |
|---|---|
| 2026-10-21 | Reassessment against 0.11.1 and `main` at 68348dc: delivered features credited (67 of 68 tools, Content-Aware Fill workspace, brush tips and response curves, lossy DNG, Vietnamese, name-based action steps, Batch subfolders, face detection); readiness and hour estimates marked historical; milestones restated as evidence to collect |
| 2026-10-20 | 0.11.0 and 0.11.1 |
| 2026-10-17 | 0.10.0 |
| 2026-10-14 | 0.9.0 |
| 2026-10-12 | 0.8.1 and 0.8.2 |
| 2026-10-10 | Full re-measure against Photoshop 2026 27.11.0 (installed bundle inspected): breadth ~76%, ready ~45%, alpha. Progress docs reorganized to the craftrules standard (`parity.md` → `parity-checklist.md`, `parity-estimate.md` merged into `target-app-parity.md`). 0.6.0 released. |
| 2026-10-09 | Remove tool, Rotate View, crop rotation; OpenRaster and Paint.NET; 39 Affinity documents in the corpus |
| 2026-10-08 | 0.5.0. Affinity import; Magnetic Lasso, Pattern Stamp, Mixer Brush, Pencil, Patch, Vertical Type on the toolbar |
| 2026-10-07 | 0.3.0. 10 UI languages; shortcut audit 214 → 0 failures; TIFF decode 10× faster |
| 2026-10-05 | 0.2.0, first signed and notarized release. Honest assessment: ready for real work 25–35% |
| 2026-10-03 | Menu breadth 625/625 |
| 2026-10-01 | Menu breadth 224 → 532 of 625 in a day |
| 2026-09-30 | First commit |

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-21 | major | Reassessment section; headline, audience, dimension and feature tables labelled historical (2026-10-10); "Why alpha" rewritten around missing evidence; language coverage re-measured (1,918 keys, Vietnamese 100%); Upcoming replaced by five evidence-based milestones without hours |
| 2026-10-13 | minor | Focus item 1 quoted the oracle floor as 133 of 256; it is 145 of 258 (`crates/io/tests/corpus.rs`, gaps.md G1). Open issues 579 → 953 (2026-10-13) |
| 2026-10-11 | minor | Toolbox count 56 of 68 (12 missing) with the Convert Point tool (#3204) |
| 2026-10-10 | minor | Readiness by audience with hours per tier |
| 2026-10-10 | minor | Mainstream-practitioner and essentials-user numbers added |
| 2026-10-10 | minor | Alpha gate result (passes; stays alpha) in the stage explanation |
| 2026-10-10 | major | First version, per craftrules `standards/progress-docs.md` |
