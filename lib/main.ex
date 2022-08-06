defmodule Solution do
  @spec poor_pigs(buckets :: integer, minutes_to_die :: integer, minutes_to_test :: integer) :: integer
  def poor_pigs(buckets, minutes_to_die, minutes_to_test) do
    result = :math.ceil(:math.log(buckets) / :math.log(minutes_to_test / minutes_to_die + 1))
    Kernel.trunc(result)
  end
end

defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
        buckets: 1000,
        minutes_to_die: 15,
        minutes_to_test: 60,
      },
      %{
        buckets: 4,
        minutes_to_die: 15,
        minutes_to_test: 15,
      },
      %{
        buckets: 4,
        minutes_to_die: 15,
        minutes_to_test: 30,
      },
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    buckets = input.buckets;
    minutes_to_die = input.minutes_to_die;
    minutes_to_test = input.minutes_to_test;
    result = Solution.poor_pigs(buckets, minutes_to_die, minutes_to_test);
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()
