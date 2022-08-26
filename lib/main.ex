defmodule Solution do
  use Bitwise

  @spec reordered_power_of2(n :: integer) :: boolean
  def reordered_power_of2(n) do
    get_counts(n) |> loop(0)
  end

  @spec get_counts(n :: integer) :: [integer]
  def get_counts(n), do: get_counts(n, List.duplicate(0, 10))

  @spec get_counts(n :: integer, result :: [integer]) :: [integer]
  def get_counts(n, result) when n > 0 do
    idx = rem(n, 10)
    get_counts(div(n, 10), List.update_at(result, idx, &(&1 + 1)))
  end

  def get_counts(_, result), do: result

  @spec loop(counts :: [integer], i :: integer) :: boolean
  def loop(counts, i) when i < 31 do
    if counts == get_counts(1 <<< i) do
      true
    else
      loop(counts, i + 1)
    end
  end

  def loop(_, _), do: false
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        n: 1
      },
      %{
        n: 10
      },
      %{
        n: 46
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    n = input.n
    result = Solution.reordered_power_of2(n)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil
end

Main.main()
