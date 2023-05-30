class MyHashSet {
  /** @type {Array<boolean | null>} */
  #nums
  constructor () {
    this.#nums = new Array(1000000).fill(null)
  }

  /**
    * @param {number} key
    * @returns {undefined}
    */
  add (key) {
    this.#nums[key] = true
  }

  /**
    * @param {number} key
    * @returns {undefined}
    */
  remove (key) {
    this.#nums[key] = null
  }

  /**
    * @param {number} key
    * @returns {boolean}
    */
  contains (key) {
    return this.#nums[key] != null
  }
}

async function main () {
  const myHashSet = new MyHashSet()
  myHashSet.add(1)
  myHashSet.add(2)
  console.log(myHashSet.contains(1))
  console.log(myHashSet.contains(3))
  myHashSet.add(2)
  console.log(myHashSet.contains(2))
  myHashSet.remove(2)
  console.log(myHashSet.contains(2))
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
