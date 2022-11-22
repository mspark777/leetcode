function numSquares (n: number): number {
  const memos = new Array<number>(n + 1).fill(Number.MAX_SAFE_INTEGER)

  memos[0] = 0
  let cur = 1
  let squire = 1

  while (squire <= n) {
    for (let i = squire; i <= n; i += 1) {
      memos[i] = Math.min(memos[i - squire] + 1, memos[i])
    }

    cur += 1
    squire = cur * cur
  }

  return memos[n]
}

async function main (): Promise<void> {
  const inputs: number[] = [
    12, 13
  ]

  for (const n of inputs) {
    const result = numSquares(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
