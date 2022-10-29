function earliestFullBloom (plantTime: number[], growTime: number[]): number {
  const N = growTime.length
  const indices = new Array<number>(N)
  for (let i = 0; i < N; i += 1) {
    indices[i] = i
  }

  indices.sort((a, b) => growTime[b] - growTime[a])
  let result = 0
  let curPlantTime = 0
  for (let i = 0; i < N; i += 1) {
    const idx = indices[i]
    const ptime = plantTime[idx]
    const time = curPlantTime + ptime + growTime[idx]
    result = Math.max(result, time)
    curPlantTime += ptime
  }

  return result
}

interface Input {
  readonly plantTime: number[]
  readonly growTime: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      plantTime: [1, 4, 3],
      growTime: [2, 3, 1]
    },
    {
      plantTime: [1, 2, 3, 2],
      growTime: [2, 1, 2, 1]
    },
    {
      plantTime: [1],
      growTime: [1]
    }
  ]

  for (const { plantTime, growTime } of inputs) {
    const result = earliestFullBloom(plantTime, growTime)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
