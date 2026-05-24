#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to lidarslam_msgs__msg__SubMap

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SubMap {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cloud: sensor_msgs::msg::PointCloud2,

}



impl Default for SubMap {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SubMap::default())
  }
}

impl rosidl_runtime_rs::Message for SubMap {
  type RmwMsg = super::msg::rmw::SubMap;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        distance: msg.distance,
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        cloud: sensor_msgs::msg::PointCloud2::into_rmw_message(std::borrow::Cow::Owned(msg.cloud)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      distance: msg.distance,
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
        cloud: sensor_msgs::msg::PointCloud2::into_rmw_message(std::borrow::Cow::Borrowed(&msg.cloud)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      distance: msg.distance,
      pose: geometry_msgs::msg::Pose::from_rmw_message(msg.pose),
      cloud: sensor_msgs::msg::PointCloud2::from_rmw_message(msg.cloud),
    }
  }
}


// Corresponds to lidarslam_msgs__msg__MapArray

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MapArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub submaps: Vec<super::msg::SubMap>,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MapArray::default())
  }
}

impl rosidl_runtime_rs::Message for MapArray {
  type RmwMsg = super::msg::rmw::MapArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        submaps: msg.submaps
          .into_iter()
          .map(|elem| super::msg::SubMap::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        cloud_coordinate: msg.cloud_coordinate,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        submaps: msg.submaps
          .iter()
          .map(|elem| super::msg::SubMap::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      cloud_coordinate: msg.cloud_coordinate,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      submaps: msg.submaps
          .into_iter()
          .map(super::msg::SubMap::from_rmw_message)
          .collect(),
      cloud_coordinate: msg.cloud_coordinate,
    }
  }
}


