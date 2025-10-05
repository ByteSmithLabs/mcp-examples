dfx canister call --ic bfggx-7yaaa-aaaai-q32gq-cai register_resource_server '(
  record {
    initial_service_principal = principal "qfn7t-6aaaa-aaaab-aceyq-cai";
    scopes = vec {
        record { 0 = "openid"; 1 = "See your Prometheus principal." };
    };
    name = "Token transferring";
    uris = vec { "https://qfn7t-6aaaa-aaaab-aceyq-cai.icp0.io/mcp"};
    logo_uri = "https://placehold.co/128x128/1a1a1a/ffffff/png?text=token%20transferring"
  }
)'