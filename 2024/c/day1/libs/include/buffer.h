#ifndef BUFFER_H
#define BUFFER_H

#include <stdlib.h>
#include <stdio.h>
#include "linked_list.h"

typedef struct {
    int *data;
    unsigned int length;
    unsigned int capacity;
} ValBuffer;

ValBuffer ValBuffer_init();
int ValBuffer_append(ValBuffer *buf, int val);
void ValBuffer_free(ValBuffer *buf);
int ValBuffer_sort(ValBuffer *buf);

typedef struct {
    char *data;
    unsigned int length;
    unsigned int capacity;
} CharBuffer;

CharBuffer CharBuffer_init();
void CharBuffer_free(CharBuffer *buf);
void CharBuffer_reset(CharBuffer *buf);
int CharBuffer_append(CharBuffer *buf, char c);
int CharBuffer_popInt(CharBuffer *buf);

#endif /* BUFFER_H */
