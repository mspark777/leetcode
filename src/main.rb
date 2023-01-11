# @param node [Integer]
# @param prev [Integer]
# @param adj_mat [Array<Array<Integer>>]
# @param has_apple [Array<Boolean>]
# @return [Integer]
def dfs(node, prev, adj_mat, has_apple)
  total_time = 0
  child_time = 0

  for child in adj_mat[node]
    next if child == prev

    child_time = dfs(child, node, adj_mat, has_apple)
    total_time += child_time + 2 if (child_time > 0) or has_apple[child]
  end

  total_time
end

# @param n [Integer]
# @param edges [Array<Array<Integer>>]
# @param has_apple [Array<Boolean>]
# @return [Integer]
def min_time(n, edges, has_apple)
  adj_mat = Array.new(n) { [] }

  for edge in edges
    l = edge[0]
    r = edge[1]
    adj_mat[l].push(r)
    adj_mat[r].push(l)
  end

  dfs(0, -1, adj_mat, has_apple)
end

def main
  inputs = [
    { n: 7, edges: [[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
      hasApple: [false, false, true, false, true, true, false] },
    { n: 7, edges: [[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
      hasApple: [false, false, true, false, false, true, false] },
    { n: 7, edges: [[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
      hasApple: [false, false, false, false, false, false, false] }
  ]

  inputs.each do |input|
    result = min_time input[:n], input[:edges], input[:hasApple]
    puts result
  end
end

main
