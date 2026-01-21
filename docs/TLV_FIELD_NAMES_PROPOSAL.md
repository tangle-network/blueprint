# TLV Schema Field Names - Problem Analysis and Fix Proposal

**Author:** Claude Code
**Date:** 2026-01-21
**Status:** Proposal
**Affects:** tnt-core contracts, blueprint CLI

---

## Executive Summary

The TLV (Type-Length-Value) schema format used by tnt-core contracts does not store field names. This prevents users from submitting job parameters using object format (e.g., `{"name": "Alice"}`), forcing them to use positional array format instead (e.g., `["Alice"]`). This document proposes extending the TLV format to include field names.

---

## Problem Statement

### User Experience Issue

When a user defines a job schema with named parameters:

```json
{
  "params_schema": "[{\"name\": \"recipient\", \"type\": \"address\"}, {\"name\": \"amount\", \"type\": \"uint256\"}]"
}
```

They expect to submit jobs using intuitive object format:

```json
{"recipient": "0x1234...", "amount": 1000}
```

But this fails with:

```
Error: parameter 0 is unnamed; provide an array instead of an object
```

Users must instead use positional arrays:

```json
["0x1234...", 1000]
```

This is less readable and error-prone, especially for jobs with many parameters.

### Root Cause

The TLV binary format stores only type information, not field names.

**Original JSON schema:**
```json
[{"name": "recipient", "type": "address"}, {"name": "amount", "type": "uint256"}]
```

**After TLV encoding (what's stored on-chain):**
```
[field_count=2][Address, arrayLen=0, childCount=0][Uint256, arrayLen=0, childCount=0]
```

The names `"recipient"` and `"amount"` are completely lost.

### Discovery Context

This issue was discovered during CLI testing (documented in `JOB_SYSTEM_TEST_PROGRESS.md`, Test 2.3). Initially suspected to be a CLI bug, investigation revealed it's a fundamental limitation of the TLV format design in tnt-core.

---

## Technical Analysis

### Current TLV Format

**Location:** `tnt-core/src/v2/libraries/SchemaLib.sol`

**Binary structure:**
```
[2 bytes: uint16 field_count (big-endian)]
For each field:
  [1 byte:  BlueprintFieldKind enum (0-22)]
  [2 bytes: uint16 arrayLength (big-endian)]
  [2 bytes: uint16 childCount (big-endian)]
  [... children recursively ...]
```

**Total: 5 bytes per field node (header only)**

### Current Solidity Struct

**Location:** `tnt-core/src/v2/libraries/Types.sol` (lines 198-202)

```solidity
struct BlueprintFieldType {
    BlueprintFieldKind kind;
    uint16 arrayLength;
    BlueprintFieldType[] children;
    // ❌ No name field
}
```

### Data Flow

```
User defines schema (JSON with names)
         ↓
CLI converts to TLV binary (names discarded) ← PROBLEM HERE
         ↓
Contract stores TLV bytes
         ↓
CLI reads TLV back (no names available)
         ↓
User submits params (can't match by name)
```

---

## Proposed Solution

### Extended TLV Format (Version 2)

Add a version byte and field names to the binary format:

```
[1 byte:  version (0x02 for new format)]
[2 bytes: uint16 field_count (big-endian)]
For each field:
  [1 byte:  BlueprintFieldKind enum (0-22)]
  [2 bytes: uint16 arrayLength (big-endian)]
  [2 bytes: uint16 childCount (big-endian)]
  [1-4 bytes: compact-encoded name length]
  [N bytes: field name UTF-8 string]
  [... children recursively ...]
```

### Updated Solidity Struct

```solidity
struct BlueprintFieldType {
    BlueprintFieldKind kind;
    uint16 arrayLength;
    BlueprintFieldType[] children;
    string name;  // ← NEW FIELD
}
```

### Compact Length Encoding

Reuse existing `_readCompactLength()` format from SchemaLib:

| Length Range | Encoding | Bytes Used |
|--------------|----------|------------|
| 0-127 | `0x00-0x7F` | 1 byte |
| 128-16,383 | `0x80-0xBF` + 1 byte | 2 bytes |
| 16,384-2,097,151 | `0xC0-0xDF` + 2 bytes | 3 bytes |
| Larger | `0xE0-0xEF` + 3 bytes | 4 bytes |

Most field names are < 32 characters, so 1 byte length encoding covers typical cases.

---

## Implementation Plan

### Phase 1: Contract Changes (tnt-core)

#### 1.1 Update Types.sol

**File:** `src/v2/libraries/Types.sol`
**Lines:** 198-202

```solidity
// BEFORE
struct BlueprintFieldType {
    BlueprintFieldKind kind;
    uint16 arrayLength;
    BlueprintFieldType[] children;
}

// AFTER
struct BlueprintFieldType {
    BlueprintFieldKind kind;
    uint16 arrayLength;
    BlueprintFieldType[] children;
    string name;
}
```

#### 1.2 Update SchemaLib.sol

**File:** `src/v2/libraries/SchemaLib.sol`

**Add version constant:**
```solidity
uint8 internal constant SCHEMA_VERSION_1 = 0x01;  // Legacy (no names)
uint8 internal constant SCHEMA_VERSION_2 = 0x02;  // With names
uint8 internal constant CURRENT_SCHEMA_VERSION = SCHEMA_VERSION_2;
```

**Update `encodeSchema()` (lines 30-47):**
```solidity
function encodeSchema(Types.BlueprintFieldType[] memory source) internal pure returns (bytes memory) {
    if (source.length == 0) {
        return bytes("");
    }
    if (source.length > type(uint16).max) {
        revert Errors.SchemaTooLarge();
    }

    // Calculate total size including names
    uint256 totalSize = 1 + 2; // version + field_count
    totalSize += _calculateEncodedSize(source);

    bytes memory out = new bytes(totalSize);

    // Write version
    out[0] = bytes1(CURRENT_SCHEMA_VERSION);

    // Write field count
    _writeUint16(out, 1, uint16(source.length));

    uint256 cursor = 3;
    for (uint256 i = 0; i < source.length; ++i) {
        cursor = _writeField(out, cursor, source[i]);
    }
    return out;
}
```

**Add `_calculateEncodedSize()` helper:**
```solidity
function _calculateEncodedSize(Types.BlueprintFieldType[] memory fields) private pure returns (uint256 size) {
    for (uint256 i = 0; i < fields.length; ++i) {
        size += _calculateFieldSize(fields[i]);
    }
}

function _calculateFieldSize(Types.BlueprintFieldType memory field) private pure returns (uint256 size) {
    // Header (5 bytes) + name length encoding + name bytes
    size = NODE_HEADER_SIZE;
    size += _compactLengthSize(bytes(field.name).length);
    size += bytes(field.name).length;

    // Add children recursively
    for (uint256 i = 0; i < field.children.length; ++i) {
        size += _calculateFieldSize(field.children[i]);
    }
}
```

**Update `_writeField()` (lines 55-70):**
```solidity
function _writeField(bytes memory out, uint256 cursor, Types.BlueprintFieldType memory field)
    private pure returns (uint256)
{
    // Write 5-byte header
    _writeHeader(out, cursor, field);
    cursor += NODE_HEADER_SIZE;

    // Write field name
    cursor = _writeCompactString(out, cursor, field.name);

    // Write children recursively
    for (uint256 i = 0; i < field.children.length; ++i) {
        cursor = _writeField(out, cursor, field.children[i]);
    }
    return cursor;
}
```

**Add `_writeCompactString()` helper:**
```solidity
function _writeCompactString(bytes memory out, uint256 cursor, string memory str)
    private pure returns (uint256)
{
    bytes memory strBytes = bytes(str);
    uint256 len = strBytes.length;

    // Write compact length
    cursor = _writeCompactLength(out, cursor, len);

    // Write string bytes
    for (uint256 i = 0; i < len; ++i) {
        out[cursor + i] = strBytes[i];
    }

    return cursor + len;
}
```

**Update `_validateField()` (lines 181-289) to skip names:**
```solidity
function _validateField(...) private pure returns (...) {
    // Read header (same as before)
    (Types.BlueprintFieldKind kind, uint16 arrayLength, uint16 childCount) = _readHeader(schema, schemaCursor);
    schemaCursor += NODE_HEADER_SIZE;

    // Skip field name (NEW)
    if (schemaVersion >= SCHEMA_VERSION_2) {
        uint256 nameLen;
        (nameLen, schemaCursor) = _readCompactLength(schema, schemaCursor);
        schemaCursor += nameLen;  // Skip name bytes
    }

    // Rest of validation unchanged...
}
```

#### 1.3 Update Test Helpers

**Files to update:**
- `test/support/SchemaTestUtils.sol`
- `test/support/BlueprintDefinitionHelper.sol`
- `test/v2/libraries/SchemaLibFuzz.t.sol`

All places that create `BlueprintFieldType` must now include a name:

```solidity
// BEFORE
Types.BlueprintFieldType memory field = Types.BlueprintFieldType({
    kind: Types.BlueprintFieldKind.String,
    arrayLength: 0,
    children: new Types.BlueprintFieldType[](0)
});

// AFTER
Types.BlueprintFieldType memory field = Types.BlueprintFieldType({
    kind: Types.BlueprintFieldKind.String,
    arrayLength: 0,
    children: new Types.BlueprintFieldType[](0),
    name: "fieldName"
});
```

### Phase 2: CLI Changes (blueprint)

#### 2.1 Update TLV Encoder

**File:** `cli/src/command/deploy/definition.rs`

Update `encode_json_schema_to_tlv()` to write version 2 format with names.

#### 2.2 Update TLV Decoder

**File:** `cli/src/command/jobs/helpers.rs`

**Update `decode_tlv_schema()` (lines 766-788):**
```rust
fn decode_tlv_schema(data: &[u8]) -> Result<Vec<SchemaParam>> {
    if data.is_empty() {
        return Ok(vec![]);
    }

    let mut cursor = 0;

    // Check version
    let version = data[cursor];
    cursor += 1;

    let has_names = version >= 2;

    // Read field count
    let field_count = u16::from_be_bytes([data[cursor], data[cursor + 1]]) as usize;
    cursor += 2;

    let mut fields = Vec::with_capacity(field_count);
    for _ in 0..field_count {
        let (field, next) = decode_tlv_field(data, cursor, has_names)?;
        fields.push(field);
        cursor = next;
    }

    Ok(fields)
}
```

**Update `decode_tlv_field()` to read names:**
```rust
fn decode_tlv_field(data: &[u8], cursor: usize, has_names: bool) -> Result<(SchemaParam, usize)> {
    // Read 5-byte header
    let kind = data[cursor];
    let array_len = u16::from_be_bytes([data[cursor + 1], data[cursor + 2]]);
    let child_count = u16::from_be_bytes([data[cursor + 3], data[cursor + 4]]);
    let mut cursor = cursor + 5;

    // Read name if version 2+
    let name = if has_names {
        let (name_len, next) = read_compact_length(data, cursor)?;
        cursor = next;
        let name = String::from_utf8_lossy(&data[cursor..cursor + name_len]).to_string();
        cursor += name_len;
        Some(name)
    } else {
        None
    };

    // Read children recursively
    let mut children = Vec::new();
    for _ in 0..child_count {
        let (child, next) = decode_tlv_field(data, cursor, has_names)?;
        children.push(child);
        cursor = next;
    }

    let ty = tlv_kind_to_sol_type(kind, array_len, &children)?;

    Ok((SchemaParam { name, ty, components: children }, cursor))
}
```

### Phase 3: Backwards Compatibility

#### Version Detection

The decoder must handle both formats:

```rust
fn decode_tlv_schema(data: &[u8]) -> Result<Vec<SchemaParam>> {
    if data.is_empty() {
        return Ok(vec![]);
    }

    // Detect version
    let first_byte = data[0];

    // Version 1 (legacy): first 2 bytes are field count (big-endian uint16)
    // Version 2+: first byte is version number (0x02+)
    //
    // Heuristic: if first_byte <= 0x01, could be either:
    //   - Version 1 with field_count starting with 0x00 or 0x01
    //   - Version 1 or 2 explicitly
    // If first_byte >= 0x02, it's definitely a version number

    let (version, field_count_offset) = if first_byte >= 0x02 {
        (first_byte, 1)  // Explicit version
    } else {
        // Legacy format - assume version 1
        (1u8, 0)
    };

    let has_names = version >= 2;
    // ... rest of decoding
}
```

#### Migration Path

Existing blueprints with version 1 schemas continue to work:
- Validation works (names not needed for type checking)
- CLI shows `arg_0`, `arg_1`, etc. (same as current behavior)
- Object-format params still fail (expected)

New blueprints deployed with updated CLI get version 2 schemas:
- CLI shows actual field names
- Object-format params work

No migration of existing data required.

---

## Size Impact Analysis

### Per-Field Overhead

| Component | Version 1 | Version 2 (empty name) | Version 2 (10-char name) |
|-----------|-----------|------------------------|--------------------------|
| Header | 5 bytes | 5 bytes | 5 bytes |
| Name length | - | 1 byte | 1 byte |
| Name data | - | 0 bytes | 10 bytes |
| **Total** | **5 bytes** | **6 bytes** | **16 bytes** |

### Example Schemas

| Schema | Fields | Version 1 | Version 2 |
|--------|--------|-----------|-----------|
| Simple (1 string) | 1 | 7 bytes | 19 bytes |
| Transfer (address, uint256) | 2 | 12 bytes | 40 bytes |
| Complex (5 fields, avg 12-char names) | 5 | 27 bytes | 100 bytes |
| Nested struct (10 fields total) | 10 | 52 bytes | 195 bytes |

### Gas Cost Estimate

Storage cost: ~20,000 gas per 32-byte slot

| Schema | V1 Slots | V2 Slots | Additional Gas |
|--------|----------|----------|----------------|
| Simple | 1 | 1 | 0 |
| Transfer | 1 | 2 | ~20,000 |
| Complex | 1 | 4 | ~60,000 |
| Nested | 2 | 7 | ~100,000 |

This is a one-time cost during blueprint deployment, not per job submission.

---

## Implementation Checklist

### tnt-core Repository

- [ ] Update `BlueprintFieldType` struct in `Types.sol`
- [ ] Add version constants to `SchemaLib.sol`
- [ ] Implement `_calculateEncodedSize()` helper
- [ ] Implement `_calculateFieldSize()` helper
- [ ] Implement `_writeCompactString()` helper
- [ ] Implement `_writeCompactLength()` helper
- [ ] Update `encodeSchema()` for version 2
- [ ] Update `_writeField()` to write names
- [ ] Update `_validateField()` to skip names
- [ ] Update `_validateEncoded()` for version detection
- [ ] Update `SchemaTestUtils.sol` helpers
- [ ] Update `BlueprintDefinitionHelper.sol` helpers
- [ ] Update fuzz tests in `SchemaLibFuzz.t.sol`
- [ ] Add version 1 ↔ version 2 compatibility tests
- [ ] Measure gas cost differences

### blueprint Repository (CLI)

- [ ] Update `encode_json_schema_to_tlv()` for version 2
- [ ] Update `decode_tlv_schema()` for version detection
- [ ] Update `decode_tlv_field()` to read names
- [ ] Add `read_compact_length()` helper
- [ ] Update object-format param matching logic
- [ ] Add unit tests for version 2 encoding/decoding
- [ ] Add integration tests for object-format params
- [ ] Update CLI documentation

---

## Testing Strategy

### Unit Tests

1. **Encoding tests:**
   - Empty schema
   - Single field with/without name
   - Multiple fields with various name lengths
   - Nested structs with names at all levels
   - Arrays with named element types

2. **Decoding tests:**
   - Version 1 (legacy) schemas
   - Version 2 schemas
   - Round-trip: encode → decode → compare

3. **Validation tests:**
   - Version 1 schema validates version 1 payload
   - Version 2 schema validates same payloads
   - Names don't affect validation logic

### Integration Tests

1. **CLI object-format params:**
   - Deploy blueprint with named schema
   - Submit job with object format
   - Verify job executes correctly

2. **Backwards compatibility:**
   - Deploy blueprint with old CLI (version 1)
   - Use new CLI to list jobs (shows arg_0, arg_1)
   - Submit with array format (works)
   - Submit with object format (fails gracefully)

---

## Alternatives Considered

### Alternative 1: Store names off-chain

Store a name mapping in IPFS or similar, reference by hash in contract.

**Pros:**
- No contract changes
- Minimal on-chain storage

**Cons:**
- Requires off-chain infrastructure
- Names not guaranteed available
- More complex CLI logic

### Alternative 2: Use ABI encoding for schemas

Store schemas as ABI-encoded tuples which include names.

**Pros:**
- Well-established format
- Tooling already exists

**Cons:**
- Much larger storage (ABI is verbose)
- Would require rewriting validation logic
- Breaking change to existing format

### Alternative 3: Keep as-is, document limitation

Accept that object-format params don't work.

**Pros:**
- No implementation work
- No risk of bugs

**Cons:**
- Poor user experience
- Inconsistent with user expectations
- Array format is error-prone for complex schemas

**Recommendation:** Implement the proposed TLV v2 format. It's the cleanest solution with minimal overhead and full backwards compatibility.

---

## References

- `tnt-core/src/v2/libraries/SchemaLib.sol` - Schema encoding/validation
- `tnt-core/src/v2/libraries/Types.sol` - Type definitions
- `blueprint/cli/src/command/deploy/definition.rs` - CLI TLV encoder
- `blueprint/cli/src/command/jobs/helpers.rs` - CLI TLV decoder
- `blueprint/docs/JOB_SYSTEM_TEST_PROGRESS.md` - Test 2.3 discovery context
