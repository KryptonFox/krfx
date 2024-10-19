import { Hono } from 'hono'
import upload from '@/routes/api/files/upload'

const files = new Hono()
files.route('/upload', upload)

export default files
