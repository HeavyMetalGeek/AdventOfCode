cmake_minimum_required(VERSION 3.16)
project(day1 LANGUAGES C)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(
    CMAKE_RUNTIME_OUTPUT_DIRECTORY ${PROJECT_SOURCE_DIR}/bin/
)
include_directories(
	include
    libs/include
	/usr/include/
)

file(GLOB PART1_SOURCES ${CMAKE_SOURCE_DIR} "src/*_part1.c")
file(GLOB PART2_SOURCES ${CMAKE_SOURCE_DIR} "src/*_part2.c")
file(GLOB LIB_SOURCES ${CMAKE_SOURCE_DIR} "libs/src/*.c")

add_library(aoc STATIC ${LIB_SOURCES})
target_include_directories(aoc PUBLIC libs/include)

add_executable(part1
  ${PART1_SOURCES}
)

target_link_libraries(part1
  ${LIBS}
  pthread
  aoc
 )

add_executable(part2
  ${PART2_SOURCES}
)

target_link_libraries(part2
  ${LIBS}
  pthread
  aoc
 )
