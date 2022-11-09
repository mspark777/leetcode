class StockSpanner {
  constructor () {
    /** @type {number[][]} */
    this.stack = []
  }

  /**
   * @param {number} price
   * @returns {number}
  */
  next (price) {
    const { stack } = this
    let span = 1

    for (let top = stack.pop(); top != null; top = stack.pop()) {
      const [p, s] = top
      if (p <= price) {
        span += s
      } else {
        stack.push(top)
        break
      }
    }

    stack.push([price, span])
    return span
  }
}

async function main () {
  const stockSpanner = new StockSpanner()
  console.log(
    stockSpanner.next(100),
    stockSpanner.next(80),
    stockSpanner.next(60),
    stockSpanner.next(70),
    stockSpanner.next(60),
    stockSpanner.next(75),
    stockSpanner.next(85)
  )
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
