defmodule Solution do
  @spec number_of_weak_characters(properties :: [[integer]]) :: integer
  def number_of_weak_characters(properties) do
    properties
    |> Enum.sort(&cmp/2)
    |> solve(0, 0)
  end

  @spec cmp(a :: [integer], b :: [integer]) :: boolean
  defp cmp([attack_a, defense_a], [attack_b, defense_b])
       when attack_a == attack_b,
       do: defense_a < defense_b

  defp cmp([attack_a, _defence_a], [attack_b, _defence_b]),
    do: attack_b < attack_a

  @spec solve(properties :: [[integer]], max_defence :: integer, result :: integer) :: integer
  defp solve([[_attack, defense] | properties], max_defense, result)
       when max_defense > defense,
       do: solve(properties, max_defense, result + 1)

  defp solve([[_attack, defense] | properties], _max_defense, result),
    do: solve(properties, defense, result)

  defp solve([], _max_defense, result), do: result
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        properties: [[5, 5], [6, 3], [3, 6]]
      },
      %{
        properties: [[2, 2], [3, 3]]
      },
      %{
        properties: [[1, 5], [10, 4], [4, 3]]
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    properties = input.properties
    result = Solution.number_of_weak_characters(properties)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil
end

Main.main()
