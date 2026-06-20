cmake_minimum_required(VERSION 3.24)
project(LineWidth)

target_link_libraries(
  line_width
  PRIVATE
  alpha_dependency
  beta_dependency
  gamma_dependency
  delta_dependency
  epsilon_dependency
)
