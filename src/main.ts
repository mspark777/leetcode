function minRefuelStops (target: number, startFuel: number, stations: number[][]): number {
  const stationCount = stations.length
  const dp = new Array<number>(stationCount + 1).fill(0)
  dp[0] = startFuel

  for (let i = 0; i < stationCount; i += 1) {
    for (let t = i; t >= 0; t -= 1) {
      const [position, fuel] = stations[i]
      const memo = dp[t]
      if (memo >= position) {
        const next = t + 1
        dp[next] = Math.max(dp[next], dp[t] + fuel)
      }
    }
  }

  for (let i = 0; i <= stationCount; i += 1) {
    if (dp[i] >= target) {
      return i
    }
  }

  return -1
}

interface Input {
  readonly target: number
  readonly startFuel: number
  readonly stations: number[][]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      target: 1, startFuel: 1, stations: []
    },
    {
      target: 100, startFuel: 1, stations: [[10, 100]]
    },
    {
      target: 100, startFuel: 10, stations: [[10, 60], [20, 30], [30, 30], [60, 40]]
    },
    {
      target: 100, startFuel: 50, stations: [[50, 50]]
    }
  ]

  for (const { target, startFuel, stations } of inputs) {
    const result = minRefuelStops(target, startFuel, stations)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
