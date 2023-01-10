# @param n [Integer]
# @param memo [Array<Integer>]
# @return [Integer]
def solve(n, memo)
  return 0 if n == 0
  return 1 if n == 1
  return memo[n] if memo[n] != 0

  memo[n] = if (n & 1) == 1
              solve(n / 2, memo) + 1
            else
              solve(n / 2, memo)
            end

  memo[n]
end

# @param n [Integer]
# @return [Array<Integer>]
def count_bits(n)
  result = Array.new(n + 1) { 0 }

  for i in 1..n
    result[i] = solve(i, result)
  end

  result
end

def main
  inputs = [
    2, 5
  ]

  inputs.each do |n|
    result = count_bits n
    puts result.join ', '
  end
end

main
