class Helper {
  constructor () {
    /** @type {Map<number, number>} */
    this.map = new Map()
    /** @type {number} */
    this.islands = 0
  }

  /**
   * @param {number} x
   * @returns {number}
  */
  find (x) {
    const { map } = this
    if (!map.has(x)) {
      map.set(x, x)
      this.islands += 1
    }

    const p = map.get(x)
    if (x !== p) {
      map.set(x, this.find(p))
    }

    return map.get(x)
  }

  /**
   * @param {number} x
   * @param {number} y
   * @returns {undefined}
  */
  uni (x, y) {
    x = this.find(x)
    y = this.find(y)
    if (x !== y) {
      this.map.set(x, y)
      this.islands -= 1
    }
  }

  /**
   * @returns {number}
  */
  getIsLands () {
    return this.islands
  }
}

/**
 * @param {number[][]} stones
 * @returns {number}
*/
function removeStones (stones) {
  const helper = new Helper()
  for (const [x, y] of stones) {
    helper.uni(x, ~y)
  }

  return stones.length - helper.getIsLands()
}

async function main () {
  const inputs = [
    [[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]],
    [[0, 0], [0, 2], [1, 1], [2, 0], [2, 2]],
    [[0, 0], [0, 1], [1, 0], [1, 1], [2, 1], [2, 2], [3, 2], [3, 3], [3, 4], [4, 3], [4, 4]]
  ]

  for (const stones of inputs) {
    const result = removeStones(stones)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
