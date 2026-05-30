# Testing & Verification Steps

## Build Instructions

### Prerequisites
- Rust 1.75+ (install from https://rustup.rs/)
- Node.js 20+ (for frontend, if needed)
- PostgreSQL 16+ (for backend, if needed)

### Building the CLI

```bash
# From the repository root
cd cli

# Build in release mode
cargo build --release

# Install to system
cargo install --path .
```

After installation, the `soroban-registry` command will be available globally.

## Test Environment Setup

### 1. Start the Registry Backend

If running locally, ensure the registry API is running:

```bash
# Option 1: Using Docker Compose
docker-compose up -d

# Option 2: Run backend locally
cd backend
cargo run --bin api
# API will be available at http://localhost:3001
```

### 2. Set Environment Variables

```bash
# Optional: override API URL if not using default
export API_URL="http://localhost:3001"

# Optional: set network (defaults to mainnet)
export NETWORK="testnet"
```

## Manual Testing

### Test 1: Single Contract Verification - Basic

**Command:**
```bash
soroban-registry contract verify CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U \
  --network testnet
```

**Expected Output:**
- ✔ or ✗ status indicator
- Contract name, address, network
- Publisher information
- Verification status: VERIFIED, UNVERIFIED, or FAILED
- Security scan status
- Audit information (if available)
- Error and warning messages (if any)

**Success Criteria:**
- ✅ Command exits with code 0
- ✅ Output displays contract information
- ✅ Verification status is clearly indicated

---

### Test 2: Single Contract Verification - JSON Output

**Command:**
```bash
soroban-registry contract verify CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U \
  --network testnet --json
```

**Expected Output:**
JSON object with structure:
```json
{
  "address": "CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U",
  "network": "testnet",
  "name": "Contract Name",
  "is_verified": true,
  "verification_status": "verified",
  "errors": [],
  "warnings": [],
  "publisher": "...",
  "wasm_hash": "...",
  "verified_at": "2026-05-26T..."
}
```

**Success Criteria:**
- ✅ Output is valid JSON
- ✅ Can be parsed with `jq`
- ✅ Contains all expected fields
- ✅ No human-readable formatting

---

### Test 3: Cache Functionality - Performance

**Step 1 - First Run (Cache Miss):**
```bash
rm -f ~/.soroban-registry/verification_cache.json
time soroban-registry contract verify CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U \
  --network testnet
```

**Expected Output:**
- Command completes in 100-500ms
- Shows "Initiating verification..." message

**Step 2 - Second Run (Cache Hit):**
```bash
time soroban-registry contract verify CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U \
  --network testnet
```

**Expected Output:**
- Command completes in <10ms (should be significantly faster)
- Shows "Loaded from cache" message

**Step 3 - With --no-cache Flag:**
```bash
time soroban-registry contract verify CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U \
  --network testnet --no-cache
```

**Expected Output:**
- Command completes in 100-500ms (similar to Step 1)
- Does NOT show "Loaded from cache" message
- Hits API again

**Success Criteria:**
- ✅ Cache hit is 10-100x faster than API call
- ✅ `--no-cache` bypasses cache correctly
- ✅ Cache file exists at `~/.soroban-registry/verification_cache.json`

---

### Test 4: Strict Mode - With Verified Contract

**Command:**
```bash
soroban-registry contract verify CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U \
  --network testnet --strict --no-cache
```

**Expected Output:**
- Status display with verification result
- If verified with no issues: success message

**Success Criteria:**
- ✅ Exit code is 0 (success) if verification passes
- ✅ Command output is clear about verification result

---

### Test 5: Strict Mode - With Unverified Contract

**Command:**
```bash
soroban-registry contract verify "INVALID_OR_UNVERIFIED_CONTRACT" \
  --network testnet --strict --no-cache
```

**Expected Output:**
- Error message showing verification failed
- Clear indication of the issue

**Success Criteria:**
- ✅ Exit code is 1 (failure)
- ✅ Error message mentions strict mode or verification failure
- ✅ Output clearly indicates why strict mode failed

---

### Test 6: Batch Verification - Basic

**Command:**
```bash
soroban-registry contract verify "CADAYHQLQIAVGK6TQY74N6VD7RBQBMHTF5BT7HWMM4JYJCQPJPVPD5U,CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4,CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA4BAAAA" \
  --network testnet --batch --no-cache
```

**Expected Output:**
- Header showing "Batch Contract Verification"
- Number of contracts in batch
- Per-contract status with ✓ or ✗
- Summary showing:
  - Total verified count
  - Total unverified count
  - Total errors
  - Total warnings

**Success Criteria:**
- ✅ Batch header displayed
- ✅ All 3 contracts are processed
- ✅ Summary statistics are shown
- ✅ Exit code 0 on completion (unless --strict used with failures)

---

### Test 7: Batch Verification - JSON Output

**Command:**
```bash
soroban-registry contract verify "CONTRACT1,CONTRACT2,CONTRACT3" \
  --network testnet --batch --json --no-cache
```

**Expected Output:**
JSON with structure:
```json
{
  "batch": {
    "total": 3,
    "network": "testnet",
    "strict_mode": false
  },
  "results": [
    { /* individual contract result */ },
    { /* individual contract result */ },
    { /* individual contract result */ }
  ],
  "summary": {
    "verified": 1,
    "unverified": 2,
    "total_errors": 3,
    "total_warnings": 5
  }
}
```

**Success Criteria:**
- ✅ Output is valid JSON
- ✅ Can be parsed with `jq`
- ✅ Summary section contains correct counts
- ✅ Results array has 3 entries

---

### Test 8: Batch Verification - Size Limit

**Command:**
```bash
# Try to verify 51 contracts (exceeds limit of 50)
CONTRACTS=$(printf 'CONTRACT,' | head -c 51 | sed 's/,$//g')
soroban-registry contract verify "$CONTRACTS" --network testnet --batch
```

**Expected Output:**
Error message indicating batch size exceeded:
```
Batch size 51 exceeds maximum of 50 contracts
```

**Success Criteria:**
- ✅ Command fails with error message
- ✅ Exit code is 1
- ✅ Error message is clear about the limit

---

### Test 9: Batch Verification - Strict Mode

**Command:**
```bash
soroban-registry contract verify "CONTRACT1,CONTRACT2,CONTRACT3" \
  --network testnet --batch --strict --no-cache
```

**Expected Output:**
- Per-contract status displayed
- Batch summary shown
- If any contract has errors/warnings: failure message

**Success Criteria:**
- ✅ If all contracts verified: exit code 0
- ✅ If any contract has issues: exit code 1
- ✅ Error message shows count of errors/warnings

---

### Test 10: Cache File Structure

**Command:**
```bash
# Run a verification to create cache
soroban-registry contract verify CONTRACT --network testnet

# Check cache file
cat ~/.soroban-registry/verification_cache.json | jq .
```

**Expected Output:**
JSON structure like:
```json
{
  "testnet:CONTRACT": {
    "result": { /* VerificationResult */ },
    "cached_at": "2026-05-26T12:34:56.123456Z",
    "detail": { /* optional detail */ }
  }
}
```

**Success Criteria:**
- ✅ Cache file exists
- ✅ Structure is valid JSON
- ✅ Contains `cached_at` timestamp
- ✅ Contains verification result

---

### Test 11: Error Handling - Network Error

**Command:**
```bash
soroban-registry contract verify CONTRACT \
  --api-url "http://invalid-url:9999" \
  --network testnet --no-cache
```

**Expected Output:**
Error message about connection failure

**Success Criteria:**
- ✅ Error is caught and reported
- ✅ Exit code is 1
- ✅ Error message is helpful

---

### Test 12: Error Handling - Invalid Address

**Command:**
```bash
soroban-registry contract verify "" --network testnet
```

**Expected Output:**
Error message about invalid or missing address

**Success Criteria:**
- ✅ Error is caught and reported
- ✅ Exit code is 1

---

## Automated Testing

Run the comprehensive test suite:

```bash
# Make test script executable
chmod +x test_contract_verify.sh

# Run all tests
./test_contract_verify.sh

# Run with verbose output
bash -x test_contract_verify.sh
```

**Expected Output:**
```
═══════════════════════════════════════════════════════════
Contract Verification Command Test Suite
═══════════════════════════════════════════════════════════

▶ TEST: Single contract verification
✓ PASS: Single verification succeeded

▶ TEST: Cache functionality (24-hour TTL)
ℹ INFO: First run (API call)...
ℹ INFO: Second run (cache hit)...
✓ PASS: Cache hit is faster than API call

▶ TEST: Strict mode (--strict flag)
✓ PASS: Strict mode: verified contract passed

... (more tests)

═══════════════════════════════════════════════════════════
Test Summary
═══════════════════════════════════════════════════════════
Passed: 7
Failed: 0

All tests passed! ✓
```

---

## Verification Checklist

After running all tests, verify the following:

### Functionality ✅
- [ ] Single contract verification works
- [ ] Cache reduces response time dramatically
- [ ] --no-cache bypasses cache
- [ ] --strict mode fails on issues
- [ ] --batch processes multiple contracts
- [ ] --json outputs valid JSON
- [ ] Batch size limit enforced (max 50)
- [ ] Error messages are helpful

### Cache ✅
- [ ] Cache file created at ~/.soroban-registry/verification_cache.json
- [ ] Cache format is valid JSON
- [ ] Cache contains timestamp
- [ ] Expired entries would be pruned
- [ ] Cache can be cleared manually

### Performance ✅
- [ ] Cache hit: <10ms
- [ ] API call: 100-500ms
- [ ] Batch of 5: <3 seconds
- [ ] Batch of 50: <30 seconds

### Error Handling ✅
- [ ] Network errors reported clearly
- [ ] Invalid addresses handled
- [ ] Batch size limit enforced
- [ ] Strict mode violations reported
- [ ] Exit codes correct (0 = success, 1 = failure)

### Output Quality ✅
- [ ] Human-readable format is clear
- [ ] JSON output is valid and parseable
- [ ] Batch summary shows all metrics
- [ ] Error messages are actionable
- [ ] Security findings clearly presented

---

## CI/CD Integration Examples

### GitHub Actions

```yaml
- name: Verify Soroban Contracts
  run: |
    soroban-registry contract verify "${{ env.CONTRACT_ADDRESS }}" \
      --network mainnet \
      --strict \
      --json > verification-report.json

- name: Check Verification Report
  run: |
    cat verification-report.json | jq '.is_verified'
```

### GitLab CI

```yaml
verify_contracts:
  script:
    - soroban-registry contract verify "$CONTRACT_ID" 
        --network testnet 
        --strict
  artifacts:
    reports:
      dotenv: verification.txt
```

---

## Troubleshooting

### Issue: "Contract not found in registry"
**Solution**: Verify the contract has been published to the registry

### Issue: Cache hit not faster
**Solution**: Check that cache file exists: `ls ~/.soroban-registry/verification_cache.json`

### Issue: Strict mode fails unexpectedly
**Solution**: Run without --strict to see warnings: `soroban-registry contract verify <ADDR> --network testnet`

### Issue: Batch verification slow
**Solution**: Normal for large batches (50 contracts takes 5-25s). Use --json to suppress output.

---

## Performance Benchmarking

Create a script to benchmark performance:

```bash
#!/bin/bash
echo "Benchmarking contract verification..."

# Clear cache
rm -f ~/.soroban-registry/verification_cache.json

# Test 1: Cold run (API call)
echo "Cold run (API):"
time soroban-registry contract verify CONTRACT --network testnet --json > /dev/null

# Test 2: Warm run (cache hit)
echo "Warm run (cache):"
time soroban-registry contract verify CONTRACT --network testnet --json > /dev/null

# Test 3: Force fresh (--no-cache)
echo "Force fresh (--no-cache):"
time soroban-registry contract verify CONTRACT --network testnet --no-cache --json > /dev/null

# Test 4: Batch
echo "Batch (3 contracts):"
time soroban-registry contract verify "C1,C2,C3" --network testnet --batch --json > /dev/null
```

Run and compare times to baseline expectations.

---

## Sign-Off

Once all tests pass:

1. ✅ Review test output
2. ✅ Confirm all acceptance criteria met
3. ✅ Document any deviations
4. ✅ Deploy to staging for integration testing
5. ✅ Deploy to production

**Implementation is complete and ready for deployment.**
