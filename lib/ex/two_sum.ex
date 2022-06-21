defmodule TwoSum do
  @spec two_sum(numbers :: [integer], target :: integer) :: [integer]
  def two_sum(numbers, target) do
    rnumbers = Enum.reverse(numbers)
    solve(numbers, rnumbers, target, 0, length(numbers) - 1)
  end


  @spec solve(numbers :: [integer], rnumbers :: [integer], target :: integer, i :: integer, j :: integer) :: [integer]
  defp solve([ni | numbers], [nj | rnumbers], target, i, j) when i < j do
    ns = ni + nj
    cond do
      ns < target -> solve(numbers, [nj | rnumbers], target, i + 1, j)
      ns > target -> solve([ni | numbers], rnumbers, target, i, j - 1)
      true -> [i + 1, j + 1]
    end
  end
end

defmodule TwoSum1 do
  @spec two_sum(numbers :: [integer], target :: integer) :: [integer]
  def two_sum(numbers, target) do
    rnumbers = Enum.reverse(numbers)
    solve(numbers, rnumbers, target, 0, length(numbers) - 1)
  end


  @spec solve(numbers :: [integer], rnumbers :: [integer], target :: integer, i :: integer, j :: integer) :: [integer]
  defp solve([ni | numbers], [nj | rnumbers], target, i, j) when i < j do
    ns = ni + nj
    cond do
      ns < target -> solve(numbers, [nj | rnumbers], target, i + 1, j)
      ns > target -> solve([ni | numbers], rnumbers, target, i, j - 1)
      true -> [i + 1, j + 1]
    end
  end

  defp solve(_, _, _, i, j) do
    [i, j]
  end
end
