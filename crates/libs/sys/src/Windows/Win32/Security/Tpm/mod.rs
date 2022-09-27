pub type ITpmVirtualSmartCardManager = *mut ::core::ffi::c_void;
pub type ITpmVirtualSmartCardManager2 = *mut ::core::ffi::c_void;
pub type ITpmVirtualSmartCardManager3 = *mut ::core::ffi::c_void;
pub type ITpmVirtualSmartCardManagerStatusCallback = *mut ::core::ffi::c_void;
pub const RemoteTpmVirtualSmartCardManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x152ea2a8_70dc_4c59_8b2a_32aa3ca0dcac);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSC_DEFAULT_ADMIN_ALGORITHM_ID: u32 = 130u32;
pub const TpmVirtualSmartCardManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x16a18e86_7f6e_4c20_ad89_4ffc0db7a96a);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub type TPMVSCMGR_ERROR = i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_IMPERSONATION: TPMVSCMGR_ERROR = 0i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_PIN_COMPLEXITY: TPMVSCMGR_ERROR = 1i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_READER_COUNT_LIMIT: TPMVSCMGR_ERROR = 2i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_TERMINAL_SERVICES_SESSION: TPMVSCMGR_ERROR = 3i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_INITIALIZE: TPMVSCMGR_ERROR = 4i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_CREATE: TPMVSCMGR_ERROR = 5i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_DESTROY: TPMVSCMGR_ERROR = 6i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_INITIALIZE: TPMVSCMGR_ERROR = 7i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_CREATE: TPMVSCMGR_ERROR = 8i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_DESTROY: TPMVSCMGR_ERROR = 9i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_WRITE_PROPERTY: TPMVSCMGR_ERROR = 10i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_READ_PROPERTY: TPMVSCMGR_ERROR = 11i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VREADER_INITIALIZE: TPMVSCMGR_ERROR = 12i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VREADER_CREATE: TPMVSCMGR_ERROR = 13i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VREADER_DESTROY: TPMVSCMGR_ERROR = 14i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_GENERATE_LOCATE_READER: TPMVSCMGR_ERROR = 15i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_GENERATE_FILESYSTEM: TPMVSCMGR_ERROR = 16i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_CARD_CREATE: TPMVSCMGR_ERROR = 17i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_CARD_DESTROY: TPMVSCMGR_ERROR = 18i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub type TPMVSCMGR_STATUS = i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_INITIALIZING: TPMVSCMGR_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_CREATING: TPMVSCMGR_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_DESTROYING: TPMVSCMGR_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_INITIALIZING: TPMVSCMGR_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_CREATING: TPMVSCMGR_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_DESTROYING: TPMVSCMGR_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VREADER_INITIALIZING: TPMVSCMGR_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VREADER_CREATING: TPMVSCMGR_STATUS = 7i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VREADER_DESTROYING: TPMVSCMGR_STATUS = 8i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_GENERATE_WAITING: TPMVSCMGR_STATUS = 9i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_GENERATE_AUTHENTICATING: TPMVSCMGR_STATUS = 10i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_GENERATE_RUNNING: TPMVSCMGR_STATUS = 11i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_CARD_CREATED: TPMVSCMGR_STATUS = 12i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_CARD_DESTROYED: TPMVSCMGR_STATUS = 13i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub type TPMVSC_ATTESTATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSC_ATTESTATION_NONE: TPMVSC_ATTESTATION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSC_ATTESTATION_AIK_ONLY: TPMVSC_ATTESTATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSC_ATTESTATION_AIK_AND_CERTIFICATE: TPMVSC_ATTESTATION_TYPE = 2i32;
