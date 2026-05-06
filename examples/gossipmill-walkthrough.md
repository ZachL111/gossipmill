# Gossipmill Walkthrough

This note is the quickest way to read the extra review model in `gossipmill`.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | quorum health | 136 | watch |
| stress | lease drift | 109 | watch |
| edge | replica lag | 168 | ship |
| recovery | membership churn | 130 | watch |
| stale | quorum health | 164 | ship |

Start with `edge` and `stress`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

If `stress` becomes less cautious without a clear reason, I would inspect the drag input first.
