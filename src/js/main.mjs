class SmallestInfiniteSet {
  /** @type {number} */
  #current
  /** @type {Set<number>} */
  #set
  constructor () {
    this.#current = 1
    this.#set = new Set()
  }

  /**
    * @returns {number}
    */
  popSmallest () {
    if (this.#set.size < 1) {
      const result = this.#current
      this.#current += 1

      return result
    }

    let result = Number.MAX_SAFE_INTEGER
    for (const num of this.#set) {
      result = Math.min(result, num)
    }

    this.#set.delete(result)
    return result
  }

  /**
    * @param {number} num
    * @returns {undefined}
    */
  addBack (num) {
    if (this.#current <= num) {
      return
    } else if (this.#set.has(num)) {
      return
    }

    this.#set.add(num)
  }
}

async function main () {
  const smallestInfiniteSet = new SmallestInfiniteSet()
  smallestInfiniteSet.addBack(2)
  console.log(smallestInfiniteSet.popSmallest())
  console.log(smallestInfiniteSet.popSmallest())
  console.log(smallestInfiniteSet.popSmallest())
  smallestInfiniteSet.addBack(1)
  console.log(smallestInfiniteSet.popSmallest())
  console.log(smallestInfiniteSet.popSmallest())
  console.log(smallestInfiniteSet.popSmallest())
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
