function isPerfectSquare (num: number): boolean {
  let left = 1
  let right = Math.trunc(num / 2)
  while (left <= right) {
    const mid = Math.trunc((left + right) / 2)
    const square = mid * mid
    if (num < square) {
      right = mid - 1
    } else if (num > square) {
      left = mid + 1
    } else {
      return true
    }
  }

  return num === 1
}

async function main (): Promise<void> {
  const inputs: number[] = [
    16, 14, 1
  ]

  for (const num of inputs) {
    const result = isPerfectSquare(num)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
