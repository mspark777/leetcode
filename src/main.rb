# @param bits [Array<Integer>]
# @param age [Integer]
# @return [Integer]
def query(bits, age)
  max_score = -(2**31)
  i = age
  while i > 0
    max_score = [max_score, bits[i]].max
    i -= i & (-i)
  end

  max_score
end

# @param bits [Array<Integer>]
# @param age [Integer]
# @param best [Integer]
# @return [Void]
def update(bits, age, best)
  i = age
  while i < bits.length
    bits[i] = [bits[i], best].max
    i += i & (-i)
  end
end

# @param scores [Array<Integer>]
# @param ages [Array<Integer>]
# @return [Integer]
def best_team_score(scores, ages)
  pairs = scores.zip(ages).sort

  highest_age = 0
  ages.each do |age|
    highest_age = [highest_age, age].max
  end

  bits = Array.new(highest_age + 1) { 0 }
  result = -(2**31)

  pairs.each do |pair|
    score = pair[0]
    age = pair[1]

    best = score + query(bits, age)
    update(bits, age, best)

    result = [result, best].max
  end

  result
end

def main
  inputs = [
    [[1, 3, 5, 10, 15], [1, 2, 3, 4, 5]],
    [[4, 5, 6, 5], [2, 1, 2, 1]],
    [[1, 2, 3, 5], [8, 9, 10, 1]]
  ]

  inputs.each do |input|
    result = best_team_score(input[0], input[1])
    puts(result)
  end
end

main
