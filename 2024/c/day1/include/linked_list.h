#ifndef LINKED_LIST_H
#define LINKED_LIST_H

#include <stdio.h>
#include <stdlib.h>

typedef struct ValNode ValNode;

struct ValNode {
    int value;
    ValNode *next;
};

typedef struct {
    ValNode *head;
} ValList;

ValNode* ValNode_init();
void ValNode_free(ValNode *node);

ValList ValList_init();
void ValList_free(ValList *list);
void ValList_prepend(ValList *list, int val);
void ValList_append(ValList *list, int val);
void ValList_print(ValList *list);
void ValList_insertSorted(ValList* list, int val);

#endif /* LINKED_LIST_H */
