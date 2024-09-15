import { PrismaClient } from '@prisma/client'

declare global {
  // eslint-disable-next-line no-var
  var globalPrisma: PrismaClient // <- Назовите его как угодно, кроме "prisma".
}

let prisma: PrismaClient

if (process.env.NODE_ENV === 'production') {
  prisma = new PrismaClient()
} else {
  if (!global.globalPrisma) {
    global.globalPrisma = new PrismaClient()
  }

  prisma = global.globalPrisma
}

export default prisma
