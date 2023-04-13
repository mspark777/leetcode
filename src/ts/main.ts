function validateStackSequences (pushed: number[], popped: number[]): boolean {
  const numCount = pushed.length
  const stack: number[] = []

  let popCount = 0
  for (const p of pushed) {
    stack.push(p)
    while (popCount < numCount) {
      if (stack.length < 1) {
        break
      } else if (stack.at(-1) !== popped[popCount]) {
        break
      }

      stack.pop()
      popCount += 1
    }
  }

  return popCount === numCount
}

async function main (): Promise<void> {
  const inputs: number[][][] = [
    [[1, 2, 3, 4, 5], [4, 5, 3, 2, 1]],
    [[1, 2, 3, 4, 5], [4, 3, 5, 1, 2]]
  ]

  for (const [pushed, popped] of inputs) {
    const result = validateStackSequences(pushed, popped)
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
