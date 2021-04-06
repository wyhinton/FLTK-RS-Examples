#----------------------------------------------------------------
# Generated CMake target import file for configuration "Debug".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "fltk_z" for configuration "Debug"
set_property(TARGET fltk_z APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
set_target_properties(fltk_z PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_DEBUG "C"
  IMPORTED_LOCATION_DEBUG "${_IMPORT_PREFIX}/lib/fltk_zd.lib"
  )

list(APPEND _IMPORT_CHECK_TARGETS fltk_z )
list(APPEND _IMPORT_CHECK_FILES_FOR_fltk_z "${_IMPORT_PREFIX}/lib/fltk_zd.lib" )

# Import target "fltk_jpeg" for configuration "Debug"
set_property(TARGET fltk_jpeg APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
set_target_properties(fltk_jpeg PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_DEBUG "C"
  IMPORTED_LOCATION_DEBUG "${_IMPORT_PREFIX}/lib/fltk_jpegd.lib"
  )

list(APPEND _IMPORT_CHECK_TARGETS fltk_jpeg )
list(APPEND _IMPORT_CHECK_FILES_FOR_fltk_jpeg "${_IMPORT_PREFIX}/lib/fltk_jpegd.lib" )

# Import target "fltk_png" for configuration "Debug"
set_property(TARGET fltk_png APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
set_target_properties(fltk_png PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_DEBUG "C"
  IMPORTED_LOCATION_DEBUG "${_IMPORT_PREFIX}/lib/fltk_pngd.lib"
  )

list(APPEND _IMPORT_CHECK_TARGETS fltk_png )
list(APPEND _IMPORT_CHECK_FILES_FOR_fltk_png "${_IMPORT_PREFIX}/lib/fltk_pngd.lib" )

# Import target "fltk" for configuration "Debug"
set_property(TARGET fltk APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
set_target_properties(fltk PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_DEBUG "C;CXX"
  IMPORTED_LOCATION_DEBUG "${_IMPORT_PREFIX}/lib/fltkd.lib"
  )

list(APPEND _IMPORT_CHECK_TARGETS fltk )
list(APPEND _IMPORT_CHECK_FILES_FOR_fltk "${_IMPORT_PREFIX}/lib/fltkd.lib" )

# Import target "fltk_forms" for configuration "Debug"
set_property(TARGET fltk_forms APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
set_target_properties(fltk_forms PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_DEBUG "CXX"
  IMPORTED_LOCATION_DEBUG "${_IMPORT_PREFIX}/lib/fltk_formsd.lib"
  )

list(APPEND _IMPORT_CHECK_TARGETS fltk_forms )
list(APPEND _IMPORT_CHECK_FILES_FOR_fltk_forms "${_IMPORT_PREFIX}/lib/fltk_formsd.lib" )

# Import target "fltk_images" for configuration "Debug"
set_property(TARGET fltk_images APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
set_target_properties(fltk_images PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_DEBUG "CXX"
  IMPORTED_LOCATION_DEBUG "${_IMPORT_PREFIX}/lib/fltk_imagesd.lib"
  )

list(APPEND _IMPORT_CHECK_TARGETS fltk_images )
list(APPEND _IMPORT_CHECK_FILES_FOR_fltk_images "${_IMPORT_PREFIX}/lib/fltk_imagesd.lib" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
