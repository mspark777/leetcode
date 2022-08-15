defmodule Solution do
  @spec roman_to_int(s :: String.t()) :: integer
  def roman_to_int(s) do
    s |> String.to_charlist() |> Enum.reverse() |> roman_to_int(0)
  end

  @spec roman_to_int(s :: [char], result :: integer) :: integer
  def roman_to_int([ch | chars], result) do
    case ch do
      ?I -> roman_to_int(chars, calc(result, 1))
      ?V -> roman_to_int(chars, calc(result, 5))
      ?X -> roman_to_int(chars, calc(result, 10))
      ?L -> roman_to_int(chars, calc(result, 50))
      ?C -> roman_to_int(chars, calc(result, 100))
      ?D -> roman_to_int(chars, calc(result, 500))
      ?M -> roman_to_int(chars, calc(result, 1000))
      _ -> roman_to_int(chars, calc(result, 0))
    end
  end

  def roman_to_int([], result), do: result

  @spec calc(result :: integer, num :: integer) :: integer
  def calc(result, num) when num * 4 < result, do: result - num
  def calc(result, num), do: result + num
end

defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
        s: "III"
      },
      %{
        s: "LVIII"
      },
      %{
        s: "MCMXCIV"
      }
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    s = input.s
    result = Solution.roman_to_int(s)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()
