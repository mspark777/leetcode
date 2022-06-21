export function suggestedProducts2 (products, searchWord) {
  products.sort()

  const results = []
  for (let i = 0; i < searchWord.length; i += 1) {
    products = products.filter(p => p[i] === searchWord[i])
    results.push(products.slice(0, 3))
  }

  return results
}
