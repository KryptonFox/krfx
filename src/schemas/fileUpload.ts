import { z } from 'zod'
import Name from '@/schemas/name'
import { maxFileSize } from '@@/config.json'

const FileUpload = z.object({
  file: z
    .custom<File>((c) => c instanceof File, 'File not found')
    .refine((c: File) => c.size <= maxFileSize, { message: 'File is too big' }),
  name: Name,
})

export default FileUpload
