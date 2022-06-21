defmodule MinOperations do
  @spec min_operations(nums :: [integer], x :: integer) :: integer
  def min_operations(nums, x) do
    target = Enum.sum(nums) - x
    nums_len = Enum.count(nums)
    result = -1

    if target < 0 do
      -1
    else
      if target == 0 do
        nums_len
      else
        solve(nums, nums, 0, 0, 0, target, nums_len, result)
      end
    end
  end

  def solve([l | lnums], [r | rnums], left, right, sum, target, nums_len, result) when right < nums_len do
    sum = sum + r
    if sum > target do
      sum = sum - (l + r)
      solve(lnums, [r | rnums], left + 1, right, sum, target, nums_len, result)
    else
      if sum == target do
        result = max(result, right - left + 1)
        solve([l | lnums], rnums, left, right + 1, sum, target, nums_len, result)
      else
        solve([l | lnums], rnums, left, right + 1, sum, target, nums_len, result)
      end
    end
  end

  def solve([], _, _, right, _, _, nums_len, result) when right < nums_len do
    if result != -1 do
      nums_len - result
    else
      -1
    end
  end

  def solve(_, _, _, right, _, _, nums_len, result) when right >= nums_len do
    if result != -1 do
      nums_len - result
    else
      -1
    end
  end
end
