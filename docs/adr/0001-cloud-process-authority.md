# Durable GPU process authority

The central caller uses one pinned native process per operation, while the SDK's
original quote/account/deadline authority must survive process exit. The JSON
boundary therefore uses explicit caller-owned filesystem enrollment and strict
resume over the existing SDK exact-quote lifecycle. It refuses a stateless
fallback, provider-metadata import, automatic state repair and authority blobs in
a second database protocol. Caller policy/billing stays in the existing central
store. The incompatible draft is replaced by the version exported from the CLI;
source/binary/client qualification and persistent runtime placement remain
required before adoption. A resident service and a second allocate-selection
path were rejected because they add lifecycle or change selected-quote semantics.

Public quote provenance is a projection, not a mutation of the original recipe:
new candidate arm copies omit measurements, the selected public audit record is
a distinct MeasurementReceipt and pricing_source is the actual HTTP(S) service
origin, or `configured` for an operator's price where a seller has no price API. The original full spec/env fingerprint, allocation fingerprint inputs,
version1 state and legacy full-arm decoder remain unchanged. Shared validation
resolves normalized selectors against canonical original arms, while nonempty
legacy arms retain exact equality. Public identifiers/digests remain caller-owned
public metadata; no secret scanner, attestation or opaque authority transport is
introduced. The output Rust type change requires a minor containing release.
