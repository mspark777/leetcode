defmodule Solution do
  @spec reverse_string(s :: [String.t()]) :: [String.t()]
  def reverse_string(s), do: Enum.reverse(s)
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([n | inputs]) do
    result = Solution.reverse_string(n)

    IO.puts(result)
    main(inputs)
  end

  def main([]) do
  end

  def main do
    main([
      ["h", "e", "l", "l", "o"],
      ["H", "a", "n", "n", "a", "h"]
    ])
  end
end

Main.main()
