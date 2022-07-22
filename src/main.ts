import { isPalindrome } from './solution'

interface Input {
  readonly s: string
}

async function main (): Promise<void> {
  const inputs: Input[] = [
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
