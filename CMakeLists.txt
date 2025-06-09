cmake_minimum_required(VERSION 3.20)
project(SPARK LANGUAGES CXX)

set(CMAKE_AUTOMOC ON)
set(CMAKE_CXX_STANDARD 17)

# Find Qt components
find_package(Qt5 COMPONENTS Core Gui Widgets REQUIRED)

# Add resources
qt5_add_resources(SPARK_RCC resources.qrc)

# Winterfell FFI library (imported)
add_library(winterfell_ffi STATIC IMPORTED)
set_target_properties(winterfell_ffi PROPERTIES
    IMPORTED_LOCATION ${CMAKE_CURRENT_SOURCE_DIR}/../cryptonote/external/winterfell-ffi/target/release/libwinterfell_ffi.a
)

# Executable
add_executable(SPARK
    main.cpp
    MainWindow.cpp
    ${SPARK_RCC}
)

# Include directories for Winterfell headers
target_include_directories(SPARK PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/../cryptonote/external/winterfell-ffi/include
)

# Link libraries
target_link_libraries(SPARK PRIVATE
    Qt5::Core
    Qt5::Gui
    Qt5::Widgets
    winterfell_ffi
)

# Installation
install(TARGETS SPARK
    RUNTIME DESTINATION bin
    BUNDLE DESTINATION .
) 