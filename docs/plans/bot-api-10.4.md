# Plan: Telegram Bot API 10.4

Status as of 2026-11-06: **Bot API 10.4 is not published yet.** The official
[changelog](https://core.telegram.org/bots/api-changelog) and the
[API reference](https://core.telegram.org/bots/api) both end at Bot API 10.3
(August 24, 2026), and a web search finds no announcement of 10.4 (the
[@BotNews](https://t.me/s/botnews) channel is where Telegram announces releases).
The text of the API reference was identical on two fetches made during this
work.

Because the API surface of 10.4 is unknown, nothing can be implemented in
`ronami-core` without inventing Telegram's API. This plan therefore has two
parts: what was done ahead of time, and a runbook to execute the moment 10.4 is
published.

## Baseline: 10.3 parity audit

`scripts/bot_api_audit.py` compares the live API reference with the Rust
sources. On 2026-11-06 it found:

- every method in the live reference has a payload in `ronami-core`
- every object and field has a Rust counterpart; the reported differences are
  structural (fields modelled as enum variants, flattened fields, `FileMeta`
  based files, tag fields) and are not missing API

So the repository starts the 10.4 work at full 10.3 parity.

## Done ahead of time

- `scripts/bot_api_audit.py`: downloads the API reference, parses methods and
  objects into JSON, and reports methods, objects and fields that are not
  present in the Rust code. Re-run it to get the exact 10.4 delta.
- `scripts/bot_api_changelog.py`: prints the changelog entry for a Bot API
  version, so the change list can be copied into the task list below.
- Toolchain moved to stable Rust and edition 2024, so the 10.4 code is written
  against the toolchain it will ship with.

## Runbook (when 10.4 is published)

1. **Get the delta.** Run `python3 scripts/bot_api_changelog.py 10.4` and
   `python3 scripts/bot_api_audit.py`. Copy the changelog entry into the PR
   description and into `CHANGELOG.md` / `crates/ronami-core/CHANGELOG.md`.
2. **Schema.**
   - `crates/ronami-core/schema.ron`: add or change methods and parameters, and
     update `api_version` (version and date).
   - `crates/ronami-core/custom_v2.json`: add or change objects and methods,
     and update `version` and `recent_changes`. Keep the file formatted as
     `json.dumps(data, indent=2)` without a trailing newline.
3. **Types.** For every new object add `crates/ronami-core/src/types/<name>.rs`,
   export it from `types.rs` (module and `pub use`, in alphabetical order),
   and follow the conventions in `CODE_STYLE.md`:
   - `#[serde_with::skip_serializing_none]` plus
     `#[cfg_attr(test, derive(schemars::JsonSchema))]` on every struct
   - optional `true`-only booleans are `bool` with
     `#[serde(default, skip_serializing_if = "std::ops::Not::not")]`
   - new variants of existing enums (`UpdateKind`, `AllowedUpdate`,
     `RichBlock`, `RichTextEntity`, `InputRichBlock`, `InlineKeyboardButtonKind`,
     `MessageKind`) need matching updates in `update.rs`,
     `ronami/src/dispatching/handler_description.rs` and
     `ronami/src/dispatching/filter_ext.rs`
4. **Methods.** Run `cargo test -p ronami-core --features full`; the codegen
   tests regenerate `payloads/*.rs`, `payloads.rs`, `requests/requester.rs`,
   `bot/api.rs`, the adaptors and the schema-check tests, and fail until the
   files are up to date, so run the tests twice. New methods that upload files
   also need an `impl MultipartPayload` in `requests/multipart_payload.rs`.
5. **Consumers.** Fix struct literals of changed types across both crates
   (tests in `ronami/src/dispatching/handler_ext.rs` build a full `Message`).
   Add a `filter_*` entry for each new `UpdateKind` and each new service
   message.
6. **Tests.** Add serde round-trip tests next to each new type and payload
   tests for new or changed parameters. The generated
   `rust_types_check_tests.rs` compares every object against
   `custom_v2.json`, so a mismatch between a Rust type and the schema fails
   the build.
7. **Docs and metadata.** Update the API version in both READMEs, the badges,
   `ronami-core/src/lib.rs`, `ronami/src/lib.rs`, and the highlights in the
   root `README.md`.
8. **Verify.** `cargo fmt --all --check`, `cargo clippy --all-targets
   --features full`, `cargo test --features full`, and `cargo docs`.

## Risks

- The changelog may rename or remove existing methods or parameters (10.3 did
  this for the ephemeral message parameters). Mark every such change `[**BC**]`
  in the changelog.
- 10.4 might be published under a different version number, or Telegram may
  skip it and move on to 11.0; the runbook is version-agnostic, only the
  `10.4` argument of the changelog script changes.
