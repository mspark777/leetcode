defmodule Solution do
  @spec jump(nums :: [integer]) :: integer
  def jump(nums), do: loop(nums, 0, 0, 0, 0)

  @spec loop(
          nums :: [integer],
          i :: integer,
          cufar :: integer,
          curend :: integer,
          result :: integer
        ) :: integer
  defp loop(nums, i, curfar, curend, result)

  defp loop([_ | []], _, _, _, result), do: result

  defp loop([num | nums], i, curfar, curend, result) do
    newfar = max(curfar, i + num)

    if i == curend do
      loop(nums, i + 1, newfar, newfar, result + 1)
    else
      loop(nums, i + 1, newfar, curend, result)
    end
  end
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([nums | remains]) do
    result = Solution.jump(nums)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      [2, 3, 1, 1, 4],
      [2, 3, 0, 1, 4]
    ])
  end
end

Main.main()
