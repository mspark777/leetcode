defmodule Solution do
  @spec wiggle_max_length(nums :: [integer]) :: integer
  def wiggle_max_length(nums) do
    [prev | remains] = nums
    wiggle_max_length(prev, remains, 1, 1)
  end

  def wiggle_max_length(prev, [cur | nums], _, down) when cur > prev do
    wiggle_max_length(cur, nums, down + 1, down)
  end

  def wiggle_max_length(prev, [cur | nums], up, _) when cur < prev do
    wiggle_max_length(cur, nums, up, up + 1)
  end

  def wiggle_max_length(_, [cur | nums], up, down) do
    wiggle_max_length(cur, nums, up, down)
  end

  def wiggle_max_length(_, [], up, down), do: max(up, down)
end
