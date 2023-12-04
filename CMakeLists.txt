#  Source: https://github.com/paandahl/cpp-with-rust
cmake_minimum_required(VERSION 3.13)
project(calc-rust)

set(CMAKE_CXX_STANDARD 17)

set(CARGO_MANIFEST ${CMAKE_SOURCE_DIR}/Cargo.toml)
set(CARGO_TARGET_DIR ${CMAKE_SOURCE_DIR}/target)

set(CALC_RUST_SOURCE_FILE ${CMAKE_SOURCE_DIR}/src/lib.rs)
set(CALC_RUST_BRIDGE_CPP ${CARGO_TARGET_DIR}/cxxbridge/calc-rust/src/lib.rs.cc)
set(CALC_RUST_LIB ${CARGO_TARGET_DIR}/debug/${CMAKE_STATIC_LIBRARY_PREFIX}calc_rust${CMAKE_STATIC_LIBRARY_SUFFIX})

# Add a custom command that builds the rust crate and generates C++ bridge code
add_custom_command(
        OUTPUT ${CALC_RUST_BRIDGE_CPP} ${CALC_RUST_LIB}
        COMMAND cargo build --manifest-path ${CARGO_MANIFEST}
        DEPENDS ${CALC_RUST_SOURCE_FILE}
        USES_TERMINAL
        COMMENT "Running cargo..."
)

add_executable(calc-rust src/main.cpp ${CALC_RUST_BRIDGE_CPP})

target_include_directories(
        calc-rust
        PRIVATE
        ${CARGO_TARGET_DIR}/cxxbridge/calc-rust/src/
)

target_link_libraries(calc-rust ${CALC_RUST_LIB})

set_target_properties(
        calc-rust
        PROPERTIES ADDITIONAL_CLEAN_FILES ${CARGO_TARGET_DIR}
)

# Windows-only configuration
if(WIN32)
    target_link_libraries(cpp_with_rust userenv ws2_32 bcrypt)
    set_target_properties(
            cpp_with_rust
            PROPERTIES
            MSVC_RUNTIME_LIBRARY "MultiThreadedDLL"
            RUNTIME_OUTPUT_DIRECTORY_DEBUG ${CMAKE_BINARY_DIR}
            RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}
    )
endif()
