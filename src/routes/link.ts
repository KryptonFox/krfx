import { Hono } from 'hono'
import prisma from '../../prisma/prisma'

const link = new Hono()

link.get('/:link', async (c) => {
  const link = c.req.param('link')?.trim().toLowerCase()
  if (!link) {
    return c.text('Link name is required')
  }
  const linkRec = await prisma.link.findUnique({ where: { name: link } })
  if (!linkRec) {
    return c.text('Link is not found')
  }
  if (linkRec.type === 'FILE') {
    return fetch(linkRec.url)
  } else {
    return c.redirect(linkRec.url)
  }
})

export default link
