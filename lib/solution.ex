defmodule Solution do
  @spec min_moves2(nums :: [integer]) :: integer
  def min_moves2(nums) do
    nums_len = Enum.count(nums)
    mid_index  = div(nums_len, 2)
    sorted = Enum.sort(nums)
    mid = Enum.at(sorted, mid_index)
    Enum.reduce(sorted, 0, &(&2 + abs(mid - &1)))
  end
end
