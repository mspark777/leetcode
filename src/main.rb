# @param nums [Array<Integer>]
# @return [Integer]
def jump(nums)
  result = 0
  curend = 0
  curfar = 0

  for i in (2..nums.length)
    j = i - 2
    curfar = [curfar, j + nums[j]].max

    if j == curend
      result += 1
      curend = curfar
    end
  end

  result
end

def main
  inputs = [
    [2, 3, 1, 1, 4],
    [2, 3, 0, 1, 4]
  ]

  inputs.each do |input|
    result = jump(input)
    puts(result)
  end
end

main
