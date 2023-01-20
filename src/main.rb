# @param nums [Array<Integer>]
# @param index [Integer]
# @param sequence [Array<Integer>]
# @param result [Hash<String, Array<Integer>>]
def backtrack(nums, index, sequence, result)
  if index == nums.length
    if sequence.length >= 2
      key = sequence.join(', ')
      result[key] = sequence.clone
    end
  else
    num = nums[index]
    lastseq = num
    lastseq = sequence[-1] unless sequence.empty?

    if lastseq <= num
      sequence.push(num)
      backtrack(nums, index + 1, sequence, result)
      sequence.pop
    end
    backtrack(nums, index + 1, sequence, result)
  end
end

# @param nums [Array<Integer>]
# @return [Array<Array<Integer>>]
def find_subsequences(nums)
  result = {}
  backtrack(nums, 0, [], result)
  result.values
end

def main
  inputs = [
    [4, 6, 7, 7],
    [4, 4, 3, 2, 1]
  ]

  inputs.each do |nums|
    result = find_subsequences(nums)
    puts result.join(', ')
  end
end

main
