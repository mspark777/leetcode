import { minPartitions } from './solution'

interface Input {
  readonly n: string
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { n: '32' },
    { n: '82734' },
    { n: '27346209830709182346' },
    { n: '135' }
  ]

  for (const input of inputs) {
    const result = minPartitions(input.n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
