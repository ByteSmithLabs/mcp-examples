# Alloy Wallet MCP (showcase purpose only)

This MCP server acts as a multi-chain crypto wallet, enabling asset management on Bitcoin, Ethereum, and Solana blockchains. It provides tools to get wallet addresses, check balances, and transfer native assets from the server's wallet.

## Features

- **Multi-Chain Support**: Manage assets on Bitcoin, Ethereum, and Solana.
- **Address Generation**: Retrieve the server's wallet address for each chain to receive funds.
- **Balance Inquiry**: Check the balance of any address on supported chains.
- **Asset Transfer**: Send native assets (BTC, ETH, SOL) from the server's wallet.
- **Secure Access**: All operations are protected via API key authentication.

## Environment Configuration

The canister requires the following arguments at deployment time:

- `mode`: (`Test` | `Production`) - The operational mode for the canister. When in test mode, the canister only interacts with non-main network.  
- `api_key`: A secret string used to authenticate requests. All API calls must include this key in the `x-api-key` header.
- `key_name`: The name of the threshold ECDSA key used for signing transactions (e.g., `dfx_test_key` or `key_1`). This key must be available to the canister.

## Deployment

Deploy the canister using the following command, replacing the placeholders with your configuration:

```bash
dfx deploy alloy-wallet --argument '(record { mode = variant { Production }; api_key = "YOUR_SUPER_SECRET_API_KEY"; key_name = "YOUR_ECDSA_KEY_NAME" })'
```

> When testing locally with Solana, remember setting up API keys for `sol_rpc` canister by `./provision.sh`.

After deployment, you can access the MCP endpoint at:

- **Mainnet/Playground**: `https://<CANISTER_ID>.icp0.io/mcp`
- **Local Network**: `https://<CANISTER_ID>.localhost:<BINDING_PORT>/mcp`

## Usage

All requests to the MCP endpoint must include an `x-api-key` header containing the `api_key` you configured during deployment.

**Example using `curl`:**

```bash
curl "https://<CANISTER_ID>.icp0.io/mcp" \
  -X POST \
  -H "Content-Type: application/json" \
  -H "x-api-key: YOUR_SUPER_SECRET_API_KEY" \
  -d '{
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/list",
        "params": {}
      }'
```

## Tools

The server exposes the following tools:

### get_address

- **Description**: Get the server's wallet address for a given chain. Use this tool to obtain the address for funding the wallet.
- **Parameters**:
  - `chain` (enum): The blockchain to get the address for.
    - `Bitcoin`
    - `Ethereum`
    - `Solana`

### get_balance

- **Description**: Get the asset balance of a specific wallet address on a given chain.
- **Parameters**:
  - `chain` (enum): The blockchain to query.
    - `Bitcoin`
    - `Ethereum`
    - `Solana`
  - `address` (string): The wallet address to check the balance of.

### transfer

- **Description**: Transfer native assets from the server's wallet to a specified destination address.
- **Parameters**:
  - `chain` (enum): The blockchain on which to perform the transfer.
    - `Bitcoin`
    - `Ethereum`
    - `Solana`
  - `destination_address` (string): The recipient's wallet address.
  - `amount` (number): The amount to transfer in the smallest unit of the asset.
- **Note on `amount`**:
  - **Bitcoin**: The amount is in **Satoshi** (1 BTC = 100,000,000 Satoshi).
  - **Ethereum**: The amount is in **Wei** (1 ETH = 1,000,000,000,000,000,000 Wei).
  - **Solana**: The amount is in **Lamports** (1 SOL = 1,000,000,000 Lamports).