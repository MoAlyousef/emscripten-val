cmake_minimum_required(VERSION 3.14)
project(emval_support)

add_library(emval_support emval_support.cpp val_support.cpp)

install(
  TARGETS emval_support
  ARCHIVE DESTINATION ${CMAKE_INSTALL_PREFIX}/lib
  LIBRARY DESTINATION ${CMAKE_INSTALL_PREFIX}/lib
  PUBLIC_HEADER DESTINATION ${CMAKE_INSTALL_PREFIX}/include/emval_support)