#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__msg__LslidarInformation() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__msg__LslidarInformation__init(msg: *mut LslidarInformation) -> bool;
    fn lslidar_msgs__msg__LslidarInformation__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LslidarInformation>, size: usize) -> bool;
    fn lslidar_msgs__msg__LslidarInformation__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LslidarInformation>);
    fn lslidar_msgs__msg__LslidarInformation__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LslidarInformation>, out_seq: *mut rosidl_runtime_rs::Sequence<LslidarInformation>) -> bool;
}

// Corresponds to lslidar_msgs__msg__LslidarInformation
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LslidarInformation {
    /// 雷达ip地址
    pub lidar_ip: rosidl_runtime_rs::String,

    /// 雷达目的ip地址
    pub destination_ip: rosidl_runtime_rs::String,

    /// 雷达 MAC 地址
    pub lidar_mac_address: rosidl_runtime_rs::String,

    /// 雷达目的数据端口
    pub msop_port: u16,

    /// 雷达目的设备端口
    pub difop_port: u16,

    /// 雷达序列号
    pub lidar_serial_number: rosidl_runtime_rs::String,

    /// FPGA 2号板程序版本
    pub fpga_board_2_program: rosidl_runtime_rs::String,

    /// FPGA 3号板程序版本
    pub fpga_board_3_program: rosidl_runtime_rs::String,

}



impl Default for LslidarInformation {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__msg__LslidarInformation__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__msg__LslidarInformation__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LslidarInformation {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__msg__LslidarInformation__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__msg__LslidarInformation__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__msg__LslidarInformation__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LslidarInformation {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LslidarInformation where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/msg/LslidarInformation";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__msg__LslidarInformation() }
  }
}


#[link(name = "lslidar_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__msg__LslidarPacket() -> *const std::ffi::c_void;
}

#[link(name = "lslidar_msgs__rosidl_generator_c")]
extern "C" {
    fn lslidar_msgs__msg__LslidarPacket__init(msg: *mut LslidarPacket) -> bool;
    fn lslidar_msgs__msg__LslidarPacket__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LslidarPacket>, size: usize) -> bool;
    fn lslidar_msgs__msg__LslidarPacket__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LslidarPacket>);
    fn lslidar_msgs__msg__LslidarPacket__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LslidarPacket>, out_seq: *mut rosidl_runtime_rs::Sequence<LslidarPacket>) -> bool;
}

// Corresponds to lslidar_msgs__msg__LslidarPacket
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Raw Leishen LIDAR packet.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LslidarPacket {

    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,


    // This member is not documented.
    #[allow(missing_docs)]
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub data: [u8; 1212],

}



impl Default for LslidarPacket {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lslidar_msgs__msg__LslidarPacket__init(&mut msg as *mut _) {
        panic!("Call to lslidar_msgs__msg__LslidarPacket__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LslidarPacket {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__msg__LslidarPacket__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__msg__LslidarPacket__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lslidar_msgs__msg__LslidarPacket__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LslidarPacket {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LslidarPacket where Self: Sized {
  const TYPE_NAME: &'static str = "lslidar_msgs/msg/LslidarPacket";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lslidar_msgs__msg__LslidarPacket() }
  }
}


