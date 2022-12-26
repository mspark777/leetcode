defmodule Solution do
  @spec word_pattern(pattern :: String.t(), s :: String.t()) :: boolean
  def word_pattern(pattern, s) do
    words = String.split(s, " ")
    patterns = String.to_charlist(pattern)

    word_pattern(patterns, words, %{}, %{})
  end

  def word_pattern([], [_ | _], _, _), do: false
  def word_pattern([_ | _], [], _, _), do: false
  def word_pattern([], [], _, _), do: true

  def word_pattern([pattern | patterns], [word | words], ptow, wtop) do
    pw = Map.get(ptow, pattern, word)
    wp = Map.get(wtop, word, pattern)

    case {pw == word, wp == pattern} do
      {true, true} ->
        word_pattern(patterns, words, Map.put(ptow, pattern, word), Map.put(wtop, word, pattern))

      _ ->
        false
    end
  end
end
