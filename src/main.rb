class LongestPath
  def initialize
    @result = 0
  end

  # @param current [Integer]
  # @param children [Array<Array<Integer>>]
  # @param s [String]
  # @return [Integer]
  def dfs(current, children, s)
    longest_chain = 0
    second_longest_chain = 0

    children[current].each do |child|
      child_longest_chain = dfs(child, children, s)
      next if s[current] == s[child]

      if child_longest_chain > longest_chain
        second_longest_chain = longest_chain
        longest_chain = child_longest_chain
      elsif child_longest_chain > second_longest_chain
        second_longest_chain = child_longest_chain
      end
    end

    @result = [@result, longest_chain + second_longest_chain + 1].max
    longest_chain + 1
  end

  def get_result
    @result
  end
end

# @param parent [Array<Integer>]
# @param s [String]
# @return [Integer]
def longest_path(parent, s)
  children = Array.new(parent.length) { [] }
  for i in 1..(parent.length - 1)
    children[parent[i]].push(i)
  end

  solution = LongestPath.new
  solution.dfs(0, children, s)
  solution.get_result
end

def main
  inputs = [
    { parent: [-1, 0, 0, 1, 1, 2], s: 'abacbe' },
    { parent: [-1, 0, 0, 0], s: 'aabc' }
  ]

  inputs.each do |input|
    result = longest_path input[:parent], input[:s]
    puts result
  end
end

main
