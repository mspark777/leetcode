defmodule Solution do
  @spec find_substring(s :: String.t, words :: [String.t]) :: [integer]
  def find_substring(s, words) do
    words_len = length(words)
    word_len = words |> Enum.at(0) |> String.length()
    window_size = words_len * word_len
    slen = String.length(s)
    word_count = create_word_counts(%{}, words)

    loop0(s, 0, slen - window_size, slen, word_count, word_len, []) |> Enum.reverse()
  end

  defp create_word_counts(word_count, [word | words]) do
    cnt = Map.get(word_count, word, 0)
    create_word_counts(Map.put(word_count, word, cnt + 1), words)
  end

  defp create_word_counts(word_count, []), do: word_count

  defp loop0(s, i, last_window_index, slen, word_count, word_len, result) when i <= last_window_index do
    loop0(s, i + 1, last_window_index, slen, word_count, word_len,
      loop1(s, i, i, slen, Map.merge(%{}, word_count), word_len, result)
    )
  end

  defp loop0(_, _, _, _, _, _, result), do: result

  defp loop1(s, j, i, slen, word_count, word_len, result) when j < slen do
    substr = String.slice(s, j..j + word_len - 1)
    cnt = Map.get(word_count, substr, 0)
    cond do
      cnt == 0 -> loop1(s, slen, i, slen, word_count, word_len, result)
      cnt == 1 -> loop1(s, j + word_len, i, slen, Map.delete(word_count, substr), word_len, result)
      true  -> loop1(s, j + word_len, i, slen, Map.put(word_count, substr, cnt - 1), word_len, result)
    end
  end

  defp loop1(_, _, i, _, word_count, _, result) do
    case map_size(word_count) do
      0 -> [i | result]
      _ -> result
    end
  end

end

defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
        s: "barfoothefoobarman",
        words: ["foo", "bar"]
      },
      %{
        s: "wordgoodgoodgoodbestword",
        words: ["word", "good", "best", "word"]
      },
      %{
        s: "barfoofoobarthefoobarman",
        words: ["bar", "foo", "the"]
      },
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    s = input.s
    words = input.words
    result = Solution.find_substring(s, words)
    IO.puts(result |> Enum.join(", "))
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()
