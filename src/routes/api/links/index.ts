import { Hono } from 'hono'
import create from '@/routes/api/links/create'

const links = new Hono()
links.route('/create', create)

export default links
