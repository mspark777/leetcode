function suggestedProducts (products, searchWord) {
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

async function main () {
  const inputs = [
    { products: ['mobile', 'mouse', 'moneypot', 'monitor', 'mousepad'], searchWord: 'mouse' },
    { products: ['havana'], searchWord: 'havana' },
    { products: ['bags', 'baggage', 'banner', 'box', 'cloths'], searchWord: 'bags' }
  ]

  for (const input of inputs) {
    const result = suggestedProducts(input.products, input.searchWord)
    for (const r of result) {
      console.log(r)
    }
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
