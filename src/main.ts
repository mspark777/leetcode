let guess = (n: number): number => n
function Guess (pick: number): (n: number) => number {
  return n => {
    if (n < pick) {
      return 1
    } else if (n > pick) {
      return -1
    } else {
      return 0
    }
  }
}

function guessNumber (n: number): number {
  let left = 1
  let right = n
  while (left <= right) {
    const m = Math.round((left + right) / 2)
    const res = guess(m)
    if (res < 0) {
      right = m - 1
    } else if (res > 0) {
      left = m + 1
    } else {
      return m
    }
  }

  return -1
}

async function main (): Promise<void> {
  const inputs: number[][] = [[10, 6], [1, 1], [2, 1], [2126753390, 1702766719]]

  for (const [n, pick] of inputs) {
    guess = Guess(pick)
    const result = guessNumber(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
