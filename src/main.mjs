/**
 * @param {number} k
 * @param {number[]} prices
 * @return {number}
 */
function maxProfit (k, prices) {
  if (k <= 0) {
    return 0
  }

  const transactions = new Array(k + 1)
  for (let i = 0; i <= k; i += 1) {
    transactions[i] = {
      spend: Number.MAX_SAFE_INTEGER,
      profit: 0
    }
  }

  for (const price of prices) {
    for (let i = 1; i <= k; i += 1) {
      const prev = transactions[i - 1]
      const cur = transactions[i]
      cur.spend = Math.min(cur.spend, price - prev.profit)
      cur.profit = Math.max(cur.profit, price - cur.spend)
    }
  }

  return transactions[k].profit
}

async function main () {
  const inputs = [
    {
      k: 2,
      prices: [2, 4, 1]
    },
    {
      k: 2,
      prices: [3, 2, 6, 5, 0, 3]
    },
    {
      k: 2,
      prices: [3, 2, 6, 5, 0, 3]
    }
  ]

  for (const { k, prices } of inputs) {
    const result = maxProfit(k, prices)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
