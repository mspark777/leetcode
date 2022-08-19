defmodule Solution do
  @type mymap :: %{optional(integer) => integer}
  @spec is_possible(nums :: [integer]) :: boolean
  def is_possible(nums) do
    lefts = Enum.frequencies(nums)
    rights = %{}
    loop(nums, lefts, rights)
  end

  @spec loop(nums :: [integer], lefts :: mymap, rights :: mymap) :: boolean
  defp loop([cur | nums], lefts, rights) do
    if get_value(lefts, cur) == 0 do
      loop(nums, lefts, rights)
    else
      new_lefts = inc_values(lefts, [{cur, -1}])
      before1 = cur - 1

      if get_value(rights, before1) > 0 do
        loop(nums, new_lefts, inc_values(rights, [{before1, -1}, {cur, 1}]))
      else
        after1 = cur + 1
        after2 = cur + 2

        if get_value(new_lefts, after1) > 0 and get_value(new_lefts, after2) > 0 do
          loop(
            nums,
            inc_values(new_lefts, [{after1, -1}, {after2, -1}]),
            inc_values(rights, [{after2, 1}])
          )
        else
          false
        end
      end
    end
  end

  defp loop([], _, _), do: true

  @spec get_value(m :: mymap, k :: integer) :: integer
  defp get_value(m, k), do: Map.get(m, k, 0)

  @spec inc_values(m :: mymap, keys :: [{integer, integer}]) :: mymap
  defp inc_values(m, [{k, v} | keys]) do
    value = get_value(m, k)
    inc_values(Map.put(m, k, value + v), keys)
  end

  defp inc_values(m, []), do: m
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        nums: [1, 2, 3, 3, 4, 5]
      },
      %{
        nums: [1, 2, 3, 3, 4, 4, 5, 5]
      },
      %{
        nums: [1, 2, 3, 4, 4, 5]
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    nums = input.nums
    result = Solution.is_possible(nums)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil
end

Main.main()
