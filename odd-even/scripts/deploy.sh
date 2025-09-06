dfx deploy odd-even --ic --argument '(record {
 metadata_url = "https://77yky-paaaa-aaaab-abvwa-cai.icp0.io/.well-known/oauth-protected-resource";
 resource = "https://77yky-paaaa-aaaab-abvwa-cai.icp0.io/mcp";
 issuer = "https://bfggx-7yaaa-aaaai-q32gq-cai.icp0.io";
 jwks_url = "https://bfggx-7yaaa-aaaai-q32gq-cai.icp0.io/.well-known/jwks.json";
 audience = "https://77yky-paaaa-aaaab-abvwa-cai.icp0.io/mcp";
 authorization_server = vec { "https://bfggx-7yaaa-aaaai-q32gq-cai.icp0.io" };
 scopes = vec { "openid"; "prometheus:charge" };
})' --mode reinstall