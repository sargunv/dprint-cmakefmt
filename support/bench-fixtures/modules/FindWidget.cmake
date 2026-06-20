find_path(
  WIDGET_INCLUDE_DIR
  NAMES widget/widget.h
  PATHS "${WIDGET_ROOT}" ENV WIDGET_ROOT
  PATH_SUFFIXES include include/widget
)

find_library(
  WIDGET_LIBRARY
  NAMES widget libwidget
  PATHS "${WIDGET_ROOT}" ENV WIDGET_ROOT
  PATH_SUFFIXES lib lib64
)

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(
  Widget
  REQUIRED_VARS WIDGET_LIBRARY WIDGET_INCLUDE_DIR
)

mark_as_advanced(WIDGET_INCLUDE_DIR WIDGET_LIBRARY)
