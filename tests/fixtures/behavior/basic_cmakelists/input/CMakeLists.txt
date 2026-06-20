cmake_minimum_required(VERSION 3.24)
project(ExampleProject)

add_executable(example main.cpp util.cpp)
target_compile_definitions(example PRIVATE ENABLE_FEATURE=1 USE_FAST_PATH=1)
