#include <stdio.h>

int addSame(int a) {
    return a + a;
}

int add(int a, int b) {
    return a + b;
}

int main() {
    printf("Hello, Web!\n");

    int a;
    int sum = 0;
    /* for loop execution */
    for( a = 0; a < 20; a = a + 1 ){
       sum = sum + a;
    }

    addSame(sum);

    add(1, 2);

    return 0;
}
