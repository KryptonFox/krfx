import { Hono } from 'hono'
import { serve } from '@hono/node-server'
import { cors } from 'hono/cors'
import { logger } from 'hono/logger'
import { prettyJSON } from 'hono/pretty-json'
import { trimTrailingSlash } from 'hono/trailing-slash'
import 'dotenv/config'
import routes from '@/routes'

const app = new Hono()

app.use(prettyJSON({ space: 2 }))
app.use(cors())
app.use(logger())
app.use(trimTrailingSlash())

app.route('/', routes)

const port = Number(process.env.PORT) || 3000
console.log(`Server is running on http://localhost:${port}`)

serve({
  fetch: app.fetch,
  port,
})
