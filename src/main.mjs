class KthLargest {
  /** @type {number} */
  #k
  /** @type {number[]} */
  #nums

  /**
    * @param {number} k
    * @param {number[]} nums
    */
  constructor (k, nums) {
    this.#k = k - 1
    this.#nums = nums
    this.#nums.sort((a, b) => b - a)
  }

  /**
    * @param {number} val
    * @returns {number}
    */
  add (val) {
    let i = 0
    while (i < this.#nums.length) {
      if (val > this.#nums[i]) {
        break
      } else {
        i += 1
      }
    }

    if (i < this.#nums.length) {
      this.#nums.splice(i, 0, val)
    } else {
      this.#nums.push(val)
    }

    return this.#nums[this.#k]
  }
}

async function main () {
  const kthLargest = new KthLargest(3, [4, 5, 8, 2])
  console.log(kthLargest.add(3))
  console.log(kthLargest.add(5))
  console.log(kthLargest.add(10))
  console.log(kthLargest.add(9))
  console.log(kthLargest.add(4))
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
