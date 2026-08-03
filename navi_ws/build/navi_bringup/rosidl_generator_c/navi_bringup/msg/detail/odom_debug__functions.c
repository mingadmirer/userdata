// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from navi_bringup:msg/OdomDebug.idl
// generated code does not contain a copyright notice
#include "navi_bringup/msg/detail/odom_debug__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


bool
navi_bringup__msg__OdomDebug__init(navi_bringup__msg__OdomDebug * msg)
{
  if (!msg) {
    return false;
  }
  // x
  // y
  // yaw
  return true;
}

void
navi_bringup__msg__OdomDebug__fini(navi_bringup__msg__OdomDebug * msg)
{
  if (!msg) {
    return;
  }
  // x
  // y
  // yaw
}

bool
navi_bringup__msg__OdomDebug__are_equal(const navi_bringup__msg__OdomDebug * lhs, const navi_bringup__msg__OdomDebug * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // x
  if (lhs->x != rhs->x) {
    return false;
  }
  // y
  if (lhs->y != rhs->y) {
    return false;
  }
  // yaw
  if (lhs->yaw != rhs->yaw) {
    return false;
  }
  return true;
}

bool
navi_bringup__msg__OdomDebug__copy(
  const navi_bringup__msg__OdomDebug * input,
  navi_bringup__msg__OdomDebug * output)
{
  if (!input || !output) {
    return false;
  }
  // x
  output->x = input->x;
  // y
  output->y = input->y;
  // yaw
  output->yaw = input->yaw;
  return true;
}

navi_bringup__msg__OdomDebug *
navi_bringup__msg__OdomDebug__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  navi_bringup__msg__OdomDebug * msg = (navi_bringup__msg__OdomDebug *)allocator.allocate(sizeof(navi_bringup__msg__OdomDebug), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(navi_bringup__msg__OdomDebug));
  bool success = navi_bringup__msg__OdomDebug__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
navi_bringup__msg__OdomDebug__destroy(navi_bringup__msg__OdomDebug * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    navi_bringup__msg__OdomDebug__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
navi_bringup__msg__OdomDebug__Sequence__init(navi_bringup__msg__OdomDebug__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  navi_bringup__msg__OdomDebug * data = NULL;

  if (size) {
    data = (navi_bringup__msg__OdomDebug *)allocator.zero_allocate(size, sizeof(navi_bringup__msg__OdomDebug), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = navi_bringup__msg__OdomDebug__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        navi_bringup__msg__OdomDebug__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
navi_bringup__msg__OdomDebug__Sequence__fini(navi_bringup__msg__OdomDebug__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      navi_bringup__msg__OdomDebug__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

navi_bringup__msg__OdomDebug__Sequence *
navi_bringup__msg__OdomDebug__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  navi_bringup__msg__OdomDebug__Sequence * array = (navi_bringup__msg__OdomDebug__Sequence *)allocator.allocate(sizeof(navi_bringup__msg__OdomDebug__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = navi_bringup__msg__OdomDebug__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
navi_bringup__msg__OdomDebug__Sequence__destroy(navi_bringup__msg__OdomDebug__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    navi_bringup__msg__OdomDebug__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
navi_bringup__msg__OdomDebug__Sequence__are_equal(const navi_bringup__msg__OdomDebug__Sequence * lhs, const navi_bringup__msg__OdomDebug__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!navi_bringup__msg__OdomDebug__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
navi_bringup__msg__OdomDebug__Sequence__copy(
  const navi_bringup__msg__OdomDebug__Sequence * input,
  navi_bringup__msg__OdomDebug__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(navi_bringup__msg__OdomDebug);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    navi_bringup__msg__OdomDebug * data =
      (navi_bringup__msg__OdomDebug *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!navi_bringup__msg__OdomDebug__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          navi_bringup__msg__OdomDebug__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!navi_bringup__msg__OdomDebug__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
