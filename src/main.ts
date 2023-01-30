function recursive (n: number, t0: number, t1: number, t2: number): number {
  return n > 2 ? recursive(n - 1, t1, t2, t0 + t1 + t2) : t2
}

function tribonacci (n: number): number {
  if (n === 0) {
    return 0
  } else if (n < 3) {
    return 1
  }

  return recursive(n, 0, 1, 1)
}

async function main (): Promise<void> {
  const inputs: number[] = [
    4, 25
  ]

  for (const n of inputs) {
    const result = tribonacci(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
