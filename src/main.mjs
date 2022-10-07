class MyMap extends Map {
  get0 (key) {
    return this.get(key) ?? 0
  }
}

class MyCalendarThree {
  constructor () {
    this.vals = new MyMap()
    this.lazy = new MyMap()
  }

  /**
   * @param {number} start
   * @param {number} end
   * @param {number} left
   * @param {number} right
   * @param {number} idx
   * @returns {undefined}
  */
  update (start, end, left, right, idx) {
    if ((start > right) || (end < left)) {
      return
    }

    const { vals, lazy } = this
    if ((start <= left) && (right <= end)) {
      vals.set(idx, vals.get0(idx) + 1)
      lazy.set(idx, lazy.get0(idx) + 1)
    } else {
      const mid = Math.trunc((left + right) / 2)

      const idx2 = idx * 2
      const idx21 = idx2 + 1
      this.update(start, end, left, mid, idx2)
      this.update(start, end, mid + 1, right, idx21)

      const max = Math.max(vals.get0(idx2), vals.get0(idx21))
      vals.set(idx, lazy.get0(idx) + max)
    }
  }

  /**
   * @param {number} start
   * @param {number} end
   * @returns {number}
  */
  book (start, end) {
    this.update(start, end - 1, 0, 1000000000, 1)
    return this.vals.get0(1)
  }
}

async function main () {
  const obj = new MyCalendarThree()
  console.log(obj.book(10, 20))
  console.log(obj.book(50, 60))
  console.log(obj.book(10, 40))
  console.log(obj.book(5, 15))
  console.log(obj.book(5, 10))
  console.log(obj.book(25, 55))
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
