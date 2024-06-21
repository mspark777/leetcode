#define static_cast(t, v) ((t)v)

int max(int left, int right)
{
	return left > right ? left : right;
}

int maxSatisfied(int *customers, int customers_size, int *grumpy,
		 int grumpy_size, int minutes)
{
	customers_size = grumpy_size;

	int unrealized_customers = 0;

	for (int i = 0; i < minutes; i += 1) {
		unrealized_customers += customers[i] * grumpy[i];
	}

	int max_unrealized_customers = unrealized_customers;
	for (int i = minutes; i < customers_size; i += 1) {
		unrealized_customers += customers[i] * grumpy[i];
		unrealized_customers -=
			customers[i - minutes] * grumpy[i - minutes];

		max_unrealized_customers =
			max(max_unrealized_customers, unrealized_customers);
	}

	int result = max_unrealized_customers;
	for (int i = 0; i < customers_size; i += 1) {
		result += customers[i] * (1 - grumpy[i]);
	}

	return result;
}
