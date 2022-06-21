defmodule Solution do
  @spec maximum_unique_subarray(nums :: [integer]) :: integer
  def maximum_unique_subarray(nums) do
    visits = %{}
    left = 0
    right = 0
    sum = 0
    result = 0

    solve(nums, nums, left, right, visits, sum, result)
  end


  def solve(
    [lnum | lremains] = lnums,
    [rnum | rremains] = rnums,
    left,
    right,
    visits,
    sum,
    result
  ) do
    if Map.has_key?(visits, rnum) do
      visited = Map.get(visits, rnum)
      if left <= visited do
        sum = sum - lnum
        left = left + 1
        solve(lremains, rnums, left, right, visits, sum, result)
      else
        visits = Map.delete(visits, rnum)
        solve(lnums, rnums, left, right, visits, sum, result)
      end
    else
      sum = sum + rnum
      result = max(result, sum)
      visits = Map.put(visits, rnum, right)
      right = right + 1
      solve(lnums, rremains, left, right, visits, sum, result)
    end
  end

  def solve(_, [], _,  _, _, _, result) do
    result
  end

  def solve([], _, _, _, _, _, result) do
    result
  end
end
