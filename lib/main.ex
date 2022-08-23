defmodule Solution do
  @spec summary_ranges(nums :: [integer]) :: [String.t()]
  def summary_ranges(nums), do: map_result(nums, []) |> Enum.reverse()

  @spec map_result(nums :: [integer], result :: [String.t()]) :: [String.t()]
  defp map_result([head | nums], result) do
    {tail, newnums} = get_tail(nums, head)

    if head == tail do
      map_result(newnums, ["#{head}" | result])
    else
      map_result(newnums, ["#{head}->#{tail}" | result])
    end
  end

  defp map_result([], result), do: result

  @spec get_tail(nums :: [integer], head :: integer) :: {integer, [integer]}
  defp get_tail([num | _] = nums, head) when num != head + 1, do: {head, nums}
  defp get_tail([num | nums], _), do: get_tail(nums, num)
  defp get_tail([], head), do: {head, []}
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        nums: [0, 1, 2, 4, 5, 7]
      },
      %{
        nums: [0, 2, 3, 4, 6, 8, 9]
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    nums = input.nums
    result = Solution.summary_ranges(nums)
    IO.puts(result |> Enum.join(", "))
    main(remains)
  end

  def main([]), do: nil
end

Main.main()
