#define static_cast(t, v) ((t)v)

int get_num_of_bouguests(int *days, int size, int mid, int k)
{
	int num_of_bouquests = 0;
	int count = 0;

	for (int i = 0; i < size; i += 1) {
		int day = days[i];
		if (day <= mid) {
			count += 1;
		} else {
			count = 0;
		}

		if (count == k) {
			num_of_bouquests += 1;
			count = 0;
		}
	}

	return num_of_bouquests;
}

int max(int *days, int size)
{
	int m = 0;
	for (int i = 0; i < size; i += 1) {
		if (days[i] > m) {
			m = days[i];
		}
	}

	return m;
}

int check(int size, int m, int k)
{
	long long lsize = size;
	long long lm = m;
	long long lk = k;

	return (lm * lk) > lsize;
}

int minDays(int *days, int size, int m, int k)
{
	if (check(size, m, k)) {
		return -1;
	}

	int start = 0;
	int end = max(days, size);
	int result = -1;
	while (start <= end) {
		int mid = (start + end) / 2;
		if (get_num_of_bouguests(days, size, mid, k) >= m) {
			result = mid;
			end = mid - 1;
		} else {
			start = mid + 1;
		}
	}

	return result;
}
