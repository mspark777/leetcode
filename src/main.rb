# @param nums [Array<Integer>]
# @return [Integer]
def max_subarray_sum_circular(nums)
  curmax = 0
  curmin = 0
  maxsum = nums[0]
  minsum = nums[0]
  sum = 0

  nums.each do |num|
    curmax = [curmax, 0].max + num
    maxsum = [curmax, maxsum].max
    curmin = [curmin, 0].min + num
    minsum = [curmin, minsum].min
    sum += num
  end

  return maxsum if sum == minsum

  [maxsum, sum - minsum].max
end

def main
  inputs = [
    [1, -2, 3, -2],
    [5, -3, 5],
    [-3, -2, -3]
  ]

  inputs.each do |nums|
    result = max_subarray_sum_circular nums
    puts result
  end
end

main
