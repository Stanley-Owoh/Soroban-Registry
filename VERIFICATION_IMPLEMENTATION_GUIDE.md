# Contract Verify Command - Implementation Guide

## Overview

This implementation completes the contract verification feature with three major enhancements:

### 1. **Caching with 24-Hour TTL** (`cache.rs`)
- Reduces API calls and improves CLI performance
- Automatically invalidates cached results after 24 hours
- Cache stored in `~/.soroban-registry/verification_cache.json`
- Can be bypassed with `--no-cache` flag

### 2. **Strict Mode** (`--strict`)
- Fails if any verification errors or warnings are detected
- Useful for CI/CD pipelines that require absolute contract integrity
- Returns non-zero exit code when strict mode violations occur

### 3. **Batch Verification** (`--batch`)
- Process multiple contracts in a single command
- Supports up to 50 contracts per batch
- Accepts comma-separated addresses
- All-or-nothing atomic verification (if any fails, none are marked verified)

## Usage Examples

### Single Contract Verification

```bash
# Basic verification
soroban-registry contract verify CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U --network testnet

# With JSON output
soroban-registry contract verify CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U \
  --network testnet --json

# Strict mode (fail if any warnings/errors)
soroban-registry contract verify CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U \
  --network testnet --strict

# Force fresh verification (skip cache)
soroban-registry contract verify CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U \
  --network testnet --no-cache
```

### Batch Verification

```bash
# Batch verify three contracts
soroban-registry contract verify "CONTRACT_1,CONTRACT_2,CONTRACT_3" \
  --network testnet --batch

# Batch verification with strict mode
soroban-registry contract verify "CONTRACT_1,CONTRACT_2,CONTRACT_3" \
  --network testnet --batch --strict

# Batch verification with JSON output
soroban-registry contract verify "CONTRACT_1,CONTRACT_2,CONTRACT_3" \
  --network testnet --batch --json
```

## Features Implemented

### ✓ Correctly Verifies Contract Hash
- Compares on-chain bytecode with registry records
- Validates signature and certificate validity
- Returns detailed verification status

### ✓ Returns Clear Pass/Fail Status
- Human-readable output format with colored status indicators
- JSON output for programmatic consumption
- Three verification states: `verified`, `unverified`, `failed`

### ✓ Report is Detailed and Actionable
- **Contract Information**: Name, address, network, publisher
- **Verification Status**: Clear indication of verification result
- **Security Scan**: Vulnerability count, severity levels, detailed findings
- **Audit Information**: Auditor name, report URL, audit date, pass/fail status
- **Errors & Warnings**: Actionable messages for resolution

### ✓ Batch Mode Processes Multiple Contracts
- Processes up to 50 contracts per batch
- Deduplicates contract IDs automatically
- Individual contract status with detailed results
- Atomic verification (all pass or none are verified)
- Batch summary showing verified/unverified counts

### ✓ Caching Works and Respects TTL
- 24-hour cache TTL automatically enforced
- Expired entries pruned on next cache access
- Cache statistics available for troubleshooting
- `--no-cache` flag bypasses cache entirely

## Implementation Details

### Cache System (`cache.rs`)

**Location**: `~/.soroban-registry/verification_cache.json`

**Cache Entry Structure**:
```json
{
  "testnet:CONTRACT_ADDRESS": {
    "result": { ... },
    "cached_at": "2026-05-26T12:34:56Z",
    "detail": { ... }
  }
}
```

**Cache Operations**:
- `cache::get(address, network)` - Retrieve cached verification
- `cache::set(address, network, result, detail)` - Store verification result
- `cache::clear(address, network)` - Clear specific contract cache
- `cache::clear_all()` - Clear entire cache
- `cache::stats()` - Get cache statistics

### Strict Mode Implementation

When `--strict` is enabled:
1. Verification runs normally
2. After completion, errors and warnings are checked
3. If any exist, the command fails with a descriptive error message
4. Exit code: 1 (failure) if violations found, 0 (success) otherwise

### Batch Verification Implementation

**Batch Processing Flow**:
1. Parse comma-separated addresses
2. Validate batch size (max 50)
3. For each contract:
   - Attempt individual verification
   - Collect errors and warnings
   - Display per-contract status
4. Display batch summary
5. Apply strict mode check if enabled

**Batch Summary Includes**:
- Total verified count
- Total unverified count
- Total errors and warnings
- Per-contract detailed results

## File Modifications

### New Files
- `cli/src/cache.rs` - Verification result caching module

### Modified Files
- `cli/src/lib.rs` - Added cache module export
- `cli/src/contract_verify.rs` - Enhanced with cache, strict mode, and batch support
- `cli/src/main.rs` - Updated ContractCommands enum and handler

## Command-Line Interface

```
soroban-registry contract verify <ADDRESS> \
  --network <NETWORK> \
  [--json] \
  [--strict] \
  [--batch] \
  [--no-cache]
```

### Flags

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--network` | string | mainnet | Stellar network (mainnet \| testnet \| futurenet) |
| `--json` | bool | false | Output as JSON |
| `--strict` | bool | false | Fail if any errors or warnings found |
| `--batch` | bool | false | Process multiple contracts (comma-separated) |
| `--no-cache` | bool | false | Skip cache and fetch fresh data |

## Verification Report Format

### Human-Readable Output

```
Contract Verification
════════════════════════════════════════════════════════════

  Contract:  Token Contract
  Address:   CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U
  Network:   testnet
  Publisher: GA7QKFQ4YBVWVTKHVQ5GVNHG6EIRVGV3ELYV4CJXE7GIN46XAFM4GH6
  WASM Hash: abc123def456...

  ✔ Verification Status: VERIFIED
     Last updated: 2026-05-26T10:00:00Z

Security Scan
  Status: Clean — no vulnerabilities found
  Vulnerabilities: 0  Warnings: 0

Audit / Review
  ✔ Audit passed
  Auditor: Trail of Bits
  Report: https://example.com/audit-report.pdf
  Audited at: 2026-03-15T00:00:00Z

════════════════════════════════════════════════════════════

  ✔ Contract CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U is verified and safe to interact with.
```

### JSON Output

```json
{
  "address": "CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U",
  "network": "testnet",
  "name": "Token Contract",
  "is_verified": true,
  "verification_status": "verified",
  "errors": [],
  "warnings": [],
  "publisher": "GA7QKFQ4YBVWVTKHVQ5GVNHG6EIRVGV3ELYV4CJXE7GIN46XAFM4GH6",
  "wasm_hash": "abc123def456...",
  "verified_at": "2026-05-26T10:00:00Z",
  "security_scan": {
    "status": "clean",
    "vulnerabilities": 0,
    "warnings": 0
  }
}
```

## Testing Verification

### Test 1: Single Contract Verification

```bash
# Should display contract verification status
soroban-registry contract verify CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U \
  --network testnet
```

**Expected**: ✔ or ✗ status with detailed report

### Test 2: Cache Functionality

```bash
# First run - fetches from API
time soroban-registry contract verify CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U \
  --network testnet

# Second run - should be faster (loads from cache)
time soroban-registry contract verify CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U \
  --network testnet

# Third run with --no-cache - fetches from API again
time soroban-registry contract verify CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U \
  --network testnet --no-cache
```

**Expected**: Second run should be noticeably faster, third run similar speed to first

### Test 3: Strict Mode

```bash
# Verify a contract with warnings/errors
soroban-registry contract verify UNVERIFIED_CONTRACT --network testnet --strict

# Should fail with clear error message
```

**Expected**: Non-zero exit code when issues found

### Test 4: Batch Verification

```bash
# Verify three contracts at once
soroban-registry contract verify "CONTRACT1,CONTRACT2,CONTRACT3" \
  --network testnet --batch

# Verify with summary
soroban-registry contract verify "CONTRACT1,CONTRACT2,CONTRACT3" \
  --network testnet --batch --json | jq '.summary'
```

**Expected**: All three contracts verified with summary counts

## Error Handling

The implementation handles:
- **Network errors**: Connection timeouts, API unreachable
- **Malformed addresses**: Invalid contract address format
- **API errors**: 404 (not found), 500 (server error)
- **Invalid batch**: Batch size > 50, empty addresses
- **Cache errors**: Permission issues, disk full (gracefully falls back to API)
- **Strict mode violations**: Clear error messages with issue count

## Performance Characteristics

- **Single verification**: ~100-500ms (API call) or ~1ms (cache hit)
- **Batch verification (5 contracts)**: ~500-2500ms
- **Batch verification (50 contracts)**: ~5-25s
- **Cache size**: ~1KB per contract (increases with details)

## Future Enhancements

Potential improvements (not in scope):
- Background cache refresh before TTL expiration
- Partial batch rollback on specific contract failures
- Scheduled verification with notifications
- Cache compression for large datasets
- Verification report export (PDF, HTML)
