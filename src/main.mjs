/**
 * @param {number[][]} properties
 * @return {number}
 */
function numberOfWeakCharacters (properties) {
  properties.sort(
    ([attackA, defenceA], [attackB, defenceB]) =>
      attackA === attackB
        ? defenceA - defenceB
        : attackB - attackA
  )

  let maxDefence = Number.MIN_SAFE_INTEGER
  let result = 0
  for (const [_attack, defense] of properties) {
    if (maxDefence > defense) {
      result += 1
    } else {
      maxDefence = defense
    }
  }

  return result
}

async function main () {
  const inputs = [
    {
      properties: [[5, 5], [6, 3], [3, 6]]
    },
    {
      properties: [[2, 2], [3, 3]]
    },
    {
      properties: [[1, 5], [10, 4], [4, 3]]
    }
  ]

  for (const { properties } of inputs) {
    const result = numberOfWeakCharacters(properties)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
