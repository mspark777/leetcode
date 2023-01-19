defmodule Solution do
  @spec subarrays_div_by_k(nums :: [integer], k :: integer) :: integer
  def subarrays_div_by_k(nums, k) do
    loop(nums, k, %{0 => 1}, 0, 0)
  end

  defp loop([], _, _, _, result), do: result

  defp loop([num | nums], k, mod_groups, prefix, result) do
    prefix = (prefix + k + rem(num, k)) |> rem(k)
    count = Map.get(mod_groups, prefix, 0)

    loop(nums, k, Map.put(mod_groups, prefix, count + 1), prefix, result + count)
  end
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([%{nums: nums, k: k} | remains]) do
    result = Solution.subarrays_div_by_k(nums, k)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      %{nums: [4, 5, 0, -2, -3, 1], k: 5},
      %{nums: [5], k: 9}
    ])
  end
end

Main.main()
