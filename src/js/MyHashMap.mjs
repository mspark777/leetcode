export class MyHashMap {
  constructor () {
    this.root = {}
  }

  put (key, value) {
    this.root[key] = value
  }

  get (key) {
    return this.root[key] ?? -1
  }

  remove (key) {
    delete this.root[key]
  }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * var obj = new MyHashMap()
 * obj.put(key,value)
 * var param_2 = obj.get(key)
 * obj.remove(key)
 */
