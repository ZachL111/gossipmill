# Review Journal

I treated `gossipmill` as a project where the smallest useful behavior should still be inspectable.

The local checks classify each case as `ship`, `watch`, or `hold`. That gives the project a small review vocabulary that matches its distributed systems focus without claiming live deployment or external usage.

## Cases

- `baseline`: `quorum health`, score 136, lane `watch`
- `stress`: `lease drift`, score 109, lane `watch`
- `edge`: `replica lag`, score 168, lane `ship`
- `recovery`: `membership churn`, score 130, lane `watch`
- `stale`: `quorum health`, score 164, lane `ship`

## Note

The repository should be understandable without pretending it is larger than it is.
