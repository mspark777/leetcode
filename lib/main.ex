defmodule Solution do
  @spec can_win_nim(n :: integer) :: boolean
  def can_win_nim(n), do: rem(n, 4) != 0
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([n | inputs]) do
    result = Solution.can_win_nim(n)

    IO.puts(result)
    main(inputs)
  end

  def main([]) do
  end

  def main do
    main([
      4,
      1,
      2
    ])
  end
end

Main.main()
