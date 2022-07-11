import { createTreeFromArray, rightSideView } from './solution'

interface Input {
  readonly root: (number | null)[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      root: [1, 2, 3, null, 5, null, 4]
    },
    {
      root: [1, null, 3]
    },
    {
      root: []
    }
  ]

  for (const input of inputs) {
    const result = rightSideView(createTreeFromArray(input.root))
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
