function GCD (a: number, b: number): number {
  return b === 0 ? a : GCD(b, a % b)
}

function maxPoints (points: number[][]): number {
  const N = points.length
  if (N < 2) {
    return 1
  }

  let result = 2

  for (let i = 0; i < N; i += 1) {
    const slopes = new Map<string, number>()
    for (let j = 0; j < N; j += 1) {
      if (i === j) {
        continue
      }

      const [ix, iy] = points[i]
      const [jx, jy] = points[j]
      let x = jx - ix
      let y = jy - iy
      const gcd = GCD(Math.abs(x), Math.abs(y))
      if (gcd !== 0) {
        x = Math.trunc(x / gcd)
        y = Math.trunc(y / gcd)
      }
      const key = `${x}:${y}`
      const count = slopes.get(key) ?? 0
      slopes.set(key, count + 1)
    }

    for (const count of slopes.values()) {
      result = Math.max(result, count + 1)
    }
  }

  return result
}

async function main (): Promise<void> {
  const inputs: number[][][] = [
    [[1, 1], [2, 2], [3, 3]],
    [[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]]
  ]

  for (const points of inputs) {
    const result = maxPoints(points)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
