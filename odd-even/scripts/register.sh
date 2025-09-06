dfx canister call --ic bfggx-7yaaa-aaaai-q32gq-cai register_resource_server '(
  record {
    initial_service_principal = principal "77yky-paaaa-aaaab-abvwa-cai";
    scopes = vec {
        record { 0 = "openid"; 1 = "See your Prometheus principal." };
        record { 0 = "prometheus:charge"; 1 = "Transfer from your ICP account through ICRC-2 approval." }
    };
    name = "Odd-even server";
    uris = vec { "https://77yky-paaaa-aaaab-abvwa-cai.icp0.io/mcp"};
    accepted_payment_canisters = vec { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"};
    logo_uri = "https://placehold.co/128x128/1a1a1a/ffffff/png?text=Odd%20even"
  }
)'