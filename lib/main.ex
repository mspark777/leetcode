defmodule Solution do
  @spec contains_nearby_duplicate(nums :: [integer], k :: integer) :: boolean
  def contains_nearby_duplicate(nums, k) do
    nums |> Enum.with_index() |> loop(k, %{})
  end

  @spec loop(
          nums :: [{integer, integer}],
          k :: integer,
          index_map :: %{optional(integer) => integer}
        ) :: boolean
  defp loop([{num, i} | remains] = nums, k, index_map) do
    case Map.get(index_map, num) do
      nil -> loop(remains, k, Map.put(index_map, num, i))
      idx -> loop2(nums, idx, k, index_map)
    end
  end

  defp loop([], _, _), do: false

  defp loop2([{_, i} | _], idx, k, _) when i - idx <= k, do: true

  defp loop2([{num, i} | nums], _, k, index_map) do
    loop(nums, k, Map.put(index_map, num, i))
  end
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        nums: [1, 2, 3, 1],
        k: 3
      },
      %{
        nums: [1, 0, 1, 1],
        k: 1
      },
      %{
        nums: [1, 2, 3, 1, 2, 3],
        k: 2
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    nums = input.nums
    k = input.k
    result = Solution.contains_nearby_duplicate(nums, k)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil
end

Main.main()
