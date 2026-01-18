cmake_minimum_required(VERSION 3.22)
project(fog_cutter_lang)

set(CMAKE_CXX_FLAGS "-Wall -Wextra")
set(CMAKE_CXX_FLAGS_DEBUG "-g -O0")
set(CMAKE_CXX_FLAGS_RELEASE "-O3")

set(SOURCE_FILES
    c-src/core.c
    c-src/create.c
    c-src/free.c
    c-src/object.c
    )

set(HEADER_FILES
    c-src/headers/core.h
    c-src/headers/create.h
    c-src/headers/free.h
    c-src/headers/object.h
    )

#include_directories(
#    /c-src/headers
#)

add_executable(${PROJECT_NAME} ${SOURCE_FILES} ${HEADER_FILES})