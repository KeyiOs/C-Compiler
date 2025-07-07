#include <unistd.h>

int main() {
    char input; 
    char *even = "Even\n";
    char *odd = "Odd\n";

    if (read(0, &input, 1) == 1) {
        if (input >= '0' && input <= '9') {
            int value = input - '0';
            if (value % 2 == 0)
                write(1, even, 5);
            else
                write(1, odd, 4);
        }
    }
    
    return 0;
}
