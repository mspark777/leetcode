defmodule PriorityQueue do
  def dequeue([head | tail] = queue) do
    {m, i} = findmax(tail, head, 0, 1)
    {List.delete_at(queue, i), m}
  end

  def dequeue([]) do
    nil
  end

  def enqueue(queue, duration) do
    [duration | queue]
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
  @spec schedule_course(courses :: [[integer]]) :: integer
  def schedule_course(courses) do
    schedule_course2([], 0, Enum.sort(courses, fn [_, a], [_, b] -> b > a end))
  end

  defp schedule_course2(queue, time, [[duration, last] | courses]) do
    new_time = duration + time
    if new_time <= last do
      schedule_course2(
        PriorityQueue.enqueue(queue, duration),
        new_time,
        courses
      )
    else
      dequeued = PriorityQueue.dequeue(queue)
      if dequeued == nil do
        schedule_course2(queue, time, courses)
      else
        {newqueue, top} = dequeued
        if top > duration do
          schedule_course2(
            PriorityQueue.enqueue(newqueue, duration),
            time + duration - top,
            courses
          )
        else
          schedule_course2(queue, time, courses)
        end
      end
    end
  end

  defp schedule_course2(queue, _, []) do
    Enum.count(queue)
  end
end
