// import { createRequire } from 'module'
import { minDeletions } from './solution.mjs'

async function main () {
  const inputs = [
    { s: 'aab' },
    { s: 'aaabbbcc' },
    { s: 'ceabaacb' }
  ]

  for (const input of inputs) {
    const result = minDeletions(input.s)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
