import { isAnagram } from './solution'

interface Input {
  readonly s: string
  readonly t: string
}

async function main (): Promise<void> {
  const inputs: Input[] = [
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
