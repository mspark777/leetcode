defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
        words:  ["abc", "deq", "mee", "aqq", "dkd", "ccc"],
        pattern: "abb"
      },
      %{
        words: ["a", "b", "c"],
        pattern: "a"
      }
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    result = Solution.find_and_replace_pattern(input.words, input.pattern)
    IO.puts(result |> Enum.join(", "))
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()
