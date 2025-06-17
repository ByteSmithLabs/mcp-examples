# ICP Token Transaction history (showcase purpose only)

This MCP server provides a tool to get ICP token transaction history (latest transactions) for an account.

## Tools

The server exposes the following tools:

- **get_transaction_history**

  - **Description**: Retrieves the ICRC-1 token balance for a specified account.
  - **Parameters**:
    - `principal`: The textual principal of the account.
    - `max_results` (optional): Maximum number of results to return. Default to 5 if empty.