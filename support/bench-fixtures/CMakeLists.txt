cmake_minimum_required(VERSION 3.24)
project(DprintCMakeFmtBench LANGUAGES C CXX)

option(ENABLE_TOOLS "Build command line tools" ON)
option(ENABLE_TESTS "Build tests" ON)

add_library(bench_core STATIC src/config.cpp src/format.cpp src/parser.cpp src/spec.cpp)
target_include_directories(
  bench_core
  PUBLIC
    "$<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>"
    "$<INSTALL_INTERFACE:include>"
)
target_compile_features(bench_core PUBLIC cxx_std_20)
target_compile_definitions(
  bench_core
  PRIVATE
    BENCH_ENABLE_DIAGNOSTICS=1
    BENCH_DEFAULT_LINE_WIDTH=80
)

if(ENABLE_TOOLS)
  add_executable(bench_formatter src/main.cpp)
  target_link_libraries(bench_formatter PRIVATE bench_core Threads::Threads)
endif()

if(ENABLE_TESTS)
  enable_testing()
  add_executable(bench_tests tests/config_test.cpp tests/format_test.cpp tests/parser_test.cpp)
  target_link_libraries(bench_tests PRIVATE bench_core)
  add_test(NAME bench_tests COMMAND bench_tests)
endif()
