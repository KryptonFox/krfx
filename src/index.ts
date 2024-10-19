import { serve } from '@hono/node-server'
import { Hono } from 'hono'
import { prettyJSON } from 'hono/pretty-json'
import { cors } from 'hono/cors'
import routes from '@/routes'
import { logger } from 'hono/logger'
import 'dotenv/config'
import * as process from 'node:process'

const app = new Hono()

app.use(prettyJSON({ space: 2 }))
app.use(cors())
app.use(logger())

app.route('/', routes)

const port = Number(process.env.PORT) || 3000
console.log(`Server is running on http://localhost:${port}`)

serve({
  fetch: app.fetch,
  port,
})
