# @param tasks [Array<Integer>]
# @return [Integer]
def minimum_rounds(tasks)
  counts = Hash.new 0
  tasks.each do |task|
    counts[task] += 1
  end

  result = 0

  counts.each_value do |count|
    return -1 if count == 1

    result += count / 3
    result += 1 unless (count % 3) == 0
  end

  result
end

def main
  inputs = [
    [2, 2, 3, 3, 2, 4, 4, 4, 4, 4],
    [2, 3, 3]
  ]

  inputs.each do |tasks|
    result = minimum_rounds tasks
    puts result
  end
end

main
