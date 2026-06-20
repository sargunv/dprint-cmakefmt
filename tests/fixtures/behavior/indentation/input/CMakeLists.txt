cmake_minimum_required(VERSION 3.24)
project(Indentation)

if(BUILD_TESTING)
add_executable(indentation_test tests/indentation_test.cpp)
target_compile_definitions(indentation_test PRIVATE ENABLE_TESTS=1)
endif()
