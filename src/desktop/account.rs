use std::collections::HashMap;
use zbus::{dbus_proxy, fdo::Result};

#[dbus_proxy(
    interface = "org.freedesktop.portal.Account",
    default_path = "/org/freedesktop/portal/desktop"
)]
trait Account {
    /// GetUserInformation method
    fn get_user_information(
        &self,
        window: &str,
        options: HashMap<&str, zvariant::Value>,
    ) -> Result<String>;

    /// version property
    #[dbus_proxy(property)]
    fn version(&self) -> Result<u32>;
}
