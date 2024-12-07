import { Hono } from 'hono'
import prisma from '@@/prisma/prisma'

const route = new Hono()

route.post('/', async (c) => {
  const body = await c.req.json()
  if (!body) return c.text('No body', 400)

  const linkName = body.name as string | undefined
  if (!linkName) return c.text('No name in body', 400)

  const link = await prisma.record.findUnique({
    where: { name: linkName.toLowerCase() },
    include: { file: true, link: true },
  })

  if (!link) return c.text('Link is not found', 404)
  return c.json(link)
})

export default route
