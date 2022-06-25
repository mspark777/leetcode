import { checkPossibility } from './solution'

interface Input {
  readonly nums: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { nums: [4, 2, 3] },
    { nums: [4, 2, 1] }
  ]

  for (const input of inputs) {
    const result = checkPossibility(input.nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
