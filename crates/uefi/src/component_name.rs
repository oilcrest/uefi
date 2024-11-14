use crate::prelude::*;

#[repr(C)]
pub struct ComponentName {
    pub GetDriverName:
        extern "efiapi" fn(&ComponentName, Language: *const u8, DriverName: &mut *mut u16) -> Status,
    pub GetControllerName: extern "efiapi" fn(
        &ComponentName,
        ControllerHandle: Handle,
        ChildHandle: Handle,
        Language: *const u8,
        ControllerName: &mut *mut u16,
    ) -> Status,
    pub SupportedLanguages: *const u8,
}
