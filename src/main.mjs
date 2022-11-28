/**
 * @param {number[][]} matches
 * @returns {number[][]}
*/
function findWinners (matches) {
  const loseCounts = new Map()
  for (const [winner, loser] of matches) {
    if (!loseCounts.has(winner)) {
      loseCounts.set(winner, 0)
    }

    const count = loseCounts.get(loser) ?? 0
    loseCounts.set(loser, count + 1)
  }

  const winners = []
  const losers = []

  for (const [player, count] of loseCounts) {
    if (count < 1) {
      winners.push(player)
    } else if (count === 1) {
      losers.push(player)
    }
  }

  return [
    winners.sort((a, b) => a - b),
    losers.sort((a, b) => a - b)
  ]
}

async function main () {
  const inputs = [
    [[1, 3], [2, 3], [3, 6], [5, 6], [5, 7], [4, 5], [4, 8], [4, 9], [10, 4], [10, 9]],
    [[2, 3], [1, 3], [5, 4], [6, 4]]
  ]

  for (const matches of inputs) {
    const result = findWinners(matches)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
