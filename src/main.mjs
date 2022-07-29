// import { createRequire } from 'module'
import { findAndReplacePattern } from './solution.mjs'

async function main () {
  const inputs = [
    {
      words: ['abc', 'deq', 'mee', 'aqq', 'dkd', 'ccc'],
      pattern: 'abb'
    },
    {
      words: ['a', 'b', 'c'],
      pattern: 'a'
    }
  ]

  for (const input of inputs) {
    const result = findAndReplacePattern(input.words, input.pattern)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
