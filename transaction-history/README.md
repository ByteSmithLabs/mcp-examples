# ICP Token Transaction history (showcase purpose only)

This MCP server provides a tool to get ICP token transaction history (latest transactions) for an account.

## Deployment:
```bash
dfx deploy transaction-history --argument '("YOUR_API_KEY")' --mode install
```
After deployment on local network or playground/mainnet, you can access it at: `https://<CANISTER_ID>.icp0.io/mcp` (for playground) or `https://<CANISTER_ID>.localhost:<BINDING_PORT>/mcp` (for local).


## Tools

The server exposes the following tools:

- **get_transaction_history**

  - **Description**: Retrieves the ICRC-1 token balance for a specified account.
  - **Parameters**:
    - `principal`: The textual principal of the account.
    - `max_results` (optional): Maximum number of results to return. Default to 5 if empty.