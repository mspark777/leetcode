# @param n [Integer]
# @param t0 [Integer]
# @param t1 [Integer]
# @param t2 [Integer]
# @return [Integer]
def recursive(n, t0, t1, t2)
  return t2 if n < 3

  recursive(n - 1, t1, t2, t0 + t1 + t2)
end

# @param n [Integer]
# @return [Integer]
def tribonacci(n)
  return 0 if n < 1
  return 1 if n < 3

  recursive(n, 0, 1, 1)
end

def main
  inputs = [
    4, 25
  ]

  inputs.each do |input|
    result = tribonacci(input)
    puts(result)
  end
end

main
