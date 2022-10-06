interface Node {
  readonly value: string
  readonly timestamp: number
}

class TimeMap {
  readonly store: Map<string, Node[]>
  constructor () {
    this.store = new Map()
  }

  set (key: string, value: string, timestamp: number): void {
    if (!this.store.has(key)) {
      this.store.set(key, [])
    }

    const nodes = this.store.get(key) ?? []
    nodes.push({ value, timestamp })

    this.store.set(key, nodes)
  }

  get (key: string, timestamp: number): string {
    const nodes = this.store.get(key)
    if (nodes == null) {
      return ''
    }

    if (timestamp < nodes[0].timestamp) {
      return ''
    }

    let left = 0
    let right = nodes.length

    while (left < right) {
      const mid = Math.trunc((left + right) / 2)
      if (nodes[mid].timestamp <= timestamp) {
        left = mid + 1
      } else {
        right = mid
      }
    }

    if (right === 0) {
      return ''
    }

    return nodes[right - 1].value
  }
}

async function main (): Promise<void> {
  const timeMap = new TimeMap()
  timeMap.set('foo', 'bar', 1)
  console.log(timeMap.get('foo', 1))
  console.log(timeMap.get('foo', 3))
  timeMap.set('foo', 'bar2', 4)
  console.log(timeMap.get('foo', 4))
  console.log(timeMap.get('foo', 5))
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
