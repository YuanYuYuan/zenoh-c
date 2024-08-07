message(STATUS "zenoh-c tests")

add_custom_target(tests)

find_package(Threads REQUIRED)

file(GLOB files "${CMAKE_CURRENT_SOURCE_DIR}/*.c")
foreach(file ${files})
    get_filename_component(target ${file} NAME_WE)
    
    # Check the filename prefix to determine the test type
    if (${file} MATCHES "^.*z_api_.*$")
        set(test_type "unit")
    elseif (${file} MATCHES "^.*z_int_.*$")
        set(test_type "integration")
    elseif (${file} MATCHES "^.*z_build_.*$")
        set(test_type "build")
    else()
        message(FATAL_ERROR "Test file ${file} does not match any known type (z_api_ or z_int_ or z_build)")
    endif()

    add_executable(${target} EXCLUDE_FROM_ALL ${file})
    add_dependencies(tests ${target})
    add_dependencies(${target} zenohc::lib)
    target_link_libraries(${target} PRIVATE zenohc::lib Threads::Threads)
    copy_dlls(${target})
    set_property(TARGET ${target} PROPERTY C_STANDARD 11)
    add_test(NAME "${test_type}_${target}" COMMAND ${target})
endforeach()

