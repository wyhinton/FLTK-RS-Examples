# Install script for directory: /Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/fltk

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/Users/webbhinton/my-fltk-examples/build_app/target/release/build/fltk-sys-956b29b4dcbe1aaa/out")
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
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE DIRECTORY FILES "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/fltk/FL" USE_SOURCE_PERMISSIONS)
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE DIRECTORY FILES "/Users/webbhinton/my-fltk-examples/build_app/target/release/build/fltk-sys-956b29b4dcbe1aaa/out/build/fltk/FL" USE_SOURCE_PERMISSIONS)
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/FLTK.framework/Resources/CMake/FLTK-Targets.cmake")
    file(DIFFERENT EXPORT_FILE_CHANGED FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/FLTK.framework/Resources/CMake/FLTK-Targets.cmake"
         "/Users/webbhinton/my-fltk-examples/build_app/target/release/build/fltk-sys-956b29b4dcbe1aaa/out/build/fltk/CMakeFiles/Export/FLTK.framework/Resources/CMake/FLTK-Targets.cmake")
    if(EXPORT_FILE_CHANGED)
      file(GLOB OLD_CONFIG_FILES "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/FLTK.framework/Resources/CMake/FLTK-Targets-*.cmake")
      if(OLD_CONFIG_FILES)
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/FLTK.framework/Resources/CMake/FLTK-Targets.cmake\" will be replaced.  Removing files [${OLD_CONFIG_FILES}].")
        file(REMOVE ${OLD_CONFIG_FILES})
      endif()
    endif()
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/FLTK.framework/Resources/CMake" TYPE FILE FILES "/Users/webbhinton/my-fltk-examples/build_app/target/release/build/fltk-sys-956b29b4dcbe1aaa/out/build/fltk/CMakeFiles/Export/FLTK.framework/Resources/CMake/FLTK-Targets.cmake")
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/FLTK.framework/Resources/CMake" TYPE FILE FILES "/Users/webbhinton/my-fltk-examples/build_app/target/release/build/fltk-sys-956b29b4dcbe1aaa/out/build/fltk/CMakeFiles/Export/FLTK.framework/Resources/CMake/FLTK-Targets-release.cmake")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/FLTK.framework/Resources/CMake" TYPE FILE FILES "/Users/webbhinton/my-fltk-examples/build_app/target/release/build/fltk-sys-956b29b4dcbe1aaa/out/build/fltk/etc/FLTKConfig.cmake")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/FLTK.framework/Resources/CMake" TYPE FILE FILES "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/fltk/CMake/FLTK-Functions.cmake")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/FLTK.framework/Resources/CMake" TYPE FILE FILES "/Users/webbhinton/my-fltk-examples/build_app/target/release/build/fltk-sys-956b29b4dcbe1aaa/out/build/fltk/etc/UseFLTK.cmake")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE PROGRAM FILES "/Users/webbhinton/my-fltk-examples/build_app/target/release/build/fltk-sys-956b29b4dcbe1aaa/out/build/fltk/bin/fltk-config")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/man/man1" TYPE FILE RENAME "fluid.1" FILES "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/fltk/documentation/src/fluid.man")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/man/man1" TYPE FILE RENAME "fltk-config.1" FILES "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/fltk/documentation/src/fltk-config.man")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/man/man3" TYPE FILE RENAME "fltk.3" FILES "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/fltk/documentation/src/fltk.man")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/man/man6" TYPE FILE RENAME "blocks.6" FILES "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/fltk/documentation/src/blocks.man")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/man/man6" TYPE FILE RENAME "checkers.6" FILES "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/fltk/documentation/src/checkers.man")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/man/man6" TYPE FILE RENAME "sudoku.6" FILES "/Users/webbhinton/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/fltk/documentation/src/sudoku.man")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("/Users/webbhinton/my-fltk-examples/build_app/target/release/build/fltk-sys-956b29b4dcbe1aaa/out/build/fltk/zlib/cmake_install.cmake")
  include("/Users/webbhinton/my-fltk-examples/build_app/target/release/build/fltk-sys-956b29b4dcbe1aaa/out/build/fltk/jpeg/cmake_install.cmake")
  include("/Users/webbhinton/my-fltk-examples/build_app/target/release/build/fltk-sys-956b29b4dcbe1aaa/out/build/fltk/png/cmake_install.cmake")
  include("/Users/webbhinton/my-fltk-examples/build_app/target/release/build/fltk-sys-956b29b4dcbe1aaa/out/build/fltk/src/cmake_install.cmake")

endif()

