defmodule Solution do
  @spec intersect(nums1 :: [integer], nums2 :: [integer]) :: [integer]
  def intersect(nums1, nums2) do
    loop(nums1, Enum.frequencies(nums2), [])
  end

  def loop([], _, result), do: result

  def loop([num | nums], counts, result) do
    count = Map.get(counts, num, 0)

    if count > 0 do
      loop(nums, Map.put(counts, num, count - 1), [num | result])
    else
      loop(nums, counts, result)
    end
  end
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([[nums1, nums2] | remains]) do
    result = Solution.intersect(nums1, nums2)

    IO.puts(result |> Enum.join(", "))
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      [[1, 2, 2, 1], [2, 2]],
      [[4, 9, 5], [9, 4, 9, 8, 4]]
    ])
  end
end

Main.main()
