# @param {[]} nums
# @return {Boolean}
def can_jump(nums)
  last = nums.length - 1
  for i in (nums.length - 2).downto(0)
    cur = i + nums[i]
    last = i if cur >= last
  end

  last < 1
end

def main
  inputs = [
    [1],
    [2, 3, 1, 1, 4],
    [3, 2, 1, 0, 4]
  ]

  inputs.each do |nums|
    result = can_jump nums
    puts result
  end
end

main
