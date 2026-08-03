// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from navi_bringup:msg/OdomDebug.idl
// generated code does not contain a copyright notice

#ifndef NAVI_BRINGUP__MSG__DETAIL__ODOM_DEBUG__BUILDER_HPP_
#define NAVI_BRINGUP__MSG__DETAIL__ODOM_DEBUG__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "navi_bringup/msg/detail/odom_debug__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace navi_bringup
{

namespace msg
{

namespace builder
{

class Init_OdomDebug_yaw
{
public:
  explicit Init_OdomDebug_yaw(::navi_bringup::msg::OdomDebug & msg)
  : msg_(msg)
  {}
  ::navi_bringup::msg::OdomDebug yaw(::navi_bringup::msg::OdomDebug::_yaw_type arg)
  {
    msg_.yaw = std::move(arg);
    return std::move(msg_);
  }

private:
  ::navi_bringup::msg::OdomDebug msg_;
};

class Init_OdomDebug_y
{
public:
  explicit Init_OdomDebug_y(::navi_bringup::msg::OdomDebug & msg)
  : msg_(msg)
  {}
  Init_OdomDebug_yaw y(::navi_bringup::msg::OdomDebug::_y_type arg)
  {
    msg_.y = std::move(arg);
    return Init_OdomDebug_yaw(msg_);
  }

private:
  ::navi_bringup::msg::OdomDebug msg_;
};

class Init_OdomDebug_x
{
public:
  Init_OdomDebug_x()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_OdomDebug_y x(::navi_bringup::msg::OdomDebug::_x_type arg)
  {
    msg_.x = std::move(arg);
    return Init_OdomDebug_y(msg_);
  }

private:
  ::navi_bringup::msg::OdomDebug msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::navi_bringup::msg::OdomDebug>()
{
  return navi_bringup::msg::builder::Init_OdomDebug_x();
}

}  // namespace navi_bringup

#endif  // NAVI_BRINGUP__MSG__DETAIL__ODOM_DEBUG__BUILDER_HPP_
