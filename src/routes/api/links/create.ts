import { Hono } from 'hono'
import LinkCreate from '@/schemas/linkCreate'
import prisma from '@@/prisma/prisma'
import generateLinkName from '@/lib/generateLinkName'

const create = new Hono()

create.post('/', async (c) => {
  // validating
  const data = await LinkCreate.safeParseAsync(await c.req.json())
  if (!data.success) {
    return c.json({
      success: false,
      error: data.error.issues[0].message,
    })
  }
  // get url and name
  const url = data.data.url
  let name = data.data.name
  if (!name) {
    // generate name
    name = await generateLinkName()
  }
  // db writing
  await prisma.link.create({
    data: {
      url: url,
      displayName: name,
      name: name.toLowerCase(),
    },
  })
  // return
  return c.json({
    success: true,
    url: new URL(name, new URL(c.req.url).origin).toString(),
  })
})

export default create
