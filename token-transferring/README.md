# ICRC-1 Token Transfer (showcase purpose only)

This MCP server enables token transfers and balance queries for ICRC-1 tokens on the Internet Computer blockchain. It provides tools to manage and transfer tokens from the server's account and to retrieve information about supported tokens.

## Tools

The server exposes the following tools:

- **get_balance**

  - **Description**: Retrieves the ICRC-1 token balance for a specified account.
  - **Parameters**:
    - `principal` (optional): The textual principal of the account. If empty, defaults to the server account.
    - `ledger_canister_id`: The canister ID of the token's ledger.

- **transfer**

  - **Description**: Transfers ICRC-1 tokens from the server account to a specified destination principal.
  - **Parameters**:
    - `destination_principal`: The textual principal of the recipient.
    - `amount`: The amount to transfer, in the smallest unit of the token (e.g., for ICP with 8 decimals, 1 ICP = 100000000 units).
    - `ledger_canister_id`: The canister ID of the token's ledger.
  - **Note**: Use the `get_supported_tokens` tool to retrieve the decimals value for the token to calculate the correct amount.

- **get_principal**

  - **Description**: Retrieves the server's default account in textual principal format.
  - **Parameters**: None
  - **Usage**: Use this tool to obtain the server's principal for topping up ICRC-1 tokens to the server.

- **add_token**

  - **Description**: Adds a new token to the list of supported tokens.
  - **Parameters**:
    - `symbol`: The symbol of the token (e.g., "ICP").
    - `ledger_canister_id`: The canister ID of the token's ledger.
    - `decimals`: The number of decimal places for the token.

- **get_supported_tokens**

  - **Description**: Retrieves the list of supported tokens, including their symbols, ledger canister IDs, and decimals.
  - **Parameters**: None

## Built-in Supported Tokens

The server comes with the following built-in supported tokens:

- **ICP**

  - Ledger Canister ID: `ryjl3-tyaaa-aaaaa-aaaba-cai`
  - Decimals: 8

- **ckBTC**

  - Ledger Canister ID: `mxzaz-hqaaa-aaaar-qaada-cai`
  - Decimals: 8

- **ckETH**

  - Ledger Canister ID: `ss2fx-dyaaa-aaaar-qacoq-cai`
  - Decimals: 18

- **ckUSDC**

    - Ledger Canister ID: `xevnm-gaaaa-aaaar-qafnq-cai`

    - Decimals: 6