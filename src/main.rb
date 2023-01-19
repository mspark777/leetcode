# @param nums [Array<Integer>]
# @param k [Integer]
# @return [Integer]
def subarrays_div_by_k(nums, k)
  prefix = 0
  result = 0

  modGroups = Array.new(k) { 0 }
  modGroups[0] = 1

  nums.each do |num|
    prefix = (prefix + k + (num % k)) % k
    result += modGroups[prefix]
    modGroups[prefix] += 1
  end

  result
end

def main
  inputs = [
    { nums: [4, 5, 0, -2, -3, 1], k: 5 },
    { nums: [5], k: 9 }
  ]

  inputs.each do |input|
    result = subarrays_div_by_k input[:nums], input[:k]
    puts result
  end
end

main
