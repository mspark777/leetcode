export function maxProfit (prices: number[]): number {
  let minPrice = Number.MAX_SAFE_INTEGER
  let maxProfit = 0
  for (const price of prices) {
    minPrice = Math.min(minPrice, price)
    maxProfit = Math.max(maxProfit, price - minPrice)
  }

  return maxProfit
}
