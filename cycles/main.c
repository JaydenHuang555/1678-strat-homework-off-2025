#include "stdio.h"
#include "ioutils.h"
#include "bitdefs.h"
#include "cycles.h"
#include "stdlib.h"
#include "engine.h"
#include "serializer.h"

s32 main(s32 argc, s8 **argv) {
    u32 failed = 0;
    // iterate through args, 0 is the executing command used (ex: ./a.out) 
    for(s32 i = 1; i < argc; i++) {
        s8 *arg = argv[i], *contents;
        // open stream for reading
        FILE *stream = fopen(arg, "r");
        if(!stream) {
            eprintln("unable to open file %s\n", arg);
            failed++;
            continue;
        }
        // reads the contents from the stream
        if(!(contents = read_stream_with_comments(stream))) {
            eprintln("unable to copy contents from %s to an buffer", arg);
            fclose(stream);
            failed++;
            continue;
        }
        println("%s\n", contents);

        struct input_t input;
        // serializes the contents from the jsonc file into inputs
        if(!serialize_input(&input, contents)) {
            fclose(stream);
            free(contents);
            failed++;
            continue;
        } 

        println("able to read input");

        println("time left %f\n", input.time_left);
        println("cycles %f\n", input.cycles);
        println("scored %d\n", input.scored);

        println("averge cycle time %f per second\n", average_cycle_time_seconds(&input));

        free(contents);
        fclose(stream);
    }
    return (s32)failed;
}
