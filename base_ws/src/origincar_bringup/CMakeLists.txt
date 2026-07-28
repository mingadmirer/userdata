cmake_minimum_required(VERSION 3.5)
project(origincar_bringup)

if(NOT CMAKE_CXX_STANDARD)
  set(CMAKE_CXX_STANDARD 14)
endif()

install(DIRECTORY launch DESTINATION share/${PROJECT_NAME})
find_package(ament_cmake REQUIRED)
ament_package()
