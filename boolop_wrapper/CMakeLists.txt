cmake_minimum_required(VERSION 3.21)

# Set the target architecture.
# All modern x86/x64 processors support AVX2.
# Older x86/x64 processors may support SSE2 but not AVX2.
# Very old x86/x64 processors, or non x86/x64
# processors, do not support any of the two.
set(ENABLE_SSE2 True)
set(ENABLE_AVX2 True)

# specify the C++ standard
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# set the project name
project(boolop_wrapper)

set(TBB_TEST OFF CACHE BOOL " " FORCE)
set(TBB_EXAMPLES OFF CACHE BOOL " " FORCE)
add_subdirectory(InteractiveAndRobustMeshBooleans/arrangements/external/oneTBB)

# add the executable
add_library(${PROJECT_NAME} boolop.cpp)

target_include_directories(${PROJECT_NAME} PUBLIC
    PRIVATE
    InteractiveAndRobustMeshBooleans/code/
    InteractiveAndRobustMeshBooleans/arrangements/code/
    InteractiveAndRobustMeshBooleans/arrangements/external/Indirect_Predicates/include/
)

set(cinolib_DIR ${PROJECT_SOURCE_DIR}/InteractiveAndRobustMeshBooleans/arrangements/external/Cinolib)
set(CINOLIB_USES_OPENGL_GLFW_IMGUI ON)
set(CINOLIB_USES_SHEWCHUK_PREDICATES ON)

find_package(cinolib REQUIRED)

target_link_libraries(${PROJECT_NAME} cinolib)
target_link_libraries(${PROJECT_NAME} tbb)
target_compile_definitions(${PROJECT_NAME} PUBLIC TBB_PARALLEL=1)
target_include_directories(${PROJECT_NAME} PUBLIC ${PROJECT_SOURCE_DIR}/InteractiveAndRobustMeshBooleans/arrangements/external/abseil-cpp/)
target_include_directories(${PROJECT_NAME} PUBLIC ${PROJECT_SOURCE_DIR}/InteractiveAndRobustMeshBooleans/arrangements/external/oneTBB/)

# Compiler-specific options
if(CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
        # grant IEEE 754 compliance
        target_compile_options(${PROJECT_NAME} PUBLIC "/fp:strict")
        # use intrinsic functions
        target_compile_options(${PROJECT_NAME} PUBLIC "/Oi")
        # set target architecture
        if(ENABLE_AVX2)
                target_compile_options(${PROJECT_NAME} PUBLIC "/arch:AVX2")
        elseif(ENABLE_SSE2)
                target_compile_options(${PROJECT_NAME} PUBLIC "/arch:SSE2")
        endif()
        # reserve enough stack size
        target_link_options(${PROJECT_NAME} PUBLIC "/STACK:8421376")
        # turn off annoying warnings
        target_compile_options(${PROJECT_NAME} PUBLIC "/D _CRT_SECURE_NO_WARNINGS")
else()
        # set standard optimization level
        target_compile_options(${PROJECT_NAME} PUBLIC -O2)
        # reserve enough stack size
        target_compile_options(${PROJECT_NAME} PUBLIC -Wl,-z,stacksize=8421376)
        # grant IEEE 754 compliance
        target_compile_options(${PROJECT_NAME} PUBLIC -frounding-math)
        # set target architecture
        if(ENABLE_AVX2)
                target_compile_options(${PROJECT_NAME} PUBLIC "-mavx2")
        elseif(ENABLE_SSE2)
                target_compile_options(${PROJECT_NAME} PUBLIC "-msse2")
        endif()
endif()