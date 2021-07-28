//
// Created by zongtong on 2020/10/21.
//

#ifndef UNTITLED1_MYSLEEP_H
#define UNTITLED1_MYSLEEP_H

#endif //UNTITLED1_MYSLEEP_H

#include <time.h>

int sleep(unsigned long x) {
    clock_t c1 = clock(), c2;
    do {
        if ((c2 = clock()) == (clock_t) -1) {
            printf("0   ");
            return 0;
        }   /* 错误 */

    } while (1000.0 * (c2 - c1) / CLOCKS_PER_SEC < x);
    return 1;
}