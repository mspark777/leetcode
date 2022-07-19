// import { createRequire } from 'module'
import { generate } from './solution.mjs'

async function main () {
  const inputs = [
    {
      numRows: 5
    },
    {
      numRows: 1
    }
  ]

  for (const input of inputs) {
    const result = generate(input.numRows)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
