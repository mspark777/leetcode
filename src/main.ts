import { getRow } from './solution'

interface Input {
  readonly rowIndex: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      rowIndex: 3
    },
    {
      rowIndex: 0
    },
    {
      rowIndex: 1
    }
  ]

  for (const input of inputs) {
    const result = getRow(input.rowIndex)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
