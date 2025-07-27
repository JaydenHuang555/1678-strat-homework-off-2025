#ifndef CYCLES_H
#define CYCLES_H

#include "bitdefs.h"

struct input_t {
    f64 time_left;  
    f64 cycles;
    u32 scored;
};

#define MATCH_LEN_SEC 120
#define MATCH_AUTO_LEN 15

#define MATCH_LEN_CAN_CYCLE (MATCH_LEN_SEC - MATCH_AUTO_LEN)

#endif
