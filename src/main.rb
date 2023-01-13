require 'set'

# @param nums1 [Array<Integer>]
# @param nums2 [Array<Integer>]
# @return [Array<Integer>]
def intersection(nums1, nums2)
  nums1 & nums2
end

def main
  inputs = [
    [[1, 2, 2, 1], [2, 2]],
    [[4, 9, 5], [9, 4, 9, 8, 4]]
  ]

  inputs.each do |input|
    result = intersection input[0], input[1]
    puts result
  end
end

main
