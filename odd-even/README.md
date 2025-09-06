# odd-even

A small MCP server that lets authenticated users place a bet on whether a random value is odd or even. Users start a game by depositing an amount of ICRC-1 tokens (ICP by default). The server locks the bet, generates randomness using the IC randomness source, computes a SHA‑256 hash of the result and seed, stores the in-progress game in stable memory, and returns the game hash so the user can verify fairness later.

Location: `odd-even/`  
Key files: `src/lib.rs`, `src/repo.rs`, `src/runtime.rs`

## Authentication

This is a multi-user MCP server that uses OAuth for authentication. Init arguments configure OAuth and issuer JWKS metadata. Requests must present a valid access token; the token subject is mapped to an Internet Computer `Principal` and used as the user identity.

Init args expected (see `src/lib.rs` `InitArgs`):
- `metadata_url` — OAuth/OpenID metadata URL
- `resource` — resource identifier
- `issuer` — issuer identifier
- `jwks_url` — JWKS URL
- `authorization_server` — list of authorization server URLs
- `audience` — expected audience
- `scopes` — supported scopes

## Tools

Exposed via the MCP tools API:

- start
  - Description: Start a game by transferring tokens from the caller into the canister and recording an in-progress game.
  - Parameters:
    - `amount` (u64) — amount in token base units (ICP: 1 ICP = 100_000_000 units)
  - Limits: min 10_000_000 (0.1 ICP), max 500_000_000 (5 ICP)
  - Behavior: fetches randomness, determines parity (Odd/Even), computes `hash = sha256("<Result>|<RandomHex>")`, calls `transfer_from` on the ledger to move funds to the canister, and stores `GameInfo`.

- play
  - Description: Resolve a previously started game by submitting a guess.
  - Parameters:
    - `guess` — `"Odd"` or `"Even"`
  - Behavior: looks up the caller's game, checks expiration (7 days), compares guess with stored result, removes the in-progress game, and returns the result. If matching, the payout logic runs (see code).

- admin_delete (update)
  - Description: Controller-only method to delete a stored game for a principal.
  - Parameter: `principal` — principal to remove
  - Access: only canister controllers can call this (checked via `ic_cdk::is_controller`).

- HTTP handlers
  - `http_request` (query): responds OK and enables HTTP upgrade used by MCP adapters.
  - `http_request_update` (update): main HTTP entry that integrates OAuth and MCP tooling.

## Data model

`GameInfo` (stored in stable memory via `src/repo.rs`) contains:
- `amount: u64` — bet amount in base units
- `timestamp_nanos: u64` — start time in nanoseconds
- `result: String` — `"Odd"` or `"Even"`
- `random_hex: String` — hex of random bytes
- `hash: String` — SHA-256 hex of `<result>|<random_hex>`

Stable storage uses Candid encoding and `StableBTreeMap`, so entries survive upgrades.

## Randomness & fairness

- Randomness source: `ic_cdk::management_canister::raw_rand()`.
- Result parity computed by XORing the random bytes and taking the LSB:
  - 0 -> `Even`
  - 1 -> `Odd`
- Hash: SHA-256 of the string `"<Result>|<RandomHex>"` encoded as hex. The hash is returned at `start` so users can later verify the revealed result and seed.

## Amount units, ledger, and limits

- Amounts are in token base units. For ICP the code uses 8 decimals (1 ICP = 100_000_000 units).
- Valid `start.amount` range: 10_000_000 ..= 500_000_000.
- Default ledger canister used in code: ICP ledger `ryjl3-tyaaa-aaaaa-aaaba-cai`. The implementation uses `icrc_ledger_client::ICRC1Client` and `transfer_from`.

## Game lifetime

- A game expires after 7 days (604,800,000,000,000 ns). Expired games cannot be played; the code removes/handles such games on resolution attempts.

## Example tool payloads (conceptual)

Start:
```json
{
  "tool": "start",
  "arguments": { "amount": 100000000 }
}