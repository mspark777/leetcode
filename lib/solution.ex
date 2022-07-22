defmodule Solution do
  @spec candy(ratings :: [integer]) :: integer
  def candy(ratings) do
    [prev | remains] = ratings
    candy(prev, remains, 0, 0, 0, 0)
  end

  @spec candy(prev :: integer, ratings :: [integer], up :: integer, down :: integer, old :: integer, candies :: integer) :: integer
  def candy(prev, [cur | ratings], up, down, old, candies) when old > 0 and prev == cur do
    candy(cur, ratings, 0, 0, 0, inc_candies(candies, up, down) + 1)
  end

  def candy(prev, [cur | ratings], up, down, old, candies) when old < 0 and cur > prev do
    candy(cur, ratings, 1, 0, 1, inc_candies(candies, up, down))
  end

  def candy(prev, [cur | ratings], up, down, old, candies) when old < 0 and cur == prev do
    candy(cur, ratings, 0, 0, 0, inc_candies(candies, up, down) + 1)
  end

  def candy(prev, [cur | ratings], up, down, _old, candies) do
    cond do
      cur > prev -> candy(cur, ratings, up + 1, down, 1, candies)
      cur < prev -> candy(cur, ratings, up, down + 1, -1, candies)
      true -> candy(cur, ratings, up, down, 0, candies + 1)
    end
  end

  def candy(_prev, [], up, down, _old, candies), do: inc_candies(candies, up, down) + 1

  @spec inc_candies(candies :: integer, up :: integer, down :: integer) :: integer
  def inc_candies(candies, up, down), do: candies + count(up) + count(down) + max(up, down)

  @spec calc_slop(cur :: integer, prev :: integer) :: integer
  def calc_slop(cur, prev) do
    cond do
      cur > prev -> 1
      cur < prev -> -1
      true -> 0
    end
  end

  @spec count(n :: integer) :: integer
  def count(n), do: div(n * (n + 1), 2)
end
