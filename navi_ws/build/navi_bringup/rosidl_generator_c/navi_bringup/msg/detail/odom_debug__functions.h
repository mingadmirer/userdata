// generated from rosidl_generator_c/resource/idl__functions.h.em
// with input from navi_bringup:msg/OdomDebug.idl
// generated code does not contain a copyright notice

#ifndef NAVI_BRINGUP__MSG__DETAIL__ODOM_DEBUG__FUNCTIONS_H_
#define NAVI_BRINGUP__MSG__DETAIL__ODOM_DEBUG__FUNCTIONS_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stdlib.h>

#include "rosidl_runtime_c/visibility_control.h"
#include "navi_bringup/msg/rosidl_generator_c__visibility_control.h"

#include "navi_bringup/msg/detail/odom_debug__struct.h"

/// Initialize msg/OdomDebug message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * navi_bringup__msg__OdomDebug
 * )) before or use
 * navi_bringup__msg__OdomDebug__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_navi_bringup
bool
navi_bringup__msg__OdomDebug__init(navi_bringup__msg__OdomDebug * msg);

/// Finalize msg/OdomDebug message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_navi_bringup
void
navi_bringup__msg__OdomDebug__fini(navi_bringup__msg__OdomDebug * msg);

/// Create msg/OdomDebug message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * navi_bringup__msg__OdomDebug__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_navi_bringup
navi_bringup__msg__OdomDebug *
navi_bringup__msg__OdomDebug__create();

/// Destroy msg/OdomDebug message.
/**
 * It calls
 * navi_bringup__msg__OdomDebug__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_navi_bringup
void
navi_bringup__msg__OdomDebug__destroy(navi_bringup__msg__OdomDebug * msg);

/// Check for msg/OdomDebug message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_navi_bringup
bool
navi_bringup__msg__OdomDebug__are_equal(const navi_bringup__msg__OdomDebug * lhs, const navi_bringup__msg__OdomDebug * rhs);

/// Copy a msg/OdomDebug message.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source message pointer.
 * \param[out] output The target message pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer is null
 *   or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_navi_bringup
bool
navi_bringup__msg__OdomDebug__copy(
  const navi_bringup__msg__OdomDebug * input,
  navi_bringup__msg__OdomDebug * output);

/// Initialize array of msg/OdomDebug messages.
/**
 * It allocates the memory for the number of elements and calls
 * navi_bringup__msg__OdomDebug__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_navi_bringup
bool
navi_bringup__msg__OdomDebug__Sequence__init(navi_bringup__msg__OdomDebug__Sequence * array, size_t size);

/// Finalize array of msg/OdomDebug messages.
/**
 * It calls
 * navi_bringup__msg__OdomDebug__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_navi_bringup
void
navi_bringup__msg__OdomDebug__Sequence__fini(navi_bringup__msg__OdomDebug__Sequence * array);

/// Create array of msg/OdomDebug messages.
/**
 * It allocates the memory for the array and calls
 * navi_bringup__msg__OdomDebug__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_navi_bringup
navi_bringup__msg__OdomDebug__Sequence *
navi_bringup__msg__OdomDebug__Sequence__create(size_t size);

/// Destroy array of msg/OdomDebug messages.
/**
 * It calls
 * navi_bringup__msg__OdomDebug__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_navi_bringup
void
navi_bringup__msg__OdomDebug__Sequence__destroy(navi_bringup__msg__OdomDebug__Sequence * array);

/// Check for msg/OdomDebug message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_navi_bringup
bool
navi_bringup__msg__OdomDebug__Sequence__are_equal(const navi_bringup__msg__OdomDebug__Sequence * lhs, const navi_bringup__msg__OdomDebug__Sequence * rhs);

/// Copy an array of msg/OdomDebug messages.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source array pointer.
 * \param[out] output The target array pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer
 *   is null or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_navi_bringup
bool
navi_bringup__msg__OdomDebug__Sequence__copy(
  const navi_bringup__msg__OdomDebug__Sequence * input,
  navi_bringup__msg__OdomDebug__Sequence * output);

#ifdef __cplusplus
}
#endif

#endif  // NAVI_BRINGUP__MSG__DETAIL__ODOM_DEBUG__FUNCTIONS_H_
