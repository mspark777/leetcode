function query (bits: number[], age: number): number {
  let maxScore = Number.MIN_SAFE_INTEGER
  for (let i = age; i > 0; i -= i & (-i)) {
    maxScore = Math.max(maxScore, bits[i])
  }

  return maxScore
}

function update (bits: number[], age: number, best: number): void {
  for (let i = age; i < bits.length; i += i & (-i)) {
    bits[i] = Math.max(bits[i], best)
  }
}

function bestTeamScore (scores: number[], ages: number[]): number {
  const pairs = new Array<number[]>(ages.length)
  for (let i = 0; i < ages.length; i += 1) {
    pairs[i] = [scores[i], ages[i]]
  }

  pairs.sort((a, b) => {
    return a[0] === b[0] ? a[1] - b[1] : a[0] - b[0]
  })

  let highestAge = 0
  for (const age of ages) {
    highestAge = Math.max(age, highestAge)
  }

  const bits = new Array(highestAge + 1).fill(0)
  let result = Number.MIN_SAFE_INTEGER
  for (const [score, age] of pairs) {
    const best = score + query(bits, age)
    update(bits, age, best)

    result = Math.max(result, best)
  }

  return result
}

async function main (): Promise<void> {
  const inputs: number[][][] = [
    [[1, 3, 5, 10, 15], [1, 2, 3, 4, 5]],
    [[4, 5, 6, 5], [2, 1, 2, 1]],
    [[1, 2, 3, 5], [8, 9, 10, 1]]
  ]

  for (const [scores, ages] of inputs) {
    const result = bestTeamScore(scores, ages)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
