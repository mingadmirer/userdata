#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to lslidar_msgs__msg__LslidarInformation

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LslidarInformation {
    /// 雷达ip地址
    pub lidar_ip: std::string::String,

    /// 雷达目的ip地址
    pub destination_ip: std::string::String,

    /// 雷达 MAC 地址
    pub lidar_mac_address: std::string::String,

    /// 雷达目的数据端口
    pub msop_port: u16,

    /// 雷达目的设备端口
    pub difop_port: u16,

    /// 雷达序列号
    pub lidar_serial_number: std::string::String,

    /// FPGA 2号板程序版本
    pub fpga_board_2_program: std::string::String,

    /// FPGA 3号板程序版本
    pub fpga_board_3_program: std::string::String,

}



impl Default for LslidarInformation {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LslidarInformation::default())
  }
}

impl rosidl_runtime_rs::Message for LslidarInformation {
  type RmwMsg = super::msg::rmw::LslidarInformation;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        lidar_ip: msg.lidar_ip.as_str().into(),
        destination_ip: msg.destination_ip.as_str().into(),
        lidar_mac_address: msg.lidar_mac_address.as_str().into(),
        msop_port: msg.msop_port,
        difop_port: msg.difop_port,
        lidar_serial_number: msg.lidar_serial_number.as_str().into(),
        fpga_board_2_program: msg.fpga_board_2_program.as_str().into(),
        fpga_board_3_program: msg.fpga_board_3_program.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        lidar_ip: msg.lidar_ip.as_str().into(),
        destination_ip: msg.destination_ip.as_str().into(),
        lidar_mac_address: msg.lidar_mac_address.as_str().into(),
      msop_port: msg.msop_port,
      difop_port: msg.difop_port,
        lidar_serial_number: msg.lidar_serial_number.as_str().into(),
        fpga_board_2_program: msg.fpga_board_2_program.as_str().into(),
        fpga_board_3_program: msg.fpga_board_3_program.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      lidar_ip: msg.lidar_ip.to_string(),
      destination_ip: msg.destination_ip.to_string(),
      lidar_mac_address: msg.lidar_mac_address.to_string(),
      msop_port: msg.msop_port,
      difop_port: msg.difop_port,
      lidar_serial_number: msg.lidar_serial_number.to_string(),
      fpga_board_2_program: msg.fpga_board_2_program.to_string(),
      fpga_board_3_program: msg.fpga_board_3_program.to_string(),
    }
  }
}


// Corresponds to lslidar_msgs__msg__LslidarPacket
/// Raw Leishen LIDAR packet.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LslidarPacket {

    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,


    // This member is not documented.
    #[allow(missing_docs)]
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub data: [u8; 1212],

}



impl Default for LslidarPacket {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LslidarPacket::default())
  }
}

impl rosidl_runtime_rs::Message for LslidarPacket {
  type RmwMsg = super::msg::rmw::LslidarPacket;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
        data: msg.data,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
        data: msg.data,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
      data: msg.data,
    }
  }
}


