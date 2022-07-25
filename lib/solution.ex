defmodule Solution do
  @spec search_range(nums :: [integer], target :: integer) :: [integer]
  def search_range(nums, target) do
    tnums = List.to_tuple(nums)
    first = search(tnums, target, true)
    last = search(tnums, target, false)
    [first, last]
  end

  @spec search(nums :: {integer}, target :: integer, first :: boolean) :: integer
  def search(nums, target, first) do
    left = 0
    right = tuple_size(nums)  - 1
    search(nums, target, first, left, right, -1)
  end

  @spec search(
    nums :: {integer},
    target :: integer,
    first :: boolean,
    left :: integer,
    right :: integer,
    result :: integer
  ) :: integer
  def search(nums, target, first, left, right, result) when left <= right do
    mid = div(left + right , 2)
    num = elem(nums, mid)
    cond do
      num > target -> search(nums, target, first, left, mid - 1, result)
      num < target -> search(nums, target, first, mid + 1, right, result)
      first == false -> search(nums, target, first, mid + 1, right, mid)
      true -> search(nums, target, first, left, mid - 1, mid)
    end
  end

  def search(_, _, _, _, _, result) do
    result
  end
end
