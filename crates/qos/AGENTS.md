# Quality-of-service metrics

Read [README.md](README.md) and the current service configuration before changing metrics collection or deployment.
Preserve blueprint and service filtering when querying metrics across tenants.
The bundled Loki configuration is for development and disables authentication.
A production deployment needs authenticated access, TLS, and durable storage appropriate to its environment.
Use the serial test settings from CI when tests share Docker or network resources.
