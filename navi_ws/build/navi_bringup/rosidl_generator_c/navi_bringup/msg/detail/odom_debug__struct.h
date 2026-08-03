// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from navi_bringup:msg/OdomDebug.idl
// generated code does not contain a copyright notice

#ifndef NAVI_BRINGUP__MSG__DETAIL__ODOM_DEBUG__STRUCT_H_
#define NAVI_BRINGUP__MSG__DETAIL__ODOM_DEBUG__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in msg/OdomDebug in the package navi_bringup.
typedef struct navi_bringup__msg__OdomDebug
{
  double x;
  double y;
  double yaw;
} navi_bringup__msg__OdomDebug;

// Struct for a sequence of navi_bringup__msg__OdomDebug.
typedef struct navi_bringup__msg__OdomDebug__Sequence
{
  navi_bringup__msg__OdomDebug * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} navi_bringup__msg__OdomDebug__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // NAVI_BRINGUP__MSG__DETAIL__ODOM_DEBUG__STRUCT_H_
