defmodule MinimumLengthEncoding do
  @spec minimum_length_encoding(words :: [String.t]) :: integer
  def minimum_length_encoding(words) do
    MapSet.new()
    |> put_words(words)
    |> remove_subwords(words)
    |> MapSet.to_list
    |> Enum.reduce(0, fn s, acc ->
      acc + String.length(s) + 1
    end)
  end

  def remove_subwords(filter, [word | words]) do
    chars = String.to_charlist(word)
    filter = remove_subword(filter, chars)
    remove_subwords(filter, words)
  end

  def remove_subwords(filter, []) do
    filter
  end


  def remove_subword(filter, [_ | chars]) do
    str = to_string(chars)
    filter = MapSet.delete(filter, str)
    remove_subword(filter, chars)
  end

  def remove_subword(filter, []) do
    filter
  end

  def put_words(filter, [word | words]) do
    MapSet.put(filter, word) |> put_words(words)
  end

  def put_words(filter, []) do
    filter
  end

end
