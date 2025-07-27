#include "bitdefs.h"
#include "stdio.h"
#include "ioutils.h"

#ifndef EOF
#define EOF -1 /* for linux */
#endif

extern void *malloc(size_t);
extern void free(void*);

extern void *memset(void*, s32, size_t);

#define COMMENT_FLAG '/'

s8 *read_stream_with_comments(FILE *stream) {
    size_t offset = 0, cap = 32;
    s8 *buff = (s8*)malloc(cap); 
    if(!buff) {
        eprintln("unable to allocate memory to heap");
        return 0;
    }
    memset(buff, 0, cap);
    s8 c = 0, putback = 0;
    while((c = getc(stream)) != EOF) {
        if(c == COMMENT_FLAG && putback == COMMENT_FLAG) {
            buff[--offset]  = 0;
            while((c = getc(stream)) != '\n');
        }
        buff[offset++] = c;
        if(offset == cap) {
            s8 *next = (s8*)malloc((cap *=  2));
            if(!next) {
                eprintln("unable to allocate replacement memory to heap");
                free(buff);
                return 0;
            }
            for(size_t i = 0; i < cap; i++) next[i] = i < offset ? buff[i] : 0;
            free(buff);
            buff = next;
        }
        putback = c;
    }
    return buff;
}
