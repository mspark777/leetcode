defmodule Solution do
  @spec min_refuel_stops(target :: integer, start_fuel :: integer, stations :: [[integer]]) ::
          integer
  def min_refuel_stops(target, start_fuel, stations) do
    (stations ++ [[target, 0]])
    |> Enum.reduce_while({0, start_fuel, :gb_sets.new()}, &reduce(&1, &2))
    |> elem(0)
  end

  defp reduce(station, {result, tank, queue}), do: refuel(tank, station, queue, result)

  defp refuel(tank, [pos, fuel], queue, result) when tank >= pos do
    {:cont, {result, tank, :gb_sets.add({fuel, pos}, queue)}}
  end

  defp refuel(tank, [pos, fuel], queue, result) do
    if :gb_sets.is_empty(queue) do
      {:halt, {-1}}
    else
      {{f, _}, queue} = :gb_sets.take_largest(queue)
      refuel(tank + f, [pos, fuel], queue, result + 1)
    end
  end
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        target: 1,
        start_fuel: 1,
        stations: []
      },
      %{
        target: 100,
        start_fuel: 1,
        stations: [[10, 100]]
      },
      %{
        target: 100,
        start_fuel: 10,
        stations: [[10, 60], [20, 30], [30, 30], [60, 40]]
      },
      %{
        target: 100,
        start_fuel: 50,
        stations: [[50, 50]]
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    target = input.target
    start_fuel = input.start_fuel
    stations = input.stations
    result = Solution.min_refuel_stops(target, start_fuel, stations)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil
end

Main.main()
