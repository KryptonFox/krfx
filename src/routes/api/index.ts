import { Hono } from 'hono'
import links from '@/routes/api/links'
import files from '@/routes/api/files'

const api = new Hono()
api.route('/links', links)
api.route('/files', files)

export default api
