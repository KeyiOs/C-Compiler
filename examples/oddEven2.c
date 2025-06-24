int check_even(int value) {
    if (value % 2 == 0) {
        return 1;
    } else {
        return 0;
    }
}

int main() {
    int i;
    int result;

    for (i = 0; i < 10; i = i + 1) {
        result = check_even(i);
    }

    char *string = "ab\0s";

    return 0;
}
