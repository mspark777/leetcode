defmodule Solution do
  @spec search(nums :: [integer], target :: integer) :: integer
  def search(nums, target) do
    nums
    |> Enum.find_index(&(&1 == target))
    |> to_result()
  end

  defp to_result(nil), do: -1
  defp to_result(i), do: i
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([%{:nums => nums, :target => target} | remains]) do
    result = Solution.search(nums, target)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      %{nums: [4, 5, 6, 7, 0, 1, 2], target: 0},
      %{nums: [4, 5, 6, 7, 0, 1, 2], target: 3},
      %{nums: [1], target: 0}
    ])
  end
end

Main.main()
