import { serve } from '@hono/node-server'
import { Hono } from 'hono'
import { prettyJSON } from 'hono/pretty-json'
import { cors } from 'hono/cors'
import routes from '@/routes'

const app = new Hono()

app.use(prettyJSON({ space: 2 }))
app.use(cors())

app.route('/', routes)

const port = 3000
console.log(`Server is running on port ${port}`)

serve({
  fetch: app.fetch,
  port,
})
