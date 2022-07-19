import { generate } from './solution'

interface Input {
  readonly numRows: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      numRows: 5
    },
    {
      numRows: 1
    }
  ]

  for (const input of inputs) {
    const result = generate(input.numRows)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
