class RandomizedSet {
  constructor () {
    /** @type {number[]} */
    this.nums = []
    /** @type {Map<number, number>} */
    this.indexes = new Map()
  }

  /**
   * @param {number} val
   * @returns {boolean}
   */
  insert (val) {
    const { nums, indexes } = this

    if (indexes.has(val)) {
      return false
    }

    indexes.set(val, nums.length)
    nums.push(val)
    return true
  }

  /**
   * @param {number} val
   * @returns {boolean}
   */
  remove (val) {
    const { nums, indexes } = this
    if (!indexes.has(val)) {
      return false
    }

    const pos = indexes.get(val)
    this.#swap(pos)
    nums.pop()
    if (nums.length > 0) {
      indexes.set(nums[pos], pos)
    }

    indexes.delete(val)
    return true
  }

  /**
   * @returns {number}
   */
  getRandom () {
    const { nums } = this
    const count = nums.length
    const random = Math.round(Math.random() * count * 10)
    const index = random % count
    return nums[index]
  }

  /**
   * @param {number} pos
   * @returns {undefined}
   */
  #swap (pos) {
    const { nums } = this
    const last = nums.length - 1
    if (last < 0) {
      return
    }

    const temp = nums[pos]
    nums[pos] = nums[last]
    nums[last] = temp
  }
}

async function main () {
  const obj = new RandomizedSet()
  console.log(obj.insert(1))
  console.log(obj.remove(2))
  console.log(obj.insert(2))
  console.log(obj.getRandom())
  console.log(obj.remove(1))
  console.log(obj.insert(2))
  console.log(obj.getRandom())
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
