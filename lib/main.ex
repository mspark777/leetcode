defmodule Solution do
  @spec intersection(nums1 :: [integer], nums2 :: [integer]) :: [integer]
  def intersection(nums1, nums2) do
    MapSet.intersection(
      MapSet.new(nums1),
      MapSet.new(nums2)
    )
    |> MapSet.to_list()
  end
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([[nums1, nums2] | remains]) do
    result = Solution.intersection(nums1, nums2)

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
