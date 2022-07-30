// import { createRequire } from 'module'
import { wordSubsets } from './solution.mjs'

async function main () {
  const inputs = [
    {
      words1: ['amazon', 'apple', 'facebook', 'google', 'leetcode'],
      words2: ['e', 'o']
    },
    {
      words1: ['amazon', 'apple', 'facebook', 'google', 'leetcode'],
      words2: ['l', 'e']
    }
  ]

  for (const input of inputs) {
    const result = wordSubsets(input.words1, input.words2)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
