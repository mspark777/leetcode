// import { createRequire } from 'module'
import { getRow } from './solution.mjs'

async function main () {
  const inputs = [
    {
      rowIndex: 3
    },
    {
      rowIndex: 0
    },
    {
      rowIndex: 1
    }
  ]

  for (const input of inputs) {
    const result = getRow(input.rowIndex)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
