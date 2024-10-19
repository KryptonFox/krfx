import { Hono } from 'hono'
import FileUpload from '@/schemas/fileUpload'
import { PutObjectCommand, S3Client } from '@aws-sdk/client-s3'
import generateLinkName from '@/lib/generateLinkName'
import prisma from '@@/prisma/prisma'
import { createHash } from 'node:crypto'

const upload = new Hono()

upload.post('/', async (c) => {
  // get data
  let data
  try {
    data = await c.req.formData()
  } catch (e) {
    return c.json({ success: false, error: 'FormData not found' })
  }
  // validate data
  const validatedData = await FileUpload.safeParseAsync({
    file: data.get('file'),
    name: data.get('name'),
  })

  if (!validatedData.success) {
    return c.json({
      success: false,
      error: validatedData.error.issues[0].message,
    })
  }

  const name = validatedData.data.name || (await generateLinkName())
  const fileExt = validatedData.data.file.name.split('.').at(-1)
  const url = new URL(`${name}.${fileExt}`, process.env.CDN_URL).toString()
  const hash = createHash('md5')
    .update(new Uint8Array(await validatedData.data.file.arrayBuffer()))
    .digest('base64')

  // db writing
  await prisma.link.create({
    data: {
      url: url,
      type: 'FILE',
      displayName: name,
      name: name.toLowerCase(),
    },
  })

  // upload
  const s3Client = new S3Client({
    endpoint: process.env.AWS_ENDPOINT!,
    region: process.env.AWS_REGION!,
    credentials: {
      accessKeyId: process.env.AWS_ACCESS_KEY_ID!,
      secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY!,
    },
  })
  await s3Client.send(
    new PutObjectCommand({
      Bucket: 'krfx-storage',
      Key: `${name}.${fileExt}`,
      Body: new Uint8Array(await validatedData.data.file.arrayBuffer()),
      ContentMD5: hash,
      ContentType: validatedData.data.file.type,
    }),
  )
  // response
  return c.json({
    success: true,
    url: new URL(name, new URL(c.req.url).origin).toString(),
    directURL: url,
  })
})

export default upload
