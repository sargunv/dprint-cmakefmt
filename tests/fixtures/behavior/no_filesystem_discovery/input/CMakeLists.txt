cmake_minimum_required(VERSION 3.24)
project(NoFilesystemDiscovery)

add_executable(discovery_probe src/main.cpp src/extra.cpp src/platform.cpp)
