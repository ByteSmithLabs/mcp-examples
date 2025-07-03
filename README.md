# mcp-examples

A repo of some Model Context Protocol servers built on Internet Computer Canister.

This repo only aims to provide some showcases for the capability of a MCP server on IC Canister environment. Further consideration is needed when using the code on production.

### Test with temporary canister - the canister will be reinstall after a while, do not top up a lot of money

```bash
bash scripts/deploy.sh token-transferring
bash scripts/deploy.sh transaction-history
bash scripts/deploy.sh allot-wallet
```
