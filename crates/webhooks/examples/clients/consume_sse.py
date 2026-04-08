"""
Subscribe to job events via SSE (Server-Sent Events).

pip install httpx-sse httpx
"""

import httpx
from httpx_sse import connect_sse
import json
import sys


def watch_job(base_url: str, job_id: str):
    """Stream job events until completion."""
    url = f"{base_url}/v1/jobs/{job_id}/events"

    with httpx.Client() as client:
        with connect_sse(client, "GET", url) as event_source:
            for event in event_source.iter_sse():
                data = json.loads(event.data)
                status = data["status"]
                print(f"[{event.event}] {json.dumps(data, indent=2)}")

                if status in ("completed", "failed", "cancelled"):
                    return data


def setup_webhook_receiver(port: int = 8080, secret: str = ""):
    """
    Minimal webhook receiver that verifies HMAC-SHA256 signatures.

    The operator POSTs to your URL with:
      X-Webhook-Signature: sha256=<hex>
      X-Job-Id: <job_id>
      Content-Type: application/json
    """
    from http.server import HTTPServer, BaseHTTPRequestHandler
    import hmac
    import hashlib

    class WebhookHandler(BaseHTTPRequestHandler):
        def do_POST(self):
            content_length = int(self.headers.get("Content-Length", 0))
            body = self.rfile.read(content_length)

            # Verify HMAC signature
            sig_header = self.headers.get("X-Webhook-Signature", "")
            if secret and sig_header:
                expected_sig = sig_header.removeprefix("sha256=")
                computed = hmac.new(
                    secret.encode(), body, hashlib.sha256
                ).hexdigest()
                if not hmac.compare_digest(computed, expected_sig):
                    self.send_response(401)
                    self.end_headers()
                    self.wfile.write(b"invalid signature")
                    return

            job_id = self.headers.get("X-Job-Id", "unknown")
            payload = json.loads(body)
            print(f"[webhook] job={job_id} status={payload['event']['status']}")

            if payload["event"].get("result"):
                print(f"  result: {json.dumps(payload['event']['result'], indent=2)}")

            self.send_response(200)
            self.end_headers()

    print(f"Webhook receiver listening on :{port}")
    HTTPServer(("0.0.0.0", port), WebhookHandler).serve_forever()


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage:")
        print("  python consume_sse.py sse <base_url> <job_id>")
        print("  python consume_sse.py webhook <port> [secret]")
        sys.exit(1)

    mode = sys.argv[1]
    if mode == "sse":
        result = watch_job(sys.argv[2], sys.argv[3])
        print(f"\nFinal: {result['status']}")
    elif mode == "webhook":
        port = int(sys.argv[2]) if len(sys.argv) > 2 else 8080
        secret = sys.argv[3] if len(sys.argv) > 3 else ""
        setup_webhook_receiver(port, secret)
