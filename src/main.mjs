class Baskets {
  constructor () {
    /** @type {Map<number, number} */
    this.map = new Map()
  }

  /**
    * @param {number} key
    * @returns {number}
    */
  get (key) {
    return this.map.get(key) ?? 0
  }

  /**
    * @param {number} key
    * @param {number} value
    * @returns {undefined}
    */
  set (key, value) {
    this.map.set(key, value)
  }

  /**
    * @param {number} key
    * @returns {undefined}
    */
  delete (key) {
    this.map.delete(key)
  }

  /**
    * @param {number} key
    * @returns {undefined}
    */
  increase (key) {
    this.set(key, this.get(key) + 1)
  }

  /**
    * @param {number} key
    * @returns {undefined}
    */
  decrease (key) {
    this.set(key, this.get(key) - 1)
  }

  /**
    * @returns {number}
    */
  size () {
    return this.map.size
  }
}

/**
  * @param {number[]} fruits
  * @returns {number}
  */
function totalFruit (fruits) {
  const baskets = new Baskets()
  let left = 0
  let result = 0

  for (let right = 0; right < fruits.length; right += 1) {
    const rfruit = fruits[right]
    baskets.increase(rfruit)

    while (baskets.size() > 2) {
      const lfruit = fruits[left]
      baskets.decrease(lfruit)
      if (baskets.get(lfruit) === 0) {
        baskets.delete(lfruit)
      }
      left += 1
    }

    result = Math.max(result, right - left + 1)
  }

  return result
}

async function main () {
  const inputs = [
    [1, 2, 1],
    [0, 1, 2, 2],
    [1, 2, 3, 2, 2],
    [3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]
  ]

  for (const fruits of inputs) {
    const result = totalFruit(fruits)
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
