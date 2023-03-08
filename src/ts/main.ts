function hoursRequired (piles: bigint[], k: bigint): bigint {
  if (k === 0n) {
    return BigInt(Number.MAX_SAFE_INTEGER)
  }

  let hours = 0n
  for (const pile of piles) {
    if ((pile % k) !== 0n) {
      hours += 1n
    }

    hours += (pile / k)
  }

  return hours
}

function minEatingSpeed0 (piles: bigint[], h: bigint): bigint {
  let sum = 0n
  let maxPile = 0n
  for (const pile of piles) {
    sum += pile
    if (pile > maxPile) {
      maxPile = pile
    }
  }

  let left = sum / h
  let right = maxPile
  while (left < right) {
    const middle = (left + right) / 2n
    const required = hoursRequired(piles, middle)
    if (required > h) {
      left = middle + 1n
    } else {
      right = middle
    }
  }

  return left
}

function minEatingSpeed (piles: number[], h: number): number {
  const result = minEatingSpeed0(piles.map(p => BigInt(p)), BigInt(h))
  return Number(result)
}

interface Input {
  readonly piles: number[]
  readonly h: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { piles: [3, 6, 7, 11], h: 8 },
    { piles: [30, 11, 23, 4, 20], h: 5 },
    { piles: [30, 11, 23, 4, 20], h: 6 }
  ]

  for (const { piles, h } of inputs) {
    const result = minEatingSpeed(piles, h)
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
