#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "lidarslam_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lidarslam_msgs__msg__SubMap() -> *const std::ffi::c_void;
}

#[link(name = "lidarslam_msgs__rosidl_generator_c")]
extern "C" {
    fn lidarslam_msgs__msg__SubMap__init(msg: *mut SubMap) -> bool;
    fn lidarslam_msgs__msg__SubMap__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SubMap>, size: usize) -> bool;
    fn lidarslam_msgs__msg__SubMap__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SubMap>);
    fn lidarslam_msgs__msg__SubMap__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SubMap>, out_seq: *mut rosidl_runtime_rs::Sequence<SubMap>) -> bool;
}

// Corresponds to lidarslam_msgs__msg__SubMap
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SubMap {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cloud: sensor_msgs::msg::rmw::PointCloud2,

}



impl Default for SubMap {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lidarslam_msgs__msg__SubMap__init(&mut msg as *mut _) {
        panic!("Call to lidarslam_msgs__msg__SubMap__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SubMap {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lidarslam_msgs__msg__SubMap__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lidarslam_msgs__msg__SubMap__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lidarslam_msgs__msg__SubMap__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SubMap {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SubMap where Self: Sized {
  const TYPE_NAME: &'static str = "lidarslam_msgs/msg/SubMap";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lidarslam_msgs__msg__SubMap() }
  }
}


#[link(name = "lidarslam_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__lidarslam_msgs__msg__MapArray() -> *const std::ffi::c_void;
}

#[link(name = "lidarslam_msgs__rosidl_generator_c")]
extern "C" {
    fn lidarslam_msgs__msg__MapArray__init(msg: *mut MapArray) -> bool;
    fn lidarslam_msgs__msg__MapArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MapArray>, size: usize) -> bool;
    fn lidarslam_msgs__msg__MapArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MapArray>);
    fn lidarslam_msgs__msg__MapArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MapArray>, out_seq: *mut rosidl_runtime_rs::Sequence<MapArray>) -> bool;
}

// Corresponds to lidarslam_msgs__msg__MapArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MapArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub submaps: rosidl_runtime_rs::Sequence<super::super::msg::rmw::SubMap>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cloud_coordinate: i8,

}

impl MapArray {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LOCAL: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const GLOBAL: i8 = 1;

}


impl Default for MapArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !lidarslam_msgs__msg__MapArray__init(&mut msg as *mut _) {
        panic!("Call to lidarslam_msgs__msg__MapArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MapArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lidarslam_msgs__msg__MapArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lidarslam_msgs__msg__MapArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { lidarslam_msgs__msg__MapArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MapArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MapArray where Self: Sized {
  const TYPE_NAME: &'static str = "lidarslam_msgs/msg/MapArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__lidarslam_msgs__msg__MapArray() }
  }
}


