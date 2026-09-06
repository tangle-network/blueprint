# Payment ingress

Read [README.md](README.md) and [the example configuration](config/x402.example.toml) before changing payment or job policy behavior.
MPP and x402 use the same job pricing and policy enforcement; preserve those checks through both interfaces.
Read the manifest's explanation before enabling additional `mpp` features: the selected features deliberately avoid unrelated chain dependencies.
Keep secret validation, challenge expiry, and replay protection enforced.
Verify restricted jobs still require authentication after payment succeeds.
