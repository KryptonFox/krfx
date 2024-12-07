import { Hono } from 'hono'
import file from './file'
import link from './link'

const route = new Hono()

route.route('/link', link)
route.route('/file', file)

export default route
