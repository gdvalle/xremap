use serde::Deserialize;
use zbus::names::{OwnedBusName, OwnedInterfaceName, OwnedMemberName};
use zbus::zvariant::OwnedObjectPath;

#[derive(Clone, Debug, Deserialize)]
pub struct DbusMethodCall {
    #[serde(default)]
    pub bus: BusType,
    pub destination: OwnedBusName,
    pub path: OwnedObjectPath,
    #[serde(default)]
    pub interface: Option<OwnedInterfaceName>,
    pub method: OwnedMemberName,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum BusType {
    #[default]
    Session,
    System,
}
