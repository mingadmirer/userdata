#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "navi_bringup__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__navi_bringup__msg__OdomDebug() -> *const std::ffi::c_void;
}

#[link(name = "navi_bringup__rosidl_generator_c")]
extern "C" {
    fn navi_bringup__msg__OdomDebug__init(msg: *mut OdomDebug) -> bool;
    fn navi_bringup__msg__OdomDebug__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<OdomDebug>, size: usize) -> bool;
    fn navi_bringup__msg__OdomDebug__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<OdomDebug>);
    fn navi_bringup__msg__OdomDebug__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<OdomDebug>, out_seq: *mut rosidl_runtime_rs::Sequence<OdomDebug>) -> bool;
}

// Corresponds to navi_bringup__msg__OdomDebug
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OdomDebug {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw: f64,

}



impl Default for OdomDebug {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !navi_bringup__msg__OdomDebug__init(&mut msg as *mut _) {
        panic!("Call to navi_bringup__msg__OdomDebug__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for OdomDebug {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { navi_bringup__msg__OdomDebug__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { navi_bringup__msg__OdomDebug__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { navi_bringup__msg__OdomDebug__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for OdomDebug {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for OdomDebug where Self: Sized {
  const TYPE_NAME: &'static str = "navi_bringup/msg/OdomDebug";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__navi_bringup__msg__OdomDebug() }
  }
}


