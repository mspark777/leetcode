defmodule Solution do
  @spec is_power_of_three(n :: integer) :: boolean
  def is_power_of_three(n) when n <= 0, do: false
  def is_power_of_three(n) when n == 1, do: true
  def is_power_of_three(n) when rem(n, 3) == 0, do: div(n, 3) |> is_power_of_three()
  def is_power_of_three(_), do: false
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        n: 27
      },
      %{
        n: 0
      },
      %{
        n: 9
      },
      %{
        n: 45
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([%{:n => n} | remains]) do
    result = Solution.is_power_of_three(n)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil
end

Main.main()
