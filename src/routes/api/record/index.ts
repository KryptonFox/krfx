import { Hono } from 'hono'
import create from './create'
import get from './get'

const route = new Hono()

route.route('/create', create)
route.route('/get', get)

export default route
