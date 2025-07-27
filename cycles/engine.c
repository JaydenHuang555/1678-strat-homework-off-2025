#include "stdio.h"
#include "bitdefs.h"
#include "cycles.h"


// Output projected # of cycles/game pieces possible or average cycle time


f64 average_cycle_time_seconds(struct input_t *input) {
    f64 played = MATCH_LEN_CAN_CYCLE - input->time_left; 
    return input->cycles / played;
}

