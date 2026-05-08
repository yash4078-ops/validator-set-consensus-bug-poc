# validator-set-consensus-bug-poc
PoC demonstrating duplicate validator entries causing proposer bias in consensus system
# Validator Duplication in ValidatorSet Causes Proposer Bias and Voting Power Inflation

## Summary

The `ValidatorSet::new()` implementation in the consensus types module does not enforce uniqueness of validators based on either `address` or `public_key`.

As a result, the same validator can be inserted multiple times into the validator set with different voting powers. Since deduplication relies on full struct equality (including voting power), duplicate logical identities are not removed.

This leads to multiple consensus-level inconsistencies:

- Duplicate validator entries are accepted into the validator set
- Voting power is inflated due to duplicate aggregation
- RoundRobin proposer selection treats duplicates as separate validators
- A single validator can receive multiple proposer slots within one rotation cycle

---

## Root Cause

In `ValidatorSet::new()`:

- Validators are sorted and then deduplicated using `vals.dedup()`
- `dedup()` relies on derived `PartialEq/Eq`
- `Validator` equality includes `voting_power`
- Therefore, validators with same `address/public_key` but different voting power are NOT considered duplicates

Relevant code:

```rust
vals.sort_unstable_by(...);
vals.dedup();

Additionally, there is no explicit uniqueness validation for:

address
public_key
Affected Component
crates/types/src/validator_set.rs
ValidatorSet::new()
sort_validators()
Proof of Concept

The PoC is available in:

poc/test_dup.rs
Execution:
cargo run --example test_dup
Observed Behavior:
Same validator appears multiple times in set
Voting power is counted multiple times
RoundRobin selects same validator in alternating rounds

Example output:

validator count = 2
total vp = 1009

round=0 proposer=... power=999
round=1 proposer=... power=10
round=2 proposer=... power=999
Impact

This issue breaks fundamental consensus assumptions:

1. Voting Power Inflation

Duplicate validator entries increase total voting power incorrectly.

2. Proposer Bias

RoundRobin selection treats duplicates as separate validators, giving one validator multiple proposer slots.

3. Fairness Violation

Validator uniqueness assumption is broken, leading to unfair block proposer distribution.

4. Consensus Integrity Risk

In edge cases, duplicated entries can distort quorum calculation and consensus behavior.

Supporting Evidence

Screenshots included in /screenshots:

01_duplicate_validator_proposer_bias_poc.png → PoC output
02_validator_set_missing_uniqueness_check.png → missing validation
03_roundrobin_usage_reference.png → selector usage
04_roundrobin_duplicate_slot_logic.png → proposer rotation logic
05_partialeq_dedup_rootcause.png → root cause in dedup behavior
Recommendation

Enforce strict uniqueness in ValidatorSet::new():

Reject duplicate address
Reject duplicate public_key
OR deduplicate based on identity fields instead of full struct equality

Example fix:

// enforce uniqueness by address or public key before insertion
Conclusion

This issue allows a single validator identity to be treated as multiple independent validators, resulting in inflated voting power and biased proposer selection in RoundRobin consensus.
