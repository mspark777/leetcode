defmodule Solution do
  @spec is_anagram(s :: String.t, t :: String.t) :: boolean
  def is_anagram(s, t) do
    counter = s |> String.to_charlist |> create_counter(%{})
    t |> String.to_charlist |> remove_count(counter)
  end

  def create_counter([ch | s], counter) do
    count = Map.get(counter, ch, 0)
    create_counter(s, Map.put(counter, ch, count + 1))
  end

  def create_counter([], counter), do: counter

  def remove_count([ch | s], counter) do
    count = Map.get(counter, ch)
    cond do
      count == nil -> false
      count == 1 -> remove_count(s, Map.delete(counter, ch))
      true -> remove_count(s, Map.put(counter, ch, count - 1))
    end
  end

  def remove_count([], counter), do: map_size(counter) < 1
end
