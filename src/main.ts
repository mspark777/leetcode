function feasible (weights: number[], capacity: number, days: number): boolean {
  let daysNeeded = 1
  let currentLoad = 0
  for (const weight of weights) {
    currentLoad += weight
    if (currentLoad > capacity) {
      daysNeeded += 1
      currentLoad = weight
    }

    if (daysNeeded > days) {
      return false
    }
  }

  return true
}

function shipWithinDays (weights: number[], days: number): number {
  let totalLoad = 0
  let maxLoad = 0
  for (const weight of weights) {
    totalLoad += weight
    maxLoad = Math.max(maxLoad, weight)
  }

  let left = BigInt(maxLoad)
  let right = BigInt(totalLoad)
  while (left < right) {
    const middle = (left + right) / 2n
    if (feasible(weights, Number(middle), days)) {
      right = middle
    } else {
      left = middle + 1n
    }
  }

  return Number(left)
}

interface Input {
  readonly weights: number[]
  readonly days: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { weights: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], days: 5 },
    { weights: [3, 2, 2, 4, 1, 4], days: 3 },
    { weights: [1, 2, 3, 1, 1], days: 4 }
  ]

  for (const { weights, days } of inputs) {
    const result = shipWithinDays(weights, days)
    console.log(result)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
