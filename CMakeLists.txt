cmake_minimum_required(VERSION 3.15)
project(rust_cpp_playground LANGUAGES CXX)

add_subdirectory(corrosion)

corrosion_import_crate(MANIFEST_PATH Cargo.toml)

add_executable(test test.cc)
target_link_libraries(test PUBLIC rust_cpp_playground)
