# Lower versions may work, but I'm not going to confirm that right now.
cmake_minimum_required(VERSION 3.23)

project(libthreadpool LANGUAGES C)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(cargo_command cargo build --target-dir="${CMAKE_CURRENT_BINARY_DIR}")
else ()
    set(cargo_command cargo build --release --target-dir="${CMAKE_CURRENT_BINARY_DIR}")
endif ()

message("Current binary dir: ${CMAKE_CURRENT_BINARY_DIR}")

add_custom_target(libthreadpool ALL
    COMMENT "Using cargo to compile libthreadpool"
    COMMAND ${cargo_command}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(libthreadpool PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})

enable_testing()
# Always show the command line output from running tests
set(CMAKE_CTEST_ARGUMENTS "--verbose")

add_test(NAME test_libthreadpool
    COMMAND cargo test
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})

