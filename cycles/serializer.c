#include "stdio.h"
#include "ioutils.h"
#include "bitdefs.h"
#include "cjson/cJSON.h"
#include "cycles.h"
#include "stdlib.h"
#include "stdarg.h"

static cJSON *check(cJSON *data, const char *target, const char *message) {
    if(!data || !target || !message) {
        eprintln("stuff was null");
        return 0;
    }
    cJSON *out = cJSON_GetObjectItem(data, target);
    if(!out) {
        eprintf("%s", message);
    }
    return out;
}

static s8 validate_type_as_double(size_t argc, ...) {
    va_list ap;
    s8 ret = 1;
    va_start(ap, argc);
    cJSON *next;
    for(size_t i = 0; i < argc; i++) 
        if(!cJSON_IsNumber((next = va_arg(ap, cJSON*)))) {
            ret = 0;
            eprintln("%s value is not a number", next->string);
            break;
        } 
    va_end(ap);
    return ret;
}

struct input_t *serialize_input(struct input_t *input, const char *contents) {
    if(!contents || !input) {
        eprintln("%s was null", !input ? "input" : "file contents");
        return 0; 
    }

    cJSON *data = cJSON_Parse(contents); 
    if(!data) {
        eprintln("unable to parse json");
        return 0;
    }
    cJSON *time_left, *cycles, *scored;
    
    time_left = check(data, "time left", "unable to get info for time left\nplease include the \' \"time left\" : 0.0 \' (ur value as an floating 64 bit decimal)\n");
    cycles = check(data, "cycles", "unable to get info for cycles\nplease include the \' \"cycles\" : 0.0 \' (ur value as an floating 64 bit decimal)\n");
    scored =  check(data, "scored", "unable to get info for scored\nplease include the \' \"scored\" : 0.0 \' (ur value as an floating 64 bit decimal)\n");

    if(!time_left || !cycles || !scored) {
        cJSON_Delete(data);
        return 0;
    } 

    if(!validate_type_as_double(3, time_left, cycles, scored)) {
        cJSON_Delete(data);
        return 0;
    }

    input->time_left = time_left->valuedouble * 60; 
    input->cycles = cycles->valuedouble;
    input->scored = scored->valuedouble;

    cJSON_Delete(data);
    return input; 
}
