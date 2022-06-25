defmodule Solution do
  @spec check_possibility(nums :: [integer]) :: boolean
  def check_possibility(nums) do
    [prev | remains] = nums
    check_possibility(prev, remains, 0, 1)
  end

  def check_possibility(prev, [cur | remains], count, i) do
    if cur < prev do
      check_possibility(prev, cur, remains, count + 1, i + 1)
    else
      check_possibility(prev, cur, remains, count, i + 1)
    end
  end

  def check_possibility(_, [], _, _) do
    true
  end

  def check_possibility(_, _, _, count, _) when count > 1 do
    false
  end

  def check_possibility(_, prev, [cur | remains], count, i) when cur >= prev do
      check_possibility(prev, cur, remains, count, i + 1)
  end

  def check_possibility(pprev, _, [cur | remains], count, i) when pprev <= cur do
    check_possibility(pprev, cur, remains, count + 1, i + 1)
  end


  def check_possibility(_, prev, [_ | remains], count, i) do
    check_possibility(prev, prev, remains, count + 1, i + 1)
  end


  def check_possibility(_, _, [], count, _) do
    count < 2
  end
end
