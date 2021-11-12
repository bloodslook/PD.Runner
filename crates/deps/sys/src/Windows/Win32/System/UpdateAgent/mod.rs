#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AddServiceFlag(pub i32);
pub const asfAllowPendingRegistration: AddServiceFlag = AddServiceFlag(1i32);
pub const asfAllowOnlineRegistration: AddServiceFlag = AddServiceFlag(2i32);
pub const asfRegisterServiceWithAU: AddServiceFlag = AddServiceFlag(4i32);
impl ::core::marker::Copy for AddServiceFlag {}
impl ::core::clone::Clone for AddServiceFlag {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutoDownloadMode(pub i32);
pub const adLetWindowsUpdateDecide: AutoDownloadMode = AutoDownloadMode(0i32);
pub const adNeverAutoDownload: AutoDownloadMode = AutoDownloadMode(1i32);
pub const adAlwaysAutoDownload: AutoDownloadMode = AutoDownloadMode(2i32);
impl ::core::marker::Copy for AutoDownloadMode {}
impl ::core::clone::Clone for AutoDownloadMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutoSelectionMode(pub i32);
pub const asLetWindowsUpdateDecide: AutoSelectionMode = AutoSelectionMode(0i32);
pub const asAutoSelectIfDownloaded: AutoSelectionMode = AutoSelectionMode(1i32);
pub const asNeverAutoSelect: AutoSelectionMode = AutoSelectionMode(2i32);
pub const asAlwaysAutoSelect: AutoSelectionMode = AutoSelectionMode(3i32);
impl ::core::marker::Copy for AutoSelectionMode {}
impl ::core::clone::Clone for AutoSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub const AutomaticUpdates: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3219230364, data2: 28039, data3: 17488, data4: [179, 124, 224, 47, 11, 55, 56, 3] };
#[repr(transparent)]
pub struct AutomaticUpdatesNotificationLevel(pub i32);
pub const aunlNotConfigured: AutomaticUpdatesNotificationLevel = AutomaticUpdatesNotificationLevel(0i32);
pub const aunlDisabled: AutomaticUpdatesNotificationLevel = AutomaticUpdatesNotificationLevel(1i32);
pub const aunlNotifyBeforeDownload: AutomaticUpdatesNotificationLevel = AutomaticUpdatesNotificationLevel(2i32);
pub const aunlNotifyBeforeInstallation: AutomaticUpdatesNotificationLevel = AutomaticUpdatesNotificationLevel(3i32);
pub const aunlScheduledInstallation: AutomaticUpdatesNotificationLevel = AutomaticUpdatesNotificationLevel(4i32);
impl ::core::marker::Copy for AutomaticUpdatesNotificationLevel {}
impl ::core::clone::Clone for AutomaticUpdatesNotificationLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomaticUpdatesPermissionType(pub i32);
pub const auptSetNotificationLevel: AutomaticUpdatesPermissionType = AutomaticUpdatesPermissionType(1i32);
pub const auptDisableAutomaticUpdates: AutomaticUpdatesPermissionType = AutomaticUpdatesPermissionType(2i32);
pub const auptSetIncludeRecommendedUpdates: AutomaticUpdatesPermissionType = AutomaticUpdatesPermissionType(3i32);
pub const auptSetFeaturedUpdatesEnabled: AutomaticUpdatesPermissionType = AutomaticUpdatesPermissionType(4i32);
pub const auptSetNonAdministratorsElevated: AutomaticUpdatesPermissionType = AutomaticUpdatesPermissionType(5i32);
impl ::core::marker::Copy for AutomaticUpdatesPermissionType {}
impl ::core::clone::Clone for AutomaticUpdatesPermissionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomaticUpdatesScheduledInstallationDay(pub i32);
pub const ausidEveryDay: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(0i32);
pub const ausidEverySunday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(1i32);
pub const ausidEveryMonday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(2i32);
pub const ausidEveryTuesday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(3i32);
pub const ausidEveryWednesday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(4i32);
pub const ausidEveryThursday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(5i32);
pub const ausidEveryFriday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(6i32);
pub const ausidEverySaturday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(7i32);
impl ::core::marker::Copy for AutomaticUpdatesScheduledInstallationDay {}
impl ::core::clone::Clone for AutomaticUpdatesScheduledInstallationDay {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomaticUpdatesUserType(pub i32);
pub const auutCurrentUser: AutomaticUpdatesUserType = AutomaticUpdatesUserType(1i32);
pub const auutLocalAdministrator: AutomaticUpdatesUserType = AutomaticUpdatesUserType(2i32);
impl ::core::marker::Copy for AutomaticUpdatesUserType {}
impl ::core::clone::Clone for AutomaticUpdatesUserType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DeploymentAction(pub i32);
pub const daNone: DeploymentAction = DeploymentAction(0i32);
pub const daInstallation: DeploymentAction = DeploymentAction(1i32);
pub const daUninstallation: DeploymentAction = DeploymentAction(2i32);
pub const daDetection: DeploymentAction = DeploymentAction(3i32);
pub const daOptionalInstallation: DeploymentAction = DeploymentAction(4i32);
impl ::core::marker::Copy for DeploymentAction {}
impl ::core::clone::Clone for DeploymentAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DownloadPhase(pub i32);
pub const dphInitializing: DownloadPhase = DownloadPhase(1i32);
pub const dphDownloading: DownloadPhase = DownloadPhase(2i32);
pub const dphVerifying: DownloadPhase = DownloadPhase(3i32);
impl ::core::marker::Copy for DownloadPhase {}
impl ::core::clone::Clone for DownloadPhase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DownloadPriority(pub i32);
pub const dpLow: DownloadPriority = DownloadPriority(1i32);
pub const dpNormal: DownloadPriority = DownloadPriority(2i32);
pub const dpHigh: DownloadPriority = DownloadPriority(3i32);
pub const dpExtraHigh: DownloadPriority = DownloadPriority(4i32);
impl ::core::marker::Copy for DownloadPriority {}
impl ::core::clone::Clone for DownloadPriority {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomaticUpdates(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomaticUpdates2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomaticUpdatesResults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomaticUpdatesSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomaticUpdatesSettings2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomaticUpdatesSettings3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICategory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICategoryCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDownloadCompletedCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDownloadCompletedCallbackArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDownloadJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDownloadProgress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDownloadProgressChangedCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDownloadProgressChangedCallbackArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDownloadResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInstallationAgent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInstallationBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInstallationCompletedCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInstallationCompletedCallbackArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInstallationJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInstallationProgress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInstallationProgressChangedCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInstallationProgressChangedCallbackArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInstallationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInvalidProductLicenseException(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchCompletedCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchCompletedCallbackArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStringCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdate2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdate3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdate4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdate5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateDownloadContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateDownloadContent2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateDownloadContentCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateDownloadResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateDownloader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateException(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateExceptionCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateHistoryEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateHistoryEntry2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateHistoryEntryCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateIdentity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateInstallationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateInstaller(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateInstaller2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateInstaller3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateInstaller4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateLockdown(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateSearcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateSearcher2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateSearcher3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateService2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateServiceCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateServiceManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateServiceManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateServiceRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateSession2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateSession3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebProxy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowsDriverUpdate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowsDriverUpdate2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowsDriverUpdate3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowsDriverUpdate4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowsDriverUpdate5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowsDriverUpdateEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowsDriverUpdateEntryCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowsUpdateAgentInfo(pub *mut ::core::ffi::c_void);
pub const InstallationAgent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 830378748, data2: 5753, data3: 18173, data4: [160, 181, 240, 137, 20, 221, 134, 35] };
#[repr(transparent)]
pub struct InstallationImpact(pub i32);
pub const iiNormal: InstallationImpact = InstallationImpact(0i32);
pub const iiMinor: InstallationImpact = InstallationImpact(1i32);
pub const iiRequiresExclusiveHandling: InstallationImpact = InstallationImpact(2i32);
impl ::core::marker::Copy for InstallationImpact {}
impl ::core::clone::Clone for InstallationImpact {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InstallationRebootBehavior(pub i32);
pub const irbNeverReboots: InstallationRebootBehavior = InstallationRebootBehavior(0i32);
pub const irbAlwaysRequiresReboot: InstallationRebootBehavior = InstallationRebootBehavior(1i32);
pub const irbCanRequestReboot: InstallationRebootBehavior = InstallationRebootBehavior(2i32);
impl ::core::marker::Copy for InstallationRebootBehavior {}
impl ::core::clone::Clone for InstallationRebootBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
pub const LIBID_WUApiLib: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3046558879, data2: 22245, data3: 16798, data4: [166, 34, 224, 27, 180, 87, 67, 30] };
#[repr(transparent)]
pub struct OperationResultCode(pub i32);
pub const orcNotStarted: OperationResultCode = OperationResultCode(0i32);
pub const orcInProgress: OperationResultCode = OperationResultCode(1i32);
pub const orcSucceeded: OperationResultCode = OperationResultCode(2i32);
pub const orcSucceededWithErrors: OperationResultCode = OperationResultCode(3i32);
pub const orcFailed: OperationResultCode = OperationResultCode(4i32);
pub const orcAborted: OperationResultCode = OperationResultCode(5i32);
impl ::core::marker::Copy for OperationResultCode {}
impl ::core::clone::Clone for OperationResultCode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchScope(pub i32);
pub const searchScopeDefault: SearchScope = SearchScope(0i32);
pub const searchScopeMachineOnly: SearchScope = SearchScope(1i32);
pub const searchScopeCurrentUserOnly: SearchScope = SearchScope(2i32);
pub const searchScopeMachineAndCurrentUser: SearchScope = SearchScope(3i32);
pub const searchScopeMachineAndAllUsers: SearchScope = SearchScope(4i32);
pub const searchScopeAllUsers: SearchScope = SearchScope(5i32);
impl ::core::marker::Copy for SearchScope {}
impl ::core::clone::Clone for SearchScope {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ServerSelection(pub i32);
pub const ssDefault: ServerSelection = ServerSelection(0i32);
pub const ssManagedServer: ServerSelection = ServerSelection(1i32);
pub const ssWindowsUpdate: ServerSelection = ServerSelection(2i32);
pub const ssOthers: ServerSelection = ServerSelection(3i32);
impl ::core::marker::Copy for ServerSelection {}
impl ::core::clone::Clone for ServerSelection {
    fn clone(&self) -> Self {
        *self
    }
}
pub const StringCollection: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1925807476,
    data2: 31803,
    data3: 16558,
    data4: [183, 125, 171, 219, 34, 235, 166, 251],
};
pub const SystemInformation: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3223034784, data2: 48807, data3: 16826, data4: [182, 4, 208, 163, 111, 70, 145, 51] };
pub const UPDATE_LOCKDOWN_WEBSITE_ACCESS: u32 = 1u32;
pub const UpdateCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 325293155, data2: 219, data3: 17990, data4: [128, 61, 82, 128, 38, 20, 13, 136] };
pub const UpdateDownloader: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1538221386,
    data2: 23047,
    data3: 16996,
    data4: [162, 85, 159, 245, 76, 113, 81, 231],
};
#[repr(transparent)]
pub struct UpdateExceptionContext(pub i32);
pub const uecGeneral: UpdateExceptionContext = UpdateExceptionContext(1i32);
pub const uecWindowsDriver: UpdateExceptionContext = UpdateExceptionContext(2i32);
pub const uecWindowsInstaller: UpdateExceptionContext = UpdateExceptionContext(3i32);
pub const uecSearchIncomplete: UpdateExceptionContext = UpdateExceptionContext(4i32);
impl ::core::marker::Copy for UpdateExceptionContext {}
impl ::core::clone::Clone for UpdateExceptionContext {
    fn clone(&self) -> Self {
        *self
    }
}
pub const UpdateInstaller: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3537960575,
    data2: 53822,
    data3: 18657,
    data4: [147, 192, 111, 168, 204, 52, 100, 116],
};
#[repr(transparent)]
pub struct UpdateLockdownOption(pub i32);
pub const uloForWebsiteAccess: UpdateLockdownOption = UpdateLockdownOption(1i32);
impl ::core::marker::Copy for UpdateLockdownOption {}
impl ::core::clone::Clone for UpdateLockdownOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UpdateOperation(pub i32);
pub const uoInstallation: UpdateOperation = UpdateOperation(1i32);
pub const uoUninstallation: UpdateOperation = UpdateOperation(2i32);
impl ::core::marker::Copy for UpdateOperation {}
impl ::core::clone::Clone for UpdateOperation {
    fn clone(&self) -> Self {
        *self
    }
}
pub const UpdateSearcher: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3063539176,
    data2: 26623,
    data3: 16759,
    data4: [136, 176, 54, 132, 163, 56, 139, 251],
};
pub const UpdateServiceManager: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4174533593, data2: 35236, data3: 19882, data4: [135, 182, 17, 104, 54, 159, 11, 33] };
#[repr(transparent)]
pub struct UpdateServiceOption(pub i32);
pub const usoNonVolatileService: UpdateServiceOption = UpdateServiceOption(1i32);
impl ::core::marker::Copy for UpdateServiceOption {}
impl ::core::clone::Clone for UpdateServiceOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UpdateServiceRegistrationState(pub i32);
pub const usrsNotRegistered: UpdateServiceRegistrationState = UpdateServiceRegistrationState(1i32);
pub const usrsRegistrationPending: UpdateServiceRegistrationState = UpdateServiceRegistrationState(2i32);
pub const usrsRegistered: UpdateServiceRegistrationState = UpdateServiceRegistrationState(3i32);
impl ::core::marker::Copy for UpdateServiceRegistrationState {}
impl ::core::clone::Clone for UpdateServiceRegistrationState {
    fn clone(&self) -> Self {
        *self
    }
}
pub const UpdateSession: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1286880639,
    data2: 32494,
    data3: 18694,
    data4: [134, 152, 96, 218, 28, 56, 242, 254],
};
#[repr(transparent)]
pub struct UpdateType(pub i32);
pub const utSoftware: UpdateType = UpdateType(1i32);
pub const utDriver: UpdateType = UpdateType(2i32);
impl ::core::marker::Copy for UpdateType {}
impl ::core::clone::Clone for UpdateType {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WU_E_ALL_UPDATES_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124318i32 as _);
pub const WU_E_AUCLIENT_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107969i32 as _);
pub const WU_E_AU_CALL_CANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124267i32 as _);
pub const WU_E_AU_DETECT_SVCID_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145083386i32 as _);
pub const WU_E_AU_LEGACYCLIENTDISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145083389i32 as _);
pub const WU_E_AU_NONLEGACYSERVER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145083390i32 as _);
pub const WU_E_AU_NOSERVICE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145083392i32 as _);
pub const WU_E_AU_NO_REGISTERED_SERVICE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145083387i32 as _);
pub const WU_E_AU_OOBE_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145083384i32 as _);
pub const WU_E_AU_PAUSED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145083388i32 as _);
pub const WU_E_AU_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145079297i32 as _);
pub const WU_E_BAD_FILE_URL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124282i32 as _);
pub const WU_E_BAD_XML_HARDWARECAPABILITY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145079038i32 as _);
pub const WU_E_BIN_SOURCE_ABSENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124308i32 as _);
pub const WU_E_CALLBACK_COOKIE_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145062907i32 as _);
pub const WU_E_CALL_CANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124341i32 as _);
pub const WU_E_CALL_CANCELLED_BY_HIDE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124262i32 as _);
pub const WU_E_CALL_CANCELLED_BY_INTERACTIVE_SEARCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124253i32 as _);
pub const WU_E_CALL_CANCELLED_BY_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124261i32 as _);
pub const WU_E_CALL_CANCELLED_BY_POLICY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124305i32 as _);
pub const WU_E_COULDNOTCANCEL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124342i32 as _);
pub const WU_E_CYCLE_DETECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124337i32 as _);
pub const WU_E_DM_BG_ERROR_TOKEN_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099761i32 as _);
pub const WU_E_DM_BITSTRANSFERERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099767i32 as _);
pub const WU_E_DM_CONTENTCHANGED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099765i32 as _);
pub const WU_E_DM_DOSVC_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099746i32 as _);
pub const WU_E_DM_DOWNLOADFILEMISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099758i32 as _);
pub const WU_E_DM_DOWNLOADFILEPATHUNKNOWN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099759i32 as _);
pub const WU_E_DM_DOWNLOADLIMITEDBYUPDATESIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099764i32 as _);
pub const WU_E_DM_DOWNLOADLOCATIONCHANGED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099766i32 as _);
pub const WU_E_DM_DOWNLOADSANDBOXNOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099760i32 as _);
pub const WU_E_DM_DOWNLOAD_VOLUME_CONFLICT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099749i32 as _);
pub const WU_E_DM_FAILTOCONNECTTOBITS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099768i32 as _);
pub const WU_E_DM_FALLINGBACKTOBITS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099750i32 as _);
pub const WU_E_DM_HARDRESERVEID_CONFLICT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099747i32 as _);
pub const WU_E_DM_INCORRECTFILEHASH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099774i32 as _);
pub const WU_E_DM_NEEDDOWNLOADREQUEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099772i32 as _);
pub const WU_E_DM_NONETWORK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099771i32 as _);
pub const WU_E_DM_NOTDOWNLOADED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099769i32 as _);
pub const WU_E_DM_READRANGEFAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099756i32 as _);
pub const WU_E_DM_SANDBOX_HASH_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099748i32 as _);
pub const WU_E_DM_UNAUTHORIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099762i32 as _);
pub const WU_E_DM_UNAUTHORIZED_DOMAIN_USER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099752i32 as _);
pub const WU_E_DM_UNAUTHORIZED_LOCAL_USER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099753i32 as _);
pub const WU_E_DM_UNAUTHORIZED_MSA_USER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099751i32 as _);
pub const WU_E_DM_UNAUTHORIZED_NO_USER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099754i32 as _);
pub const WU_E_DM_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095681i32 as _);
pub const WU_E_DM_UNKNOWNALGORITHM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099773i32 as _);
pub const WU_E_DM_UPDATEREMOVED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099757i32 as _);
pub const WU_E_DM_URLNOTAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099775i32 as _);
pub const WU_E_DM_WRONGBITSVERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145099770i32 as _);
pub const WU_E_DOWNLOAD_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124300i32 as _);
pub const WU_E_DRV_DEVICE_PROBLEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145075192i32 as _);
pub const WU_E_DRV_MISSING_ATTRIBUTE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145075195i32 as _);
pub const WU_E_DRV_NOPROP_OR_LEGACY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145075198i32 as _);
pub const WU_E_DRV_NO_METADATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145075196i32 as _);
pub const WU_E_DRV_NO_PRINTER_CONTENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145075193i32 as _);
pub const WU_E_DRV_PRUNED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145075199i32 as _);
pub const WU_E_DRV_REG_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145075197i32 as _);
pub const WU_E_DRV_SYNC_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145075194i32 as _);
pub const WU_E_DRV_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071105i32 as _);
pub const WU_E_DS_BADVERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091578i32 as _);
pub const WU_E_DS_CANNOTREGISTER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091568i32 as _);
pub const WU_E_DS_CANTDELETE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091573i32 as _);
pub const WU_E_DS_DATANOTAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091554i32 as _);
pub const WU_E_DS_DATANOTLOADED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091553i32 as _);
pub const WU_E_DS_DECLINENOTALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091562i32 as _);
pub const WU_E_DS_DUPLICATEUPDATEID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091565i32 as _);
pub const WU_E_DS_IMPERSONATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091555i32 as _);
pub const WU_E_DS_INUSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091583i32 as _);
pub const WU_E_DS_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091582i32 as _);
pub const WU_E_DS_INVALIDOPERATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091558i32 as _);
pub const WU_E_DS_INVALIDTABLENAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091579i32 as _);
pub const WU_E_DS_LOCKTIMEOUTEXPIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091572i32 as _);
pub const WU_E_DS_MISSINGDATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091576i32 as _);
pub const WU_E_DS_MISSINGREF: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091575i32 as _);
pub const WU_E_DS_NEEDWINDOWSSERVICE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091559i32 as _);
pub const WU_E_DS_NOCATEGORIES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091571i32 as _);
pub const WU_E_DS_NODATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091577i32 as _);
pub const WU_E_DS_NODATA_CCR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091546i32 as _);
pub const WU_E_DS_NODATA_COOKIE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091548i32 as _);
pub const WU_E_DS_NODATA_DOWNLOADJOB: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091544i32 as _);
pub const WU_E_DS_NODATA_EULA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091550i32 as _);
pub const WU_E_DS_NODATA_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091545i32 as _);
pub const WU_E_DS_NODATA_NOSUCHREVISION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091552i32 as _);
pub const WU_E_DS_NODATA_NOSUCHUPDATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091551i32 as _);
pub const WU_E_DS_NODATA_SERVICE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091549i32 as _);
pub const WU_E_DS_NODATA_TIMER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091547i32 as _);
pub const WU_E_DS_NODATA_TMI: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091543i32 as _);
pub const WU_E_DS_RESETREQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091556i32 as _);
pub const WU_E_DS_ROWEXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091570i32 as _);
pub const WU_E_DS_SCHEMAMISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091557i32 as _);
pub const WU_E_DS_SERVICEEXPIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091563i32 as _);
pub const WU_E_DS_SESSIONLOCKMISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091560i32 as _);
pub const WU_E_DS_SHUTDOWN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091584i32 as _);
pub const WU_E_DS_STOREFILELOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091569i32 as _);
pub const WU_E_DS_TABLEINCORRECT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091580i32 as _);
pub const WU_E_DS_TABLEMISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091581i32 as _);
pub const WU_E_DS_TABLESESSIONMISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091561i32 as _);
pub const WU_E_DS_UNABLETOSTART: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091567i32 as _);
pub const WU_E_DS_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145087489i32 as _);
pub const WU_E_DS_UNKNOWNHANDLER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091574i32 as _);
pub const WU_E_DS_UNKNOWNSERVICE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091564i32 as _);
pub const WU_E_DUPLICATE_ITEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124333i32 as _);
pub const WU_E_EE_CLUSTER_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145067001i32 as _);
pub const WU_E_EE_INVALID_ATTRIBUTEDATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145067002i32 as _);
pub const WU_E_EE_INVALID_EXPRESSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145067006i32 as _);
pub const WU_E_EE_INVALID_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145067004i32 as _);
pub const WU_E_EE_MISSING_METADATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145067005i32 as _);
pub const WU_E_EE_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145067003i32 as _);
pub const WU_E_EE_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145062913i32 as _);
pub const WU_E_EE_UNKNOWN_EXPRESSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145067007i32 as _);
pub const WU_E_EULAS_DECLINED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124317i32 as _);
pub const WU_E_EULA_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124301i32 as _);
pub const WU_E_EXCLUSIVE_INSTALL_CONFLICT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124327i32 as _);
pub const WU_E_EXTENDEDERROR_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124257i32 as _);
pub const WU_E_EXTENDEDERROR_NOTSET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124258i32 as _);
pub const WU_E_FILETRUST_DUALSIGNATURE_ECC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145078526i32 as _);
pub const WU_E_FILETRUST_DUALSIGNATURE_RSA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145078527i32 as _);
pub const WU_E_FILETRUST_SHA2SIGNATURE_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124255i32 as _);
pub const WU_E_IDLESHUTDOWN_OPCOUNT_DISCOVERY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124273i32 as _);
pub const WU_E_IDLESHUTDOWN_OPCOUNT_DOWNLOAD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124271i32 as _);
pub const WU_E_IDLESHUTDOWN_OPCOUNT_INSTALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124270i32 as _);
pub const WU_E_IDLESHUTDOWN_OPCOUNT_OTHER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124269i32 as _);
pub const WU_E_IDLESHUTDOWN_OPCOUNT_SEARCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124272i32 as _);
pub const WU_E_IDLESHUTDOWN_OPCOUNT_SERVICEREGISTRATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124256i32 as _);
pub const WU_E_INFRASTRUCTUREFILE_INVALID_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124275i32 as _);
pub const WU_E_INFRASTRUCTUREFILE_REQUIRES_SSL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124274i32 as _);
pub const WU_E_INSTALLATION_RESULTS_INVALID_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145112062i32 as _);
pub const WU_E_INSTALLATION_RESULTS_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145112061i32 as _);
pub const WU_E_INSTALLATION_RESULTS_UNKNOWN_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145112063i32 as _);
pub const WU_E_INSTALL_JOB_NOT_SUSPENDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124251i32 as _);
pub const WU_E_INSTALL_JOB_RESUME_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124252i32 as _);
pub const WU_E_INSTALL_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124330i32 as _);
pub const WU_E_INSTALL_USERCONTEXT_ACCESSDENIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124250i32 as _);
pub const WU_E_INTERACTIVE_CALL_CANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124268i32 as _);
pub const WU_E_INVALIDINDEX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124345i32 as _);
pub const WU_E_INVALID_CRITERIA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124302i32 as _);
pub const WU_E_INVALID_EVENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145062909i32 as _);
pub const WU_E_INVALID_EVENT_PAYLOAD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095677i32 as _);
pub const WU_E_INVALID_EVENT_PAYLOADSIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095676i32 as _);
pub const WU_E_INVALID_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124303i32 as _);
pub const WU_E_INVALID_INSTALL_REQUESTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124332i32 as _);
pub const WU_E_INVALID_NOTIFICATION_INFO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124280i32 as _);
pub const WU_E_INVALID_OPERATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124298i32 as _);
pub const WU_E_INVALID_PRODUCT_LICENSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124311i32 as _);
pub const WU_E_INVALID_PROXY_SERVER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124304i32 as _);
pub const WU_E_INVALID_RELATIONSHIP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124335i32 as _);
pub const WU_E_INVALID_SERIALIZATION_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124264i32 as _);
pub const WU_E_INVALID_UPDATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124323i32 as _);
pub const WU_E_INVALID_UPDATE_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124314i32 as _);
pub const WU_E_INVALID_VOLUMEID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124260i32 as _);
pub const WU_E_INVENTORY_GET_INVENTORY_TYPE_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145087486i32 as _);
pub const WU_E_INVENTORY_PARSEFAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145087487i32 as _);
pub const WU_E_INVENTORY_RESULT_UPLOAD_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145087485i32 as _);
pub const WU_E_INVENTORY_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145087484i32 as _);
pub const WU_E_INVENTORY_WMI_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145087483i32 as _);
pub const WU_E_ITEMNOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124344i32 as _);
pub const WU_E_LEGACYSERVER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124309i32 as _);
pub const WU_E_LOW_BATTERY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124276i32 as _);
pub const WU_E_MAX_CAPACITY_REACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124350i32 as _);
pub const WU_E_METADATATRUST_CERTIFICATECHAIN_VERIFICATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095344i32 as _);
pub const WU_E_METADATATRUST_UNTRUSTED_CERTIFICATECHAIN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095343i32 as _);
pub const WU_E_METADATA_BAD_FRAGMENTSIGNING_CONFIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095417i32 as _);
pub const WU_E_METADATA_BAD_SIGNATURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095360i32 as _);
pub const WU_E_METADATA_CERT_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095296i32 as _);
pub const WU_E_METADATA_CERT_UNTRUSTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095293i32 as _);
pub const WU_E_METADATA_CONFIG_INVALID_BINARY_ENCODING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095423i32 as _);
pub const WU_E_METADATA_FAILURE_PROCESSING_FRAGMENTSIGNING_CONFIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095416i32 as _);
pub const WU_E_METADATA_FETCH_CONFIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095422i32 as _);
pub const WU_E_METADATA_INTCERT_BAD_TRANSPORT_ENCODING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095294i32 as _);
pub const WU_E_METADATA_INVALID_PARAMETER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095420i32 as _);
pub const WU_E_METADATA_LEAFCERT_BAD_TRANSPORT_ENCODING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095295i32 as _);
pub const WU_E_METADATA_NOOP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095424i32 as _);
pub const WU_E_METADATA_NO_VERIFICATION_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095418i32 as _);
pub const WU_E_METADATA_SIGNATURE_VERIFY_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095358i32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_ALL_BAD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095321i32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_CACHELOOKUP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095319i32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_CERTCHAIN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095323i32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095328i32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_NODATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095320i32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_REFRESHONLINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095322i32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_SIGNATURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095324i32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095297i32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_UNTRUSTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095326i32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_VALIDITYWINDOW_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095298i32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_VALIDITY_WINDOW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095325i32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_VERIFICATION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095327i32 as _);
pub const WU_E_METADATA_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095419i32 as _);
pub const WU_E_METADATA_UNSUPPORTED_HASH_ALG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095359i32 as _);
pub const WU_E_METADATA_XML_BASE64CERDATA_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095384i32 as _);
pub const WU_E_METADATA_XML_FRAGMENTSIGNING_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095391i32 as _);
pub const WU_E_METADATA_XML_INTERMEDIATECERT_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095386i32 as _);
pub const WU_E_METADATA_XML_LEAFCERT_ID_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095385i32 as _);
pub const WU_E_METADATA_XML_LEAFCERT_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095387i32 as _);
pub const WU_E_METADATA_XML_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095392i32 as _);
pub const WU_E_METADATA_XML_MODE_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095389i32 as _);
pub const WU_E_METADATA_XML_MODE_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095390i32 as _);
pub const WU_E_METADATA_XML_VALIDITY_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095388i32 as _);
pub const WU_E_MISSING_HANDLER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124310i32 as _);
pub const WU_E_MSI_NOT_CONFIGURED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145120254i32 as _);
pub const WU_E_MSI_NOT_PRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145120251i32 as _);
pub const WU_E_MSI_WRONG_APP_CONTEXT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145120252i32 as _);
pub const WU_E_MSI_WRONG_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145120255i32 as _);
pub const WU_E_MSP_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145120253i32 as _);
pub const WU_E_MSP_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116161i32 as _);
pub const WU_E_NETWORK_COST_EXCEEDS_POLICY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124263i32 as _);
pub const WU_E_NON_UI_MODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107971i32 as _);
pub const WU_E_NOOP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124340i32 as _);
pub const WU_E_NOT_APPLICABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124329i32 as _);
pub const WU_E_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124348i32 as _);
pub const WU_E_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124297i32 as _);
pub const WU_E_NO_CONNECTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124321i32 as _);
pub const WU_E_NO_INTERACTIVE_USER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124320i32 as _);
pub const WU_E_NO_SERVER_CORE_SUPPORT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124288i32 as _);
pub const WU_E_NO_SERVICE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124351i32 as _);
pub const WU_E_NO_SUCH_HANDLER_PLUGIN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124265i32 as _);
pub const WU_E_NO_UI_SUPPORT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124285i32 as _);
pub const WU_E_NO_UPDATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124316i32 as _);
pub const WU_E_NO_USERTOKEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124328i32 as _);
pub const WU_E_OL_INVALID_SCANFILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095679i32 as _);
pub const WU_E_OL_NEWCLIENT_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095678i32 as _);
pub const WU_E_OL_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145091585i32 as _);
pub const WU_E_OPERATIONINPROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124343i32 as _);
pub const WU_E_ORPHANED_DOWNLOAD_JOB: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124277i32 as _);
pub const WU_E_OUTOFRANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124279i32 as _);
pub const WU_E_PER_MACHINE_UPDATE_ACCESS_DENIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124284i32 as _);
pub const WU_E_POLICY_NOT_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124326i32 as _);
pub const WU_E_PT_ADDRESS_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123256i32 as _);
pub const WU_E_PT_ADDRESS_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123255i32 as _);
pub const WU_E_PT_CATALOG_SYNC_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123274i32 as _);
pub const WU_E_PT_CONFIG_PROP_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107926i32 as _);
pub const WU_E_PT_DOUBLE_INITIALIZATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107950i32 as _);
pub const WU_E_PT_ECP_FAILURE_TO_DECOMPRESS_CAB_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107916i32 as _);
pub const WU_E_PT_ECP_FAILURE_TO_EXTRACT_DIGEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107917i32 as _);
pub const WU_E_PT_ECP_FILE_LOCATION_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107915i32 as _);
pub const WU_E_PT_ECP_INIT_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107920i32 as _);
pub const WU_E_PT_ECP_INVALID_FILE_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107919i32 as _);
pub const WU_E_PT_ECP_INVALID_METADATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107918i32 as _);
pub const WU_E_PT_ECP_SUCCEEDED_WITH_ERRORS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107921i32 as _);
pub const WU_E_PT_ENDPOINTURL_NOTAVAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123265i32 as _);
pub const WU_E_PT_ENDPOINT_DISCONNECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123264i32 as _);
pub const WU_E_PT_ENDPOINT_REFRESH_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123266i32 as _);
pub const WU_E_PT_ENDPOINT_UNREACHABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123272i32 as _);
pub const WU_E_PT_EXCEEDED_MAX_SERVER_TRIPS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107952i32 as _);
pub const WU_E_PT_FILE_LOCATIONS_CHANGED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107931i32 as _);
pub const WU_E_PT_HTTP_STATUS_BAD_GATEWAY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107935i32 as _);
pub const WU_E_PT_HTTP_STATUS_BAD_METHOD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107942i32 as _);
pub const WU_E_PT_HTTP_STATUS_BAD_REQUEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107946i32 as _);
pub const WU_E_PT_HTTP_STATUS_CONFLICT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107939i32 as _);
pub const WU_E_PT_HTTP_STATUS_DENIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107945i32 as _);
pub const WU_E_PT_HTTP_STATUS_FORBIDDEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107944i32 as _);
pub const WU_E_PT_HTTP_STATUS_GATEWAY_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107933i32 as _);
pub const WU_E_PT_HTTP_STATUS_GONE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107938i32 as _);
pub const WU_E_PT_HTTP_STATUS_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107943i32 as _);
pub const WU_E_PT_HTTP_STATUS_NOT_MAPPED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107925i32 as _);
pub const WU_E_PT_HTTP_STATUS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107936i32 as _);
pub const WU_E_PT_HTTP_STATUS_PROXY_AUTH_REQ: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107941i32 as _);
pub const WU_E_PT_HTTP_STATUS_REQUEST_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107940i32 as _);
pub const WU_E_PT_HTTP_STATUS_SERVER_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107937i32 as _);
pub const WU_E_PT_HTTP_STATUS_SERVICE_UNAVAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107934i32 as _);
pub const WU_E_PT_HTTP_STATUS_VERSION_NOT_SUP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107932i32 as _);
pub const WU_E_PT_INVALID_COMPUTER_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107949i32 as _);
pub const WU_E_PT_INVALID_CONFIG_PROP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107927i32 as _);
pub const WU_E_PT_INVALID_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123271i32 as _);
pub const WU_E_PT_INVALID_OPERATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123263i32 as _);
pub const WU_E_PT_INVALID_URL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123270i32 as _);
pub const WU_E_PT_LOAD_SHEDDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107923i32 as _);
pub const WU_E_PT_NO_AUTH_COOKIES_CREATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107928i32 as _);
pub const WU_E_PT_NO_AUTH_PLUGINS_REQUESTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107929i32 as _);
pub const WU_E_PT_NO_MANAGED_RECOVER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103826i32 as _);
pub const WU_E_PT_NO_TRANSLATION_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123257i32 as _);
pub const WU_E_PT_NUMERIC_OVERFLOW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123261i32 as _);
pub const WU_E_PT_NWS_NOT_LOADED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123269i32 as _);
pub const WU_E_PT_OBJECT_FAULTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123262i32 as _);
pub const WU_E_PT_OPERATION_ABANDONED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123259i32 as _);
pub const WU_E_PT_OPERATION_ABORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123260i32 as _);
pub const WU_E_PT_OTHER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123254i32 as _);
pub const WU_E_PT_PROXY_AUTH_SCHEME_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123268i32 as _);
pub const WU_E_PT_QUOTA_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123258i32 as _);
pub const WU_E_PT_REFRESH_CACHE_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107947i32 as _);
pub const WU_E_PT_REGISTRATION_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107930i32 as _);
pub const WU_E_PT_SAME_REDIR_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103827i32 as _);
pub const WU_E_PT_SECURITY_SYSTEM_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123253i32 as _);
pub const WU_E_PT_SECURITY_VERIFICATION_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123273i32 as _);
pub const WU_E_PT_SOAPCLIENT_BASE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107968i32 as _);
pub const WU_E_PT_SOAPCLIENT_CONNECT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107964i32 as _);
pub const WU_E_PT_SOAPCLIENT_GENERATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107965i32 as _);
pub const WU_E_PT_SOAPCLIENT_INITIALIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107967i32 as _);
pub const WU_E_PT_SOAPCLIENT_OUTOFMEMORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107966i32 as _);
pub const WU_E_PT_SOAPCLIENT_PARSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107958i32 as _);
pub const WU_E_PT_SOAPCLIENT_PARSEFAULT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107960i32 as _);
pub const WU_E_PT_SOAPCLIENT_READ: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107959i32 as _);
pub const WU_E_PT_SOAPCLIENT_SEND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107963i32 as _);
pub const WU_E_PT_SOAPCLIENT_SERVER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107962i32 as _);
pub const WU_E_PT_SOAPCLIENT_SOAPFAULT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107961i32 as _);
pub const WU_E_PT_SOAP_CLIENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107955i32 as _);
pub const WU_E_PT_SOAP_MUST_UNDERSTAND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107956i32 as _);
pub const WU_E_PT_SOAP_SERVER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107954i32 as _);
pub const WU_E_PT_SOAP_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107957i32 as _);
pub const WU_E_PT_SUS_SERVER_NOT_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107951i32 as _);
pub const WU_E_PT_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103873i32 as _);
pub const WU_E_PT_WINHTTP_NAME_NOT_RESOLVED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107924i32 as _);
pub const WU_E_PT_WMI_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107953i32 as _);
pub const WU_E_RANGEOVERLAP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124347i32 as _);
pub const WU_E_REBOOT_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145083385i32 as _);
pub const WU_E_REDIRECTOR_ATTRPROVIDER_EXCEEDED_MAX_NAMEVALUE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103864i32 as _);
pub const WU_E_REDIRECTOR_ATTRPROVIDER_INVALID_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103863i32 as _);
pub const WU_E_REDIRECTOR_ATTRPROVIDER_INVALID_VALUE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103862i32 as _);
pub const WU_E_REDIRECTOR_CONNECT_POLICY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103860i32 as _);
pub const WU_E_REDIRECTOR_ID_SMALLER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103869i32 as _);
pub const WU_E_REDIRECTOR_INVALID_RESPONSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103866i32 as _);
pub const WU_E_REDIRECTOR_LOAD_XML: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103871i32 as _);
pub const WU_E_REDIRECTOR_ONLINE_DISALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103859i32 as _);
pub const WU_E_REDIRECTOR_SLS_GENERIC_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103861i32 as _);
pub const WU_E_REDIRECTOR_S_FALSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103870i32 as _);
pub const WU_E_REDIRECTOR_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103617i32 as _);
pub const WU_E_REDIRECTOR_UNKNOWN_SERVICE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103868i32 as _);
pub const WU_E_REDIRECTOR_UNSUPPORTED_CONTENTTYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103867i32 as _);
pub const WU_E_REG_VALUE_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124334i32 as _);
pub const WU_E_REPORTER_EVENTCACHECORRUPT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145062911i32 as _);
pub const WU_E_REPORTER_EVENTNAMESPACEPARSEFAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145062910i32 as _);
pub const WU_E_REPORTER_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145058817i32 as _);
pub const WU_E_REVERT_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124281i32 as _);
pub const WU_E_SELFUPDATE_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124325i32 as _);
pub const WU_E_SELFUPDATE_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071087i32 as _);
pub const WU_E_SELFUPDATE_REQUIRED_ADMIN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071086i32 as _);
pub const WU_E_SELFUPDATE_SKIP_ON_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071096i32 as _);
pub const WU_E_SERVER_BUSY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145062908i32 as _);
pub const WU_E_SERVICEPROP_NOTAVAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145123267i32 as _);
pub const WU_E_SERVICE_NOT_REGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145095675i32 as _);
pub const WU_E_SERVICE_STOP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124322i32 as _);
pub const WU_E_SETUP_ALREADYRUNNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071091i32 as _);
pub const WU_E_SETUP_ALREADY_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071101i32 as _);
pub const WU_E_SETUP_BLOCKED_CONFIGURATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071093i32 as _);
pub const WU_E_SETUP_DEFERRABLE_REBOOT_PENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071084i32 as _);
pub const WU_E_SETUP_FAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071082i32 as _);
pub const WU_E_SETUP_HANDLER_EXEC_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071089i32 as _);
pub const WU_E_SETUP_INVALID_IDENTDATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071102i32 as _);
pub const WU_E_SETUP_INVALID_INFDATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071103i32 as _);
pub const WU_E_SETUP_INVALID_REGISTRY_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071088i32 as _);
pub const WU_E_SETUP_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124278i32 as _);
pub const WU_E_SETUP_NON_DEFERRABLE_REBOOT_PENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071083i32 as _);
pub const WU_E_SETUP_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071100i32 as _);
pub const WU_E_SETUP_REBOOTREQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071090i32 as _);
pub const WU_E_SETUP_REBOOT_TO_FIX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071092i32 as _);
pub const WU_E_SETUP_REGISTRATION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071097i32 as _);
pub const WU_E_SETUP_SKIP_UPDATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071095i32 as _);
pub const WU_E_SETUP_SOURCE_VERSION_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071099i32 as _);
pub const WU_E_SETUP_TARGET_VERSION_GREATER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071098i32 as _);
pub const WU_E_SETUP_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145067009i32 as _);
pub const WU_E_SETUP_UNSUPPORTED_CONFIGURATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071094i32 as _);
pub const WU_E_SETUP_WRONG_SERVER_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145071085i32 as _);
pub const WU_E_SIH_ACTION_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103611i32 as _);
pub const WU_E_SIH_ANOTHER_INSTANCE_RUNNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103597i32 as _);
pub const WU_E_SIH_BLOCKED_FOR_PLATFORM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103598i32 as _);
pub const WU_E_SIH_DNSRESILIENCY_OFF: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103596i32 as _);
pub const WU_E_SIH_ENGINE_EXCEPTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103599i32 as _);
pub const WU_E_SIH_INVALIDHASH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103609i32 as _);
pub const WU_E_SIH_NONSTDEXCEPTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103600i32 as _);
pub const WU_E_SIH_NO_ENGINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103608i32 as _);
pub const WU_E_SIH_PARSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103605i32 as _);
pub const WU_E_SIH_POLICY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103602i32 as _);
pub const WU_E_SIH_POST_REBOOT_INSTALL_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103607i32 as _);
pub const WU_E_SIH_POST_REBOOT_NO_CACHED_SLS_RESPONSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103606i32 as _);
pub const WU_E_SIH_PPL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103603i32 as _);
pub const WU_E_SIH_SECURITY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103604i32 as _);
pub const WU_E_SIH_SLS_PARSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103610i32 as _);
pub const WU_E_SIH_STDEXCEPTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103601i32 as _);
pub const WU_E_SIH_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103361i32 as _);
pub const WU_E_SIH_VERIFY_DOWNLOAD_ENGINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103615i32 as _);
pub const WU_E_SIH_VERIFY_DOWNLOAD_PAYLOAD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103614i32 as _);
pub const WU_E_SIH_VERIFY_STAGE_ENGINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103613i32 as _);
pub const WU_E_SIH_VERIFY_STAGE_PAYLOAD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145103612i32 as _);
pub const WU_E_SKIPPED_UPDATE_INSTALLATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145079035i32 as _);
pub const WU_E_SLS_INVALID_REVISION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145078783i32 as _);
pub const WU_E_SOURCE_ABSENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124307i32 as _);
pub const WU_E_SYSPREP_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124287i32 as _);
pub const WU_E_SYSTEM_UNSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124266i32 as _);
pub const WU_E_TIME_OUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124319i32 as _);
pub const WU_E_TOOMANYRANGES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124346i32 as _);
pub const WU_E_TOO_DEEP_RELATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124336i32 as _);
pub const WU_E_TOO_MANY_RESYNC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124295i32 as _);
pub const WU_E_TRAYICON_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145112060i32 as _);
pub const WU_E_TRUST_PROVIDER_UNKNOWN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145078524i32 as _);
pub const WU_E_TRUST_SUBJECT_NOT_TRUSTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145078525i32 as _);
pub const WU_E_UH_APPX_DEFAULT_PACKAGE_VOLUME_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116127i32 as _);
pub const WU_E_UH_APPX_INSTALLED_PACKAGE_VOLUME_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116126i32 as _);
pub const WU_E_UH_APPX_INVALID_PACKAGE_VOLUME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116128i32 as _);
pub const WU_E_UH_APPX_NOT_PRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116130i32 as _);
pub const WU_E_UH_APPX_PACKAGE_FAMILY_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116125i32 as _);
pub const WU_E_UH_APPX_SYSTEM_VOLUME_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116124i32 as _);
pub const WU_E_UH_BADCBSPACKAGEID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116141i32 as _);
pub const WU_E_UH_BADHANDLERXML: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116151i32 as _);
pub const WU_E_UH_CALLED_BACK_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116136i32 as _);
pub const WU_E_UH_CANREQUIREINPUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116150i32 as _);
pub const WU_E_UH_CUSTOMINSTALLER_INVALID_SIGNATURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116135i32 as _);
pub const WU_E_UH_DECRYPTFAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116132i32 as _);
pub const WU_E_UH_DOESNOTSUPPORTACTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116156i32 as _);
pub const WU_E_UH_FALLBACKERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116144i32 as _);
pub const WU_E_UH_FALLBACKTOSELFCONTAINED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116148i32 as _);
pub const WU_E_UH_HANDLER_DISABLEDUNTILREBOOT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116131i32 as _);
pub const WU_E_UH_INCONSISTENT_FILE_NAMES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116145i32 as _);
pub const WU_E_UH_INSTALLERFAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116149i32 as _);
pub const WU_E_UH_INSTALLERHUNG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116153i32 as _);
pub const WU_E_UH_INVALIDMETADATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116154i32 as _);
pub const WU_E_UH_INVALID_TARGETSESSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116133i32 as _);
pub const WU_E_UH_LOCALONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116159i32 as _);
pub const WU_E_UH_NEEDANOTHERDOWNLOAD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116147i32 as _);
pub const WU_E_UH_NEW_SERVICING_STACK_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116137i32 as _);
pub const WU_E_UH_NOTIFYFAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116146i32 as _);
pub const WU_E_UH_NOTREADYTOCOMMIT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116129i32 as _);
pub const WU_E_UH_OPERATIONCANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116152i32 as _);
pub const WU_E_UH_POSTREBOOTRESULTUNKNOWN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116139i32 as _);
pub const WU_E_UH_POSTREBOOTSTILLPENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116140i32 as _);
pub const WU_E_UH_POSTREBOOTUNEXPECTEDSTATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116138i32 as _);
pub const WU_E_UH_REMOTEALREADYACTIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116157i32 as _);
pub const WU_E_UH_REMOTEUNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116160i32 as _);
pub const WU_E_UH_TOOMANYDOWNLOADREQUESTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116143i32 as _);
pub const WU_E_UH_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145112065i32 as _);
pub const WU_E_UH_UNEXPECTEDCBSRESPONSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116142i32 as _);
pub const WU_E_UH_UNKNOWNHANDLER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116158i32 as _);
pub const WU_E_UH_UNSUPPORTED_INSTALLCONTEXT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116134i32 as _);
pub const WU_E_UH_WRONGHANDLER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145116155i32 as _);
pub const WU_E_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145120257i32 as _);
pub const WU_E_UNINSTALL_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124312i32 as _);
pub const WU_E_UNKNOWN_HARDWARECAPABILITY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145079039i32 as _);
pub const WU_E_UNKNOWN_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124349i32 as _);
pub const WU_E_UNKNOWN_SERVICE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124286i32 as _);
pub const WU_E_UNRECOGNIZED_VOLUMEID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124259i32 as _);
pub const WU_E_UNSUPPORTED_SEARCHSCOPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124283i32 as _);
pub const WU_E_UPDATE_MERGE_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145079036i32 as _);
pub const WU_E_UPDATE_NOT_APPROVED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124254i32 as _);
pub const WU_E_UPDATE_NOT_PROCESSED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124299i32 as _);
pub const WU_E_URL_TOO_LONG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124313i32 as _);
pub const WU_E_USER_ACCESS_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124315i32 as _);
pub const WU_E_WINHTTP_INVALID_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124296i32 as _);
pub const WU_E_WMI_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145079037i32 as _);
pub const WU_E_WUCLTUI_UNSUPPORTED_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145107970i32 as _);
pub const WU_E_WUTASK_CANCELINSTALL_DISALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145079291i32 as _);
pub const WU_E_WUTASK_INPROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145079295i32 as _);
pub const WU_E_WUTASK_NOT_STARTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145079293i32 as _);
pub const WU_E_WUTASK_RETRY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145079292i32 as _);
pub const WU_E_WUTASK_STATUS_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145079294i32 as _);
pub const WU_E_WU_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124306i32 as _);
pub const WU_E_XML_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124338i32 as _);
pub const WU_E_XML_MISSINGDATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145124339i32 as _);
pub const WU_S_ALREADY_DOWNLOADED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2359304i32 as _);
pub const WU_S_ALREADY_INSTALLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2359302i32 as _);
pub const WU_S_ALREADY_REVERTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2359306i32 as _);
pub const WU_S_ALREADY_UNINSTALLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2359303i32 as _);
pub const WU_S_DM_ALREADYDOWNLOADING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2383873i32 as _);
pub const WU_S_MARKED_FOR_DISCONNECT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2359300i32 as _);
pub const WU_S_METADATA_IGNORED_SIGNATURE_VERIFICATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2388226i32 as _);
pub const WU_S_METADATA_SKIPPED_BY_ENFORCEMENTMODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2388225i32 as _);
pub const WU_S_REBOOT_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2359301i32 as _);
pub const WU_S_SEARCH_CRITERIA_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2359312i32 as _);
pub const WU_S_SEARCH_LOAD_SHEDDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2392065i32 as _);
pub const WU_S_SELFUPDATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2359298i32 as _);
pub const WU_S_SERVICE_STOP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2359297i32 as _);
pub const WU_S_SIH_NOOP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2379777i32 as _);
pub const WU_S_SOME_UPDATES_SKIPPED_ON_BATTERY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2359305i32 as _);
pub const WU_S_UH_DOWNLOAD_SIZE_CALCULATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2367510i32 as _);
pub const WU_S_UH_INSTALLSTILLPENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2367509i32 as _);
pub const WU_S_UPDATE_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2359299i32 as _);
pub const WebProxy: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1694827471,
    data2: 37128,
    data3: 19932,
    data4: [162, 206, 108, 35, 65, 225, 197, 130],
};
pub const WindowsUpdateAgentInfo: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3270020143, data2: 28507, data3: 19114, data4: [137, 75, 85, 200, 71, 173, 58, 45] };