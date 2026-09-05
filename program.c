int putchar(int c);

int factorial(int n) {
    if (n <= 1) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

int add(int a, int b) {
    return a + b;
}

int main(void) {
    int x = 5;
    int y;

    y = 3;

    int sum = add(x, y); // 8
    int diff = x - y;
    int prod = x * y;
    int quot = x / y;
    int rem = x % y;
    int neg = -x;
    int bit = ~x;

    int bigger = (x > y) ? x : y;
    int ok = (x == 5 && y == 3) || !(x != 5);

    {
        int x = 10;
        int inner = x + 1;
        sum = sum + inner; // 19
    }

    int i = 0;
    while (i < 3) {
        sum = sum + i;
        i = i + 1;
    } // sum = 22

    for (i = 0; i < 3; i = i + 1) {
        diff = diff + i;
    }

    do {
        prod = prod - 1;
    } while (prod > 0);

    if (ok) {
        sum = sum + factorial(4); // 46
    } else {
        sum = sum - 1;
    }

    putchar(104); // h
    putchar(101); // e
    putchar(108); // l
    putchar(108); // l
    putchar(111); // o
    putchar(10);  // newline

    return sum; // 46
}
