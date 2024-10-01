import { serve } from '@hono/node-server'
import { Hono } from 'hono'
import api from './routes/api'
import { prettyJSON } from 'hono/pretty-json'
import link from './routes/link'
import { cors } from 'hono/cors'

const app = new Hono()

app.use(prettyJSON({ space: 2 }))
app.use(cors())

app.route('/', api)
app.route('/', link)

const port = 3000
console.log(`Server is running on port ${port}`)

serve({
  fetch: app.fetch,
  port,
})
