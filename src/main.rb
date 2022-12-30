# @param graph [Array<Array<Integer>>]
# @param results [Array<Array<Integer>>]
# @param path [Array<Integer>]
# @param cur [Integer]
# @return [Void]
def dfs(graph, results, path, cur)
  path.push cur

  if cur == (graph.length - 1)
    results.push path.clone
  else
    graph[cur].each do |n|
      dfs graph, results, path, n
    end
  end

  path.pop
end

# @param graph [Array<Array<Integer>>]
# @return [Array<Array<Integer>>]
def all_paths_source_target(graph)
  results = []
  path = []
  dfs graph, results, path, 0

  results
end

def main
  inputs = [
    [[1, 2], [3], [3], []],
    [[4, 3, 1], [3, 2, 4], [3], [4], []]
  ]

  inputs.each do |graph|
    result = all_paths_source_target graph
    puts result
  end
end

main
