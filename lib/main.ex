defmodule Solution do
  @spec detect_capital_use(word :: String.t()) :: boolean
  def detect_capital_use(word) do
    cond do
      String.upcase(word) == word ->
        true

      String.downcase(word) == word ->
        true

      true ->
        {first, remains} = {
          word |> String.slice(0..0) |> String.upcase(),
          word |> String.slice(1..-1) |> String.downcase()
        }

        word == first <> remains
    end
  end
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([word | inputs]) do
    result = Solution.detect_capital_use(word)

    IO.puts(result)
    main(inputs)
  end

  def main([]) do
  end

  def main do
    main([
      "USA",
      "Google",
      "leetcode",
      "FlaG"
    ])
  end
end

Main.main()
