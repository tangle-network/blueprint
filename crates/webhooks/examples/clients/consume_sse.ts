/**
 * Subscribe to job events via SSE (Server-Sent Events).
 *
 * Works in Node.js 18+ (native fetch + EventSource) or browsers.
 * For Node.js < 18: npm install eventsource
 */

// ── SSE Consumer ────────────────────────────────────────────────────────

interface JobEvent {
  status: 'queued' | 'processing' | 'completed' | 'failed' | 'cancelled'
  progress?: number
  result?: Record<string, unknown>
  error?: string
  timestamp: number
}

/**
 * Watch a job via SSE until it reaches a terminal state.
 * Returns a promise that resolves with the final event.
 */
export function watchJob(baseUrl: string, jobId: string): Promise<JobEvent> {
  return new Promise((resolve, reject) => {
    const url = `${baseUrl}/v1/jobs/${jobId}/events`
    const source = new EventSource(url)

    const onEvent = (type: string) => (e: MessageEvent) => {
      const event: JobEvent = JSON.parse(e.data)
      console.log(`[${type}]`, JSON.stringify(event, null, 2))

      if (['completed', 'failed', 'cancelled'].includes(event.status)) {
        source.close()
        resolve(event)
      }
    }

    source.addEventListener('queued', onEvent('queued'))
    source.addEventListener('processing', onEvent('processing'))
    source.addEventListener('completed', onEvent('completed'))
    source.addEventListener('failed', onEvent('failed'))
    source.addEventListener('cancelled', onEvent('cancelled'))

    source.onerror = (e) => {
      source.close()
      reject(new Error(`SSE connection error: ${e}`))
    }
  })
}

// ── Webhook Receiver ────────────────────────────────────────────────────

import { createHmac, timingSafeEqual } from 'node:crypto'
import { createServer, type IncomingMessage, type ServerResponse } from 'node:http'

interface WebhookPayload {
  job_id: string
  event: JobEvent
}

/**
 * Verify HMAC-SHA256 signature from operator webhook delivery.
 */
function verifySignature(body: Buffer, signature: string, secret: string): boolean {
  const expected = signature.replace('sha256=', '')
  const computed = createHmac('sha256', secret).update(body).digest('hex')
  try {
    return timingSafeEqual(Buffer.from(computed, 'hex'), Buffer.from(expected, 'hex'))
  } catch {
    return false
  }
}

/**
 * Start a webhook receiver server.
 */
export function startWebhookReceiver(
  port: number,
  secret: string,
  onEvent: (payload: WebhookPayload) => void,
) {
  const server = createServer((req: IncomingMessage, res: ServerResponse) => {
    if (req.method !== 'POST') {
      res.writeHead(405)
      res.end()
      return
    }

    const chunks: Buffer[] = []
    req.on('data', (chunk: Buffer) => chunks.push(chunk))
    req.on('end', () => {
      const body = Buffer.concat(chunks)

      // Verify signature
      const sig = req.headers['x-webhook-signature'] as string
      if (secret && sig && !verifySignature(body, sig, secret)) {
        res.writeHead(401)
        res.end('invalid signature')
        return
      }

      const payload: WebhookPayload = JSON.parse(body.toString())
      onEvent(payload)

      res.writeHead(200)
      res.end('ok')
    })
  })

  server.listen(port, () => {
    console.log(`Webhook receiver listening on :${port}`)
  })

  return server
}

// ── CLI ─────────────────────────────────────────────────────────────────

const [,, mode, ...args] = process.argv

if (mode === 'sse') {
  const [baseUrl, jobId] = args
  watchJob(baseUrl, jobId).then((event) => {
    console.log(`\nFinal: ${event.status}`)
    process.exit(event.status === 'completed' ? 0 : 1)
  })
} else if (mode === 'webhook') {
  const port = parseInt(args[0] || '8080')
  const secret = args[1] || ''
  startWebhookReceiver(port, secret, (payload) => {
    console.log(`[webhook] job=${payload.job_id} status=${payload.event.status}`)
    if (payload.event.result) {
      console.log('  result:', JSON.stringify(payload.event.result, null, 2))
    }
  })
} else {
  console.log('Usage:')
  console.log('  npx tsx consume_sse.ts sse <base_url> <job_id>')
  console.log('  npx tsx consume_sse.ts webhook <port> [secret]')
}
