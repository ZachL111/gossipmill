# gossipmill

`gossipmill` explores distributed systems with a small Rust codebase and local fixtures. The technical goal is to simulate gossip membership and failure suspicion over lossy links.

## Project Rationale

I want this repository to be useful as a quick reading exercise: fixtures first, implementation second, verifier last.

## Gossipmill Review Notes

For a quick review, compare `replica lag` with `lease drift` before reading the middle cases.

## Feature Set

- `fixtures/domain_review.csv` adds cases for quorum health and lease drift.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/gossipmill-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `replica lag` and `lease drift`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Architecture

The fixture data drives the tests. The code stays thin, while `metadata/domain-review.json` and `config/review-profile.json` explain what each case is meant to protect.

The added Rust path is deliberately direct, with fixtures doing most of the explaining.

## Usage

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Test Command

The same command runs the local verification path. The highest-scoring domain case is `edge` at 168, which lands in `ship`. The most cautious case is `stress` at 109, which lands in `watch`.

## Next Improvements

The repository is intentionally scoped to local checks. I would expand it by adding adversarial fixtures before adding features.
