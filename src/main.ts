import { isPossible } from './solution'

interface Input {
  readonly target: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { target: [9, 3, 5] },
    { target: [1, 1, 1, 2] },
    { target: [8, 5] },
    { target: [1, 1000000000] }
  ]

  for (const input of inputs) {
    const result = isPossible(input.target)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
