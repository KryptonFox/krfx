import { Hono } from 'hono'
import api from '@/routes/api'
import link from '@/routes/link'

const routes = new Hono()

routes.get('/', (c) => c.redirect('https://web.krfx.ru'))
routes.route('/api', api)
routes.route('/', link)

export default routes
