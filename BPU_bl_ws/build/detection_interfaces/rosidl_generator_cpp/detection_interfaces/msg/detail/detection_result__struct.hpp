// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from detection_interfaces:msg/DetectionResult.idl
// generated code does not contain a copyright notice

#ifndef DETECTION_INTERFACES__MSG__DETAIL__DETECTION_RESULT__STRUCT_HPP_
#define DETECTION_INTERFACES__MSG__DETAIL__DETECTION_RESULT__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__detection_interfaces__msg__DetectionResult __attribute__((deprecated))
#else
# define DEPRECATED__detection_interfaces__msg__DetectionResult __declspec(deprecated)
#endif

namespace detection_interfaces
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct DetectionResult_
{
  using Type = DetectionResult_<ContainerAllocator>;

  explicit DetectionResult_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->name = "";
      this->x1 = 0.0f;
      this->y1 = 0.0f;
      this->x2 = 0.0f;
      this->y2 = 0.0f;
      this->confidence = 0.0f;
    }
  }

  explicit DetectionResult_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : name(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->name = "";
      this->x1 = 0.0f;
      this->y1 = 0.0f;
      this->x2 = 0.0f;
      this->y2 = 0.0f;
      this->confidence = 0.0f;
    }
  }

  // field types and members
  using _name_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _name_type name;
  using _x1_type =
    float;
  _x1_type x1;
  using _y1_type =
    float;
  _y1_type y1;
  using _x2_type =
    float;
  _x2_type x2;
  using _y2_type =
    float;
  _y2_type y2;
  using _confidence_type =
    float;
  _confidence_type confidence;

  // setters for named parameter idiom
  Type & set__name(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->name = _arg;
    return *this;
  }
  Type & set__x1(
    const float & _arg)
  {
    this->x1 = _arg;
    return *this;
  }
  Type & set__y1(
    const float & _arg)
  {
    this->y1 = _arg;
    return *this;
  }
  Type & set__x2(
    const float & _arg)
  {
    this->x2 = _arg;
    return *this;
  }
  Type & set__y2(
    const float & _arg)
  {
    this->y2 = _arg;
    return *this;
  }
  Type & set__confidence(
    const float & _arg)
  {
    this->confidence = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    detection_interfaces::msg::DetectionResult_<ContainerAllocator> *;
  using ConstRawPtr =
    const detection_interfaces::msg::DetectionResult_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<detection_interfaces::msg::DetectionResult_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<detection_interfaces::msg::DetectionResult_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      detection_interfaces::msg::DetectionResult_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<detection_interfaces::msg::DetectionResult_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      detection_interfaces::msg::DetectionResult_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<detection_interfaces::msg::DetectionResult_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<detection_interfaces::msg::DetectionResult_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<detection_interfaces::msg::DetectionResult_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__detection_interfaces__msg__DetectionResult
    std::shared_ptr<detection_interfaces::msg::DetectionResult_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__detection_interfaces__msg__DetectionResult
    std::shared_ptr<detection_interfaces::msg::DetectionResult_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const DetectionResult_ & other) const
  {
    if (this->name != other.name) {
      return false;
    }
    if (this->x1 != other.x1) {
      return false;
    }
    if (this->y1 != other.y1) {
      return false;
    }
    if (this->x2 != other.x2) {
      return false;
    }
    if (this->y2 != other.y2) {
      return false;
    }
    if (this->confidence != other.confidence) {
      return false;
    }
    return true;
  }
  bool operator!=(const DetectionResult_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct DetectionResult_

// alias to use template instance with default allocator
using DetectionResult =
  detection_interfaces::msg::DetectionResult_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace detection_interfaces

#endif  // DETECTION_INTERFACES__MSG__DETAIL__DETECTION_RESULT__STRUCT_HPP_
