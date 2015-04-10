long long
__mulodi4(long long a, long long b, int* overflow)
{
    const int N = (int)(sizeof(long long) * 8);
    const long long MIN = (long long)1 << (N-1);
    const long long MAX = ~MIN;
    *overflow = 0; 
    long long result = a * b;
    if (a == MIN)
    {
        if (b != 0 && b != 1)
	    *overflow = 1;
	return result;
    }
    if (b == MIN)
    {
        if (a != 0 && a != 1)
	    *overflow = 1;
        return result;
    }
    long long sa = a >> (N - 1);
    long long abs_a = (a ^ sa) - sa;
    long long sb = b >> (N - 1);
    long long abs_b = (b ^ sb) - sb;
    if (abs_a < 2 || abs_b < 2)
        return result;
    if (sa == sb)
    {
        if (abs_a > MAX / abs_b)
            *overflow = 1;
    }
    else
    {
        if (abs_a > MIN / -abs_b)
            *overflow = 1;
    }
    return result;
}
