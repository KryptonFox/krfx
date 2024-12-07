import { Hono } from 'hono'
import LinkCreate from '@/schemas/linkCreate'
import generateLinkName from '@/lib/generateLinkName'
import prisma from '@@/prisma/prisma'

const route = new Hono()

route.post('/', async (c) => {
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
  await prisma.record.create({
    data: {
      name: name.toLowerCase(),
      displayName: name,
      ownerType: 'ANONYMOUS',
      type: 'LINK',
      link: {
        create: {
          url,
        },
      },
    },
  })

  // return
  return c.json({
    success: true,
    url: new URL(name, new URL(c.req.url).origin).toString(),
  })
})

export default route
