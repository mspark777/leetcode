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

# @param list [Array<Integer>]
# @param root [TreeNode]
def travel(list, root)
  return unless root

  list.push root.val
  travel list, root.left
  travel list, root.right
end

# @param root [TreeNode]
# @return [Array<Integer>]
def preorder_traversal(root)
  result = []
  travel(result, root)
  result
end

def main
  inputs = [
    newright(1, newleft(2, newval(3))),
    nil,
    newval(1)
  ]

  inputs.each do |root|
    result = preorder_traversal root
    puts result.join ', '
  end
end

main
