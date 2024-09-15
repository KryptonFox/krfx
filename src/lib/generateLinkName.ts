import { linkName } from '../../config.json'
import prisma from '../../prisma/prisma'

function rand(): number {
  return Math.floor(Math.random() * (linkName.generator.charset.length - 1))
}

export default async function generateLinkName(): Promise<string> {
  let name = ''
  do {
    for (let i = 0; i < linkName.generator.length; i++) {
      name += linkName.generator.charset[rand()]
    }
  } while (await prisma.link.findUnique({ where: { name: name } }))
  return name
}
