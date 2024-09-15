import { z } from 'zod'
import prisma from '../../prisma/prisma'
import { linkName } from '../../config.json'

const LinkCreate = z.object({
  url: z
    .string({ required_error: 'URL is required' })
    .trim()
    .url('URL is invalid'),
  name: z
    .string()
    .trim()
    .min(linkName.validator.minLength, 'The name is too short')
    .regex(/^[0-9a-zA-Z_\-]+$/gm, 'Restricted characters')
    .refine(async (val) => {
      return !(await prisma.link.findUnique({ where: { name: val } }))
    }, 'Name is used')
    .optional(),
})

export default LinkCreate
