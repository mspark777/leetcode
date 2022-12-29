class NumArray
  # @param nums [Array<Integer>]
  def initialize(nums)
    @sums = [0]
    nums.each do |num|
      @sums.append(@sums[-1] + num)
    end
  end

  # @param left [Integer]
  # @param right [Integer]
  # @return [Integer]
  def sum_range(left, right)
    @sums[right + 1] - @sums[left]
  end
end

def main
  obj = NumArray.new([-2, 0, 3, -5, 2, -1])
  puts obj.sum_range(0, 2)
  puts obj.sum_range(2, 5)
  puts obj.sum_range(0, 5)
end

main
