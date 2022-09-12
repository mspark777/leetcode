function bagOfTokensScore (tokens: number[], power: number): number {
  tokens.sort((a, b) => a - b)

  let score = 0
  let result = 0
  let i = 0
  let j = tokens.length - 1
  while ((i <= j) && ((power >= tokens[i]) || (score > 0))) {
    while ((i <= j) && (power >= tokens[i])) {
      power -= tokens[i]
      i += 1
      score += 1
    }

    result = Math.max(result, score)

    if ((i <= j) && (score > 0)) {
      power += tokens[j]
      j -= 1
      score -= 1
    }
  }

  return result
}

interface Input {
  readonly tokens: number[]
  readonly power: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      tokens: [100],
      power: 50
    },
    {
      tokens: [100, 200],
      power: 150
    },
    {
      tokens: [100, 200, 300, 400],
      power: 200
    }
  ]

  for (const { tokens, power } of inputs) {
    const result = bagOfTokensScore(tokens, power)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
