defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
          words1: ["amazon", "apple", "facebook", "google", "leetcode"],
          words2: ["e", "o"],
      },
      %{
          words1: ["amazon", "apple", "facebook", "google", "leetcode"],
          words2: ["l", "e"],
      }
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    result = Solution.word_subsets(input.words1, input.words2)
    IO.puts(result |> Enum.join(", "))
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()
