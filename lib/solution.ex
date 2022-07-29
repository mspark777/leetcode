defmodule Solution do
  @spec find_and_replace_pattern(words :: [String.t], pattern :: String.t) :: [String.t]
  def find_and_replace_pattern(words, pattern) do
    find_and_replace_pattern(words, String.to_charlist(pattern), [])
  end

  @spec find_and_replace_pattern(
    words :: [String.t],
    pattern :: charlist,
    result :: [String.t]
  ) :: [String.t]
  defp find_and_replace_pattern([word | words], pattern, result) do
    if find_pattern(String.to_charlist(word), pattern, %{}, %{}) do
      find_and_replace_pattern(words, pattern, [word | result])
    else
      find_and_replace_pattern(words, pattern, result)
    end
  end

  defp find_and_replace_pattern([], _, result), do: result

  @spec find_pattern(
    words :: charlist,
    pattern :: charlist,
    wmap :: %{char => char},
    pmap :: %{char => char}
  ) :: boolean
  defp find_pattern([wc | word], [pc | pattern], wmap, pmap) do
    wmap = updatemap(wmap, wc, pc)
    pmap = updatemap(pmap, pc, wc)

    check = Map.get(wmap, wc) != pc or Map.get(pmap, pc) != wc
    if check do
      false
    else
      find_pattern(word, pattern, wmap, pmap)
    end
  end

  defp find_pattern([], [], _, _), do: true
  defp find_pattern([], [_pc | _pattern], _, _), do: false
  defp find_pattern([_wc | _word], [], _, _), do: false

  @spec updatemap(
    m :: %{char => char},
    k :: char,
    v :: char
  ) :: %{char => char}
  defp updatemap(m, k, v) do
    if Map.has_key?(m, k) do
      m
    else
      Map.put(m, k, v)
    end
  end
end
