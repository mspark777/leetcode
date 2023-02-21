defmodule Solution do
  @spec single_non_duplicate(nums :: [integer]) :: integer
  def single_non_duplicate(nums) do
    tnums = List.to_tuple(nums)
    bsearch(tnums, 0, tuple_size(tnums) - 1)
  end

  @spec bsearch(nums :: {integer}, left :: integer, right :: integer) :: integer
  defp bsearch(nums, left, right) when left < right do
    middle = div(left + right, 2)

    if rem(middle, 2) == 1 do
      if elem(nums, middle) != elem(nums, middle + 1) do
        bsearch(nums, middle + 1, right)
      else
        bsearch(nums, left, middle)
      end
    else
      if elem(nums, middle) == elem(nums, middle + 1) do
        bsearch(nums, middle + 1, right)
      else
        bsearch(nums, left, middle)
      end
    end
  end

  defp bsearch(nums, left, _), do: elem(nums, left)
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([nums | remains]) do
    result = Solution.single_non_duplicate(nums)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      [1, 1, 2, 3, 3, 4, 4, 8, 8],
      [3, 3, 7, 7, 10, 11, 11]
    ])
  end
end

Main.main()
