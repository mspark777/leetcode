/**
 * // This is the Iterator's API interface.
 * // You should not implement it, or speculate about its implementation.
 * function Iterator() {
 *    @ return {number}
 *    this.next = function() { // return the next number of the iterator
 *       ...
 *    };
 *
 *    @return {boolean}
 *    this.hasNext = function() { // return true if it still has numbers
 *       ...
 *    };
 * };
 */

export class PeekingIterator {
  constructor (iterator) {
    this.iter = iterator
    this.canNext = this.iter.hasNext()
    this.current = this.iter.next()
  }

  peek () {
    return this.current
  }

  next () {
    const current = this.current
    this.canNext = this.iter.hasNext()
    this.current = this.iter.next()

    return current
  }

  hasNext () {
    return this.canNext
  }
}

/**
 * Your PeekingIterator object will be instantiated and called as such:
 * var obj = new PeekingIterator(arr)
 * var param_1 = obj.peek()
 * var param_2 = obj.next()
 * var param_3 = obj.hasNext()
 */
