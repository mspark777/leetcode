class MyCalendar {
  constructor () {
    /** @type number[][] */
    this.books = []
  }

  /**
   * @param {number} start
   * @param {number} end
   * @return {boolean}
   */
  book (start, end) {
    const books = this.books
    for (const [s, e] of books) {
      const l = Math.max(start, s)
      const r = Math.min(end, e)
      if (l < r) {
        return false
      }
    }

    books.push([start, end])
    return true
  }
}

async function main () {
  const inputs = [
    {
      book: [[10, 20], [15, 25], [20, 30]]
    }
  ]

  const calendar = new MyCalendar()
  for (const input of inputs) {
    for (const [s, e] of input.book) {
      const result = calendar.book(s, e)
      console.log(result)
    }
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
