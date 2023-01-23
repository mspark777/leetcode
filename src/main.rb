# @param n [Integer] n
# @param trust [Array<Array<Integer>>]
# @return [Integer]
def find_judge(n, trust)
  counts = Array.new(n) { 0 }
  trust.each do |info|
    counts[info[0] - 1] -= 1
    counts[info[1] - 1] += 1
  end

  judge = n - 1
  counts.each_with_index do |count, person|
    return person + 1 if count == judge
  end

  -1
end

def main
  inputs = [
    { n: 2, trust: [[1, 2]] },
    { n: 3, trust: [[1, 3], [2, 3]] },
    { n: 3, trust: [[1, 3], [2, 3], [3, 1]] },
    { n: 1, trust: [] }
  ]

  inputs.each do |input|
    result = find_judge(input[:n], input[:trust])
    puts(result)
  end
end

main
