# @param nums1 [Array<Integer>]
# @param nums2 [Array<Integer>]
# @return [Array<Integer>]
def intersect(nums1, nums2)
  counts = Hash.new(0)
  nums1.each do |num|
    counts[num] += 1
  end

  result = []
  nums2.each do |num|
    next unless counts.include? num

    count = counts[num]
    if count > 0
      result.push(num)
      counts[num] = count - 1
    end
  end

  result
end

def main
  inputs = [
    [[1, 2, 2, 1], [2, 2]],
    [[4, 9, 5], [9, 4, 9, 8, 4]]
  ]

  inputs.each do |input|
    result = intersect input[0], input[1]
    puts result
  end
end

main
