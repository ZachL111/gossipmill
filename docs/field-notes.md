# Field Notes

This note keeps the distributed systems assumptions visible beside the checks.

The domain cases cover `quorum health`, `lease drift`, `replica lag`, and `membership churn`. They sit beside the smaller starter fixture so the project has both a compact scoring check and a domain-flavored review check.

The widest spread is between `replica lag` and `lease drift`, so those are the first two cases I would preserve during a refactor.

The local verifier covers this data so the notes stay tied to code.
