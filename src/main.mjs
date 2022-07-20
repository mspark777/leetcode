// import { createRequire } from 'module'
import { numMatchingSubseq } from './solution.mjs'

async function main () {
  const inputs = [
    {
      s: 'abcde',
      words: ['a', 'bb', 'acd', 'ace']
    },
    {
      s: 'dsahjpjauf',
      words: ['ahjpjau', 'ja', 'ahbwzgqnuk', 'tnmlanowax']
    }
  ]

  for (const input of inputs) {
    const result = numMatchingSubseq(input.s, input.words)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
