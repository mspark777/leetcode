defmodule NumArray do
  @spec init_(nums :: [integer]) :: any
  def init_(nums) do
    if :ets.info(:numarr) == :undefined do
      :ets.new(:numarr, [:named_table, :set])
    end
    narr = Build.build(nums)
    :ets.insert(:numarr, {:arr, narr})
  end

  @spec update(index :: integer, val :: integer) :: any
  def update(index, val) do
    [{_, narr}] = :ets.lookup(:numarr, :arr)
    narr = Update.update(index, val, narr)
    :ets.insert(:numarr, {:arr, narr})
  end

  @spec sum_range(left :: integer, right :: integer) :: integer
  def sum_range(left, right) do
    [{_, narr}] = :ets.lookup(:numarr, :arr)
    SumRange.sum_range(left, right, narr)
  end
end

defmodule Build do
  def build(nums) do
    len = length(nums)
    build1(nums, 0, %{0 => 0, -1 => len})
  end

  def build1([num | nums], i, result) do
    len = Map.get(result, -1)
    build1(nums, i + 1, Map.put(result,  i + len, num))
  end

  def build1([], _, result) do
    build2(result, Map.get(result, -1) - 1)
  end

  def build2(result, i) when i > 0 do
    result = Map.put(result, i,
      Map.get(result, i * 2) + Map.get(result, i * 2 + 1)
    )
    build2(result, i - 1)
  end

  def build2(result, _), do: result
end

defmodule Update do
  def update(index, val, result) do
    index = index + Map.get(result, -1)
    update(index, Map.put(result, index, val))
  end

  def update(index, result) when index > 0 and rem(index, 2) == 0 do
    left = index
    right = index + 1
    index = div(index, 2)
    update(index, Map.put(result, index,
      Map.get(result, left) + Map.get(result, right)
    ))
  end

  def update(index, result) when index > 0 do
    left = index - 1
    right = index
    index = div(index, 2)
    update(index, Map.put(result, index,
      Map.get(result, left) + Map.get(result, right)
    ))
  end

  def update(_, result), do: result
end

defmodule SumRange do
  def sum_range(left, right, result) do
    len = Map.get(result, -1)
    left = left + len
    right = right + len
    sum_range(left, right, 0, result)
  end

  def sum_range(left, right, sum, result) when left <= right  do
    {sum, left} = if rem(left, 2) == 1 do
      {sum + Map.get(result, left), left + 1}
    else
      {sum, left}
    end

    {sum, right} = if rem(right, 2) == 0 do
      {sum + Map.get(result, right), right - 1}
    else
      {sum, right}
    end

    left = div(left, 2)
    right = div(right, 2)
    sum_range(left, right, sum, result)
  end

  def sum_range(_, _, sum, _), do: sum
end
