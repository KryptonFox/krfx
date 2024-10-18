import { Hono } from 'hono'
import links from '@/routes/api/links'

const api = new Hono()
api.route('/links', links)

export default api
