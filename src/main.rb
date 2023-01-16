# @param intervals [Array<Array<Integer>>]
# @param new_interval [Array<Integer>]
# @return [Array<Array<Integer>>]
def insert(intervals, new_interval)
  pos = [*intervals.each_index].bsearch { |i| intervals[i][0] > new_interval[0] }
  if pos.nil?
    intervals.push new_interval
  else
    intervals.insert pos, new_interval
  end

  result = [intervals[0]]

  (1..(intervals.length - 1)).each do |i|
    last = result.last
    interval = intervals[i]
    if interval[0] > last[1]
      result.push interval
    else
      last[1] = [last[1], interval[1]].max
    end
  end

  result
end

def main
  inputs = [
    { intervals: [[1, 3], [6, 9]], newInterval: [2, 5] },
    { intervals: [[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]], newInterval: [4, 8] },
    { intervals: [], newInterval: [5, 7] }
  ]

  inputs.each do |input|
    result = insert input[:intervals], input[:newInterval]
    puts result.join ', '
  end
end

main
