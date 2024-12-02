#include "linked_list.h"

ValNode* ValNode_init(int val) {
    ValNode *new_node = (ValNode*)malloc(sizeof(ValNode));
    new_node->value = val;
    new_node->next = NULL;
    return new_node;
}

void ValNode_free(ValNode *node) {
    if (node->next != NULL) {
        ValNode_free(node->next);
    }
    free(node);
    node = NULL;
}

ValList ValList_init() {
    ValList list;
    list.head = NULL;
    return list;
}

void ValList_free(ValList *list) {
    ValNode_free(list->head);
    free(list);
    list = NULL;
}

void ValList_insert(ValNode* current_node, ValNode* insert_node) {
    insert_node->next = current_node->next;
    current_node->next = insert_node;
}

void ValList_prepend(ValList *list, int val) {
    ValNode *new_node = ValNode_init(val);
    if (list->head == NULL) {
        list->head = new_node;
        return;
    }
    new_node->next = list->head;
    list->head = new_node;
}

void ValList_append(ValList *list, int val) {
    ValNode *new_node = ValNode_init(val);
    if (list->head == NULL) {
        list->head = new_node;
        return;
    }
    ValNode *node = list->head;
    while (node->next != NULL) {
        node = node->next;
    }
    node->next = new_node;
}

void ValList_print(ValList *list) {
    ValNode *node = list->head;
    while (node != NULL) {
        printf("Node Value: %d\n", node->value);
        node = node->next;
    }
}

void ValList_insertSorted(ValList* list, int val) {
    ValNode *new_node = (ValNode*)malloc(sizeof(ValNode));
    new_node->value = val;
    new_node->next = NULL;
    if (list->head == NULL) {
        list->head = new_node;
        return;
    }
    ValNode *current = list->head;
    if (val < current->value) {
        ValList_prepend(list, val);
        return;
    }
    while (current->next != NULL && current->next->value < val) {
        current = current->next;
    }
    ValList_insert(current, new_node);
}
