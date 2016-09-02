use ::Guid;

/// Marker trait for all COM-compatible interfaces.
pub trait ComInterface {
    /// The type that defines the VTable of this interface.
    type Vtbl: Sized;
}

/// Provides a way to get the IID for a COM/WinRT interface.
/// This should be implemented for all interfaces, except parameterized ones,
/// because IIDs of parameterized interfaces depend on concrete instantiations
/// of the parameter types.
pub trait ComIid {
    // TODO: use associated constant once that is stable
    //const IID: REFIID;
    fn iid() -> &'static Guid;
}

// extend some definitions from winapi (re-export existing types where possible!)
DEFINE_IID!(IID_IUnknown, 0x00000000, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46);
pub type IUnknown = ::w::IUnknown;
impl ComIid for IUnknown { fn iid() -> &'static Guid { &IID_IUnknown } }
impl ComInterface for IUnknown { type Vtbl = ::w::IUnknownVtbl; }

DEFINE_IID!(IID_IRestrictedErrorInfo, 0x82BA7092, 0x4C88, 0x427D, 0xA7, 0xBC, 0x16, 0xDD, 0x93, 0xFE, 0xB6, 0x7E);
pub type IRestrictedErrorInfo = ::w::IRestrictedErrorInfo;
pub type IRestrictedErrorInfoVtbl = ::w::IRestrictedErrorInfoVtbl;
impl ComIid for IRestrictedErrorInfo { fn iid() -> &'static Guid { &IID_IRestrictedErrorInfo } }
impl ComInterface for IRestrictedErrorInfo { type Vtbl = IRestrictedErrorInfoVtbl; }

DEFINE_IID!(IID_IAgileObject, 0x94EA2B94, 0xE9CC, 0x49E0, 0xC0, 0xFF, 0xEE, 0x64, 0xCA, 0x8F, 0x5B, 0x90);

#[repr(C)] #[derive(Debug)]
pub struct IAgileObject {
    lpVtbl: *const ::w::IUnknownVtbl // IAgileObject has no methods besides what IUnknown has
}
impl ::std::ops::Deref for IAgileObject {
    type Target = IUnknown;
    #[inline]
    fn deref(&self) -> &IUnknown {
        unsafe { ::std::mem::transmute(self) }
    }
}
impl ::std::ops::DerefMut for IAgileObject {
    #[inline]
    fn deref_mut(&mut self) -> &mut IUnknown {
        unsafe { ::std::mem::transmute(self) }
    }
}
impl ComIid for IAgileObject { fn iid() -> &'static Guid { &IID_IAgileObject } }
impl ComInterface for IAgileObject { type Vtbl = ::w::IUnknownVtbl; }