import { fib } from './solution'

interface Input {
  readonly n: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      n: 2
    },
    {
      n: 3
    },
    {
      n: 4
    },
    {
      n: 9
    }
  ]

  for (const input of inputs) {
    const result = fib(input.n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
