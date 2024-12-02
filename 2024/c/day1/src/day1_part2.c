#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "linked_list.h"
#include "buffer.h"

int main(int argc, char **argv) {
    FILE *input_file;
    CharBuffer char_buffer = CharBuffer_init();
    ValBuffer col1_buffer = ValBuffer_init();
    ValBuffer col2_buffer = ValBuffer_init();

    input_file = fopen("day1_part1.txt", "r");
    if (input_file == NULL) {
        printf("Failed to open file.\n");
        return -1;
    }

    char c = '\0';
    bool end_of_val = false;
    while ((c = fgetc(input_file)) != EOF) {
        // End of the first column value
        if (c == ' ') {
            // Skip extra spaces
            if (end_of_val) continue;
            // Null terminate the series of char values
            if (CharBuffer_append(&char_buffer, '\0') != 0) {
                // Append failure
                return -1;
            }
            int val = CharBuffer_popInt(&char_buffer);
            ValBuffer_append(&col1_buffer, val);
            end_of_val = true;
            continue;
        }
        end_of_val = false;
        if (c == '\r') {
            // Probably carriage return...we don't want to store this
            continue;
        }
        // End of the second column value
        if (c == '\n') {
            if (CharBuffer_append(&char_buffer, '\0') != 0) {
                // Append failure
                return -1;
            }
            int val = CharBuffer_popInt(&char_buffer);
            ValBuffer_append(&col2_buffer, val);
            continue;
        }
        CharBuffer_append(&char_buffer, c);
    }

    if (col1_buffer.length != col2_buffer.length) {
        printf(
            "Unbalanced columns: [%d] - [%d]\n",
            col1_buffer.length,
            col2_buffer.length);
        return -1;
    }

    int sum = 0;
    for (int i = 0; i < col1_buffer.length; ++i) {
        int value = col1_buffer.data[i];
        unsigned int instances = 0;
        for (int j = 0; j < col2_buffer.length; ++j) {
            if (col2_buffer.data[j] == value) {
                instances++;
            }
        }
        sum += value * instances;
    }

    printf("Similarity Score: %d\n", sum);

    // Clean up
    fclose(input_file);
    CharBuffer_free(&char_buffer);
    ValBuffer_free(&col1_buffer);
    ValBuffer_free(&col2_buffer);

    return 0;
}
