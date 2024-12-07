import { Hono } from 'hono'
import prisma from '../../prisma/prisma'

const link = new Hono()

link.get('/:link', async (c) => {
  const link = c.req.param('link')?.trim().toLowerCase()
  if (!link) return c.text('Link name is required', 400)

  const linkRec = await prisma.record.findUnique({
    where: { name: link },
    include: { file: true, link: true },
  })
  if (!linkRec) return c.text('Link is not found', 404)

  if (linkRec.type === 'FILE' && linkRec.file) {
    return fetch(linkRec.file.cdnUrl)
  } else if (linkRec.type === 'LINK' && linkRec.link) {
    return c.redirect(linkRec.link.url)
  }
})

export default link
