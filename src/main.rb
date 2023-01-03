# @param nums [Array<Integer>]
# @param target [Integer]
# @return [Integer]
def search(nums, target)
  left = 0
  right = nums.length - 1
  while left <= right
    mid = (left + right) / 2
    return mid if nums[mid] == target

    if nums[mid] >= nums[left]
      if (target >= nums[left]) && (target < nums[mid])
        right = mid - 1
      else
        left = mid + 1
      end
    elsif (target > nums[mid]) && (target <= nums[right])
      left = mid + 1
    else
      right = mid - 1
    end
  end

  -1
end

def main
  inputs = [
    { nums: [4, 5, 6, 7, 0, 1, 2], target: 0 },
    { nums: [4, 5, 6, 7, 0, 1, 2], target: 3 },
    { nums: [1], target: 0 }
  ]

  inputs.each do |input|
    result = search input[:nums], input[:target]
    puts result
  end
end

main
