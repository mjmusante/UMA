#include <unistd.h>

void print_num(int val)
{
        char num[32];
        int top = 0;

        if (val < 0)
                return;

        num[top++] = (val % 10) + '0';

        while ((val /= 10) > 0)
                num[top++] = (val % 10) + '0';

        while (top >= 0)
                write(1, &num[--top], 1);
}

int main()
{
        print_num(0);
        write(1, "\n", 1);
        print_num(3);
        write(1, "\n", 1);
        print_num(42);
        write(1, "\n", 1);
        print_num(256);
        write(1, "\n", 1);
        print_num(1234);
        write(1, "\n", 1);
        print_num(49494);
        write(1, "\n", 1);
        print_num(150629);
        write(1, "\n", 1);
        print_num(9182734);
        write(1, "\n", 1);
}
