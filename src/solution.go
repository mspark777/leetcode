package main

type pair struct {
	n int
	i int
}

func merge(pairs []pair, l int, mid int, r int, result []int) {
	i := l
	j := mid + 1
	k := 0
	temp := make([]pair, r-l+1)
	count := 0

	for (i <= mid) && (j <= r) {
		ip := &pairs[i]
		jp := &pairs[j]
		if ip.n <= jp.n {
			result[ip.i] += count
			temp[k] = *ip

			i += 1
			k += 1
		} else {
			count += 1
			temp[k] = *jp
			k += 1
			j += 1
		}
	}

	for i <= mid {
		p := &pairs[i]
		result[p.i] += count
		temp[k] = *p
		i += 1
		k += 1
	}

	for j <= r {
		temp[k] = pairs[j]
		k += 1
		j += 1
	}

	for i2, p := range temp {
		pairs[l+i2] = p
	}
}

func mergesort(pairs []pair, l int, r int, result []int) {
	if l >= r {
		return
	}

	mid := (l + r) / 2
	mergesort(pairs, l, mid, result)
	mergesort(pairs, mid+1, r, result)
	merge(pairs, l, mid, r, result)
}

func countSmaller(nums []int) []int {
	n := len(nums)
	result := make([]int, n)
	pairs := make([]pair, n)
	for i := 0; i < n; i += 1 {
		pairs[i] = pair{n: nums[i], i: i}
	}

	mergesort(pairs, 0, n-1, result)
	return result
}
