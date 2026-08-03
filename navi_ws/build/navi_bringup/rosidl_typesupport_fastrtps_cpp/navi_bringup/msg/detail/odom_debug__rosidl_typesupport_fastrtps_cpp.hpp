// generated from rosidl_typesupport_fastrtps_cpp/resource/idl__rosidl_typesupport_fastrtps_cpp.hpp.em
// with input from navi_bringup:msg/OdomDebug.idl
// generated code does not contain a copyright notice

#ifndef NAVI_BRINGUP__MSG__DETAIL__ODOM_DEBUG__ROSIDL_TYPESUPPORT_FASTRTPS_CPP_HPP_
#define NAVI_BRINGUP__MSG__DETAIL__ODOM_DEBUG__ROSIDL_TYPESUPPORT_FASTRTPS_CPP_HPP_

#include "rosidl_runtime_c/message_type_support_struct.h"
#include "rosidl_typesupport_interface/macros.h"
#include "navi_bringup/msg/rosidl_typesupport_fastrtps_cpp__visibility_control.h"
#include "navi_bringup/msg/detail/odom_debug__struct.hpp"

#ifndef _WIN32
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-parameter"
# ifdef __clang__
#  pragma clang diagnostic ignored "-Wdeprecated-register"
#  pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
# endif
#endif
#ifndef _WIN32
# pragma GCC diagnostic pop
#endif

#include "fastcdr/Cdr.h"

namespace navi_bringup
{

namespace msg
{

namespace typesupport_fastrtps_cpp
{

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_navi_bringup
cdr_serialize(
  const navi_bringup::msg::OdomDebug & ros_message,
  eprosima::fastcdr::Cdr & cdr);

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_navi_bringup
cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  navi_bringup::msg::OdomDebug & ros_message);

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_navi_bringup
get_serialized_size(
  const navi_bringup::msg::OdomDebug & ros_message,
  size_t current_alignment);

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_navi_bringup
max_serialized_size_OdomDebug(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);

}  // namespace typesupport_fastrtps_cpp

}  // namespace msg

}  // namespace navi_bringup

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_navi_bringup
const rosidl_message_type_support_t *
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, navi_bringup, msg, OdomDebug)();

#ifdef __cplusplus
}
#endif

#endif  // NAVI_BRINGUP__MSG__DETAIL__ODOM_DEBUG__ROSIDL_TYPESUPPORT_FASTRTPS_CPP_HPP_
