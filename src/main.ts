import { maxScore } from './solution'

interface Input {
  readonly cardPoints: number[]
  readonly k: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { cardPoints: [1, 2, 3, 4, 5, 6, 1], k: 3 },
    { cardPoints: [2, 2, 2], k: 2 },
    { cardPoints: [9, 7, 7, 9, 7, 7, 9], k: 7 }
  ]

  for (const input of inputs) {
    const result = maxScore(input.cardPoints, input.k)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
