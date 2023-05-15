package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func newTreeNode(val int, left, right *TreeNode) *TreeNode {
	return &TreeNode{Val: val, Left: left, Right: right}
}

func newTreeLeft(val int, left *TreeNode) *TreeNode {
	return newTreeNode(val, left, nil)
}

func newTreeRight(val int, right *TreeNode) *TreeNode {
	return newTreeNode(val, nil, right)
}

func newTreeVal(val int) *TreeNode {
	return newTreeNode(val, nil, nil)
}

type ListNode struct {
	Val  int
	Next *ListNode
}

func newListNode(val int, next *ListNode) *ListNode {
	return &ListNode{Val: val, Next: next}
}

func newList(vals []int) *ListNode {
	head := newListNode(0, nil)
	tail := head
	for _, val := range vals {
		node := newListNode(val, nil)
		tail.Next = node
		tail = node
	}

	return head.Next
}

func listToArr(node *ListNode) []int {
	nums := []int{}
	for node != nil {
		nums = append(nums, node.Val)
		node = node.Next
	}

	return nums
}

func newCycleList(vals []int, pos int) *ListNode {
	head := newListNode(0, nil)
	tail := head
	var target *ListNode = nil
	for i, val := range vals {
		node := newListNode(val, nil)
		if i == pos {
			target = node
		}

		tail.Next = node
		tail = node
	}

	tail.Next = target
	return head.Next
}

type TrieNode struct {
	links []*TrieNode
	ended bool
}

func TrieNodeConstructor() *TrieNode {
	node := new(TrieNode)
	node.links = make([]*TrieNode, 26)
	node.ended = false

	return node
}

func (this *TrieNode) ContainsKey(ch rune) bool {
	return this.Get(ch) != nil
}

func (this *TrieNode) Get(ch rune) *TrieNode {
	i := this.getIndex(ch)
	if this.inRange(i) {
		return this.links[i]
	}

	return nil
}

func (this *TrieNode) Put(ch rune, node *TrieNode) {
	i := this.getIndex(ch)
	if this.inRange(i) {
		this.links[i] = node
	}
}

func (this *TrieNode) SetEnd() {
	this.ended = true
}

func (this *TrieNode) IsEnd() bool {
	return this.ended
}

func (this *TrieNode) getIndex(ch rune) int {
	return int(ch - rune('a'))
}

func (this *TrieNode) inRange(i int) bool {
	return (i >= 0) && (i < len(this.links))
}

type Trie struct {
	root *TrieNode
}

func TrieConstructor() Trie {
	return Trie{root: TrieNodeConstructor()}
}

func (this *Trie) Insert(word string) {
	node := this.root
	for _, ch := range word {
		if !node.ContainsKey(ch) {
			node.Put(ch, TrieNodeConstructor())
		}

		node = node.Get(ch)
	}

	node.SetEnd()
}

func (this *Trie) Search(word string) bool {
	node := this.searchPrefix(word)
	return (node != nil) && node.IsEnd()
}

func (this *Trie) StartsWith(prefix string) bool {
	return this.searchPrefix(prefix) != nil
}

func (this *Trie) searchPrefix(prefix string) *TrieNode {
	node := this.root
	for _, ch := range prefix {
		node = node.Get(ch)
		if node == nil {
			return nil
		}
	}

	return node
}

type UnionFind struct {
	parents []int
	ranks   []int
}

func UnionFindConstructor(size int) *UnionFind {
	parents := make([]int, size)
	ranks := make([]int, size)

	for i := 0; i < size; i++ {
		parents[i] = i
	}

	return &UnionFind{parents, ranks}
}

func (this *UnionFind) Find(x int) int {
	if this.parents[x] != x {
		this.parents[x] = this.Find(this.parents[x])
	}

	return this.parents[x]
}

func (this *UnionFind) Union(x, y int) {
	xset := this.Find(x)
	yset := this.Find(y)
	if xset == yset {
		return
	}

	if this.ranks[xset] < this.ranks[yset] {
		this.parents[xset] = yset
	} else if this.ranks[xset] > this.ranks[yset] {
		this.parents[yset] = xset
	} else {
		this.parents[yset] = xset
		this.ranks[xset] += 1
	}
}
