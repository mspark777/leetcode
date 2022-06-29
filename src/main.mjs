// import { createRequire } from 'module'
import { reconstructQueue } from './solution.mjs'

async function main () {
  const inputs = [
    { people: [[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]] },
    { people: [[6, 0], [5, 0], [4, 0], [3, 2], [2, 2], [1, 4]] }
  ]

  for (const input of inputs) {
    const result = reconstructQueue(input.people)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
