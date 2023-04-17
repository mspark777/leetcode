function kidsWithCandies (candies: number[], extraCandies: number): boolean[] {
  const maxCandy = Math.max(...candies) - extraCandies
  return candies.map(candy => candy >= maxCandy)
}

async function main (): Promise<void> {
  const inputs: Array<[number[], number]> = [
    [[2, 3, 5, 1, 3], 3],
    [[4, 2, 1, 1, 2], 1],
    [[12, 1, 12], 10]
  ]

  for (const [candies, extraCandies] of inputs) {
    const result = kidsWithCandies(candies, extraCandies)
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
