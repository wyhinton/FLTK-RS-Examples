# Install script for directory: C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/fltk

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/rls/debug/build/fltk-sys-762000bc27d0417d/out")
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
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE DIRECTORY FILES "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/fltk/FL" USE_SOURCE_PERMISSIONS)
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE DIRECTORY FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/rls/debug/build/fltk-sys-762000bc27d0417d/out/build/fltk/FL" USE_SOURCE_PERMISSIONS)
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/CMake/FLTK-Targets.cmake")
    file(DIFFERENT EXPORT_FILE_CHANGED FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/CMake/FLTK-Targets.cmake"
         "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/rls/debug/build/fltk-sys-762000bc27d0417d/out/build/fltk/CMakeFiles/Export/CMake/FLTK-Targets.cmake")
    if(EXPORT_FILE_CHANGED)
      file(GLOB OLD_CONFIG_FILES "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/CMake/FLTK-Targets-*.cmake")
      if(OLD_CONFIG_FILES)
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/CMake/FLTK-Targets.cmake\" will be replaced.  Removing files [${OLD_CONFIG_FILES}].")
        file(REMOVE ${OLD_CONFIG_FILES})
      endif()
    endif()
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/CMake" TYPE FILE FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/rls/debug/build/fltk-sys-762000bc27d0417d/out/build/fltk/CMakeFiles/Export/CMake/FLTK-Targets.cmake")
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Dd][Ee][Bb][Uu][Gg])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/CMake" TYPE FILE FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/rls/debug/build/fltk-sys-762000bc27d0417d/out/build/fltk/CMakeFiles/Export/CMake/FLTK-Targets-debug.cmake")
  endif()
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Mm][Ii][Nn][Ss][Ii][Zz][Ee][Rr][Ee][Ll])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/CMake" TYPE FILE FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/rls/debug/build/fltk-sys-762000bc27d0417d/out/build/fltk/CMakeFiles/Export/CMake/FLTK-Targets-minsizerel.cmake")
  endif()
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Rr][Ee][Ll][Ww][Ii][Tt][Hh][Dd][Ee][Bb][Ii][Nn][Ff][Oo])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/CMake" TYPE FILE FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/rls/debug/build/fltk-sys-762000bc27d0417d/out/build/fltk/CMakeFiles/Export/CMake/FLTK-Targets-relwithdebinfo.cmake")
  endif()
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/CMake" TYPE FILE FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/rls/debug/build/fltk-sys-762000bc27d0417d/out/build/fltk/CMakeFiles/Export/CMake/FLTK-Targets-release.cmake")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/CMake" TYPE FILE FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/rls/debug/build/fltk-sys-762000bc27d0417d/out/build/fltk/etc/FLTKConfig.cmake")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/CMake" TYPE FILE FILES "C:/Users/Primary User/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-0.12.0/cfltk/fltk/CMake/FLTK-Functions.cmake")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/CMake" TYPE FILE FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/rls/debug/build/fltk-sys-762000bc27d0417d/out/build/fltk/etc/UseFLTK.cmake")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE PROGRAM FILES "C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/rls/debug/build/fltk-sys-762000bc27d0417d/out/build/fltk/bin/fltk-config")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/rls/debug/build/fltk-sys-762000bc27d0417d/out/build/fltk/zlib/cmake_install.cmake")
  include("C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/rls/debug/build/fltk-sys-762000bc27d0417d/out/build/fltk/jpeg/cmake_install.cmake")
  include("C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/rls/debug/build/fltk-sys-762000bc27d0417d/out/build/fltk/png/cmake_install.cmake")
  include("C:/Users/Primary User/Desktop/texture synthesis app/MY_Examples/FLTK-RS-Examples/template/target/rls/debug/build/fltk-sys-762000bc27d0417d/out/build/fltk/src/cmake_install.cmake")

endif()

