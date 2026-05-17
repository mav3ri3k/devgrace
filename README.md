# devgrace

Count how polite you are to your coding agents.

`devgrace` scans local chat/session history from coding assistants, counts user-authored politeness and praise markers, and prints a small terminal report by agent.

## Supported Agents

- Claude
- Codex
- OpenCode
- Cline
- Amp
- Pi
- Zed

Only user messages are counted. Assistant/model/tool output is ignored where the local storage format exposes roles clearly.

## Run

```sh
cargo run
```

Or build a binary:

```sh
cargo build --release
./target/release/devgrace
```

## Score

The grace score is:

```text
polite matches / user messages scanned * 100
```

One message can contain multiple polite matches, so this is a politeness density score rather than a percentage of polite messages.

## What Counts

The detector includes explicit thanks, polite requests, praise phrases, casual shorthand, and Gen Z-style terms such as `pls`, `plz`, `tysm`, `big W`, `lfg`, and `goated`.

To avoid inflated scores, common technical descriptors like standalone `correct`, `clean`, `solid`, and `robust` are not counted by themselves.
