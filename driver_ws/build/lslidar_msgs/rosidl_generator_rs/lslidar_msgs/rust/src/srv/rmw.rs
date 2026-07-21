#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__AngleDistortionCorrection_Request() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__AngleDistortionCorrection_Request__init(msg: *mut AngleDistortionCorrection_Request) -> bool;
    fn lslidar_msgs__srv__AngleDistortionCorrection_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AngleDistortionCorrection_Request>, size: usize) -> bool;
    fn lslidar_msgs__srv__AngleDistortionCorrection_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AngleDistortionCorrection_Request>);
    fn lslidar_msgs__srv__AngleDistortionCorrection_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AngleDistortionCorrection_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AngleDistortionCorrection_Request>) -> bool;
}

// Corresponds to lslidar_msgs__srv__AngleDistortionCorrection_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AngleDistortionCorrection_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub angle_distortion_correction: u8,

}



impl Default for AngleDistortionCorrection_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__AngleDistortionCorrection_Request__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__AngleDistortionCorrection_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AngleDistortionCorrection_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__AngleDistortionCorrection_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__AngleDistortionCorrection_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__AngleDistortionCorrection_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AngleDistortionCorrection_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AngleDistortionCorrection_Request where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/AngleDistortionCorrection_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__AngleDistortionCorrection_Request() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__AngleDistortionCorrection_Response() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__AngleDistortionCorrection_Response__init(msg: *mut AngleDistortionCorrection_Response) -> bool;
    fn lslidar_msgs__srv__AngleDistortionCorrection_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AngleDistortionCorrection_Response>, size: usize) -> bool;
    fn lslidar_msgs__srv__AngleDistortionCorrection_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AngleDistortionCorrection_Response>);
    fn lslidar_msgs__srv__AngleDistortionCorrection_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AngleDistortionCorrection_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AngleDistortionCorrection_Response>) -> bool;
}

// Corresponds to lslidar_msgs__srv__AngleDistortionCorrection_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AngleDistortionCorrection_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for AngleDistortionCorrection_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__AngleDistortionCorrection_Response__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__AngleDistortionCorrection_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AngleDistortionCorrection_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__AngleDistortionCorrection_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__AngleDistortionCorrection_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__AngleDistortionCorrection_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AngleDistortionCorrection_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AngleDistortionCorrection_Response where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/AngleDistortionCorrection_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__AngleDistortionCorrection_Response() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__FrameRate_Request() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__FrameRate_Request__init(msg: *mut FrameRate_Request) -> bool;
    fn lslidar_msgs__srv__FrameRate_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FrameRate_Request>, size: usize) -> bool;
    fn lslidar_msgs__srv__FrameRate_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FrameRate_Request>);
    fn lslidar_msgs__srv__FrameRate_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FrameRate_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FrameRate_Request>) -> bool;
}

// Corresponds to lslidar_msgs__srv__FrameRate_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FrameRate_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_rate: u8,

}



impl Default for FrameRate_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__FrameRate_Request__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__FrameRate_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FrameRate_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__FrameRate_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__FrameRate_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__FrameRate_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FrameRate_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FrameRate_Request where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/FrameRate_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__FrameRate_Request() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__FrameRate_Response() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__FrameRate_Response__init(msg: *mut FrameRate_Response) -> bool;
    fn lslidar_msgs__srv__FrameRate_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FrameRate_Response>, size: usize) -> bool;
    fn lslidar_msgs__srv__FrameRate_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FrameRate_Response>);
    fn lslidar_msgs__srv__FrameRate_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FrameRate_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FrameRate_Response>) -> bool;
}

// Corresponds to lslidar_msgs__srv__FrameRate_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FrameRate_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for FrameRate_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__FrameRate_Response__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__FrameRate_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FrameRate_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__FrameRate_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__FrameRate_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__FrameRate_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FrameRate_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FrameRate_Response where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/FrameRate_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__FrameRate_Response() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__InvalidData_Request() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__InvalidData_Request__init(msg: *mut InvalidData_Request) -> bool;
    fn lslidar_msgs__srv__InvalidData_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<InvalidData_Request>, size: usize) -> bool;
    fn lslidar_msgs__srv__InvalidData_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<InvalidData_Request>);
    fn lslidar_msgs__srv__InvalidData_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<InvalidData_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<InvalidData_Request>) -> bool;
}

// Corresponds to lslidar_msgs__srv__InvalidData_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InvalidData_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub invalid_data: u8,

}



impl Default for InvalidData_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__InvalidData_Request__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__InvalidData_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for InvalidData_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__InvalidData_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__InvalidData_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__InvalidData_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for InvalidData_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for InvalidData_Request where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/InvalidData_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__InvalidData_Request() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__InvalidData_Response() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__InvalidData_Response__init(msg: *mut InvalidData_Response) -> bool;
    fn lslidar_msgs__srv__InvalidData_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<InvalidData_Response>, size: usize) -> bool;
    fn lslidar_msgs__srv__InvalidData_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<InvalidData_Response>);
    fn lslidar_msgs__srv__InvalidData_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<InvalidData_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<InvalidData_Response>) -> bool;
}

// Corresponds to lslidar_msgs__srv__InvalidData_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InvalidData_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for InvalidData_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__InvalidData_Response__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__InvalidData_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for InvalidData_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__InvalidData_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__InvalidData_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__InvalidData_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for InvalidData_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for InvalidData_Response where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/InvalidData_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__InvalidData_Response() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__IpAndPort_Request() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__IpAndPort_Request__init(msg: *mut IpAndPort_Request) -> bool;
    fn lslidar_msgs__srv__IpAndPort_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<IpAndPort_Request>, size: usize) -> bool;
    fn lslidar_msgs__srv__IpAndPort_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<IpAndPort_Request>);
    fn lslidar_msgs__srv__IpAndPort_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<IpAndPort_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<IpAndPort_Request>) -> bool;
}

// Corresponds to lslidar_msgs__srv__IpAndPort_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IpAndPort_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub lidar_ip: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub destination_ip: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data_port: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dev_port: u16,

}



impl Default for IpAndPort_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__IpAndPort_Request__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__IpAndPort_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for IpAndPort_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__IpAndPort_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__IpAndPort_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__IpAndPort_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for IpAndPort_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for IpAndPort_Request where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/IpAndPort_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__IpAndPort_Request() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__IpAndPort_Response() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__IpAndPort_Response__init(msg: *mut IpAndPort_Response) -> bool;
    fn lslidar_msgs__srv__IpAndPort_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<IpAndPort_Response>, size: usize) -> bool;
    fn lslidar_msgs__srv__IpAndPort_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<IpAndPort_Response>);
    fn lslidar_msgs__srv__IpAndPort_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<IpAndPort_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<IpAndPort_Response>) -> bool;
}

// Corresponds to lslidar_msgs__srv__IpAndPort_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IpAndPort_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for IpAndPort_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__IpAndPort_Response__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__IpAndPort_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for IpAndPort_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__IpAndPort_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__IpAndPort_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__IpAndPort_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for IpAndPort_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for IpAndPort_Response where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/IpAndPort_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__IpAndPort_Response() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__MotorControl_Request() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__MotorControl_Request__init(msg: *mut MotorControl_Request) -> bool;
    fn lslidar_msgs__srv__MotorControl_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotorControl_Request>, size: usize) -> bool;
    fn lslidar_msgs__srv__MotorControl_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotorControl_Request>);
    fn lslidar_msgs__srv__MotorControl_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotorControl_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MotorControl_Request>) -> bool;
}

// Corresponds to lslidar_msgs__srv__MotorControl_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorControl_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_control: u8,

}



impl Default for MotorControl_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__MotorControl_Request__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__MotorControl_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotorControl_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__MotorControl_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__MotorControl_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__MotorControl_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotorControl_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotorControl_Request where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/MotorControl_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__MotorControl_Request() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__MotorControl_Response() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__MotorControl_Response__init(msg: *mut MotorControl_Response) -> bool;
    fn lslidar_msgs__srv__MotorControl_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotorControl_Response>, size: usize) -> bool;
    fn lslidar_msgs__srv__MotorControl_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotorControl_Response>);
    fn lslidar_msgs__srv__MotorControl_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotorControl_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MotorControl_Response>) -> bool;
}

// Corresponds to lslidar_msgs__srv__MotorControl_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorControl_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for MotorControl_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__MotorControl_Response__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__MotorControl_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotorControl_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__MotorControl_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__MotorControl_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__MotorControl_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotorControl_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotorControl_Response where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/MotorControl_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__MotorControl_Response() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__MotorSpeed_Request() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__MotorSpeed_Request__init(msg: *mut MotorSpeed_Request) -> bool;
    fn lslidar_msgs__srv__MotorSpeed_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotorSpeed_Request>, size: usize) -> bool;
    fn lslidar_msgs__srv__MotorSpeed_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotorSpeed_Request>);
    fn lslidar_msgs__srv__MotorSpeed_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotorSpeed_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MotorSpeed_Request>) -> bool;
}

// Corresponds to lslidar_msgs__srv__MotorSpeed_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorSpeed_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_speed: u8,

}



impl Default for MotorSpeed_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__MotorSpeed_Request__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__MotorSpeed_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotorSpeed_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__MotorSpeed_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__MotorSpeed_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__MotorSpeed_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotorSpeed_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotorSpeed_Request where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/MotorSpeed_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__MotorSpeed_Request() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__MotorSpeed_Response() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__MotorSpeed_Response__init(msg: *mut MotorSpeed_Response) -> bool;
    fn lslidar_msgs__srv__MotorSpeed_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotorSpeed_Response>, size: usize) -> bool;
    fn lslidar_msgs__srv__MotorSpeed_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotorSpeed_Response>);
    fn lslidar_msgs__srv__MotorSpeed_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotorSpeed_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MotorSpeed_Response>) -> bool;
}

// Corresponds to lslidar_msgs__srv__MotorSpeed_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorSpeed_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for MotorSpeed_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__MotorSpeed_Response__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__MotorSpeed_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotorSpeed_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__MotorSpeed_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__MotorSpeed_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__MotorSpeed_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotorSpeed_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotorSpeed_Response where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/MotorSpeed_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__MotorSpeed_Response() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__PowerControl_Request() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__PowerControl_Request__init(msg: *mut PowerControl_Request) -> bool;
    fn lslidar_msgs__srv__PowerControl_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PowerControl_Request>, size: usize) -> bool;
    fn lslidar_msgs__srv__PowerControl_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PowerControl_Request>);
    fn lslidar_msgs__srv__PowerControl_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PowerControl_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PowerControl_Request>) -> bool;
}

// Corresponds to lslidar_msgs__srv__PowerControl_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PowerControl_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub power_control: u8,

}



impl Default for PowerControl_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__PowerControl_Request__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__PowerControl_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PowerControl_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__PowerControl_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__PowerControl_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__PowerControl_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PowerControl_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PowerControl_Request where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/PowerControl_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__PowerControl_Request() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__PowerControl_Response() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__PowerControl_Response__init(msg: *mut PowerControl_Response) -> bool;
    fn lslidar_msgs__srv__PowerControl_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PowerControl_Response>, size: usize) -> bool;
    fn lslidar_msgs__srv__PowerControl_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PowerControl_Response>);
    fn lslidar_msgs__srv__PowerControl_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PowerControl_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PowerControl_Response>) -> bool;
}

// Corresponds to lslidar_msgs__srv__PowerControl_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PowerControl_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for PowerControl_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__PowerControl_Response__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__PowerControl_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PowerControl_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__PowerControl_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__PowerControl_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__PowerControl_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PowerControl_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PowerControl_Response where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/PowerControl_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__PowerControl_Response() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__RfdRemoval_Request() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__RfdRemoval_Request__init(msg: *mut RfdRemoval_Request) -> bool;
    fn lslidar_msgs__srv__RfdRemoval_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RfdRemoval_Request>, size: usize) -> bool;
    fn lslidar_msgs__srv__RfdRemoval_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RfdRemoval_Request>);
    fn lslidar_msgs__srv__RfdRemoval_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RfdRemoval_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RfdRemoval_Request>) -> bool;
}

// Corresponds to lslidar_msgs__srv__RfdRemoval_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RfdRemoval_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub rfd_removal: u8,

}



impl Default for RfdRemoval_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__RfdRemoval_Request__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__RfdRemoval_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RfdRemoval_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__RfdRemoval_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__RfdRemoval_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__RfdRemoval_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RfdRemoval_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RfdRemoval_Request where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/RfdRemoval_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__RfdRemoval_Request() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__RfdRemoval_Response() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__RfdRemoval_Response__init(msg: *mut RfdRemoval_Response) -> bool;
    fn lslidar_msgs__srv__RfdRemoval_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RfdRemoval_Response>, size: usize) -> bool;
    fn lslidar_msgs__srv__RfdRemoval_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RfdRemoval_Response>);
    fn lslidar_msgs__srv__RfdRemoval_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RfdRemoval_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RfdRemoval_Response>) -> bool;
}

// Corresponds to lslidar_msgs__srv__RfdRemoval_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RfdRemoval_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for RfdRemoval_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__RfdRemoval_Response__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__RfdRemoval_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RfdRemoval_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__RfdRemoval_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__RfdRemoval_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__RfdRemoval_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RfdRemoval_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RfdRemoval_Response where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/RfdRemoval_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__RfdRemoval_Response() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__StandbyMode_Request() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__StandbyMode_Request__init(msg: *mut StandbyMode_Request) -> bool;
    fn lslidar_msgs__srv__StandbyMode_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StandbyMode_Request>, size: usize) -> bool;
    fn lslidar_msgs__srv__StandbyMode_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StandbyMode_Request>);
    fn lslidar_msgs__srv__StandbyMode_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StandbyMode_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<StandbyMode_Request>) -> bool;
}

// Corresponds to lslidar_msgs__srv__StandbyMode_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StandbyMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub standby_mode: u8,

}



impl Default for StandbyMode_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__StandbyMode_Request__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__StandbyMode_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StandbyMode_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__StandbyMode_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__StandbyMode_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__StandbyMode_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StandbyMode_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StandbyMode_Request where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/StandbyMode_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__StandbyMode_Request() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__StandbyMode_Response() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__StandbyMode_Response__init(msg: *mut StandbyMode_Response) -> bool;
    fn lslidar_msgs__srv__StandbyMode_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StandbyMode_Response>, size: usize) -> bool;
    fn lslidar_msgs__srv__StandbyMode_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StandbyMode_Response>);
    fn lslidar_msgs__srv__StandbyMode_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StandbyMode_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<StandbyMode_Response>) -> bool;
}

// Corresponds to lslidar_msgs__srv__StandbyMode_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StandbyMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for StandbyMode_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__StandbyMode_Response__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__StandbyMode_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StandbyMode_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__StandbyMode_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__StandbyMode_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__StandbyMode_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StandbyMode_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StandbyMode_Response where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/StandbyMode_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__StandbyMode_Response() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__TailRemoval_Request() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__TailRemoval_Request__init(msg: *mut TailRemoval_Request) -> bool;
    fn lslidar_msgs__srv__TailRemoval_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TailRemoval_Request>, size: usize) -> bool;
    fn lslidar_msgs__srv__TailRemoval_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TailRemoval_Request>);
    fn lslidar_msgs__srv__TailRemoval_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TailRemoval_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<TailRemoval_Request>) -> bool;
}

// Corresponds to lslidar_msgs__srv__TailRemoval_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TailRemoval_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub tail_removal: u8,

}



impl Default for TailRemoval_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__TailRemoval_Request__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__TailRemoval_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TailRemoval_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__TailRemoval_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__TailRemoval_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__TailRemoval_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TailRemoval_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TailRemoval_Request where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/TailRemoval_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__TailRemoval_Request() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__TailRemoval_Response() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__TailRemoval_Response__init(msg: *mut TailRemoval_Response) -> bool;
    fn lslidar_msgs__srv__TailRemoval_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TailRemoval_Response>, size: usize) -> bool;
    fn lslidar_msgs__srv__TailRemoval_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TailRemoval_Response>);
    fn lslidar_msgs__srv__TailRemoval_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TailRemoval_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<TailRemoval_Response>) -> bool;
}

// Corresponds to lslidar_msgs__srv__TailRemoval_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TailRemoval_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for TailRemoval_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__TailRemoval_Response__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__TailRemoval_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TailRemoval_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__TailRemoval_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__TailRemoval_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__TailRemoval_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TailRemoval_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TailRemoval_Response where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/TailRemoval_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__TailRemoval_Response() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__TimeMode_Request() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__TimeMode_Request__init(msg: *mut TimeMode_Request) -> bool;
    fn lslidar_msgs__srv__TimeMode_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TimeMode_Request>, size: usize) -> bool;
    fn lslidar_msgs__srv__TimeMode_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TimeMode_Request>);
    fn lslidar_msgs__srv__TimeMode_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TimeMode_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<TimeMode_Request>) -> bool;
}

// Corresponds to lslidar_msgs__srv__TimeMode_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimeMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub time_mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ntp_ip: rosidl_runtime_rs::String,

}



impl Default for TimeMode_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__TimeMode_Request__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__TimeMode_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TimeMode_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__TimeMode_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__TimeMode_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__TimeMode_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TimeMode_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TimeMode_Request where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/TimeMode_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__TimeMode_Request() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__TimeMode_Response() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__srv__TimeMode_Response__init(msg: *mut TimeMode_Response) -> bool;
    fn lslidar_msgs__srv__TimeMode_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TimeMode_Response>, size: usize) -> bool;
    fn lslidar_msgs__srv__TimeMode_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TimeMode_Response>);
    fn lslidar_msgs__srv__TimeMode_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TimeMode_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<TimeMode_Response>) -> bool;
}

// Corresponds to lslidar_msgs__srv__TimeMode_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimeMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for TimeMode_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__srv__TimeMode_Response__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__srv__TimeMode_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TimeMode_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__TimeMode_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__TimeMode_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__srv__TimeMode_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TimeMode_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TimeMode_Response where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/srv/TimeMode_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__srv__TimeMode_Response() }
  }
}






#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__AngleDistortionCorrection() -> *const std::ffi::c_void;
}

// Corresponds to lslidar_msgs__srv__AngleDistortionCorrection
#[allow(missing_docs, non_camel_case_types)]
pub struct AngleDistortionCorrection;

impl rosidl_runtime_rs::Service for AngleDistortionCorrection {
    type Request = AngleDistortionCorrection_Request;
    type Response = AngleDistortionCorrection_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__AngleDistortionCorrection() }
    }
}




#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__FrameRate() -> *const std::ffi::c_void;
}

// Corresponds to lslidar_msgs__srv__FrameRate
#[allow(missing_docs, non_camel_case_types)]
pub struct FrameRate;

impl rosidl_runtime_rs::Service for FrameRate {
    type Request = FrameRate_Request;
    type Response = FrameRate_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__FrameRate() }
    }
}




#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__InvalidData() -> *const std::ffi::c_void;
}

// Corresponds to lslidar_msgs__srv__InvalidData
#[allow(missing_docs, non_camel_case_types)]
pub struct InvalidData;

impl rosidl_runtime_rs::Service for InvalidData {
    type Request = InvalidData_Request;
    type Response = InvalidData_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__InvalidData() }
    }
}




#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__IpAndPort() -> *const std::ffi::c_void;
}

// Corresponds to lslidar_msgs__srv__IpAndPort
#[allow(missing_docs, non_camel_case_types)]
pub struct IpAndPort;

impl rosidl_runtime_rs::Service for IpAndPort {
    type Request = IpAndPort_Request;
    type Response = IpAndPort_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__IpAndPort() }
    }
}




#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__MotorControl() -> *const std::ffi::c_void;
}

// Corresponds to lslidar_msgs__srv__MotorControl
#[allow(missing_docs, non_camel_case_types)]
pub struct MotorControl;

impl rosidl_runtime_rs::Service for MotorControl {
    type Request = MotorControl_Request;
    type Response = MotorControl_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__MotorControl() }
    }
}




#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__MotorSpeed() -> *const std::ffi::c_void;
}

// Corresponds to lslidar_msgs__srv__MotorSpeed
#[allow(missing_docs, non_camel_case_types)]
pub struct MotorSpeed;

impl rosidl_runtime_rs::Service for MotorSpeed {
    type Request = MotorSpeed_Request;
    type Response = MotorSpeed_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__MotorSpeed() }
    }
}




#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__PowerControl() -> *const std::ffi::c_void;
}

// Corresponds to lslidar_msgs__srv__PowerControl
#[allow(missing_docs, non_camel_case_types)]
pub struct PowerControl;

impl rosidl_runtime_rs::Service for PowerControl {
    type Request = PowerControl_Request;
    type Response = PowerControl_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__PowerControl() }
    }
}




#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__RfdRemoval() -> *const std::ffi::c_void;
}

// Corresponds to lslidar_msgs__srv__RfdRemoval
#[allow(missing_docs, non_camel_case_types)]
pub struct RfdRemoval;

impl rosidl_runtime_rs::Service for RfdRemoval {
    type Request = RfdRemoval_Request;
    type Response = RfdRemoval_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__RfdRemoval() }
    }
}




#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__StandbyMode() -> *const std::ffi::c_void;
}

// Corresponds to lslidar_msgs__srv__StandbyMode
#[allow(missing_docs, non_camel_case_types)]
pub struct StandbyMode;

impl rosidl_runtime_rs::Service for StandbyMode {
    type Request = StandbyMode_Request;
    type Response = StandbyMode_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__StandbyMode() }
    }
}




#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__TailRemoval() -> *const std::ffi::c_void;
}

// Corresponds to lslidar_msgs__srv__TailRemoval
#[allow(missing_docs, non_camel_case_types)]
pub struct TailRemoval;

impl rosidl_runtime_rs::Service for TailRemoval {
    type Request = TailRemoval_Request;
    type Response = TailRemoval_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__TailRemoval() }
    }
}




#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__TimeMode() -> *const std::ffi::c_void;
}

// Corresponds to lslidar_msgs__srv__TimeMode
#[allow(missing_docs, non_camel_case_types)]
pub struct TimeMode;

impl rosidl_runtime_rs::Service for TimeMode {
    type Request = TimeMode_Request;
    type Response = TimeMode_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__lslidar_msgs__srv__TimeMode() }
    }
}


