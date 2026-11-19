cmake_minimum_required(VERSION 3.20)
project(rei LANGUAGES C)
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
add_library(rei_kernel OBJECT
  src/globals.c
  src/core/native-types.c
  src/native/types.c
  src/native/abi.c
  src/core/rv64-branch.c
  src/core/rv64-numbers.c
  src/vm/rv64-immediate.c
  src/core/rv64-arithmetic.c
  src/core/rv64-word.c
  src/core/rv64-immediate-op.c
  src/core/rv64-register.c
  src/core/rv64-fclass.c
  src/native/memory.c
  src/native/word.c
  src/native/vfs.c
  src/native/kernel.c
  src/native/start.c
  src/native/thx.c
)
target_include_directories(rei_kernel PUBLIC include)
target_compile_options(rei_kernel PRIVATE -ffreestanding -fno-builtin -fno-stack-protector)
