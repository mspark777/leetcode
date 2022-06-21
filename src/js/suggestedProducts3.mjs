export function suggestedProducts (products, searchWord) {
  products.sort()

  const results = new Array(searchWord.length)
  for (let i = 0; i < searchWord.length; i += 1) {
    results[i] = []
  }

  for (const product of products) {
    const len = Math.min(product.length, searchWord.length)
    for (let i = 0; i < len; i += 1) {
      if (product.charCodeAt(i) !== searchWord.charCodeAt(i)) {
        break
      } else if (results[i].length < 3) {
        results[i].push(product)
      }
    }
  }

  return results
}
