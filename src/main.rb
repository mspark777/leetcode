# @param nums [Array<Integer>]
# @param n [Integer] n
# @return [Array<Integer>]
def shuffle(nums, n)
  result = Array.new(nums.length) { 0 }

  for i in 1..n
    j = i - 1
    k = j * 2
    result[k] = nums[j]
    result[k + 1] = nums[j + n]
  end

  result
end

def main
  inputs = [
    { nums: [2, 5, 1, 3, 4, 7], n: 3 },
    { nums: [1, 2, 3, 4, 4, 3, 2, 1], n: 4 }
  ]

  inputs.each do |input|
    result = shuffle(input[:nums], input[:n])
    puts(result)
  end
end

main
