defmodule Solution do
  @morses {".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
           "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
           "-.--", "--.."}

  @spec unique_morse_representations(words :: [String.t()]) :: integer
  def unique_morse_representations(words) do
    words
    |> Enum.map(&String.to_charlist/1)
    |> Enum.map(&to_morse(&1, []))
    |> MapSet.new()
    |> MapSet.size()
  end

  @spec to_morse(word :: [char], codes :: [String.t()]) :: String.t()
  defp to_morse([ch | word], codes) do
    code = elem(@morses, ch - ?a)
    to_morse(word, [code | codes])
  end

  defp to_morse([], codes), do: codes |> Enum.reverse() |> Enum.join("")
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        words: ["gin", "zen", "gig", "msg"]
      },
      %{
        words: ["a"]
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    words = input.words
    result = Solution.unique_morse_representations(words)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil
end

Main.main()
