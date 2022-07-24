defmodule Solution do
  @spec contains_duplicate(nums :: [integer]) :: boolean
  def contains_duplicate(nums) do
    contains_duplicate(MapSet.new(), nums)
  end

  @spec contains_duplicate(set :: MapSet.t(integer), nums :: [integer]) :: boolean
  def contains_duplicate(set, [num | nums]) do
    if MapSet.member?(set, num) do
      true
    else
      contains_duplicate(MapSet.put(set, num), nums)
    end
  end

  def contains_duplicate(_, []) do
    false
  end
end
