defmodule Solution do
  @spec kth_smallest(matrix :: [[integer]], k :: integer) :: integer
  def kth_smallest(_matrix, k) do
    k
    # It's not good question for elixir...
  end
end

defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
        matrix: [[1, 5, 9], [10, 11, 13], [12, 13, 15]],
        k: 8
      },
      %{
        matrix: [[-5]],
        k: 1
      },
      %{
        matrix: [[-5, -4], [-5, -4]],
        k: 2
      },
      %{
        matrix: [[1, 2], [1, 3]],
        k: 1
      }
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    matrix = input.matrix
    k = input.k
    result = Solution.kth_smallest(matrix, k)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()
