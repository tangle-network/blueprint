# Managed rental vocabulary

**Authorization state**: The SDK-owned private filesystem record of original quote, intent hash, public key, account bindings, absolute deadline and permanent attempt fence. Provider metadata is a locator, not this authority. _Avoid_: cache, imported provider state.

**Candidate**: The SDK-encoded public offer and arm selector bound to the complete original request fingerprint; new selectors omit measurement copies, while legacy full arms remain readable. Creation uses its unchanged bytes. Encoding is reversible and supplies no account authorization. _Avoid_: credential, confidential token.

**Caller lease record**: The orchestrator-owned durable record of lease policy, billing, identity and original request/custody references. It accompanies SDK authorization state across restart without replacing it. _Avoid_: SDK state copy, second provider lifecycle.

**Measurement receipt**: Public audit hashes, timestamp, counters and topology counts projected after ranking from a private ThroughputMeasurement. It is not an execution recipe or measurement/hardware attestation. _Avoid_: redacted recipe, empty image/model placeholder.
