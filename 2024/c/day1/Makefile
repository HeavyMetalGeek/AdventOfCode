SRC_DIR = src
INCLUDE_DIR = include
BIN_DIR = bin
OBJ_DIR = obj

LIB_DIR = libs
LIB_SRC_DIR = $(LIB_DIR)/src
LIB_OBJ_DIR = $(LIB_DIR)/obj
LIB_BIN_DIR = $(LIB_DIR)/bin
LIB_INCLUDE_DIR = $(LIB_DIR)/include

SRC_P1_NAME = day1_part1
SRC_P1 = $(wildcard $(SRC_DIR)/$(SRC_P1_NAME).c)
OBJ_P1 = $(SRC_P1:$(SRC_DIR)/%.c=$(OBJ_DIR)/%.o)
TARGET_P1 = $(BIN_DIR)/part1

SRC_P2_NAME = day1_part2
SRC_P2 = $(wildcard $(SRC_DIR)/$(SRC_P2_NAME).c)
OBJ_P2 = $(SRC_P2:$(SRC_DIR)/%.c=$(OBJ_DIR)/%.o)
TARGET_P2 = $(BIN_DIR)/part2

SRC_LIBAOC = $(wildcard $(LIB_SRC_DIR)/*.c)
OBJ_LIBAOC = $(SRC_LIBAOC:$(LIB_SRC_DIR)/%.c=$(LIB_OBJ_DIR)/%.o)
TARGET_LIBAOC = $(LIB_BIN_DIR)/libaoc.a

.PHONY: run default clean

run: default
	@echo "---- RESULT PART 1----"
	./$(TARGET_P1)
	@echo "---- RESULT PART 2----"
	./$(TARGET_P2)

default: $(TARGET_P1) $(TARGET_P2)

clean:
	rm -f $(OBJ_DIR)/*.o
	rm -f $(BIN_DIR)/*
	rm -f $(LIB_OBJ_DIR)/*
	rm -f $(LIB_BIN_DIR)/*

$(TARGET_P1): $(OBJ_P1) $(TARGET_LIBAOC) | $(BIN_DIR)
	@echo "Compiling Part 1..."
	@echo "Object files: $(OBJ_P1)"
	gcc -o $@ $(OBJ_P1) -I$(INCLUDE_DIR) -L$(LIB_BIN_DIR) -laoc

$(TARGET_P2): $(OBJ_P2) $(TARGET_LIBAOC) | $(BIN_DIR)
	@echo "Compiling Part 2..."
	@echo "Object files: $(OBJ_P2)"
	gcc -o $@ $(OBJ_P2) -I$(INCLUDE_DIR) -L$(LIB_BIN_DIR) -laoc

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c | $(OBJ_DIR)
	@echo "Compiling $< to $@"
	gcc -c $< -o $@ -I$(INCLUDE_DIR)

$(TARGET_LIBAOC): $(OBJ_LIBAOC) | $(LIB_BIN_DIR)
	@echo "Building AOC lib: $@"
	@echo "Object files: $(OBJ_LIBAOC)"
	ar rcs $@ $^

$(LIB_OBJ_DIR)/%.o: $(LIB_SRC_DIR)/%.c | $(LIB_OBJ_DIR)
	@echo "Compiling lib file $< to $@"
	gcc -c $< -o $@ -I$(LIB_INCLUDE_DIR)

$(BIN_DIR) $(OBJ_DIR) $(LIB_BIN_DIR) $(LIB_OBJ_DIR):
	@echo "Creating directory $@"
	mkdir -p $@
