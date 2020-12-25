if(NOT EXISTS "/Users/webbhinton/my-fltk-examples/fuzzy_search/target/release/build/fltk-sys-956b29b4dcbe1aaa/out/build/fltk/install_manifest.txt")
   message(FATAL_ERROR
      "Cannot find install manifest: \"/Users/webbhinton/my-fltk-examples/fuzzy_search/target/release/build/fltk-sys-956b29b4dcbe1aaa/out/build/fltk/install_manifest.txt\"")
endif(NOT EXISTS "/Users/webbhinton/my-fltk-examples/fuzzy_search/target/release/build/fltk-sys-956b29b4dcbe1aaa/out/build/fltk/install_manifest.txt")

file(READ "/Users/webbhinton/my-fltk-examples/fuzzy_search/target/release/build/fltk-sys-956b29b4dcbe1aaa/out/build/fltk/install_manifest.txt" files)
string(REGEX REPLACE "\n" ";" files "${files}")

foreach(file ${files})
message(STATUS "Uninstalling \"$ENV{DESTDIR}${file}\"")
   exec_program("/Applications/CMake.app/Contents/bin/cmake"
      ARGS "-E remove -f \"$ENV{DESTDIR}${file}\""
      OUTPUT_VARIABLE rm_out
      RETURN_VALUE rm_retval
   )
   if(NOT "${rm_retval}" STREQUAL 0)
      message(FATAL_ERROR "Problem when removing \"$ENV{DESTDIR}${file}\"")
   endif(NOT "${rm_retval}" STREQUAL 0)
endforeach(file)
