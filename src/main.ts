import '@total-typescript/ts-reset'

function topologicalSort (graph: Map<number, number[]>, indegree: number[]): number[] {
  const visited: number[] = []
  const stack: number[] = []
  for (const key of graph.keys()) {
    if (indegree.at(key) === 0) {
      stack.push(key)
    }
  }

  for (let key = stack.pop(); key != null; key = stack.pop()) {
    visited.push(key)

    for (const prev of graph.get(key) ?? []) {
      const degree = indegree.at(prev) as number
      indegree[prev] = degree - 1
      if (degree === 1) {
        stack.push(prev)
      }
    }
  }

  return visited.length === graph.size ? visited : []
}

function sortItems (n: number, m: number, group: number[], beforeItems: number[][]): number[] {
  let groupID = m
  const itemGraph = new Map<number, number[]>()
  for (let i = 0; i < n; i += 1) {
    itemGraph.set(i, [])
    if (group[i] === -1) {
      group[i] = groupID
      groupID += 1
    }
  }

  const itemIndegree = new Array<number>(n).fill(0)
  const groupGraph = new Map<number, number[]>()
  const groupIndegree = new Array<number>(groupID).fill(0)
  for (let i = 0; i < groupID; i += 1) {
    groupGraph.set(i, [])
  }

  for (let curr = 0; curr < n; curr += 1) {
    for (const prev of beforeItems.at(curr) ?? []) {
      itemGraph.get(prev)?.push(curr)
      itemIndegree[curr] += 1
      if (group.at(curr) !== group.at(prev)) {
        groupGraph.get(group[prev] as number)?.push(group.at(curr) as number)
        groupIndegree[group.at(curr) as number] += 1
      }
    }
  }

  const itemOrder = topologicalSort(itemGraph, itemIndegree)
  const groupOrder = topologicalSort(groupGraph, groupIndegree)

  if (itemOrder.length < 1) {
    return []
  } else if (groupOrder.length < 1) {
    return []
  }

  const orderedGroups = new Map<number, number[]>()
  for (const item of itemOrder) {
    const key = group[item] as number
    const ordered = orderedGroups.get(key) ?? []
    ordered.push(item)
    orderedGroups.set(key, ordered)
  }

  const result: number[] = []
  for (const groupIndex of groupOrder) {
    const ordered = orderedGroups.get(groupIndex) ?? []
    result.push(...ordered)
  }

  return result
}

function main (): void {
  const inputs: Array<[number, number, number[], number[][]]> = [
    [8, 2, [-1, -1, 1, 0, 0, 1, 0, -1], [[], [6], [5], [6], [3, 6], [], [], []]],
    [8, 2, [-1, -1, 1, 0, 0, 1, 0, -1], [[], [6], [5], [6], [3], [], [4], []]]
  ]

  for (const [n, m, group, beforeItems] of inputs) {
    const result = sortItems(n, m, group, beforeItems)
    console.log(result)
  }
}
main()
