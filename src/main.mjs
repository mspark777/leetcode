// import { createRequire } from 'module'
import { isPalindrome } from './solution.mjs'

async function main () {
  const inputs = [
    {
      s: 'A man, a plan, a canal: Panama'
    },
    {
      s: 'race a car'
    },
    {
      s: ' '
    }
  ]

  for (const input of inputs) {
    const result = isPalindrome(input.s)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
