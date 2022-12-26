defmodule Solution do
  @spec can_jump(nums :: [integer]) :: boolean
  def can_jump(nums) do
    len = length(nums)
    last = len - 1
    [_ | remains] = Enum.reverse(nums)
    can_jump(last, len - 2, remains)
  end

  def can_jump(last, i, [ni | remains]) when i >= 0 do
    cur = i + ni

    if cur >= last do
      can_jump(i, i - 1, remains)
    else
      can_jump(last, i - 1, remains)
    end
  end

  def can_jump(last, _, _), do: last < 1
end

defmodule Leetcode do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([input | inputs]) do
    result = Solution.can_jump(input)
    IO.puts(result)
    main(inputs)
  end

  def main([]) do
  end

  def main do
    main([
      [1],
      [2, 3, 1, 1, 4],
      [3, 2, 1, 0, 4]
    ])
  end
end

Leetcode.main()
