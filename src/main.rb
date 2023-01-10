class TreeNode
  attr_accessor :val, :left, :right

  def initialize(val = 0, left = nil, right = nil)
    @val = val
    @left = left
    @right = right
  end
end

def newnode(val, left, right)
  TreeNode.new(val, left, right)
end

def newleft(val, left)
  newnode(val, left, nil)
end

def newright(val, right)
  newnode(val, nil, right)
end

def newval(val)
  newnode(val, nil, nil)
end

# @param p [TreeNode]
# @param q [TreeNode]
# @return [Boolean]
def is_same_tree(p, q)
  return true if p.nil? && q.nil?
  return false if p.nil? || q.nil?
  return false if p.val != q.val

  is_same_tree(p.left, q.left) && is_same_tree(p.right, q.right)
end

def main
  inputs = [
    [
      newnode(1, newval(2), newval(3)),
      newnode(1, newval(2), newval(3))
    ],
    [
      newleft(1, newval(2)),
      newright(1, newval(2))
    ],
    [
      newnode(1, newval(2), newval(1)),
      newnode(1, newval(1), newval(2))
    ]
  ]

  inputs.each do |input|
    result = is_same_tree input[0], input[1]
    puts result
  end
end

main
