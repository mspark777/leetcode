// import { createRequire } from 'module'
import { isAnagram } from './solution.mjs'

async function main () {
  const inputs = [
    {
      s: 'anagram', t: 'nagaram'
    },
    {
      s: 'rat', t: 'car'
    }
  ]

  for (const input of inputs) {
    const result = isAnagram(input.s, input.t)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
