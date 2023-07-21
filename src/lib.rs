#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use std::ffi::c_void;

use windows::{
    core::{ComInterface, IUnknown, Interface, IntoParam, Result, GUID, HRESULT, PCWSTR},
    interface_hierarchy,
    Devices::Custom::DeviceSharingMode,
    Win32::{
        Media::Audio::{ERole, WAVEFORMATEX},
        System::Com::StructuredStorage::PROPVARIANT,
        UI::Shell::PropertiesSystem::PROPERTYKEY,
    },
};

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IPolicyConfig(IUnknown);
interface_hierarchy!(IPolicyConfig, IUnknown);

pub const PolicyConfigClient: GUID = GUID::from_u128(0x870AF99C_171D_4F9E_AF0D_E63DF40C2BC9);
unsafe impl ComInterface for IPolicyConfig {
    const IID: GUID = GUID::from_u128(0xF8679F50_850A_41CF_9C72_430F290290C8);
}

impl IPolicyConfig {
    pub unsafe fn SetDefaultEndpoint(
        &self,
        device_id: impl IntoParam<PCWSTR>,
        role: ERole,
    ) -> Result<()> {
        (Interface::vtable(self).SetDefaultEndpoint)(
            Interface::as_raw(self),
            device_id.into_param().abi(),
            role,
        )
        .ok()
    }
}

unsafe impl Interface for IPolicyConfig {
    type Vtable = IPolicyConfig_Vtbl;
}

#[repr(C)]
#[doc(hidden)]
#[allow(non_camel_case_types)]
#[rustfmt::skip]
pub struct IPolicyConfig_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetMixFormat: unsafe extern "system" fn(this: *mut c_void, PCWSTR, *mut *mut WAVEFORMATEX) -> HRESULT,
    pub GetDeviceFormat: unsafe extern "system" fn(this: *mut c_void, PCWSTR, i32, *mut *mut WAVEFORMATEX) -> HRESULT,
    pub ResetDeviceFormat: unsafe extern "system" fn(this: *mut c_void, PCWSTR) -> HRESULT,
    pub SetDeviceFormat: unsafe extern "system" fn(this: *mut c_void, PCWSTR, *mut WAVEFORMATEX, *mut WAVEFORMATEX) -> HRESULT,
    pub GetProcessingPeriod: unsafe extern "system" fn(this: *mut c_void, PCWSTR, i32, *mut i64, *mut i64) -> HRESULT,
    pub SetProcessingPeriod: unsafe extern "system" fn(this: *mut c_void, PCWSTR, *mut i64) -> HRESULT,
    pub GetShareMode: unsafe extern "system" fn(this: *mut c_void, PCWSTR, *mut DeviceSharingMode) -> HRESULT,
    pub SetShareMode: unsafe extern "system" fn(this: *mut c_void, PCWSTR, *mut DeviceSharingMode) -> HRESULT,
    pub GetPropertyValue: unsafe extern "system" fn(this: *mut c_void, PCWSTR, i32, *const PROPERTYKEY, *mut PROPVARIANT) -> HRESULT,
    pub SetPropertyValue: unsafe extern "system" fn(this: *mut c_void, PCWSTR, i32, *const PROPERTYKEY, *mut PROPVARIANT) -> HRESULT,
    pub SetDefaultEndpoint: unsafe extern "system" fn(this: *mut c_void, PCWSTR, ERole) -> HRESULT,
    pub SetEndpointVisibility: unsafe extern "system" fn(this: *mut c_void, PCWSTR, i32) -> HRESULT,
}
