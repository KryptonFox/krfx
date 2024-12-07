import { z } from 'zod'
import prisma from '@@/prisma/prisma'
import { linkName } from '@@/config.json'

const Name = z
  .string()
  .trim()
  .min(linkName.validator.minLength, 'The name is too short')
  .regex(/^[0-9a-zA-Z_\-]+$/gm, 'Restricted characters')
  .refine(async (val: string) => {
    return (
      !linkName.validator.blacklist.includes(val) &&
      !(await prisma.record.findUnique({ where: { name: val } }))
    )
  }, 'Name is used')
  .or(z.literal(''))
  .optional()
  .nullable()

export default Name
