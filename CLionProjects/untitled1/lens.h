//
// Created by zongtong on 2020/10/21.
//

int len_str(char *p) {
    int len = 0;
    if ('\0' == 0) { printf("yes ,\'\\0\' and 0 are equal.\n"); }
    while (*p++ != '\0') {
//    while (*p++ != 0) {this line and previous line is equal.
        len++;

    }
    return len;
}