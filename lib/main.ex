defmodule Solution do
  @spec search_insert(nums :: [integer], target :: integer) :: integer
  def search_insert(nums, target) do
    tnums = List.to_tuple(nums)
    right = tuple_size(tnums)

    cond do
      target < elem(tnums, 0) -> 0
      target > elem(tnums, right - 1) -> right
      true -> bsearch(tnums, target, 0, right)
    end
  end

  @spec bsearch(nums :: {integer}, target :: integer, left :: integer, right :: integer) ::
          integer
  defp bsearch(nums, target, left, right) when left < right do
    middle = div(left + right, 2)
    num = elem(nums, middle)

    cond do
      num < target -> bsearch(nums, target, middle + 1, right)
      num > target -> bsearch(nums, target, left, middle)
      true -> middle
    end
  end

  defp bsearch(_, _, left, _), do: left
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([input | remains]) do
    result = Solution.search_insert(input.nums, input.target)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      %{nums: [1, 3, 5, 6], target: 5},
      %{nums: [1, 3, 5, 6], target: 2},
      %{nums: [1, 3, 5, 6], target: 7}
    ])
  end
end

Main.main()
