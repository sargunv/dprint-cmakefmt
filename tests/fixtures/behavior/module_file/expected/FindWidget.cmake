find_path(
  WIDGET_INCLUDE_DIR
  NAMES widget/widget.h
  PATHS "${WIDGET_ROOT}" ENV WIDGET_ROOT
  PATH_SUFFIXES include
)
find_library(
  WIDGET_LIBRARY
  NAMES widget
  PATHS "${WIDGET_ROOT}" ENV WIDGET_ROOT
  PATH_SUFFIXES lib
)

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(
  Widget
  DEFAULT_MSG
  WIDGET_INCLUDE_DIR
  WIDGET_LIBRARY
)
