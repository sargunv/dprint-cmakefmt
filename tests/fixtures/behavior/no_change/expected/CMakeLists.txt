cmake_minimum_required(VERSION 3.24)
project(NoChange)

add_library(no_change STATIC src/no_change.cpp)
target_include_directories(no_change PUBLIC include)
