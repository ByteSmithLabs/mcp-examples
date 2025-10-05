dfx deploy token-transferring --ic --argument '(record {
metadata_url = "https://qfn7t-6aaaa-aaaab-aceyq-cai.icp0.io/.well-known/oauth-protected-resource";
resource = "https://qfn7t-6aaaa-aaaab-aceyq-cai.icp0.io/mcp";
audience = "https://qfn7t-6aaaa-aaaab-aceyq-cai.icp0.io/mcp";
issuer = "https://bfggx-7yaaa-aaaai-q32gq-cai.icp0.io";
jwks_url = "https://bfggx-7yaaa-aaaai-q32gq-cai.icp0.io/.well-known/jwks.json";
authorization_server = vec { "https://bfggx-7yaaa-aaaai-q32gq-cai.icp0.io" };
scopes = vec { "openid" };
})' --mode reinstall