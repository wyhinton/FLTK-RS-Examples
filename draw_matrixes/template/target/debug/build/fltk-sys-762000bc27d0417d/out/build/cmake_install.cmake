# Install script for directory: C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out")
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

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Dd][Ee][Bb][Uu][Gg])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/lib/cfltk.lib")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
file(INSTALL DESTINATION "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/lib" TYPE STATIC_LIBRARY FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/build/Debug/cfltk.lib")
  elseif("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/lib/cfltk.lib")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
file(INSTALL DESTINATION "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/lib" TYPE STATIC_LIBRARY FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/build/Release/cfltk.lib")
  elseif("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Mm][Ii][Nn][Ss][Ii][Zz][Ee][Rr][Ee][Ll])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/lib/cfltk.lib")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
file(INSTALL DESTINATION "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/lib" TYPE STATIC_LIBRARY FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/build/MinSizeRel/cfltk.lib")
  elseif("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Rr][Ee][Ll][Ww][Ii][Tt][Hh][Dd][Ee][Bb][Ii][Nn][Ff][Oo])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/lib/cfltk.lib")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
file(INSTALL DESTINATION "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/lib" TYPE STATIC_LIBRARY FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/build/RelWithDebInfo/cfltk.lib")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_box.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_browser.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_button.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_dialog.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_draw.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_enums.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_group.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_image.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_input.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_menu.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_misc.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_output.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_printer.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_surface.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_table.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_text.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_tree.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_valuator.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_widget.h;C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk/cfl_window.h")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/include/cfltk" TYPE FILE FILES
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_box.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_browser.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_button.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_dialog.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_draw.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_enums.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_group.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_image.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_input.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_menu.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_misc.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_output.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_printer.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_surface.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_table.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_text.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_tree.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_valuator.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_widget.h"
    "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/include/cfl_window.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/share/cmake/cfltk/cfltkConfig.cmake")
    file(DIFFERENT EXPORT_FILE_CHANGED FILES
         "$ENV{DESTDIR}C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/share/cmake/cfltk/cfltkConfig.cmake"
         "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/build/CMakeFiles/Export/d91baf88bd416588352f274f7785f616/cfltkConfig.cmake")
    if(EXPORT_FILE_CHANGED)
      file(GLOB OLD_CONFIG_FILES "$ENV{DESTDIR}C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/share/cmake/cfltk/cfltkConfig-*.cmake")
      if(OLD_CONFIG_FILES)
        message(STATUS "Old export file \"$ENV{DESTDIR}C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/share/cmake/cfltk/cfltkConfig.cmake\" will be replaced.  Removing files [${OLD_CONFIG_FILES}].")
        file(REMOVE ${OLD_CONFIG_FILES})
      endif()
    endif()
  endif()
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/share/cmake/cfltk/cfltkConfig.cmake")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/share/cmake/cfltk" TYPE FILE FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/build/CMakeFiles/Export/d91baf88bd416588352f274f7785f616/cfltkConfig.cmake")
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Dd][Ee][Bb][Uu][Gg])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/share/cmake/cfltk/cfltkConfig-debug.cmake")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
file(INSTALL DESTINATION "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/share/cmake/cfltk" TYPE FILE FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/build/CMakeFiles/Export/d91baf88bd416588352f274f7785f616/cfltkConfig-debug.cmake")
  endif()
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Mm][Ii][Nn][Ss][Ii][Zz][Ee][Rr][Ee][Ll])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/share/cmake/cfltk/cfltkConfig-minsizerel.cmake")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
file(INSTALL DESTINATION "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/share/cmake/cfltk" TYPE FILE FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/build/CMakeFiles/Export/d91baf88bd416588352f274f7785f616/cfltkConfig-minsizerel.cmake")
  endif()
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Rr][Ee][Ll][Ww][Ii][Tt][Hh][Dd][Ee][Bb][Ii][Nn][Ff][Oo])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/share/cmake/cfltk/cfltkConfig-relwithdebinfo.cmake")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
file(INSTALL DESTINATION "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/share/cmake/cfltk" TYPE FILE FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/build/CMakeFiles/Export/d91baf88bd416588352f274f7785f616/cfltkConfig-relwithdebinfo.cmake")
  endif()
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/share/cmake/cfltk/cfltkConfig-release.cmake")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
file(INSTALL DESTINATION "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/share/cmake/cfltk" TYPE FILE FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/build/CMakeFiles/Export/d91baf88bd416588352f274f7785f616/cfltkConfig-release.cmake")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/share/cmake/cfltk/cfltkConfigVersion.cmake")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/share/cmake/cfltk" TYPE FILE FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/build/cfltkConfigVersion.cmake")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/build/fltk/cmake_install.cmake")

endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/debug/build/fltk-sys-762000bc27d0417d/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
