defmodule PriorityQueue do
  def dequeue([head | tail] = queue) do
    {m, i} = findmax(tail, head, 0, 1)
    {List.delete_at(queue, i), m}
  end

  def dequeue([]) do
    nil
  end

  def enqueue(queue, n) do
    [n | queue]
  end

  defp findmax([head | tail], m, mi, i) do
    if head > m do
      findmax(tail, head, i, i + 1)
    else
      findmax(tail, m, mi, i + 1)
    end
  end

  defp findmax([], m, mi, _) do
    {m, mi}
  end
end

defmodule Solution do
  @spec is_possible(target :: [integer]) :: boolean
  def is_possible(target) do
    {queue, sum} = enqueue_and_sum(target, [], 0)
    top = PriorityQueue.dequeue(queue)
    is_possible(top, sum)
  end

  def is_possible(nil, _) do
    true
  end

  def is_possible({_, top}, _) when top == 1 do
    true
  end

  def is_possible({_, top}, sum) when top <= (sum - top) or sum < top + 1 do
    false
  end

  def is_possible({queue, top}, sum)  do
    sum = sum - top
    top = rem(top, sum)
    sum = sum + top
    if top > 0 do
      is_possible(
        PriorityQueue.dequeue(
          PriorityQueue.enqueue(queue, top)
        ), sum)
    else
      is_possible(
        PriorityQueue.dequeue(
          PriorityQueue.enqueue(queue, sum)
        ), sum)
    end
  end


  def enqueue_and_sum([head | tail], queue, sum) do
    enqueue_and_sum(tail, PriorityQueue.enqueue(queue, head), sum + head)
  end

  def enqueue_and_sum([], queue, sum) do
    {queue, sum}
  end
end
