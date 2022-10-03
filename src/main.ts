function minCost (colors: string, neededTime: number[]): number {
  let totalTime = 0
  let currMaxTime = neededTime[0]
  for (let i = 1; i < colors.length; i += 1) {
    if (colors[i] !== colors[i - 1]) {
      currMaxTime = 0
    }

    const needed = neededTime[i]
    totalTime += Math.min(currMaxTime, needed)
    currMaxTime = Math.max(currMaxTime, needed)
  }

  return totalTime
}

interface Input {
  readonly colors: string
  readonly neededTime: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      colors: 'abaac',
      neededTime: [1, 2, 3, 4, 5]
    },
    {
      colors: 'abc',
      neededTime: [1, 2, 3]
    },
    {
      colors: 'aabaa',
      neededTime: [1, 2, 3, 4, 1]
    }
  ]

  for (const { colors, neededTime } of inputs) {
    const result = minCost(colors, neededTime)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
