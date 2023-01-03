# @param strs [Array<String>]
# @return [Integer]
def min_deletion_size(strs)
  result = 0

  for c in 0..(strs[0].length - 1) do
    for r in 1..(strs.length - 1) do
      c0 = strs[r - 1][c]
      c1 = strs[r][c]
      if c0 > c1
        result += 1
        break
      end
    end
  end

  result
end

def main
  inputs = [
    %w[cba daf ghi],
    %w[a b],
    %w[zyx wvu tsr]
  ]

  inputs.each do |strs|
    result = min_deletion_size strs
    puts result
  end
end

main
