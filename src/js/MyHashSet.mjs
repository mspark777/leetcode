export class MyHashSet {
  constructor () {
    this.data = {}
  }

  add (key) {
    this.data[key] = true
  }

  remove (key) {
    delete this.data[key]
  }

  contains (key) {
    return !!this.data[key]
  }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * var obj = new MyHashSet()
 * obj.add(key)
 * obj.remove(key)
 * var param_3 = obj.contains(key)
 */
