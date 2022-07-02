import { maxArea } from './solution'

interface Input {
  readonly h: number
  readonly w: number
  readonly horizontalCuts: number[]
  readonly verticalCuts: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      h: 5,
      w: 4,
      horizontalCuts: [1, 2, 4],
      verticalCuts: [1, 3]
    },
    {
      h: 5,
      w: 4,
      horizontalCuts: [3, 1],
      verticalCuts: [1]
    },
    {
      h: 5,
      w: 4,
      horizontalCuts: [3],
      verticalCuts: [3]
    },
    {
      h: 1000000000,
      w: 1000000000,
      horizontalCuts: [2],
      verticalCuts: [2]
    }
  ]

  for (const input of inputs) {
    const result = maxArea(input.h, input.w, input.horizontalCuts, input.verticalCuts)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
