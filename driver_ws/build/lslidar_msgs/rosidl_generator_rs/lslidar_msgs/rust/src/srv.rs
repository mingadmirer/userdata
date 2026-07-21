#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to lslidar_msgs__srv__AngleDistortionCorrection_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AngleDistortionCorrection_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub angle_distortion_correction: u8,

}



impl Default for AngleDistortionCorrection_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AngleDistortionCorrection_Request::default())
  }
}

impl rosidl_runtime_rs::Message for AngleDistortionCorrection_Request {
  type RmwMsg = super::srv::rmw::AngleDistortionCorrection_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        angle_distortion_correction: msg.angle_distortion_correction,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      angle_distortion_correction: msg.angle_distortion_correction,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      angle_distortion_correction: msg.angle_distortion_correction,
    }
  }
}


// Corresponds to lslidar_msgs__srv__AngleDistortionCorrection_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AngleDistortionCorrection_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for AngleDistortionCorrection_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AngleDistortionCorrection_Response::default())
  }
}

impl rosidl_runtime_rs::Message for AngleDistortionCorrection_Response {
  type RmwMsg = super::srv::rmw::AngleDistortionCorrection_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
    }
  }
}


// Corresponds to lslidar_msgs__srv__FrameRate_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FrameRate_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_rate: u8,

}



impl Default for FrameRate_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FrameRate_Request::default())
  }
}

impl rosidl_runtime_rs::Message for FrameRate_Request {
  type RmwMsg = super::srv::rmw::FrameRate_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        frame_rate: msg.frame_rate,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      frame_rate: msg.frame_rate,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      frame_rate: msg.frame_rate,
    }
  }
}


// Corresponds to lslidar_msgs__srv__FrameRate_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FrameRate_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for FrameRate_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FrameRate_Response::default())
  }
}

impl rosidl_runtime_rs::Message for FrameRate_Response {
  type RmwMsg = super::srv::rmw::FrameRate_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
    }
  }
}


// Corresponds to lslidar_msgs__srv__InvalidData_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InvalidData_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub invalid_data: u8,

}



impl Default for InvalidData_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::InvalidData_Request::default())
  }
}

impl rosidl_runtime_rs::Message for InvalidData_Request {
  type RmwMsg = super::srv::rmw::InvalidData_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        invalid_data: msg.invalid_data,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      invalid_data: msg.invalid_data,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      invalid_data: msg.invalid_data,
    }
  }
}


// Corresponds to lslidar_msgs__srv__InvalidData_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InvalidData_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for InvalidData_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::InvalidData_Response::default())
  }
}

impl rosidl_runtime_rs::Message for InvalidData_Response {
  type RmwMsg = super::srv::rmw::InvalidData_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
    }
  }
}


// Corresponds to lslidar_msgs__srv__IpAndPort_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IpAndPort_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub lidar_ip: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub destination_ip: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data_port: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dev_port: u16,

}



impl Default for IpAndPort_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::IpAndPort_Request::default())
  }
}

impl rosidl_runtime_rs::Message for IpAndPort_Request {
  type RmwMsg = super::srv::rmw::IpAndPort_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        lidar_ip: msg.lidar_ip.as_str().into(),
        destination_ip: msg.destination_ip.as_str().into(),
        data_port: msg.data_port,
        dev_port: msg.dev_port,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        lidar_ip: msg.lidar_ip.as_str().into(),
        destination_ip: msg.destination_ip.as_str().into(),
      data_port: msg.data_port,
      dev_port: msg.dev_port,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      lidar_ip: msg.lidar_ip.to_string(),
      destination_ip: msg.destination_ip.to_string(),
      data_port: msg.data_port,
      dev_port: msg.dev_port,
    }
  }
}


// Corresponds to lslidar_msgs__srv__IpAndPort_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IpAndPort_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for IpAndPort_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::IpAndPort_Response::default())
  }
}

impl rosidl_runtime_rs::Message for IpAndPort_Response {
  type RmwMsg = super::srv::rmw::IpAndPort_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
    }
  }
}


// Corresponds to lslidar_msgs__srv__MotorControl_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorControl_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_control: u8,

}



impl Default for MotorControl_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MotorControl_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MotorControl_Request {
  type RmwMsg = super::srv::rmw::MotorControl_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        motor_control: msg.motor_control,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      motor_control: msg.motor_control,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      motor_control: msg.motor_control,
    }
  }
}


// Corresponds to lslidar_msgs__srv__MotorControl_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorControl_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for MotorControl_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MotorControl_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MotorControl_Response {
  type RmwMsg = super::srv::rmw::MotorControl_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
    }
  }
}


// Corresponds to lslidar_msgs__srv__MotorSpeed_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorSpeed_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_speed: u8,

}



impl Default for MotorSpeed_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MotorSpeed_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MotorSpeed_Request {
  type RmwMsg = super::srv::rmw::MotorSpeed_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        motor_speed: msg.motor_speed,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      motor_speed: msg.motor_speed,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      motor_speed: msg.motor_speed,
    }
  }
}


// Corresponds to lslidar_msgs__srv__MotorSpeed_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorSpeed_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for MotorSpeed_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MotorSpeed_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MotorSpeed_Response {
  type RmwMsg = super::srv::rmw::MotorSpeed_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
    }
  }
}


// Corresponds to lslidar_msgs__srv__PowerControl_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PowerControl_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub power_control: u8,

}



impl Default for PowerControl_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PowerControl_Request::default())
  }
}

impl rosidl_runtime_rs::Message for PowerControl_Request {
  type RmwMsg = super::srv::rmw::PowerControl_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        power_control: msg.power_control,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      power_control: msg.power_control,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      power_control: msg.power_control,
    }
  }
}


// Corresponds to lslidar_msgs__srv__PowerControl_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PowerControl_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for PowerControl_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PowerControl_Response::default())
  }
}

impl rosidl_runtime_rs::Message for PowerControl_Response {
  type RmwMsg = super::srv::rmw::PowerControl_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
    }
  }
}


// Corresponds to lslidar_msgs__srv__RfdRemoval_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RfdRemoval_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub rfd_removal: u8,

}



impl Default for RfdRemoval_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RfdRemoval_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RfdRemoval_Request {
  type RmwMsg = super::srv::rmw::RfdRemoval_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        rfd_removal: msg.rfd_removal,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      rfd_removal: msg.rfd_removal,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      rfd_removal: msg.rfd_removal,
    }
  }
}


// Corresponds to lslidar_msgs__srv__RfdRemoval_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RfdRemoval_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for RfdRemoval_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RfdRemoval_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RfdRemoval_Response {
  type RmwMsg = super::srv::rmw::RfdRemoval_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
    }
  }
}


// Corresponds to lslidar_msgs__srv__StandbyMode_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StandbyMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub standby_mode: u8,

}



impl Default for StandbyMode_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StandbyMode_Request::default())
  }
}

impl rosidl_runtime_rs::Message for StandbyMode_Request {
  type RmwMsg = super::srv::rmw::StandbyMode_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        standby_mode: msg.standby_mode,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      standby_mode: msg.standby_mode,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      standby_mode: msg.standby_mode,
    }
  }
}


// Corresponds to lslidar_msgs__srv__StandbyMode_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StandbyMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for StandbyMode_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StandbyMode_Response::default())
  }
}

impl rosidl_runtime_rs::Message for StandbyMode_Response {
  type RmwMsg = super::srv::rmw::StandbyMode_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
    }
  }
}


// Corresponds to lslidar_msgs__srv__TailRemoval_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TailRemoval_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub tail_removal: u8,

}



impl Default for TailRemoval_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::TailRemoval_Request::default())
  }
}

impl rosidl_runtime_rs::Message for TailRemoval_Request {
  type RmwMsg = super::srv::rmw::TailRemoval_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        tail_removal: msg.tail_removal,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      tail_removal: msg.tail_removal,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      tail_removal: msg.tail_removal,
    }
  }
}


// Corresponds to lslidar_msgs__srv__TailRemoval_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TailRemoval_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for TailRemoval_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::TailRemoval_Response::default())
  }
}

impl rosidl_runtime_rs::Message for TailRemoval_Response {
  type RmwMsg = super::srv::rmw::TailRemoval_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
    }
  }
}


// Corresponds to lslidar_msgs__srv__TimeMode_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimeMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub time_mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ntp_ip: std::string::String,

}



impl Default for TimeMode_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::TimeMode_Request::default())
  }
}

impl rosidl_runtime_rs::Message for TimeMode_Request {
  type RmwMsg = super::srv::rmw::TimeMode_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        time_mode: msg.time_mode,
        ntp_ip: msg.ntp_ip.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      time_mode: msg.time_mode,
        ntp_ip: msg.ntp_ip.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      time_mode: msg.time_mode,
      ntp_ip: msg.ntp_ip.to_string(),
    }
  }
}


// Corresponds to lslidar_msgs__srv__TimeMode_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimeMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for TimeMode_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::TimeMode_Response::default())
  }
}

impl rosidl_runtime_rs::Message for TimeMode_Response {
  type RmwMsg = super::srv::rmw::TimeMode_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
    }
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


