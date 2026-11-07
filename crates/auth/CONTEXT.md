# Authentication vocabulary

**Owner predicate**: The application's asynchronous authorization decision for an Ethereum signer and a resource; its grant contains the authoritative expiry and capabilities. _Avoid_: signature verifier, token validator.

**Resource scope**: A canonical resource identifier inside a registered service, distinct from the capability required for an operation. _Avoid_: bearer credential, capability.

**Owner domain**: Server-selected audience and optional chain, operator, and contract identity bound into owner-session challenges and tokens. _Avoid_: caller-provided tenant.
