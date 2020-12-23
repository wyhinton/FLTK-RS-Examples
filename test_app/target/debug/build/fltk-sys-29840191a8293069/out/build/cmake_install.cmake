# Install script for directory: /Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/lib/libcfltk.a")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/lib" TYPE STATIC_LIBRARY FILES "/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/build/libcfltk.a")
  if(EXISTS "$ENV{DESTDIR}/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/lib/libcfltk.a" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/lib/libcfltk.a")
    execute_process(COMMAND "/usr/bin/ranlib" "$ENV{DESTDIR}/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/lib/libcfltk.a")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_box.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_browser.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_button.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_dialog.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_draw.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_enums.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_group.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_image.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_input.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_menu.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_misc.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_output.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_printer.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_surface.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_table.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_text.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_tree.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_valuator.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_widget.h;/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk/cfl_window.h")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/include/cfltk" TYPE FILE FILES
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_box.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_browser.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_button.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_dialog.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_draw.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_enums.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_group.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_image.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_input.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_menu.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_misc.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_output.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_printer.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_surface.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_table.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_text.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_tree.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_valuator.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_widget.h"
    "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_window.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/share/cmake/cfltk/cfltkConfig.cmake")
    file(DIFFERENT EXPORT_FILE_CHANGED FILES
         "$ENV{DESTDIR}/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/share/cmake/cfltk/cfltkConfig.cmake"
         "/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/build/CMakeFiles/Export/_Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/share/cmake/cfltk/cfltkConfig.cmake")
    if(EXPORT_FILE_CHANGED)
      file(GLOB OLD_CONFIG_FILES "$ENV{DESTDIR}/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/share/cmake/cfltk/cfltkConfig-*.cmake")
      if(OLD_CONFIG_FILES)
        message(STATUS "Old export file \"$ENV{DESTDIR}/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/share/cmake/cfltk/cfltkConfig.cmake\" will be replaced.  Removing files [${OLD_CONFIG_FILES}].")
        file(REMOVE ${OLD_CONFIG_FILES})
      endif()
    endif()
  endif()
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/share/cmake/cfltk/cfltkConfig.cmake")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/share/cmake/cfltk" TYPE FILE FILES "/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/build/CMakeFiles/Export/_Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/share/cmake/cfltk/cfltkConfig.cmake")
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/share/cmake/cfltk/cfltkConfig-release.cmake")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
file(INSTALL DESTINATION "/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/share/cmake/cfltk" TYPE FILE FILES "/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/build/CMakeFiles/Export/_Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/share/cmake/cfltk/cfltkConfig-release.cmake")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/share/cmake/cfltk/cfltkConfigVersion.cmake")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/share/cmake/cfltk" TYPE FILE FILES "/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/build/cfltkConfigVersion.cmake")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/build/fltk/cmake_install.cmake")

endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/Users/webbhinton/my-fltk-examples/build_app/target/debug/build/fltk-sys-29840191a8293069/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
