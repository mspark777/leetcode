defmodule Solution do
  @spec combination_sum4(nums :: [integer], target :: integer) :: integer
  def combination_sum4(nums, target) do
    %{0 => 1}
    |> loop0(nums, 1, target)
    |> Map.get(target, 0)
  end

  @spec loop0(
    result :: %{integer => integer},
    nums :: [integer],
    i :: integer,
    target :: integer
  ) :: %{integer => integer}
  defp loop0(result, nums, i, target) when i <= target do
    loop0(loop1(result, nums, i), nums, i + 1, target)
  end

  defp loop0(result, _, _, _), do: result

  @spec loop1(
    result :: %{integer => integer},
    nums :: [integer],
    i :: integer
  ) :: %{integer => integer}
  defp loop1(result, [num | nums], i) when i >= num do
    com = Map.get(result, i - num, 0) + Map.get(result, i, 0)
    loop1(Map.put(result, i, com), nums, i)
  end

  defp loop1(result, [_ | nums], i), do: loop1(result, nums, i)
  defp loop1(result, [], _), do: result
end

defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
        nums: [1, 2, 3],
        target: 4
      },
      %{
        nums: [9],
        target: 3
      },
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    nums = input.nums
    target = input.target
    result = Solution.combination_sum4(nums, target)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()
