import { z } from 'zod'
import Name from '@/schemas/name'

const LinkCreate = z.object({
  url: z
    .string({ required_error: 'URL is required' })
    .trim()
    .url('URL is invalid')
    .transform((url: string) => new URL(url).toString()),
  name: Name,
})

export default LinkCreate
