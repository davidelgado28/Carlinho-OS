cmake_minimum_required(VERSION 3.15)
project(CarlinhosOS CXX)

set(CMAKE_CXX_STANDARD 17)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/lang/include)
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/win32)
add_library(win32_compat STATIC win32/win32_compat.cpp)
add_subdirectory(shell)

message(STATUS "[Carlinhos OS] Configuração do monorepo concluída com sucesso.")
