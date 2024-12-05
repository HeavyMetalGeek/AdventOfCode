#include "linked_list.h"
#include "buffer.h"

typedef struct {
    void* data;
    unsigned int length;
    unsigned int capacity;
} Buffer;

int Buffer_init(Buffer** out_buf, size_t unit_size) {
    (*out_buf) = malloc(sizeof(Buffer));
    if (out_buf == NULL) {
        return -1;
    }
    (*out_buf)->length = 0;
    (*out_buf)->capacity = 255;
    (*out_buf)->data = calloc((*out_buf)->capacity, unit_size);
    if ((*out_buf)->data == NULL) {
        return -1;
    }
    return 0;
}

int Buffer_init_int(Buffer** out_buf) {
    (*out_buf) = malloc(sizeof(Buffer));
    if ((*out_buf) == NULL) {
        return -1;
    }
    (*out_buf)->length = 0;
    (*out_buf)->capacity = 255;
    (*out_buf)->data = calloc((*out_buf)->capacity, sizeof(int));
    if ((*out_buf)->data == NULL) {
        return -1;
    }
    return 0;
}

int Buffer_init_char(Buffer** out_buf) {
    (*out_buf) = malloc(sizeof(Buffer));
    if ((*out_buf) == NULL) {
        return -1;
    }
    (*out_buf)->length = 0;
    (*out_buf)->capacity = 255;
    (*out_buf)->data = calloc((*out_buf)->capacity, sizeof(char));
    if ((*out_buf)->data == NULL) {
        return -1;
    }
    return 0;
}

void Buffer_free(Buffer** buf) {
    free((*buf)->data);
    (*buf)->data = NULL;
    free(*buf);
    *buf = NULL;
}

int Buffer_append_int(Buffer *buf, int val) {
    if (buf->length == buf->capacity) {
        buf->capacity *= 2;
        buf->data = (int*)realloc(buf->data, sizeof(int) * buf->capacity);
        if (buf->data == NULL) {
            printf("ERROR: Unable to reallocate memory.");
            return -1;
        }
    }
    if (buf->data == NULL) {
        printf("Data was NULL.");
        return -1;
    }
    ((int*)buf->data)[buf->length] = val;
    buf->length += 1;
    return 0;
}

int Buffer_append_char(Buffer *buf, char val) {
    if (buf->length == buf->capacity) {
        buf->capacity *= 2;
        buf->data = (char*)realloc(buf->data, sizeof(char) * buf->capacity);
        if (buf->data == NULL) {
            printf("ERROR: Unable to reallocate memory.");
            return -1;
        }
    }
    if (buf->data == NULL) {
        printf("Data was NULL.");
        return -1;
    }
    ((char*)buf->data)[buf->length] = val;
    buf->length += 1;
    return 0;
}

int Buffer_append_long(Buffer *buf, long val) {
    if (buf->length == buf->capacity) {
        buf->capacity *= 2;
        buf->data = (void*)realloc(buf->data, sizeof(long) * buf->capacity);
        if (buf->data == NULL) {
            printf("ERROR: Unable to reallocate memory.");
            return -1;
        }
    }
    if (buf->data == NULL) {
        printf("Data was NULL.");
        return -1;
    }
    ((long*)buf->data)[buf->length] = val;
    buf->length += 1;
    return 0;
}


ValBuffer ValBuffer_init() {
    ValBuffer buf;
    buf.length = 0;
    buf.capacity = 255;
    buf.data = (int*)calloc(buf.capacity, sizeof(int));
    return buf;
}

int ValBuffer_append(ValBuffer *buf, int val) {
    if (buf->length == buf->capacity) {
        buf->capacity *= 2;
        buf->data = (int*)realloc(buf->data, sizeof(int) * buf->capacity);
        if (buf->data == NULL) {
            printf("ERROR: Unable to reallocate memory.");
            return -1;
        }
    }
    if (buf->data == NULL) {
        printf("Data was NULL.");
        return -1;
    }
    buf->data[buf->length] = val;
    buf->length += 1;
    return 0;
}

void ValBuffer_free(ValBuffer *buf) {
    free(buf->data);
    buf->data = NULL;
}

int ValBuffer_sort(ValBuffer *buf) {
    ValList list = ValList_init();
    for (int i = 0; i < buf->length; ++i) {
        ValList_insertSorted(&list, buf->data[i]);
    }
    ValNode* current = list.head;
    if (current == NULL) {
        printf("Buffer is empty.\n");
        return 0;
    }
    for (int i = 0; i < buf->length; ++i) {
        if (current == NULL) {
            printf("Found NULL node before reaching end of buffer.\n");
            return -1;
        }
        buf->data[i] = current->value;
        current = current->next;
    }
    ValNode_free(list.head);
    return 0;
}

CharBuffer CharBuffer_init() {
    CharBuffer buf;
    buf.length = 0;
    buf.capacity = 255;
    buf.data = (char*)calloc(buf.capacity, sizeof(char));
    return buf;
}

int CharBuffer_append(CharBuffer *buf, char c) {
    if (buf->length == buf->capacity) {
        buf->capacity *= 2;
        buf->data = (char*)realloc(buf->data, buf->capacity);
        if (buf->data == NULL) {
            printf("ERROR: Unable to reallocate memory.");
            return -1;
        }
    }
    if (buf->data == NULL) {
        printf("Data was NULL.");
        return -1;
    }
    buf->data[buf->length] = c;
    buf->length += 1;
    return 0;
}

void CharBuffer_reset(CharBuffer *buf) { buf->length = 0; }

void CharBuffer_free(CharBuffer *buf) {
    free(buf->data);
    buf->data = NULL;
}

int CharBuffer_popInt(CharBuffer *buf) {
    int val = atoi(buf->data);
    CharBuffer_reset(buf);
    return val;
}
