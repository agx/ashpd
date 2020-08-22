mod account;
mod background;
mod camera;
mod device;
mod email;
mod file_chooser;
mod inhibit;
mod location;
mod memory_monitor;
mod network_monitor;
mod notification;
mod open_uri;
mod print;
mod remote_desktop;
mod screencast;
mod screenshot;
mod secret;
mod settings;
mod trash;
mod wallpaper;

pub use account::AccountProxy;
pub use background::BackgroundProxy;
pub use camera::CameraProxy;
pub use device::DeviceProxy;
pub use email::EmailProxy;
pub use file_chooser::FileChooserProxy;
pub use inhibit::InhibitProxy;
pub use location::LocationProxy;
pub use memory_monitor::MemoryMonitorProxy;
pub use network_monitor::NetworkMonitorProxy;
pub use notification::NotificationProxy;
pub use open_uri::OpenURIProxy;
pub use print::PrintProxy;
pub use remote_desktop::RemoteDesktopProxy;
pub use screencast::ScreenCastProxy;
pub use screenshot::ScreenshotProxy;
pub use secret::SecretProxy;
pub use settings::SettingsProxy;
pub use trash::TrashProxy;
pub use wallpaper::WallpaperProxy;
