import { Hono } from 'hono'
import record from './record'

const api = new Hono()

api.route('/record', record)

export default api
