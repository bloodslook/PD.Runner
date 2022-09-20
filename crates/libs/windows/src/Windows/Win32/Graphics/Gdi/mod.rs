#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateFontPackage(puchsrcbuffer: *const u8, ulsrcbuffersize: u32, ppuchfontpackagebuffer: *mut *mut u8, pulfontpackagebuffersize: *mut u32, pulbyteswritten: *mut u32, usflag: u16, usttcindex: u16, ussubsetformat: u16, ussubsetlanguage: u16, ussubsetplatform: CREATE_FONT_PACKAGE_SUBSET_PLATFORM, ussubsetencoding: CREATE_FONT_PACKAGE_SUBSET_ENCODING, pussubsetkeeplist: *const u16, ussubsetlistcount: u16, lpfnallocate: CFP_ALLOCPROC, lpfnreallocate: CFP_REALLOCPROC, lpfnfree: CFP_FREEPROC, lpvreserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn CreateFontPackage(puchsrcbuffer: *const u8, ulsrcbuffersize: u32, ppuchfontpackagebuffer: *mut *mut u8, pulfontpackagebuffersize: *mut u32, pulbyteswritten: *mut u32, usflag: u16, usttcindex: u16, ussubsetformat: u16, ussubsetlanguage: u16, ussubsetplatform: CREATE_FONT_PACKAGE_SUBSET_PLATFORM, ussubsetencoding: CREATE_FONT_PACKAGE_SUBSET_ENCODING, pussubsetkeeplist: *const u16, ussubsetlistcount: u16, lpfnallocate: *mut ::core::ffi::c_void, lpfnreallocate: *mut ::core::ffi::c_void, lpfnfree: *mut ::core::ffi::c_void, lpvreserved: *mut ::core::ffi::c_void) -> u32;
    }
    CreateFontPackage(
        ::core::mem::transmute(puchsrcbuffer),
        ulsrcbuffersize,
        ::core::mem::transmute(ppuchfontpackagebuffer),
        ::core::mem::transmute(pulfontpackagebuffersize),
        ::core::mem::transmute(pulbyteswritten),
        usflag,
        usttcindex,
        ussubsetformat,
        ussubsetlanguage,
        ussubsetplatform,
        ussubsetencoding,
        ::core::mem::transmute(pussubsetkeeplist),
        ussubsetlistcount,
        ::core::mem::transmute(lpfnallocate),
        ::core::mem::transmute(lpfnreallocate),
        ::core::mem::transmute(lpfnfree),
        ::core::mem::transmute(lpvreserved),
    )
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn MergeFontPackage(puchmergefontbuffer: *const u8, ulmergefontbuffersize: u32, puchfontpackagebuffer: *const u8, ulfontpackagebuffersize: u32, ppuchdestbuffer: *mut *mut u8, puldestbuffersize: *mut u32, pulbyteswritten: *mut u32, usmode: u16, lpfnallocate: CFP_ALLOCPROC, lpfnreallocate: CFP_REALLOCPROC, lpfnfree: CFP_FREEPROC, lpvreserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn MergeFontPackage(puchmergefontbuffer: *const u8, ulmergefontbuffersize: u32, puchfontpackagebuffer: *const u8, ulfontpackagebuffersize: u32, ppuchdestbuffer: *mut *mut u8, puldestbuffersize: *mut u32, pulbyteswritten: *mut u32, usmode: u16, lpfnallocate: *mut ::core::ffi::c_void, lpfnreallocate: *mut ::core::ffi::c_void, lpfnfree: *mut ::core::ffi::c_void, lpvreserved: *mut ::core::ffi::c_void) -> u32;
    }
    MergeFontPackage(::core::mem::transmute(puchmergefontbuffer), ulmergefontbuffersize, ::core::mem::transmute(puchfontpackagebuffer), ulfontpackagebuffersize, ::core::mem::transmute(ppuchdestbuffer), ::core::mem::transmute(puldestbuffersize), ::core::mem::transmute(pulbyteswritten), usmode, ::core::mem::transmute(lpfnallocate), ::core::mem::transmute(lpfnreallocate), ::core::mem::transmute(lpfnfree), ::core::mem::transmute(lpvreserved))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AbortPath<'a, P0>(hdc: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AbortPath(hdc: HDC) -> super::super::Foundation::BOOL;
    }
    AbortPath(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddFontMemResourceEx(pfileview: &[u8], pvresrved: *mut ::core::ffi::c_void, pnumfonts: *const u32) -> super::super::Foundation::HANDLE {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AddFontMemResourceEx(pfileview: *const ::core::ffi::c_void, cjsize: u32, pvresrved: *mut ::core::ffi::c_void, pnumfonts: *const u32) -> super::super::Foundation::HANDLE;
    }
    AddFontMemResourceEx(::core::mem::transmute(pfileview.as_ptr()), pfileview.len() as _, ::core::mem::transmute(pvresrved), ::core::mem::transmute(pnumfonts))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn AddFontResourceA<'a, P0>(param0: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AddFontResourceA(param0: ::windows::core::PCSTR) -> i32;
    }
    AddFontResourceA(param0.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn AddFontResourceExA<'a, P0>(name: P0, fl: FONT_RESOURCE_CHARACTERISTICS, res: *mut ::core::ffi::c_void) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AddFontResourceExA(name: ::windows::core::PCSTR, fl: FONT_RESOURCE_CHARACTERISTICS, res: *mut ::core::ffi::c_void) -> i32;
    }
    AddFontResourceExA(name.into(), fl, ::core::mem::transmute(res))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn AddFontResourceExW<'a, P0>(name: P0, fl: FONT_RESOURCE_CHARACTERISTICS, res: *mut ::core::ffi::c_void) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AddFontResourceExW(name: ::windows::core::PCWSTR, fl: FONT_RESOURCE_CHARACTERISTICS, res: *mut ::core::ffi::c_void) -> i32;
    }
    AddFontResourceExW(name.into(), fl, ::core::mem::transmute(res))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn AddFontResourceW<'a, P0>(param0: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AddFontResourceW(param0: ::windows::core::PCWSTR) -> i32;
    }
    AddFontResourceW(param0.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AlphaBlend<'a, P0, P1>(hdcdest: P0, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: P1, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, ftn: BLENDFUNCTION) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AlphaBlend(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, ftn: BLENDFUNCTION) -> super::super::Foundation::BOOL;
    }
    AlphaBlend(hdcdest.into(), xorigindest, yorigindest, wdest, hdest, hdcsrc.into(), xoriginsrc, yoriginsrc, wsrc, hsrc, ::core::mem::transmute(ftn))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AngleArc<'a, P0>(hdc: P0, x: i32, y: i32, r: u32, startangle: f32, sweepangle: f32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AngleArc(hdc: HDC, x: i32, y: i32, r: u32, startangle: f32, sweepangle: f32) -> super::super::Foundation::BOOL;
    }
    AngleArc(hdc.into(), x, y, r, startangle, sweepangle)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AnimatePalette<'a, P0>(hpal: P0, istartindex: u32, ppe: &[PALETTEENTRY]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HPALETTE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AnimatePalette(hpal: HPALETTE, istartindex: u32, centries: u32, ppe: *const PALETTEENTRY) -> super::super::Foundation::BOOL;
    }
    AnimatePalette(hpal.into(), istartindex, ppe.len() as _, ::core::mem::transmute(ppe.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Arc<'a, P0>(hdc: P0, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn Arc(hdc: HDC, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> super::super::Foundation::BOOL;
    }
    Arc(hdc.into(), x1, y1, x2, y2, x3, y3, x4, y4)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ArcTo<'a, P0>(hdc: P0, left: i32, top: i32, right: i32, bottom: i32, xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ArcTo(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32, xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> super::super::Foundation::BOOL;
    }
    ArcTo(hdc.into(), left, top, right, bottom, xr1, yr1, xr2, yr2)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BeginPaint<'a, P0>(hwnd: P0, lppaint: *mut PAINTSTRUCT) -> HDC
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn BeginPaint(hwnd: super::super::Foundation::HWND, lppaint: *mut PAINTSTRUCT) -> HDC;
    }
    BeginPaint(hwnd.into(), ::core::mem::transmute(lppaint))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BeginPath<'a, P0>(hdc: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn BeginPath(hdc: HDC) -> super::super::Foundation::BOOL;
    }
    BeginPath(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BitBlt<'a, P0, P1>(hdc: P0, x: i32, y: i32, cx: i32, cy: i32, hdcsrc: P1, x1: i32, y1: i32, rop: ROP_CODE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn BitBlt(hdc: HDC, x: i32, y: i32, cx: i32, cy: i32, hdcsrc: HDC, x1: i32, y1: i32, rop: ROP_CODE) -> super::super::Foundation::BOOL;
    }
    BitBlt(hdc.into(), x, y, cx, cy, hdcsrc.into(), x1, y1, rop)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelDC<'a, P0>(hdc: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CancelDC(hdc: HDC) -> super::super::Foundation::BOOL;
    }
    CancelDC(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChangeDisplaySettingsA(lpdevmode: ::core::option::Option<*const DEVMODEA>, dwflags: CDS_TYPE) -> DISP_CHANGE {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ChangeDisplaySettingsA(lpdevmode: *const DEVMODEA, dwflags: CDS_TYPE) -> DISP_CHANGE;
    }
    ChangeDisplaySettingsA(::core::mem::transmute(lpdevmode.unwrap_or(::std::ptr::null())), dwflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChangeDisplaySettingsExA<'a, P0, P1>(lpszdevicename: P0, lpdevmode: ::core::option::Option<*const DEVMODEA>, hwnd: P1, dwflags: CDS_TYPE, lparam: ::core::option::Option<*const ::core::ffi::c_void>) -> DISP_CHANGE
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ChangeDisplaySettingsExA(lpszdevicename: ::windows::core::PCSTR, lpdevmode: *const DEVMODEA, hwnd: super::super::Foundation::HWND, dwflags: CDS_TYPE, lparam: *const ::core::ffi::c_void) -> DISP_CHANGE;
    }
    ChangeDisplaySettingsExA(lpszdevicename.into(), ::core::mem::transmute(lpdevmode.unwrap_or(::std::ptr::null())), hwnd.into(), dwflags, ::core::mem::transmute(lparam.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChangeDisplaySettingsExW<'a, P0, P1>(lpszdevicename: P0, lpdevmode: ::core::option::Option<*const DEVMODEW>, hwnd: P1, dwflags: CDS_TYPE, lparam: ::core::option::Option<*const ::core::ffi::c_void>) -> DISP_CHANGE
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ChangeDisplaySettingsExW(lpszdevicename: ::windows::core::PCWSTR, lpdevmode: *const DEVMODEW, hwnd: super::super::Foundation::HWND, dwflags: CDS_TYPE, lparam: *const ::core::ffi::c_void) -> DISP_CHANGE;
    }
    ChangeDisplaySettingsExW(lpszdevicename.into(), ::core::mem::transmute(lpdevmode.unwrap_or(::std::ptr::null())), hwnd.into(), dwflags, ::core::mem::transmute(lparam.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChangeDisplaySettingsW(lpdevmode: ::core::option::Option<*const DEVMODEW>, dwflags: CDS_TYPE) -> DISP_CHANGE {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ChangeDisplaySettingsW(lpdevmode: *const DEVMODEW, dwflags: CDS_TYPE) -> DISP_CHANGE;
    }
    ChangeDisplaySettingsW(::core::mem::transmute(lpdevmode.unwrap_or(::std::ptr::null())), dwflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Chord<'a, P0>(hdc: P0, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn Chord(hdc: HDC, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> super::super::Foundation::BOOL;
    }
    Chord(hdc.into(), x1, y1, x2, y2, x3, y3, x4, y4)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClientToScreen<'a, P0>(hwnd: P0, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ClientToScreen(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    ClientToScreen(hwnd.into(), ::core::mem::transmute(lppoint))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CloseEnhMetaFile<'a, P0>(hdc: P0) -> HENHMETAFILE
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CloseEnhMetaFile(hdc: HDC) -> HENHMETAFILE;
    }
    CloseEnhMetaFile(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseFigure<'a, P0>(hdc: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CloseFigure(hdc: HDC) -> super::super::Foundation::BOOL;
    }
    CloseFigure(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CloseMetaFile<'a, P0>(hdc: P0) -> HMETAFILE
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CloseMetaFile(hdc: HDC) -> HMETAFILE;
    }
    CloseMetaFile(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CombineRgn<'a, P0, P1, P2>(hrgndst: P0, hrgnsrc1: P1, hrgnsrc2: P2, imode: RGN_COMBINE_MODE) -> GDI_REGION_TYPE
where
    P0: ::std::convert::Into<HRGN>,
    P1: ::std::convert::Into<HRGN>,
    P2: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CombineRgn(hrgndst: HRGN, hrgnsrc1: HRGN, hrgnsrc2: HRGN, imode: RGN_COMBINE_MODE) -> GDI_REGION_TYPE;
    }
    CombineRgn(hrgndst.into(), hrgnsrc1.into(), hrgnsrc2.into(), imode)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CombineTransform(lpxfout: *mut XFORM, lpxf1: *const XFORM, lpxf2: *const XFORM) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CombineTransform(lpxfout: *mut XFORM, lpxf1: *const XFORM, lpxf2: *const XFORM) -> super::super::Foundation::BOOL;
    }
    CombineTransform(::core::mem::transmute(lpxfout), ::core::mem::transmute(lpxf1), ::core::mem::transmute(lpxf2))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CopyEnhMetaFileA<'a, P0, P1>(henh: P0, lpfilename: P1) -> HENHMETAFILE
where
    P0: ::std::convert::Into<HENHMETAFILE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CopyEnhMetaFileA(henh: HENHMETAFILE, lpfilename: ::windows::core::PCSTR) -> HENHMETAFILE;
    }
    CopyEnhMetaFileA(henh.into(), lpfilename.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CopyEnhMetaFileW<'a, P0, P1>(henh: P0, lpfilename: P1) -> HENHMETAFILE
where
    P0: ::std::convert::Into<HENHMETAFILE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CopyEnhMetaFileW(henh: HENHMETAFILE, lpfilename: ::windows::core::PCWSTR) -> HENHMETAFILE;
    }
    CopyEnhMetaFileW(henh.into(), lpfilename.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CopyMetaFileA<'a, P0, P1>(param0: P0, param1: P1) -> HMETAFILE
where
    P0: ::std::convert::Into<HMETAFILE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CopyMetaFileA(param0: HMETAFILE, param1: ::windows::core::PCSTR) -> HMETAFILE;
    }
    CopyMetaFileA(param0.into(), param1.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CopyMetaFileW<'a, P0, P1>(param0: P0, param1: P1) -> HMETAFILE
where
    P0: ::std::convert::Into<HMETAFILE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CopyMetaFileW(param0: HMETAFILE, param1: ::windows::core::PCWSTR) -> HMETAFILE;
    }
    CopyMetaFileW(param0.into(), param1.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopyRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CopyRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    CopyRect(::core::mem::transmute(lprcdst), ::core::mem::transmute(lprcsrc))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateBitmap(nwidth: i32, nheight: i32, nplanes: u32, nbitcount: u32, lpbits: ::core::option::Option<*const ::core::ffi::c_void>) -> HBITMAP {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateBitmap(nwidth: i32, nheight: i32, nplanes: u32, nbitcount: u32, lpbits: *const ::core::ffi::c_void) -> HBITMAP;
    }
    CreateBitmap(nwidth, nheight, nplanes, nbitcount, ::core::mem::transmute(lpbits.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateBitmapIndirect(pbm: *const BITMAP) -> HBITMAP {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateBitmapIndirect(pbm: *const BITMAP) -> HBITMAP;
    }
    CreateBitmapIndirect(::core::mem::transmute(pbm))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateBrushIndirect(plbrush: *const LOGBRUSH) -> HBRUSH {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateBrushIndirect(plbrush: *const LOGBRUSH) -> HBRUSH;
    }
    CreateBrushIndirect(::core::mem::transmute(plbrush))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateCompatibleBitmap<'a, P0>(hdc: P0, cx: i32, cy: i32) -> HBITMAP
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateCompatibleBitmap(hdc: HDC, cx: i32, cy: i32) -> HBITMAP;
    }
    CreateCompatibleBitmap(hdc.into(), cx, cy)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateCompatibleDC<'a, P0>(hdc: P0) -> CreatedHDC
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateCompatibleDC(hdc: HDC) -> CreatedHDC;
    }
    CreateCompatibleDC(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDCA<'a, P0, P1, P2>(pwszdriver: P0, pwszdevice: P1, pszport: P2, pdm: ::core::option::Option<*const DEVMODEA>) -> CreatedHDC
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDCA(pwszdriver: ::windows::core::PCSTR, pwszdevice: ::windows::core::PCSTR, pszport: ::windows::core::PCSTR, pdm: *const DEVMODEA) -> CreatedHDC;
    }
    CreateDCA(pwszdriver.into(), pwszdevice.into(), pszport.into(), ::core::mem::transmute(pdm.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDCW<'a, P0, P1, P2>(pwszdriver: P0, pwszdevice: P1, pszport: P2, pdm: ::core::option::Option<*const DEVMODEW>) -> CreatedHDC
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDCW(pwszdriver: ::windows::core::PCWSTR, pwszdevice: ::windows::core::PCWSTR, pszport: ::windows::core::PCWSTR, pdm: *const DEVMODEW) -> CreatedHDC;
    }
    CreateDCW(pwszdriver.into(), pwszdevice.into(), pszport.into(), ::core::mem::transmute(pdm.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateDIBPatternBrush(h: isize, iusage: DIB_USAGE) -> HBRUSH {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDIBPatternBrush(h: isize, iusage: DIB_USAGE) -> HBRUSH;
    }
    CreateDIBPatternBrush(h, iusage)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateDIBPatternBrushPt(lppackeddib: *const ::core::ffi::c_void, iusage: DIB_USAGE) -> HBRUSH {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDIBPatternBrushPt(lppackeddib: *const ::core::ffi::c_void, iusage: DIB_USAGE) -> HBRUSH;
    }
    CreateDIBPatternBrushPt(::core::mem::transmute(lppackeddib), iusage)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDIBSection<'a, P0, P1>(hdc: P0, pbmi: *const BITMAPINFO, usage: DIB_USAGE, ppvbits: *mut *mut ::core::ffi::c_void, hsection: P1, offset: u32) -> ::windows::core::Result<HBITMAP>
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDIBSection(hdc: HDC, pbmi: *const BITMAPINFO, usage: DIB_USAGE, ppvbits: *mut *mut ::core::ffi::c_void, hsection: super::super::Foundation::HANDLE, offset: u32) -> HBITMAP;
    }
    let result__ = CreateDIBSection(hdc.into(), ::core::mem::transmute(pbmi), usage, ::core::mem::transmute(ppvbits), hsection.into(), offset);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateDIBitmap<'a, P0>(hdc: P0, pbmih: ::core::option::Option<*const BITMAPINFOHEADER>, flinit: u32, pjbits: ::core::option::Option<*const ::core::ffi::c_void>, pbmi: ::core::option::Option<*const BITMAPINFO>, iusage: DIB_USAGE) -> HBITMAP
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDIBitmap(hdc: HDC, pbmih: *const BITMAPINFOHEADER, flinit: u32, pjbits: *const ::core::ffi::c_void, pbmi: *const BITMAPINFO, iusage: DIB_USAGE) -> HBITMAP;
    }
    CreateDIBitmap(hdc.into(), ::core::mem::transmute(pbmih.unwrap_or(::std::ptr::null())), flinit, ::core::mem::transmute(pjbits.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pbmi.unwrap_or(::std::ptr::null())), iusage)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateDiscardableBitmap<'a, P0>(hdc: P0, cx: i32, cy: i32) -> HBITMAP
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDiscardableBitmap(hdc: HDC, cx: i32, cy: i32) -> HBITMAP;
    }
    CreateDiscardableBitmap(hdc.into(), cx, cy)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateEllipticRgn(x1: i32, y1: i32, x2: i32, y2: i32) -> HRGN {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateEllipticRgn(x1: i32, y1: i32, x2: i32, y2: i32) -> HRGN;
    }
    CreateEllipticRgn(x1, y1, x2, y2)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateEllipticRgnIndirect(lprect: *const super::super::Foundation::RECT) -> HRGN {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateEllipticRgnIndirect(lprect: *const super::super::Foundation::RECT) -> HRGN;
    }
    CreateEllipticRgnIndirect(::core::mem::transmute(lprect))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateEnhMetaFileA<'a, P0, P1, P2>(hdc: P0, lpfilename: P1, lprc: ::core::option::Option<*const super::super::Foundation::RECT>, lpdesc: P2) -> HdcMetdataEnhFileHandle
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateEnhMetaFileA(hdc: HDC, lpfilename: ::windows::core::PCSTR, lprc: *const super::super::Foundation::RECT, lpdesc: ::windows::core::PCSTR) -> HdcMetdataEnhFileHandle;
    }
    CreateEnhMetaFileA(hdc.into(), lpfilename.into(), ::core::mem::transmute(lprc.unwrap_or(::std::ptr::null())), lpdesc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateEnhMetaFileW<'a, P0, P1, P2>(hdc: P0, lpfilename: P1, lprc: ::core::option::Option<*const super::super::Foundation::RECT>, lpdesc: P2) -> HdcMetdataEnhFileHandle
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateEnhMetaFileW(hdc: HDC, lpfilename: ::windows::core::PCWSTR, lprc: *const super::super::Foundation::RECT, lpdesc: ::windows::core::PCWSTR) -> HdcMetdataEnhFileHandle;
    }
    CreateEnhMetaFileW(hdc.into(), lpfilename.into(), ::core::mem::transmute(lprc.unwrap_or(::std::ptr::null())), lpdesc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateFontA<'a, P0>(cheight: i32, cwidth: i32, cescapement: i32, corientation: i32, cweight: i32, bitalic: u32, bunderline: u32, bstrikeout: u32, icharset: u32, ioutprecision: FONT_OUTPUT_PRECISION, iclipprecision: FONT_CLIP_PRECISION, iquality: FONT_QUALITY, ipitchandfamily: FONT_PITCH_AND_FAMILY, pszfacename: P0) -> HFONT
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateFontA(cheight: i32, cwidth: i32, cescapement: i32, corientation: i32, cweight: i32, bitalic: u32, bunderline: u32, bstrikeout: u32, icharset: u32, ioutprecision: FONT_OUTPUT_PRECISION, iclipprecision: FONT_CLIP_PRECISION, iquality: FONT_QUALITY, ipitchandfamily: FONT_PITCH_AND_FAMILY, pszfacename: ::windows::core::PCSTR) -> HFONT;
    }
    CreateFontA(cheight, cwidth, cescapement, corientation, cweight, bitalic, bunderline, bstrikeout, icharset, ioutprecision, iclipprecision, iquality, ipitchandfamily, pszfacename.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateFontIndirectA(lplf: *const LOGFONTA) -> HFONT {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateFontIndirectA(lplf: *const LOGFONTA) -> HFONT;
    }
    CreateFontIndirectA(::core::mem::transmute(lplf))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateFontIndirectExA(param0: *const ENUMLOGFONTEXDVA) -> HFONT {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateFontIndirectExA(param0: *const ENUMLOGFONTEXDVA) -> HFONT;
    }
    CreateFontIndirectExA(::core::mem::transmute(param0))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateFontIndirectExW(param0: *const ENUMLOGFONTEXDVW) -> HFONT {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateFontIndirectExW(param0: *const ENUMLOGFONTEXDVW) -> HFONT;
    }
    CreateFontIndirectExW(::core::mem::transmute(param0))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateFontIndirectW(lplf: *const LOGFONTW) -> HFONT {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateFontIndirectW(lplf: *const LOGFONTW) -> HFONT;
    }
    CreateFontIndirectW(::core::mem::transmute(lplf))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateFontW<'a, P0>(cheight: i32, cwidth: i32, cescapement: i32, corientation: i32, cweight: i32, bitalic: u32, bunderline: u32, bstrikeout: u32, icharset: u32, ioutprecision: FONT_OUTPUT_PRECISION, iclipprecision: FONT_CLIP_PRECISION, iquality: FONT_QUALITY, ipitchandfamily: FONT_PITCH_AND_FAMILY, pszfacename: P0) -> HFONT
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateFontW(cheight: i32, cwidth: i32, cescapement: i32, corientation: i32, cweight: i32, bitalic: u32, bunderline: u32, bstrikeout: u32, icharset: u32, ioutprecision: FONT_OUTPUT_PRECISION, iclipprecision: FONT_CLIP_PRECISION, iquality: FONT_QUALITY, ipitchandfamily: FONT_PITCH_AND_FAMILY, pszfacename: ::windows::core::PCWSTR) -> HFONT;
    }
    CreateFontW(cheight, cwidth, cescapement, corientation, cweight, bitalic, bunderline, bstrikeout, icharset, ioutprecision, iclipprecision, iquality, ipitchandfamily, pszfacename.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateHalftonePalette<'a, P0>(hdc: P0) -> HPALETTE
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateHalftonePalette(hdc: HDC) -> HPALETTE;
    }
    CreateHalftonePalette(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateHatchBrush<'a, P0>(ihatch: HATCH_BRUSH_STYLE, color: P0) -> HBRUSH
where
    P0: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateHatchBrush(ihatch: HATCH_BRUSH_STYLE, color: super::super::Foundation::COLORREF) -> HBRUSH;
    }
    CreateHatchBrush(ihatch, color.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateICA<'a, P0, P1, P2>(pszdriver: P0, pszdevice: P1, pszport: P2, pdm: ::core::option::Option<*const DEVMODEA>) -> CreatedHDC
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateICA(pszdriver: ::windows::core::PCSTR, pszdevice: ::windows::core::PCSTR, pszport: ::windows::core::PCSTR, pdm: *const DEVMODEA) -> CreatedHDC;
    }
    CreateICA(pszdriver.into(), pszdevice.into(), pszport.into(), ::core::mem::transmute(pdm.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateICW<'a, P0, P1, P2>(pszdriver: P0, pszdevice: P1, pszport: P2, pdm: ::core::option::Option<*const DEVMODEW>) -> CreatedHDC
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateICW(pszdriver: ::windows::core::PCWSTR, pszdevice: ::windows::core::PCWSTR, pszport: ::windows::core::PCWSTR, pdm: *const DEVMODEW) -> CreatedHDC;
    }
    CreateICW(pszdriver.into(), pszdevice.into(), pszport.into(), ::core::mem::transmute(pdm.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateMetaFileA<'a, P0>(pszfile: P0) -> HdcMetdataFileHandle
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateMetaFileA(pszfile: ::windows::core::PCSTR) -> HdcMetdataFileHandle;
    }
    CreateMetaFileA(pszfile.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateMetaFileW<'a, P0>(pszfile: P0) -> HdcMetdataFileHandle
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateMetaFileW(pszfile: ::windows::core::PCWSTR) -> HdcMetdataFileHandle;
    }
    CreateMetaFileW(pszfile.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreatePalette(plpal: *const LOGPALETTE) -> HPALETTE {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreatePalette(plpal: *const LOGPALETTE) -> HPALETTE;
    }
    CreatePalette(::core::mem::transmute(plpal))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreatePatternBrush<'a, P0>(hbm: P0) -> HBRUSH
where
    P0: ::std::convert::Into<HBITMAP>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreatePatternBrush(hbm: HBITMAP) -> HBRUSH;
    }
    CreatePatternBrush(hbm.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePen<'a, P0>(istyle: PEN_STYLE, cwidth: i32, color: P0) -> HPEN
where
    P0: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreatePen(istyle: PEN_STYLE, cwidth: i32, color: super::super::Foundation::COLORREF) -> HPEN;
    }
    CreatePen(istyle, cwidth, color.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePenIndirect(plpen: *const LOGPEN) -> HPEN {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreatePenIndirect(plpen: *const LOGPEN) -> HPEN;
    }
    CreatePenIndirect(::core::mem::transmute(plpen))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePolyPolygonRgn(pptl: *const super::super::Foundation::POINT, pc: &[i32], imode: CREATE_POLYGON_RGN_MODE) -> HRGN {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreatePolyPolygonRgn(pptl: *const super::super::Foundation::POINT, pc: *const i32, cpoly: i32, imode: CREATE_POLYGON_RGN_MODE) -> HRGN;
    }
    CreatePolyPolygonRgn(::core::mem::transmute(pptl), ::core::mem::transmute(pc.as_ptr()), pc.len() as _, imode)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePolygonRgn(pptl: &[super::super::Foundation::POINT], imode: CREATE_POLYGON_RGN_MODE) -> HRGN {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreatePolygonRgn(pptl: *const super::super::Foundation::POINT, cpoint: i32, imode: CREATE_POLYGON_RGN_MODE) -> HRGN;
    }
    CreatePolygonRgn(::core::mem::transmute(pptl.as_ptr()), pptl.len() as _, imode)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateRectRgn(x1: i32, y1: i32, x2: i32, y2: i32) -> HRGN {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateRectRgn(x1: i32, y1: i32, x2: i32, y2: i32) -> HRGN;
    }
    CreateRectRgn(x1, y1, x2, y2)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateRectRgnIndirect(lprect: *const super::super::Foundation::RECT) -> HRGN {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateRectRgnIndirect(lprect: *const super::super::Foundation::RECT) -> HRGN;
    }
    CreateRectRgnIndirect(::core::mem::transmute(lprect))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn CreateRoundRectRgn(x1: i32, y1: i32, x2: i32, y2: i32, w: i32, h: i32) -> HRGN {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateRoundRectRgn(x1: i32, y1: i32, x2: i32, y2: i32, w: i32, h: i32) -> HRGN;
    }
    CreateRoundRectRgn(x1, y1, x2, y2, w, h)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateScalableFontResourceA<'a, P0, P1, P2>(fdwhidden: u32, lpszfont: P0, lpszfile: P1, lpszpath: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateScalableFontResourceA(fdwhidden: u32, lpszfont: ::windows::core::PCSTR, lpszfile: ::windows::core::PCSTR, lpszpath: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
    }
    CreateScalableFontResourceA(fdwhidden, lpszfont.into(), lpszfile.into(), lpszpath.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateScalableFontResourceW<'a, P0, P1, P2>(fdwhidden: u32, lpszfont: P0, lpszfile: P1, lpszpath: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateScalableFontResourceW(fdwhidden: u32, lpszfont: ::windows::core::PCWSTR, lpszfile: ::windows::core::PCWSTR, lpszpath: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    CreateScalableFontResourceW(fdwhidden, lpszfont.into(), lpszfile.into(), lpszpath.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateSolidBrush<'a, P0>(color: P0) -> HBRUSH
where
    P0: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateSolidBrush(color: super::super::Foundation::COLORREF) -> HBRUSH;
    }
    CreateSolidBrush(color.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DPtoLP<'a, P0>(hdc: P0, lppt: &mut [super::super::Foundation::POINT]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DPtoLP(hdc: HDC, lppt: *mut super::super::Foundation::POINT, c: i32) -> super::super::Foundation::BOOL;
    }
    DPtoLP(hdc.into(), ::core::mem::transmute(lppt.as_ptr()), lppt.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteDC<'a, P0>(hdc: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<CreatedHDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DeleteDC(hdc: CreatedHDC) -> super::super::Foundation::BOOL;
    }
    DeleteDC(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteEnhMetaFile<'a, P0>(hmf: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HENHMETAFILE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DeleteEnhMetaFile(hmf: HENHMETAFILE) -> super::super::Foundation::BOOL;
    }
    DeleteEnhMetaFile(hmf.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteMetaFile<'a, P0>(hmf: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMETAFILE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DeleteMetaFile(hmf: HMETAFILE) -> super::super::Foundation::BOOL;
    }
    DeleteMetaFile(hmf.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteObject<'a, P0>(ho: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HGDIOBJ>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DeleteObject(ho: HGDIOBJ) -> super::super::Foundation::BOOL;
    }
    DeleteObject(ho.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawAnimatedRects<'a, P0>(hwnd: P0, idani: i32, lprcfrom: *const super::super::Foundation::RECT, lprcto: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DrawAnimatedRects(hwnd: super::super::Foundation::HWND, idani: i32, lprcfrom: *const super::super::Foundation::RECT, lprcto: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    DrawAnimatedRects(hwnd.into(), idani, ::core::mem::transmute(lprcfrom), ::core::mem::transmute(lprcto))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawCaption<'a, P0, P1>(hwnd: P0, hdc: P1, lprect: *const super::super::Foundation::RECT, flags: DRAW_CAPTION_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DrawCaption(hwnd: super::super::Foundation::HWND, hdc: HDC, lprect: *const super::super::Foundation::RECT, flags: DRAW_CAPTION_FLAGS) -> super::super::Foundation::BOOL;
    }
    DrawCaption(hwnd.into(), hdc.into(), ::core::mem::transmute(lprect), flags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawEdge<'a, P0>(hdc: P0, qrc: *mut super::super::Foundation::RECT, edge: DRAWEDGE_FLAGS, grfflags: DRAW_EDGE_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DrawEdge(hdc: HDC, qrc: *mut super::super::Foundation::RECT, edge: DRAWEDGE_FLAGS, grfflags: DRAW_EDGE_FLAGS) -> super::super::Foundation::BOOL;
    }
    DrawEdge(hdc.into(), ::core::mem::transmute(qrc), edge, grfflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn DrawEscape<'a, P0>(hdc: P0, iescape: i32, lpin: ::core::option::Option<&[u8]>) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DrawEscape(hdc: HDC, iescape: i32, cjin: i32, lpin: ::windows::core::PCSTR) -> i32;
    }
    DrawEscape(hdc.into(), iescape, lpin.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpin.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawFocusRect<'a, P0>(hdc: P0, lprc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DrawFocusRect(hdc: HDC, lprc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    DrawFocusRect(hdc.into(), ::core::mem::transmute(lprc))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawFrameControl<'a, P0>(param0: P0, param1: *mut super::super::Foundation::RECT, param2: DFC_TYPE, param3: DFCS_STATE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DrawFrameControl(param0: HDC, param1: *mut super::super::Foundation::RECT, param2: DFC_TYPE, param3: DFCS_STATE) -> super::super::Foundation::BOOL;
    }
    DrawFrameControl(param0.into(), ::core::mem::transmute(param1), param2, param3)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawStateA<'a, P0, P1, P2, P3>(hdc: P0, hbrfore: P1, qfncallback: DRAWSTATEPROC, ldata: P2, wdata: P3, x: i32, y: i32, cx: i32, cy: i32, uflags: DRAWSTATE_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HBRUSH>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
    P3: ::std::convert::Into<super::super::Foundation::WPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DrawStateA(hdc: HDC, hbrfore: HBRUSH, qfncallback: *mut ::core::ffi::c_void, ldata: super::super::Foundation::LPARAM, wdata: super::super::Foundation::WPARAM, x: i32, y: i32, cx: i32, cy: i32, uflags: DRAWSTATE_FLAGS) -> super::super::Foundation::BOOL;
    }
    DrawStateA(hdc.into(), hbrfore.into(), ::core::mem::transmute(qfncallback), ldata.into(), wdata.into(), x, y, cx, cy, uflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawStateW<'a, P0, P1, P2, P3>(hdc: P0, hbrfore: P1, qfncallback: DRAWSTATEPROC, ldata: P2, wdata: P3, x: i32, y: i32, cx: i32, cy: i32, uflags: DRAWSTATE_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HBRUSH>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
    P3: ::std::convert::Into<super::super::Foundation::WPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DrawStateW(hdc: HDC, hbrfore: HBRUSH, qfncallback: *mut ::core::ffi::c_void, ldata: super::super::Foundation::LPARAM, wdata: super::super::Foundation::WPARAM, x: i32, y: i32, cx: i32, cy: i32, uflags: DRAWSTATE_FLAGS) -> super::super::Foundation::BOOL;
    }
    DrawStateW(hdc.into(), hbrfore.into(), ::core::mem::transmute(qfncallback), ldata.into(), wdata.into(), x, y, cx, cy, uflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawTextA<'a, P0>(hdc: P0, lpchtext: &mut [u8], lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DrawTextA(hdc: HDC, lpchtext: ::windows::core::PSTR, cchtext: i32, lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT) -> i32;
    }
    DrawTextA(hdc.into(), ::core::mem::transmute(lpchtext.as_ptr()), lpchtext.len() as _, ::core::mem::transmute(lprc), format)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawTextExA<'a, P0>(hdc: P0, lpchtext: &mut [u8], lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT, lpdtp: ::core::option::Option<*const DRAWTEXTPARAMS>) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DrawTextExA(hdc: HDC, lpchtext: ::windows::core::PSTR, cchtext: i32, lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT, lpdtp: *const DRAWTEXTPARAMS) -> i32;
    }
    DrawTextExA(hdc.into(), ::core::mem::transmute(lpchtext.as_ptr()), lpchtext.len() as _, ::core::mem::transmute(lprc), format, ::core::mem::transmute(lpdtp.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawTextExW<'a, P0>(hdc: P0, lpchtext: &mut [u16], lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT, lpdtp: ::core::option::Option<*const DRAWTEXTPARAMS>) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DrawTextExW(hdc: HDC, lpchtext: ::windows::core::PWSTR, cchtext: i32, lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT, lpdtp: *const DRAWTEXTPARAMS) -> i32;
    }
    DrawTextExW(hdc.into(), ::core::mem::transmute(lpchtext.as_ptr()), lpchtext.len() as _, ::core::mem::transmute(lprc), format, ::core::mem::transmute(lpdtp.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawTextW<'a, P0>(hdc: P0, lpchtext: &mut [u16], lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DrawTextW(hdc: HDC, lpchtext: ::windows::core::PWSTR, cchtext: i32, lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT) -> i32;
    }
    DrawTextW(hdc.into(), ::core::mem::transmute(lpchtext.as_ptr()), lpchtext.len() as _, ::core::mem::transmute(lprc), format)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Ellipse<'a, P0>(hdc: P0, left: i32, top: i32, right: i32, bottom: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn Ellipse(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> super::super::Foundation::BOOL;
    }
    Ellipse(hdc.into(), left, top, right, bottom)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndPaint<'a, P0>(hwnd: P0, lppaint: *const PAINTSTRUCT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EndPaint(hwnd: super::super::Foundation::HWND, lppaint: *const PAINTSTRUCT) -> super::super::Foundation::BOOL;
    }
    EndPaint(hwnd.into(), ::core::mem::transmute(lppaint))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndPath<'a, P0>(hdc: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EndPath(hdc: HDC) -> super::super::Foundation::BOOL;
    }
    EndPath(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumDisplayDevicesA<'a, P0>(lpdevice: P0, idevnum: u32, lpdisplaydevice: *mut DISPLAY_DEVICEA, dwflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumDisplayDevicesA(lpdevice: ::windows::core::PCSTR, idevnum: u32, lpdisplaydevice: *mut DISPLAY_DEVICEA, dwflags: u32) -> super::super::Foundation::BOOL;
    }
    EnumDisplayDevicesA(lpdevice.into(), idevnum, ::core::mem::transmute(lpdisplaydevice), dwflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumDisplayDevicesW<'a, P0>(lpdevice: P0, idevnum: u32, lpdisplaydevice: *mut DISPLAY_DEVICEW, dwflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumDisplayDevicesW(lpdevice: ::windows::core::PCWSTR, idevnum: u32, lpdisplaydevice: *mut DISPLAY_DEVICEW, dwflags: u32) -> super::super::Foundation::BOOL;
    }
    EnumDisplayDevicesW(lpdevice.into(), idevnum, ::core::mem::transmute(lpdisplaydevice), dwflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumDisplayMonitors<'a, P0, P1>(hdc: P0, lprcclip: ::core::option::Option<*const super::super::Foundation::RECT>, lpfnenum: MONITORENUMPROC, dwdata: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumDisplayMonitors(hdc: HDC, lprcclip: *const super::super::Foundation::RECT, lpfnenum: *mut ::core::ffi::c_void, dwdata: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    }
    EnumDisplayMonitors(hdc.into(), ::core::mem::transmute(lprcclip.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpfnenum), dwdata.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumDisplaySettingsA<'a, P0>(lpszdevicename: P0, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEA) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumDisplaySettingsA(lpszdevicename: ::windows::core::PCSTR, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEA) -> super::super::Foundation::BOOL;
    }
    EnumDisplaySettingsA(lpszdevicename.into(), imodenum, ::core::mem::transmute(lpdevmode))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumDisplaySettingsExA<'a, P0>(lpszdevicename: P0, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEA, dwflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumDisplaySettingsExA(lpszdevicename: ::windows::core::PCSTR, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEA, dwflags: u32) -> super::super::Foundation::BOOL;
    }
    EnumDisplaySettingsExA(lpszdevicename.into(), imodenum, ::core::mem::transmute(lpdevmode), dwflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumDisplaySettingsExW<'a, P0>(lpszdevicename: P0, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEW, dwflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumDisplaySettingsExW(lpszdevicename: ::windows::core::PCWSTR, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEW, dwflags: u32) -> super::super::Foundation::BOOL;
    }
    EnumDisplaySettingsExW(lpszdevicename.into(), imodenum, ::core::mem::transmute(lpdevmode), dwflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumDisplaySettingsW<'a, P0>(lpszdevicename: P0, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEW) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumDisplaySettingsW(lpszdevicename: ::windows::core::PCWSTR, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEW) -> super::super::Foundation::BOOL;
    }
    EnumDisplaySettingsW(lpszdevicename.into(), imodenum, ::core::mem::transmute(lpdevmode))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumEnhMetaFile<'a, P0, P1>(hdc: P0, hmf: P1, proc: ENHMFENUMPROC, param3: ::core::option::Option<*const ::core::ffi::c_void>, lprect: ::core::option::Option<*const super::super::Foundation::RECT>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HENHMETAFILE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumEnhMetaFile(hdc: HDC, hmf: HENHMETAFILE, proc: *mut ::core::ffi::c_void, param3: *const ::core::ffi::c_void, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    EnumEnhMetaFile(hdc.into(), hmf.into(), ::core::mem::transmute(proc), ::core::mem::transmute(param3.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lprect.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumFontFamiliesA<'a, P0, P1, P2>(hdc: P0, lplogfont: P1, lpproc: FONTENUMPROCA, lparam: P2) -> i32
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumFontFamiliesA(hdc: HDC, lplogfont: ::windows::core::PCSTR, lpproc: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> i32;
    }
    EnumFontFamiliesA(hdc.into(), lplogfont.into(), ::core::mem::transmute(lpproc), lparam.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumFontFamiliesExA<'a, P0, P1>(hdc: P0, lplogfont: *const LOGFONTA, lpproc: FONTENUMPROCA, lparam: P1, dwflags: u32) -> i32
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumFontFamiliesExA(hdc: HDC, lplogfont: *const LOGFONTA, lpproc: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM, dwflags: u32) -> i32;
    }
    EnumFontFamiliesExA(hdc.into(), ::core::mem::transmute(lplogfont), ::core::mem::transmute(lpproc), lparam.into(), dwflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumFontFamiliesExW<'a, P0, P1>(hdc: P0, lplogfont: *const LOGFONTW, lpproc: FONTENUMPROCW, lparam: P1, dwflags: u32) -> i32
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumFontFamiliesExW(hdc: HDC, lplogfont: *const LOGFONTW, lpproc: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM, dwflags: u32) -> i32;
    }
    EnumFontFamiliesExW(hdc.into(), ::core::mem::transmute(lplogfont), ::core::mem::transmute(lpproc), lparam.into(), dwflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumFontFamiliesW<'a, P0, P1, P2>(hdc: P0, lplogfont: P1, lpproc: FONTENUMPROCW, lparam: P2) -> i32
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumFontFamiliesW(hdc: HDC, lplogfont: ::windows::core::PCWSTR, lpproc: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> i32;
    }
    EnumFontFamiliesW(hdc.into(), lplogfont.into(), ::core::mem::transmute(lpproc), lparam.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumFontsA<'a, P0, P1, P2>(hdc: P0, lplogfont: P1, lpproc: FONTENUMPROCA, lparam: P2) -> i32
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumFontsA(hdc: HDC, lplogfont: ::windows::core::PCSTR, lpproc: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> i32;
    }
    EnumFontsA(hdc.into(), lplogfont.into(), ::core::mem::transmute(lpproc), lparam.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumFontsW<'a, P0, P1, P2>(hdc: P0, lplogfont: P1, lpproc: FONTENUMPROCW, lparam: P2) -> i32
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumFontsW(hdc: HDC, lplogfont: ::windows::core::PCWSTR, lpproc: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> i32;
    }
    EnumFontsW(hdc.into(), lplogfont.into(), ::core::mem::transmute(lpproc), lparam.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumMetaFile<'a, P0, P1, P2>(hdc: P0, hmf: P1, proc: MFENUMPROC, param3: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HMETAFILE>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumMetaFile(hdc: HDC, hmf: HMETAFILE, proc: *mut ::core::ffi::c_void, param3: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    }
    EnumMetaFile(hdc.into(), hmf.into(), ::core::mem::transmute(proc), param3.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumObjects<'a, P0, P1>(hdc: P0, ntype: OBJ_TYPE, lpfunc: GOBJENUMPROC, lparam: P1) -> i32
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumObjects(hdc: HDC, ntype: OBJ_TYPE, lpfunc: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> i32;
    }
    EnumObjects(hdc.into(), ntype, ::core::mem::transmute(lpfunc), lparam.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EqualRect(lprc1: *const super::super::Foundation::RECT, lprc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EqualRect(lprc1: *const super::super::Foundation::RECT, lprc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    EqualRect(::core::mem::transmute(lprc1), ::core::mem::transmute(lprc2))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EqualRgn<'a, P0, P1>(hrgn1: P0, hrgn2: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HRGN>,
    P1: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EqualRgn(hrgn1: HRGN, hrgn2: HRGN) -> super::super::Foundation::BOOL;
    }
    EqualRgn(hrgn1.into(), hrgn2.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn ExcludeClipRect<'a, P0>(hdc: P0, left: i32, top: i32, right: i32, bottom: i32) -> GDI_REGION_TYPE
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ExcludeClipRect(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> GDI_REGION_TYPE;
    }
    ExcludeClipRect(hdc.into(), left, top, right, bottom)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExcludeUpdateRgn<'a, P0, P1>(hdc: P0, hwnd: P1) -> i32
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ExcludeUpdateRgn(hdc: HDC, hwnd: super::super::Foundation::HWND) -> i32;
    }
    ExcludeUpdateRgn(hdc.into(), hwnd.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExtCreatePen(ipenstyle: PEN_STYLE, cwidth: u32, plbrush: *const LOGBRUSH, pstyle: ::core::option::Option<&[u32]>) -> HPEN {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ExtCreatePen(ipenstyle: PEN_STYLE, cwidth: u32, plbrush: *const LOGBRUSH, cstyle: u32, pstyle: *const u32) -> HPEN;
    }
    ExtCreatePen(ipenstyle, cwidth, ::core::mem::transmute(plbrush), pstyle.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pstyle.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExtCreateRegion(lpx: ::core::option::Option<*const XFORM>, ncount: u32, lpdata: *const RGNDATA) -> HRGN {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ExtCreateRegion(lpx: *const XFORM, ncount: u32, lpdata: *const RGNDATA) -> HRGN;
    }
    ExtCreateRegion(::core::mem::transmute(lpx.unwrap_or(::std::ptr::null())), ncount, ::core::mem::transmute(lpdata))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExtFloodFill<'a, P0, P1>(hdc: P0, x: i32, y: i32, color: P1, r#type: EXT_FLOOD_FILL_TYPE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ExtFloodFill(hdc: HDC, x: i32, y: i32, color: super::super::Foundation::COLORREF, r#type: EXT_FLOOD_FILL_TYPE) -> super::super::Foundation::BOOL;
    }
    ExtFloodFill(hdc.into(), x, y, color.into(), r#type)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn ExtSelectClipRgn<'a, P0, P1>(hdc: P0, hrgn: P1, mode: RGN_COMBINE_MODE) -> GDI_REGION_TYPE
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ExtSelectClipRgn(hdc: HDC, hrgn: HRGN, mode: RGN_COMBINE_MODE) -> GDI_REGION_TYPE;
    }
    ExtSelectClipRgn(hdc.into(), hrgn.into(), mode)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExtTextOutA<'a, P0, P1>(hdc: P0, x: i32, y: i32, options: ETO_OPTIONS, lprect: ::core::option::Option<*const super::super::Foundation::RECT>, lpstring: P1, c: u32, lpdx: ::core::option::Option<*const i32>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ExtTextOutA(hdc: HDC, x: i32, y: i32, options: ETO_OPTIONS, lprect: *const super::super::Foundation::RECT, lpstring: ::windows::core::PCSTR, c: u32, lpdx: *const i32) -> super::super::Foundation::BOOL;
    }
    ExtTextOutA(hdc.into(), x, y, options, ::core::mem::transmute(lprect.unwrap_or(::std::ptr::null())), lpstring.into(), c, ::core::mem::transmute(lpdx.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExtTextOutW<'a, P0, P1>(hdc: P0, x: i32, y: i32, options: ETO_OPTIONS, lprect: ::core::option::Option<*const super::super::Foundation::RECT>, lpstring: P1, c: u32, lpdx: ::core::option::Option<*const i32>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ExtTextOutW(hdc: HDC, x: i32, y: i32, options: ETO_OPTIONS, lprect: *const super::super::Foundation::RECT, lpstring: ::windows::core::PCWSTR, c: u32, lpdx: *const i32) -> super::super::Foundation::BOOL;
    }
    ExtTextOutW(hdc.into(), x, y, options, ::core::mem::transmute(lprect.unwrap_or(::std::ptr::null())), lpstring.into(), c, ::core::mem::transmute(lpdx.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FillPath<'a, P0>(hdc: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FillPath(hdc: HDC) -> super::super::Foundation::BOOL;
    }
    FillPath(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FillRect<'a, P0, P1>(hdc: P0, lprc: *const super::super::Foundation::RECT, hbr: P1) -> i32
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HBRUSH>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FillRect(hdc: HDC, lprc: *const super::super::Foundation::RECT, hbr: HBRUSH) -> i32;
    }
    FillRect(hdc.into(), ::core::mem::transmute(lprc), hbr.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FillRgn<'a, P0, P1, P2>(hdc: P0, hrgn: P1, hbr: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HRGN>,
    P2: ::std::convert::Into<HBRUSH>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FillRgn(hdc: HDC, hrgn: HRGN, hbr: HBRUSH) -> super::super::Foundation::BOOL;
    }
    FillRgn(hdc.into(), hrgn.into(), hbr.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FixBrushOrgEx<'a, P0>(hdc: P0, x: i32, y: i32, ptl: ::core::option::Option<*const super::super::Foundation::POINT>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FixBrushOrgEx(hdc: HDC, x: i32, y: i32, ptl: *const super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    FixBrushOrgEx(hdc.into(), x, y, ::core::mem::transmute(ptl.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlattenPath<'a, P0>(hdc: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FlattenPath(hdc: HDC) -> super::super::Foundation::BOOL;
    }
    FlattenPath(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FloodFill<'a, P0, P1>(hdc: P0, x: i32, y: i32, color: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FloodFill(hdc: HDC, x: i32, y: i32, color: super::super::Foundation::COLORREF) -> super::super::Foundation::BOOL;
    }
    FloodFill(hdc.into(), x, y, color.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FrameRect<'a, P0, P1>(hdc: P0, lprc: *const super::super::Foundation::RECT, hbr: P1) -> i32
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HBRUSH>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FrameRect(hdc: HDC, lprc: *const super::super::Foundation::RECT, hbr: HBRUSH) -> i32;
    }
    FrameRect(hdc.into(), ::core::mem::transmute(lprc), hbr.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FrameRgn<'a, P0, P1, P2>(hdc: P0, hrgn: P1, hbr: P2, w: i32, h: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HRGN>,
    P2: ::std::convert::Into<HBRUSH>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FrameRgn(hdc: HDC, hrgn: HRGN, hbr: HBRUSH, w: i32, h: i32) -> super::super::Foundation::BOOL;
    }
    FrameRgn(hdc.into(), hrgn.into(), hbr.into(), w, h)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GdiAlphaBlend<'a, P0, P1>(hdcdest: P0, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: P1, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, ftn: BLENDFUNCTION) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GdiAlphaBlend(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, ftn: BLENDFUNCTION) -> super::super::Foundation::BOOL;
    }
    GdiAlphaBlend(hdcdest.into(), xorigindest, yorigindest, wdest, hdest, hdcsrc.into(), xoriginsrc, yoriginsrc, wsrc, hsrc, ::core::mem::transmute(ftn))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GdiComment<'a, P0>(hdc: P0, lpdata: &[u8]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GdiComment(hdc: HDC, nsize: u32, lpdata: *const u8) -> super::super::Foundation::BOOL;
    }
    GdiComment(hdc.into(), lpdata.len() as _, ::core::mem::transmute(lpdata.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GdiFlush() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GdiFlush() -> super::super::Foundation::BOOL;
    }
    GdiFlush()
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GdiGetBatchLimit() -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GdiGetBatchLimit() -> u32;
    }
    GdiGetBatchLimit()
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GdiGradientFill<'a, P0>(hdc: P0, pvertex: &[TRIVERTEX], pmesh: *const ::core::ffi::c_void, ncount: u32, ulmode: GRADIENT_FILL) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GdiGradientFill(hdc: HDC, pvertex: *const TRIVERTEX, nvertex: u32, pmesh: *const ::core::ffi::c_void, ncount: u32, ulmode: GRADIENT_FILL) -> super::super::Foundation::BOOL;
    }
    GdiGradientFill(hdc.into(), ::core::mem::transmute(pvertex.as_ptr()), pvertex.len() as _, ::core::mem::transmute(pmesh), ncount, ulmode)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GdiSetBatchLimit(dw: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GdiSetBatchLimit(dw: u32) -> u32;
    }
    GdiSetBatchLimit(dw)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GdiTransparentBlt<'a, P0, P1>(hdcdest: P0, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: P1, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, crtransparent: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GdiTransparentBlt(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, crtransparent: u32) -> super::super::Foundation::BOOL;
    }
    GdiTransparentBlt(hdcdest.into(), xorigindest, yorigindest, wdest, hdest, hdcsrc.into(), xoriginsrc, yoriginsrc, wsrc, hsrc, crtransparent)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetArcDirection<'a, P0>(hdc: P0) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetArcDirection(hdc: HDC) -> i32;
    }
    GetArcDirection(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAspectRatioFilterEx<'a, P0>(hdc: P0, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetAspectRatioFilterEx(hdc: HDC, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    }
    GetAspectRatioFilterEx(hdc.into(), ::core::mem::transmute(lpsize))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetBitmapBits<'a, P0>(hbit: P0, lpvbits: &mut [u8]) -> i32
where
    P0: ::std::convert::Into<HBITMAP>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetBitmapBits(hbit: HBITMAP, cb: i32, lpvbits: *mut ::core::ffi::c_void) -> i32;
    }
    GetBitmapBits(hbit.into(), lpvbits.len() as _, ::core::mem::transmute(lpvbits.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetBitmapDimensionEx<'a, P0>(hbit: P0, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HBITMAP>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetBitmapDimensionEx(hbit: HBITMAP, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    }
    GetBitmapDimensionEx(hbit.into(), ::core::mem::transmute(lpsize))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetBkColor<'a, P0>(hdc: P0) -> super::super::Foundation::COLORREF
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetBkColor(hdc: HDC) -> super::super::Foundation::COLORREF;
    }
    GetBkColor(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetBkMode<'a, P0>(hdc: P0) -> BACKGROUND_MODE
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetBkMode(hdc: HDC) -> BACKGROUND_MODE;
    }
    GetBkMode(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetBoundsRect<'a, P0>(hdc: P0, lprect: *mut super::super::Foundation::RECT, flags: u32) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetBoundsRect(hdc: HDC, lprect: *mut super::super::Foundation::RECT, flags: u32) -> u32;
    }
    GetBoundsRect(hdc.into(), ::core::mem::transmute(lprect), flags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetBrushOrgEx<'a, P0>(hdc: P0, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetBrushOrgEx(hdc: HDC, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    GetBrushOrgEx(hdc.into(), ::core::mem::transmute(lppt))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCharABCWidthsA<'a, P0>(hdc: P0, wfirst: u32, wlast: u32, lpabc: *mut ABC) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCharABCWidthsA(hdc: HDC, wfirst: u32, wlast: u32, lpabc: *mut ABC) -> super::super::Foundation::BOOL;
    }
    GetCharABCWidthsA(hdc.into(), wfirst, wlast, ::core::mem::transmute(lpabc))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCharABCWidthsFloatA<'a, P0>(hdc: P0, ifirst: u32, ilast: u32, lpabc: *mut ABCFLOAT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCharABCWidthsFloatA(hdc: HDC, ifirst: u32, ilast: u32, lpabc: *mut ABCFLOAT) -> super::super::Foundation::BOOL;
    }
    GetCharABCWidthsFloatA(hdc.into(), ifirst, ilast, ::core::mem::transmute(lpabc))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCharABCWidthsFloatW<'a, P0>(hdc: P0, ifirst: u32, ilast: u32, lpabc: *mut ABCFLOAT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCharABCWidthsFloatW(hdc: HDC, ifirst: u32, ilast: u32, lpabc: *mut ABCFLOAT) -> super::super::Foundation::BOOL;
    }
    GetCharABCWidthsFloatW(hdc.into(), ifirst, ilast, ::core::mem::transmute(lpabc))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCharABCWidthsI<'a, P0>(hdc: P0, gifirst: u32, cgi: u32, pgi: ::core::option::Option<*const u16>, pabc: *mut ABC) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCharABCWidthsI(hdc: HDC, gifirst: u32, cgi: u32, pgi: *const u16, pabc: *mut ABC) -> super::super::Foundation::BOOL;
    }
    GetCharABCWidthsI(hdc.into(), gifirst, cgi, ::core::mem::transmute(pgi.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pabc))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCharABCWidthsW<'a, P0>(hdc: P0, wfirst: u32, wlast: u32, lpabc: *mut ABC) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCharABCWidthsW(hdc: HDC, wfirst: u32, wlast: u32, lpabc: *mut ABC) -> super::super::Foundation::BOOL;
    }
    GetCharABCWidthsW(hdc.into(), wfirst, wlast, ::core::mem::transmute(lpabc))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCharWidth32A<'a, P0>(hdc: P0, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCharWidth32A(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL;
    }
    GetCharWidth32A(hdc.into(), ifirst, ilast, ::core::mem::transmute(lpbuffer))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCharWidth32W<'a, P0>(hdc: P0, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCharWidth32W(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL;
    }
    GetCharWidth32W(hdc.into(), ifirst, ilast, ::core::mem::transmute(lpbuffer))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCharWidthA<'a, P0>(hdc: P0, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCharWidthA(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL;
    }
    GetCharWidthA(hdc.into(), ifirst, ilast, ::core::mem::transmute(lpbuffer))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCharWidthFloatA<'a, P0>(hdc: P0, ifirst: u32, ilast: u32, lpbuffer: *mut f32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCharWidthFloatA(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut f32) -> super::super::Foundation::BOOL;
    }
    GetCharWidthFloatA(hdc.into(), ifirst, ilast, ::core::mem::transmute(lpbuffer))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCharWidthFloatW<'a, P0>(hdc: P0, ifirst: u32, ilast: u32, lpbuffer: *mut f32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCharWidthFloatW(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut f32) -> super::super::Foundation::BOOL;
    }
    GetCharWidthFloatW(hdc.into(), ifirst, ilast, ::core::mem::transmute(lpbuffer))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCharWidthI<'a, P0>(hdc: P0, gifirst: u32, cgi: u32, pgi: ::core::option::Option<*const u16>, piwidths: *mut i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCharWidthI(hdc: HDC, gifirst: u32, cgi: u32, pgi: *const u16, piwidths: *mut i32) -> super::super::Foundation::BOOL;
    }
    GetCharWidthI(hdc.into(), gifirst, cgi, ::core::mem::transmute(pgi.unwrap_or(::std::ptr::null())), ::core::mem::transmute(piwidths))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCharWidthW<'a, P0>(hdc: P0, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCharWidthW(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL;
    }
    GetCharWidthW(hdc.into(), ifirst, ilast, ::core::mem::transmute(lpbuffer))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetCharacterPlacementA<'a, P0>(hdc: P0, lpstring: &[u8], nmexextent: i32, lpresults: *mut GCP_RESULTSA, dwflags: GET_CHARACTER_PLACEMENT_FLAGS) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCharacterPlacementA(hdc: HDC, lpstring: ::windows::core::PCSTR, ncount: i32, nmexextent: i32, lpresults: *mut GCP_RESULTSA, dwflags: GET_CHARACTER_PLACEMENT_FLAGS) -> u32;
    }
    GetCharacterPlacementA(hdc.into(), ::core::mem::transmute(lpstring.as_ptr()), lpstring.len() as _, nmexextent, ::core::mem::transmute(lpresults), dwflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetCharacterPlacementW<'a, P0>(hdc: P0, lpstring: &[u16], nmexextent: i32, lpresults: *mut GCP_RESULTSW, dwflags: GET_CHARACTER_PLACEMENT_FLAGS) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCharacterPlacementW(hdc: HDC, lpstring: ::windows::core::PCWSTR, ncount: i32, nmexextent: i32, lpresults: *mut GCP_RESULTSW, dwflags: GET_CHARACTER_PLACEMENT_FLAGS) -> u32;
    }
    GetCharacterPlacementW(hdc.into(), ::core::mem::transmute(lpstring.as_ptr()), lpstring.len() as _, nmexextent, ::core::mem::transmute(lpresults), dwflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClipBox<'a, P0>(hdc: P0, lprect: *mut super::super::Foundation::RECT) -> GDI_REGION_TYPE
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClipBox(hdc: HDC, lprect: *mut super::super::Foundation::RECT) -> GDI_REGION_TYPE;
    }
    GetClipBox(hdc.into(), ::core::mem::transmute(lprect))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetClipRgn<'a, P0, P1>(hdc: P0, hrgn: P1) -> i32
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClipRgn(hdc: HDC, hrgn: HRGN) -> i32;
    }
    GetClipRgn(hdc.into(), hrgn.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetColorAdjustment<'a, P0>(hdc: P0, lpca: *mut COLORADJUSTMENT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetColorAdjustment(hdc: HDC, lpca: *mut COLORADJUSTMENT) -> super::super::Foundation::BOOL;
    }
    GetColorAdjustment(hdc.into(), ::core::mem::transmute(lpca))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetCurrentObject<'a, P0>(hdc: P0, r#type: OBJ_TYPE) -> HGDIOBJ
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCurrentObject(hdc: HDC, r#type: OBJ_TYPE) -> HGDIOBJ;
    }
    GetCurrentObject(hdc.into(), r#type)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentPositionEx<'a, P0>(hdc: P0, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCurrentPositionEx(hdc: HDC, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    GetCurrentPositionEx(hdc.into(), ::core::mem::transmute(lppt))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDC<'a, P0>(hwnd: P0) -> HDC
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDC(hwnd: super::super::Foundation::HWND) -> HDC;
    }
    GetDC(hwnd.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDCBrushColor<'a, P0>(hdc: P0) -> super::super::Foundation::COLORREF
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDCBrushColor(hdc: HDC) -> super::super::Foundation::COLORREF;
    }
    GetDCBrushColor(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDCEx<'a, P0, P1>(hwnd: P0, hrgnclip: P1, flags: GET_DCX_FLAGS) -> HDC
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDCEx(hwnd: super::super::Foundation::HWND, hrgnclip: HRGN, flags: GET_DCX_FLAGS) -> HDC;
    }
    GetDCEx(hwnd.into(), hrgnclip.into(), flags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDCOrgEx<'a, P0>(hdc: P0, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDCOrgEx(hdc: HDC, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    GetDCOrgEx(hdc.into(), ::core::mem::transmute(lppt))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDCPenColor<'a, P0>(hdc: P0) -> super::super::Foundation::COLORREF
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDCPenColor(hdc: HDC) -> super::super::Foundation::COLORREF;
    }
    GetDCPenColor(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetDIBColorTable<'a, P0>(hdc: P0, istart: u32, prgbq: &mut [RGBQUAD]) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDIBColorTable(hdc: HDC, istart: u32, centries: u32, prgbq: *mut RGBQUAD) -> u32;
    }
    GetDIBColorTable(hdc.into(), istart, prgbq.len() as _, ::core::mem::transmute(prgbq.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetDIBits<'a, P0, P1>(hdc: P0, hbm: P1, start: u32, clines: u32, lpvbits: ::core::option::Option<*mut ::core::ffi::c_void>, lpbmi: *mut BITMAPINFO, usage: DIB_USAGE) -> i32
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HBITMAP>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDIBits(hdc: HDC, hbm: HBITMAP, start: u32, clines: u32, lpvbits: *mut ::core::ffi::c_void, lpbmi: *mut BITMAPINFO, usage: DIB_USAGE) -> i32;
    }
    GetDIBits(hdc.into(), hbm.into(), start, clines, ::core::mem::transmute(lpvbits.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpbmi), usage)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetDeviceCaps<'a, P0>(hdc: P0, index: GET_DEVICE_CAPS_INDEX) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDeviceCaps(hdc: HDC, index: GET_DEVICE_CAPS_INDEX) -> i32;
    }
    GetDeviceCaps(hdc.into(), index)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetEnhMetaFileA<'a, P0>(lpname: P0) -> HENHMETAFILE
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetEnhMetaFileA(lpname: ::windows::core::PCSTR) -> HENHMETAFILE;
    }
    GetEnhMetaFileA(lpname.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetEnhMetaFileBits<'a, P0>(hemf: P0, lpdata: ::core::option::Option<&mut [u8]>) -> u32
where
    P0: ::std::convert::Into<HENHMETAFILE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetEnhMetaFileBits(hemf: HENHMETAFILE, nsize: u32, lpdata: *mut u8) -> u32;
    }
    GetEnhMetaFileBits(hemf.into(), lpdata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetEnhMetaFileDescriptionA<'a, P0>(hemf: P0, lpdescription: ::core::option::Option<&mut [u8]>) -> u32
where
    P0: ::std::convert::Into<HENHMETAFILE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetEnhMetaFileDescriptionA(hemf: HENHMETAFILE, cchbuffer: u32, lpdescription: ::windows::core::PSTR) -> u32;
    }
    GetEnhMetaFileDescriptionA(hemf.into(), lpdescription.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpdescription.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetEnhMetaFileDescriptionW<'a, P0>(hemf: P0, lpdescription: ::core::option::Option<&mut [u16]>) -> u32
where
    P0: ::std::convert::Into<HENHMETAFILE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetEnhMetaFileDescriptionW(hemf: HENHMETAFILE, cchbuffer: u32, lpdescription: ::windows::core::PWSTR) -> u32;
    }
    GetEnhMetaFileDescriptionW(hemf.into(), lpdescription.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpdescription.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetEnhMetaFileHeader<'a, P0>(hemf: P0, nsize: u32, lpenhmetaheader: ::core::option::Option<*mut ENHMETAHEADER>) -> u32
where
    P0: ::std::convert::Into<HENHMETAFILE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetEnhMetaFileHeader(hemf: HENHMETAFILE, nsize: u32, lpenhmetaheader: *mut ENHMETAHEADER) -> u32;
    }
    GetEnhMetaFileHeader(hemf.into(), nsize, ::core::mem::transmute(lpenhmetaheader.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetEnhMetaFilePaletteEntries<'a, P0>(hemf: P0, lppaletteentries: ::core::option::Option<&mut [PALETTEENTRY]>) -> u32
where
    P0: ::std::convert::Into<HENHMETAFILE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetEnhMetaFilePaletteEntries(hemf: HENHMETAFILE, nnumentries: u32, lppaletteentries: *mut PALETTEENTRY) -> u32;
    }
    GetEnhMetaFilePaletteEntries(hemf.into(), lppaletteentries.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lppaletteentries.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetEnhMetaFileW<'a, P0>(lpname: P0) -> HENHMETAFILE
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetEnhMetaFileW(lpname: ::windows::core::PCWSTR) -> HENHMETAFILE;
    }
    GetEnhMetaFileW(lpname.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetFontData<'a, P0>(hdc: P0, dwtable: u32, dwoffset: u32, pvbuffer: ::core::option::Option<&mut [u8]>) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetFontData(hdc: HDC, dwtable: u32, dwoffset: u32, pvbuffer: *mut ::core::ffi::c_void, cjbuffer: u32) -> u32;
    }
    GetFontData(hdc.into(), dwtable, dwoffset, ::core::mem::transmute(pvbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pvbuffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetFontLanguageInfo<'a, P0>(hdc: P0) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetFontLanguageInfo(hdc: HDC) -> u32;
    }
    GetFontLanguageInfo(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetFontUnicodeRanges<'a, P0>(hdc: P0, lpgs: ::core::option::Option<*mut GLYPHSET>) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetFontUnicodeRanges(hdc: HDC, lpgs: *mut GLYPHSET) -> u32;
    }
    GetFontUnicodeRanges(hdc.into(), ::core::mem::transmute(lpgs.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetGlyphIndicesA<'a, P0, P1>(hdc: P0, lpstr: P1, c: i32, pgi: *mut u16, fl: u32) -> u32
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetGlyphIndicesA(hdc: HDC, lpstr: ::windows::core::PCSTR, c: i32, pgi: *mut u16, fl: u32) -> u32;
    }
    GetGlyphIndicesA(hdc.into(), lpstr.into(), c, ::core::mem::transmute(pgi), fl)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetGlyphIndicesW<'a, P0, P1>(hdc: P0, lpstr: P1, c: i32, pgi: *mut u16, fl: u32) -> u32
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetGlyphIndicesW(hdc: HDC, lpstr: ::windows::core::PCWSTR, c: i32, pgi: *mut u16, fl: u32) -> u32;
    }
    GetGlyphIndicesW(hdc.into(), lpstr.into(), c, ::core::mem::transmute(pgi), fl)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGlyphOutlineA<'a, P0>(hdc: P0, uchar: u32, fuformat: GET_GLYPH_OUTLINE_FORMAT, lpgm: *mut GLYPHMETRICS, pvbuffer: ::core::option::Option<&mut [u8]>, lpmat2: *const MAT2) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetGlyphOutlineA(hdc: HDC, uchar: u32, fuformat: GET_GLYPH_OUTLINE_FORMAT, lpgm: *mut GLYPHMETRICS, cjbuffer: u32, pvbuffer: *mut ::core::ffi::c_void, lpmat2: *const MAT2) -> u32;
    }
    GetGlyphOutlineA(hdc.into(), uchar, fuformat, ::core::mem::transmute(lpgm), pvbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pvbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(lpmat2))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGlyphOutlineW<'a, P0>(hdc: P0, uchar: u32, fuformat: GET_GLYPH_OUTLINE_FORMAT, lpgm: *mut GLYPHMETRICS, pvbuffer: ::core::option::Option<&mut [u8]>, lpmat2: *const MAT2) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetGlyphOutlineW(hdc: HDC, uchar: u32, fuformat: GET_GLYPH_OUTLINE_FORMAT, lpgm: *mut GLYPHMETRICS, cjbuffer: u32, pvbuffer: *mut ::core::ffi::c_void, lpmat2: *const MAT2) -> u32;
    }
    GetGlyphOutlineW(hdc.into(), uchar, fuformat, ::core::mem::transmute(lpgm), pvbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pvbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(lpmat2))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetGraphicsMode<'a, P0>(hdc: P0) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetGraphicsMode(hdc: HDC) -> i32;
    }
    GetGraphicsMode(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetKerningPairsA<'a, P0>(hdc: P0, lpkernpair: ::core::option::Option<&mut [KERNINGPAIR]>) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetKerningPairsA(hdc: HDC, npairs: u32, lpkernpair: *mut KERNINGPAIR) -> u32;
    }
    GetKerningPairsA(hdc.into(), lpkernpair.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpkernpair.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetKerningPairsW<'a, P0>(hdc: P0, lpkernpair: ::core::option::Option<&mut [KERNINGPAIR]>) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetKerningPairsW(hdc: HDC, npairs: u32, lpkernpair: *mut KERNINGPAIR) -> u32;
    }
    GetKerningPairsW(hdc.into(), lpkernpair.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpkernpair.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetLayout<'a, P0>(hdc: P0) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetLayout(hdc: HDC) -> u32;
    }
    GetLayout(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetMapMode<'a, P0>(hdc: P0) -> HDC_MAP_MODE
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMapMode(hdc: HDC) -> HDC_MAP_MODE;
    }
    GetMapMode(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetMetaFileA<'a, P0>(lpname: P0) -> HMETAFILE
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMetaFileA(lpname: ::windows::core::PCSTR) -> HMETAFILE;
    }
    GetMetaFileA(lpname.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetMetaFileBitsEx<'a, P0>(hmf: P0, lpdata: ::core::option::Option<&mut [u8]>) -> u32
where
    P0: ::std::convert::Into<HMETAFILE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMetaFileBitsEx(hmf: HMETAFILE, cbbuffer: u32, lpdata: *mut ::core::ffi::c_void) -> u32;
    }
    GetMetaFileBitsEx(hmf.into(), lpdata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetMetaFileW<'a, P0>(lpname: P0) -> HMETAFILE
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMetaFileW(lpname: ::windows::core::PCWSTR) -> HMETAFILE;
    }
    GetMetaFileW(lpname.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetMetaRgn<'a, P0, P1>(hdc: P0, hrgn: P1) -> i32
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMetaRgn(hdc: HDC, hrgn: HRGN) -> i32;
    }
    GetMetaRgn(hdc.into(), hrgn.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMiterLimit<'a, P0>(hdc: P0, plimit: *mut f32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMiterLimit(hdc: HDC, plimit: *mut f32) -> super::super::Foundation::BOOL;
    }
    GetMiterLimit(hdc.into(), ::core::mem::transmute(plimit))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorInfoA<'a, P0>(hmonitor: P0, lpmi: *mut MONITORINFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMONITOR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMonitorInfoA(hmonitor: HMONITOR, lpmi: *mut MONITORINFO) -> super::super::Foundation::BOOL;
    }
    GetMonitorInfoA(hmonitor.into(), ::core::mem::transmute(lpmi))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorInfoW<'a, P0>(hmonitor: P0, lpmi: *mut MONITORINFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HMONITOR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMonitorInfoW(hmonitor: HMONITOR, lpmi: *mut MONITORINFO) -> super::super::Foundation::BOOL;
    }
    GetMonitorInfoW(hmonitor.into(), ::core::mem::transmute(lpmi))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNearestColor<'a, P0, P1>(hdc: P0, color: P1) -> super::super::Foundation::COLORREF
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetNearestColor(hdc: HDC, color: super::super::Foundation::COLORREF) -> super::super::Foundation::COLORREF;
    }
    GetNearestColor(hdc.into(), color.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNearestPaletteIndex<'a, P0, P1>(h: P0, color: P1) -> u32
where
    P0: ::std::convert::Into<HPALETTE>,
    P1: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetNearestPaletteIndex(h: HPALETTE, color: super::super::Foundation::COLORREF) -> u32;
    }
    GetNearestPaletteIndex(h.into(), color.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetObjectA<'a, P0>(h: P0, pv: ::core::option::Option<&mut [u8]>) -> i32
where
    P0: ::std::convert::Into<HGDIOBJ>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetObjectA(h: HGDIOBJ, c: i32, pv: *mut ::core::ffi::c_void) -> i32;
    }
    GetObjectA(h.into(), pv.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pv.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetObjectType<'a, P0>(h: P0) -> u32
where
    P0: ::std::convert::Into<HGDIOBJ>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetObjectType(h: HGDIOBJ) -> u32;
    }
    GetObjectType(h.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetObjectW<'a, P0>(h: P0, pv: ::core::option::Option<&mut [u8]>) -> i32
where
    P0: ::std::convert::Into<HGDIOBJ>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetObjectW(h: HGDIOBJ, c: i32, pv: *mut ::core::ffi::c_void) -> i32;
    }
    GetObjectW(h.into(), pv.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pv.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOutlineTextMetricsA<'a, P0>(hdc: P0, cjcopy: u32, potm: ::core::option::Option<*mut OUTLINETEXTMETRICA>) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetOutlineTextMetricsA(hdc: HDC, cjcopy: u32, potm: *mut OUTLINETEXTMETRICA) -> u32;
    }
    GetOutlineTextMetricsA(hdc.into(), cjcopy, ::core::mem::transmute(potm.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOutlineTextMetricsW<'a, P0>(hdc: P0, cjcopy: u32, potm: ::core::option::Option<*mut OUTLINETEXTMETRICW>) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetOutlineTextMetricsW(hdc: HDC, cjcopy: u32, potm: *mut OUTLINETEXTMETRICW) -> u32;
    }
    GetOutlineTextMetricsW(hdc.into(), cjcopy, ::core::mem::transmute(potm.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetPaletteEntries<'a, P0>(hpal: P0, istart: u32, ppalentries: ::core::option::Option<&mut [PALETTEENTRY]>) -> u32
where
    P0: ::std::convert::Into<HPALETTE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetPaletteEntries(hpal: HPALETTE, istart: u32, centries: u32, ppalentries: *mut PALETTEENTRY) -> u32;
    }
    GetPaletteEntries(hpal.into(), istart, ppalentries.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppalentries.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPath<'a, P0>(hdc: P0, apt: ::core::option::Option<*mut super::super::Foundation::POINT>, aj: ::core::option::Option<*mut u8>, cpt: i32) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetPath(hdc: HDC, apt: *mut super::super::Foundation::POINT, aj: *mut u8, cpt: i32) -> i32;
    }
    GetPath(hdc.into(), ::core::mem::transmute(apt.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(aj.unwrap_or(::std::ptr::null_mut())), cpt)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPixel<'a, P0>(hdc: P0, x: i32, y: i32) -> super::super::Foundation::COLORREF
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetPixel(hdc: HDC, x: i32, y: i32) -> super::super::Foundation::COLORREF;
    }
    GetPixel(hdc.into(), x, y)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetPolyFillMode<'a, P0>(hdc: P0) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetPolyFillMode(hdc: HDC) -> i32;
    }
    GetPolyFillMode(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetROP2<'a, P0>(hdc: P0) -> R2_MODE
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetROP2(hdc: HDC) -> R2_MODE;
    }
    GetROP2(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetRandomRgn<'a, P0, P1>(hdc: P0, hrgn: P1, i: i32) -> i32
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetRandomRgn(hdc: HDC, hrgn: HRGN, i: i32) -> i32;
    }
    GetRandomRgn(hdc.into(), hrgn.into(), i)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRasterizerCaps(lpraststat: *mut RASTERIZER_STATUS, cjbytes: u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetRasterizerCaps(lpraststat: *mut RASTERIZER_STATUS, cjbytes: u32) -> super::super::Foundation::BOOL;
    }
    GetRasterizerCaps(::core::mem::transmute(lpraststat), cjbytes)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRegionData<'a, P0>(hrgn: P0, ncount: u32, lprgndata: ::core::option::Option<*mut RGNDATA>) -> u32
where
    P0: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetRegionData(hrgn: HRGN, ncount: u32, lprgndata: *mut RGNDATA) -> u32;
    }
    GetRegionData(hrgn.into(), ncount, ::core::mem::transmute(lprgndata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRgnBox<'a, P0>(hrgn: P0, lprc: *mut super::super::Foundation::RECT) -> GDI_REGION_TYPE
where
    P0: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetRgnBox(hrgn: HRGN, lprc: *mut super::super::Foundation::RECT) -> GDI_REGION_TYPE;
    }
    GetRgnBox(hrgn.into(), ::core::mem::transmute(lprc))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetStockObject(i: GET_STOCK_OBJECT_FLAGS) -> HGDIOBJ {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetStockObject(i: GET_STOCK_OBJECT_FLAGS) -> HGDIOBJ;
    }
    GetStockObject(i)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetStretchBltMode<'a, P0>(hdc: P0) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetStretchBltMode(hdc: HDC) -> i32;
    }
    GetStretchBltMode(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetSysColorBrush(nindex: i32) -> HBRUSH {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetSysColorBrush(nindex: i32) -> HBRUSH;
    }
    GetSysColorBrush(nindex)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetSystemPaletteEntries<'a, P0>(hdc: P0, istart: u32, ppalentries: ::core::option::Option<&mut [PALETTEENTRY]>) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetSystemPaletteEntries(hdc: HDC, istart: u32, centries: u32, ppalentries: *mut PALETTEENTRY) -> u32;
    }
    GetSystemPaletteEntries(hdc.into(), istart, ppalentries.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppalentries.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetSystemPaletteUse<'a, P0>(hdc: P0) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetSystemPaletteUse(hdc: HDC) -> u32;
    }
    GetSystemPaletteUse(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetTabbedTextExtentA<'a, P0>(hdc: P0, lpstring: &[u8], lpntabstoppositions: ::core::option::Option<&[i32]>) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTabbedTextExtentA(hdc: HDC, lpstring: ::windows::core::PCSTR, chcount: i32, ntabpositions: i32, lpntabstoppositions: *const i32) -> u32;
    }
    GetTabbedTextExtentA(hdc.into(), ::core::mem::transmute(lpstring.as_ptr()), lpstring.len() as _, lpntabstoppositions.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpntabstoppositions.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetTabbedTextExtentW<'a, P0>(hdc: P0, lpstring: &[u16], lpntabstoppositions: ::core::option::Option<&[i32]>) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTabbedTextExtentW(hdc: HDC, lpstring: ::windows::core::PCWSTR, chcount: i32, ntabpositions: i32, lpntabstoppositions: *const i32) -> u32;
    }
    GetTabbedTextExtentW(hdc.into(), ::core::mem::transmute(lpstring.as_ptr()), lpstring.len() as _, lpntabstoppositions.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpntabstoppositions.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetTextAlign<'a, P0>(hdc: P0) -> TEXT_ALIGN_OPTIONS
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTextAlign(hdc: HDC) -> TEXT_ALIGN_OPTIONS;
    }
    GetTextAlign(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetTextCharacterExtra<'a, P0>(hdc: P0) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTextCharacterExtra(hdc: HDC) -> i32;
    }
    GetTextCharacterExtra(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTextColor<'a, P0>(hdc: P0) -> super::super::Foundation::COLORREF
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTextColor(hdc: HDC) -> super::super::Foundation::COLORREF;
    }
    GetTextColor(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTextExtentExPointA<'a, P0, P1>(hdc: P0, lpszstring: P1, cchstring: i32, nmaxextent: i32, lpnfit: ::core::option::Option<*mut i32>, lpndx: ::core::option::Option<*mut i32>, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTextExtentExPointA(hdc: HDC, lpszstring: ::windows::core::PCSTR, cchstring: i32, nmaxextent: i32, lpnfit: *mut i32, lpndx: *mut i32, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    }
    GetTextExtentExPointA(hdc.into(), lpszstring.into(), cchstring, nmaxextent, ::core::mem::transmute(lpnfit.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpndx.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpsize))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTextExtentExPointI<'a, P0>(hdc: P0, lpwszstring: *const u16, cwchstring: i32, nmaxextent: i32, lpnfit: ::core::option::Option<*mut i32>, lpndx: ::core::option::Option<*mut i32>, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTextExtentExPointI(hdc: HDC, lpwszstring: *const u16, cwchstring: i32, nmaxextent: i32, lpnfit: *mut i32, lpndx: *mut i32, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    }
    GetTextExtentExPointI(hdc.into(), ::core::mem::transmute(lpwszstring), cwchstring, nmaxextent, ::core::mem::transmute(lpnfit.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpndx.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpsize))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTextExtentExPointW<'a, P0, P1>(hdc: P0, lpszstring: P1, cchstring: i32, nmaxextent: i32, lpnfit: ::core::option::Option<*mut i32>, lpndx: ::core::option::Option<*mut i32>, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTextExtentExPointW(hdc: HDC, lpszstring: ::windows::core::PCWSTR, cchstring: i32, nmaxextent: i32, lpnfit: *mut i32, lpndx: *mut i32, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    }
    GetTextExtentExPointW(hdc.into(), lpszstring.into(), cchstring, nmaxextent, ::core::mem::transmute(lpnfit.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpndx.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpsize))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTextExtentPoint32A<'a, P0>(hdc: P0, lpstring: &[u8], psizl: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTextExtentPoint32A(hdc: HDC, lpstring: ::windows::core::PCSTR, c: i32, psizl: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    }
    GetTextExtentPoint32A(hdc.into(), ::core::mem::transmute(lpstring.as_ptr()), lpstring.len() as _, ::core::mem::transmute(psizl))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTextExtentPoint32W<'a, P0>(hdc: P0, lpstring: &[u16], psizl: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTextExtentPoint32W(hdc: HDC, lpstring: ::windows::core::PCWSTR, c: i32, psizl: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    }
    GetTextExtentPoint32W(hdc.into(), ::core::mem::transmute(lpstring.as_ptr()), lpstring.len() as _, ::core::mem::transmute(psizl))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTextExtentPointA<'a, P0>(hdc: P0, lpstring: &[u8], lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTextExtentPointA(hdc: HDC, lpstring: ::windows::core::PCSTR, c: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    }
    GetTextExtentPointA(hdc.into(), ::core::mem::transmute(lpstring.as_ptr()), lpstring.len() as _, ::core::mem::transmute(lpsz))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTextExtentPointI<'a, P0>(hdc: P0, pgiin: &[u16], psize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTextExtentPointI(hdc: HDC, pgiin: *const u16, cgi: i32, psize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    }
    GetTextExtentPointI(hdc.into(), ::core::mem::transmute(pgiin.as_ptr()), pgiin.len() as _, ::core::mem::transmute(psize))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTextExtentPointW<'a, P0>(hdc: P0, lpstring: &[u16], lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTextExtentPointW(hdc: HDC, lpstring: ::windows::core::PCWSTR, c: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    }
    GetTextExtentPointW(hdc.into(), ::core::mem::transmute(lpstring.as_ptr()), lpstring.len() as _, ::core::mem::transmute(lpsz))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetTextFaceA<'a, P0>(hdc: P0, lpname: ::core::option::Option<&mut [u8]>) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTextFaceA(hdc: HDC, c: i32, lpname: ::windows::core::PSTR) -> i32;
    }
    GetTextFaceA(hdc.into(), lpname.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetTextFaceW<'a, P0>(hdc: P0, lpname: ::core::option::Option<&mut [u16]>) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTextFaceW(hdc: HDC, c: i32, lpname: ::windows::core::PWSTR) -> i32;
    }
    GetTextFaceW(hdc.into(), lpname.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTextMetricsA<'a, P0>(hdc: P0, lptm: *mut TEXTMETRICA) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTextMetricsA(hdc: HDC, lptm: *mut TEXTMETRICA) -> super::super::Foundation::BOOL;
    }
    GetTextMetricsA(hdc.into(), ::core::mem::transmute(lptm))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTextMetricsW<'a, P0>(hdc: P0, lptm: *mut TEXTMETRICW) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTextMetricsW(hdc: HDC, lptm: *mut TEXTMETRICW) -> super::super::Foundation::BOOL;
    }
    GetTextMetricsW(hdc.into(), ::core::mem::transmute(lptm))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUpdateRect<'a, P0, P1>(hwnd: P0, lprect: ::core::option::Option<*mut super::super::Foundation::RECT>, berase: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetUpdateRect(hwnd: super::super::Foundation::HWND, lprect: *mut super::super::Foundation::RECT, berase: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    GetUpdateRect(hwnd.into(), ::core::mem::transmute(lprect.unwrap_or(::std::ptr::null_mut())), berase.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUpdateRgn<'a, P0, P1, P2>(hwnd: P0, hrgn: P1, berase: P2) -> GDI_REGION_TYPE
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<HRGN>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetUpdateRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN, berase: super::super::Foundation::BOOL) -> GDI_REGION_TYPE;
    }
    GetUpdateRgn(hwnd.into(), hrgn.into(), berase.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetViewportExtEx<'a, P0>(hdc: P0, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetViewportExtEx(hdc: HDC, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    }
    GetViewportExtEx(hdc.into(), ::core::mem::transmute(lpsize))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetViewportOrgEx<'a, P0>(hdc: P0, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetViewportOrgEx(hdc: HDC, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    GetViewportOrgEx(hdc.into(), ::core::mem::transmute(lppoint))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn GetWinMetaFileBits<'a, P0, P1>(hemf: P0, pdata16: ::core::option::Option<&mut [u8]>, imapmode: i32, hdcref: P1) -> u32
where
    P0: ::std::convert::Into<HENHMETAFILE>,
    P1: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWinMetaFileBits(hemf: HENHMETAFILE, cbdata16: u32, pdata16: *mut u8, imapmode: i32, hdcref: HDC) -> u32;
    }
    GetWinMetaFileBits(hemf.into(), pdata16.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pdata16.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), imapmode, hdcref.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowDC<'a, P0>(hwnd: P0) -> HDC
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowDC(hwnd: super::super::Foundation::HWND) -> HDC;
    }
    GetWindowDC(hwnd.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowExtEx<'a, P0>(hdc: P0, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowExtEx(hdc: HDC, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    }
    GetWindowExtEx(hdc.into(), ::core::mem::transmute(lpsize))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowOrgEx<'a, P0>(hdc: P0, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowOrgEx(hdc: HDC, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    GetWindowOrgEx(hdc.into(), ::core::mem::transmute(lppoint))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowRgn<'a, P0, P1>(hwnd: P0, hrgn: P1) -> GDI_REGION_TYPE
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN) -> GDI_REGION_TYPE;
    }
    GetWindowRgn(hwnd.into(), hrgn.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowRgnBox<'a, P0>(hwnd: P0, lprc: *mut super::super::Foundation::RECT) -> GDI_REGION_TYPE
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowRgnBox(hwnd: super::super::Foundation::HWND, lprc: *mut super::super::Foundation::RECT) -> GDI_REGION_TYPE;
    }
    GetWindowRgnBox(hwnd.into(), ::core::mem::transmute(lprc))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWorldTransform<'a, P0>(hdc: P0, lpxf: *mut XFORM) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWorldTransform(hdc: HDC, lpxf: *mut XFORM) -> super::super::Foundation::BOOL;
    }
    GetWorldTransform(hdc.into(), ::core::mem::transmute(lpxf))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GradientFill<'a, P0>(hdc: P0, pvertex: &[TRIVERTEX], pmesh: *const ::core::ffi::c_void, nmesh: u32, ulmode: GRADIENT_FILL) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GradientFill(hdc: HDC, pvertex: *const TRIVERTEX, nvertex: u32, pmesh: *const ::core::ffi::c_void, nmesh: u32, ulmode: GRADIENT_FILL) -> super::super::Foundation::BOOL;
    }
    GradientFill(hdc.into(), ::core::mem::transmute(pvertex.as_ptr()), pvertex.len() as _, ::core::mem::transmute(pmesh), nmesh, ulmode)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GrayStringA<'a, P0, P1, P2>(hdc: P0, hbrush: P1, lpoutputfunc: GRAYSTRINGPROC, lpdata: P2, ncount: i32, x: i32, y: i32, nwidth: i32, nheight: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HBRUSH>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GrayStringA(hdc: HDC, hbrush: HBRUSH, lpoutputfunc: *mut ::core::ffi::c_void, lpdata: super::super::Foundation::LPARAM, ncount: i32, x: i32, y: i32, nwidth: i32, nheight: i32) -> super::super::Foundation::BOOL;
    }
    GrayStringA(hdc.into(), hbrush.into(), ::core::mem::transmute(lpoutputfunc), lpdata.into(), ncount, x, y, nwidth, nheight)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GrayStringW<'a, P0, P1, P2>(hdc: P0, hbrush: P1, lpoutputfunc: GRAYSTRINGPROC, lpdata: P2, ncount: i32, x: i32, y: i32, nwidth: i32, nheight: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HBRUSH>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GrayStringW(hdc: HDC, hbrush: HBRUSH, lpoutputfunc: *mut ::core::ffi::c_void, lpdata: super::super::Foundation::LPARAM, ncount: i32, x: i32, y: i32, nwidth: i32, nheight: i32) -> super::super::Foundation::BOOL;
    }
    GrayStringW(hdc.into(), hbrush.into(), ::core::mem::transmute(lpoutputfunc), lpdata.into(), ncount, x, y, nwidth, nheight)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InflateRect(lprc: *mut super::super::Foundation::RECT, dx: i32, dy: i32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InflateRect(lprc: *mut super::super::Foundation::RECT, dx: i32, dy: i32) -> super::super::Foundation::BOOL;
    }
    InflateRect(::core::mem::transmute(lprc), dx, dy)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn IntersectClipRect<'a, P0>(hdc: P0, left: i32, top: i32, right: i32, bottom: i32) -> GDI_REGION_TYPE
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IntersectClipRect(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> GDI_REGION_TYPE;
    }
    IntersectClipRect(hdc.into(), left, top, right, bottom)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IntersectRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc1: *const super::super::Foundation::RECT, lprcsrc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IntersectRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc1: *const super::super::Foundation::RECT, lprcsrc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    IntersectRect(::core::mem::transmute(lprcdst), ::core::mem::transmute(lprcsrc1), ::core::mem::transmute(lprcsrc2))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InvalidateRect<'a, P0, P1>(hwnd: P0, lprect: ::core::option::Option<*const super::super::Foundation::RECT>, berase: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InvalidateRect(hwnd: super::super::Foundation::HWND, lprect: *const super::super::Foundation::RECT, berase: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    InvalidateRect(hwnd.into(), ::core::mem::transmute(lprect.unwrap_or(::std::ptr::null())), berase.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InvalidateRgn<'a, P0, P1, P2>(hwnd: P0, hrgn: P1, berase: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<HRGN>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InvalidateRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN, berase: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    InvalidateRgn(hwnd.into(), hrgn.into(), berase.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InvertRect<'a, P0>(hdc: P0, lprc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InvertRect(hdc: HDC, lprc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    InvertRect(hdc.into(), ::core::mem::transmute(lprc))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InvertRgn<'a, P0, P1>(hdc: P0, hrgn: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InvertRgn(hdc: HDC, hrgn: HRGN) -> super::super::Foundation::BOOL;
    }
    InvertRgn(hdc.into(), hrgn.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsRectEmpty(lprc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsRectEmpty(lprc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    IsRectEmpty(::core::mem::transmute(lprc))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LPtoDP<'a, P0>(hdc: P0, lppt: &mut [super::super::Foundation::POINT]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LPtoDP(hdc: HDC, lppt: *mut super::super::Foundation::POINT, c: i32) -> super::super::Foundation::BOOL;
    }
    LPtoDP(hdc.into(), ::core::mem::transmute(lppt.as_ptr()), lppt.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LineDDA<'a, P0>(xstart: i32, ystart: i32, xend: i32, yend: i32, lpproc: LINEDDAPROC, data: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LineDDA(xstart: i32, ystart: i32, xend: i32, yend: i32, lpproc: *mut ::core::ffi::c_void, data: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    }
    LineDDA(xstart, ystart, xend, yend, ::core::mem::transmute(lpproc), data.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LineTo<'a, P0>(hdc: P0, x: i32, y: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LineTo(hdc: HDC, x: i32, y: i32) -> super::super::Foundation::BOOL;
    }
    LineTo(hdc.into(), x, y)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadBitmapA<'a, P0, P1>(hinstance: P0, lpbitmapname: P1) -> HBITMAP
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadBitmapA(hinstance: super::super::Foundation::HINSTANCE, lpbitmapname: ::windows::core::PCSTR) -> HBITMAP;
    }
    LoadBitmapA(hinstance.into(), lpbitmapname.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadBitmapW<'a, P0, P1>(hinstance: P0, lpbitmapname: P1) -> HBITMAP
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadBitmapW(hinstance: super::super::Foundation::HINSTANCE, lpbitmapname: ::windows::core::PCWSTR) -> HBITMAP;
    }
    LoadBitmapW(hinstance.into(), lpbitmapname.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LockWindowUpdate<'a, P0>(hwndlock: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LockWindowUpdate(hwndlock: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    LockWindowUpdate(hwndlock.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapWindowPoints<'a, P0, P1>(hwndfrom: P0, hwndto: P1, lppoints: &mut [super::super::Foundation::POINT]) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MapWindowPoints(hwndfrom: super::super::Foundation::HWND, hwndto: super::super::Foundation::HWND, lppoints: *mut super::super::Foundation::POINT, cpoints: u32) -> i32;
    }
    MapWindowPoints(hwndfrom.into(), hwndto.into(), ::core::mem::transmute(lppoints.as_ptr()), lppoints.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MaskBlt<'a, P0, P1, P2>(hdcdest: P0, xdest: i32, ydest: i32, width: i32, height: i32, hdcsrc: P1, xsrc: i32, ysrc: i32, hbmmask: P2, xmask: i32, ymask: i32, rop: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HDC>,
    P2: ::std::convert::Into<HBITMAP>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MaskBlt(hdcdest: HDC, xdest: i32, ydest: i32, width: i32, height: i32, hdcsrc: HDC, xsrc: i32, ysrc: i32, hbmmask: HBITMAP, xmask: i32, ymask: i32, rop: u32) -> super::super::Foundation::BOOL;
    }
    MaskBlt(hdcdest.into(), xdest, ydest, width, height, hdcsrc.into(), xsrc, ysrc, hbmmask.into(), xmask, ymask, rop)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ModifyWorldTransform<'a, P0>(hdc: P0, lpxf: ::core::option::Option<*const XFORM>, mode: MODIFY_WORLD_TRANSFORM_MODE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ModifyWorldTransform(hdc: HDC, lpxf: *const XFORM, mode: MODIFY_WORLD_TRANSFORM_MODE) -> super::super::Foundation::BOOL;
    }
    ModifyWorldTransform(hdc.into(), ::core::mem::transmute(lpxf.unwrap_or(::std::ptr::null())), mode)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MonitorFromPoint(pt: super::super::Foundation::POINT, dwflags: MONITOR_FROM_FLAGS) -> HMONITOR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MonitorFromPoint(pt: super::super::Foundation::POINT, dwflags: MONITOR_FROM_FLAGS) -> HMONITOR;
    }
    MonitorFromPoint(::core::mem::transmute(pt), dwflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MonitorFromRect(lprc: *const super::super::Foundation::RECT, dwflags: MONITOR_FROM_FLAGS) -> HMONITOR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MonitorFromRect(lprc: *const super::super::Foundation::RECT, dwflags: MONITOR_FROM_FLAGS) -> HMONITOR;
    }
    MonitorFromRect(::core::mem::transmute(lprc), dwflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MonitorFromWindow<'a, P0>(hwnd: P0, dwflags: MONITOR_FROM_FLAGS) -> HMONITOR
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MonitorFromWindow(hwnd: super::super::Foundation::HWND, dwflags: MONITOR_FROM_FLAGS) -> HMONITOR;
    }
    MonitorFromWindow(hwnd.into(), dwflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveToEx<'a, P0>(hdc: P0, x: i32, y: i32, lppt: ::core::option::Option<*mut super::super::Foundation::POINT>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MoveToEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    MoveToEx(hdc.into(), x, y, ::core::mem::transmute(lppt.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn OffsetClipRgn<'a, P0>(hdc: P0, x: i32, y: i32) -> GDI_REGION_TYPE
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OffsetClipRgn(hdc: HDC, x: i32, y: i32) -> GDI_REGION_TYPE;
    }
    OffsetClipRgn(hdc.into(), x, y)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OffsetRect(lprc: *mut super::super::Foundation::RECT, dx: i32, dy: i32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OffsetRect(lprc: *mut super::super::Foundation::RECT, dx: i32, dy: i32) -> super::super::Foundation::BOOL;
    }
    OffsetRect(::core::mem::transmute(lprc), dx, dy)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn OffsetRgn<'a, P0>(hrgn: P0, x: i32, y: i32) -> GDI_REGION_TYPE
where
    P0: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OffsetRgn(hrgn: HRGN, x: i32, y: i32) -> GDI_REGION_TYPE;
    }
    OffsetRgn(hrgn.into(), x, y)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OffsetViewportOrgEx<'a, P0>(hdc: P0, x: i32, y: i32, lppt: ::core::option::Option<*mut super::super::Foundation::POINT>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OffsetViewportOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    OffsetViewportOrgEx(hdc.into(), x, y, ::core::mem::transmute(lppt.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OffsetWindowOrgEx<'a, P0>(hdc: P0, x: i32, y: i32, lppt: ::core::option::Option<*mut super::super::Foundation::POINT>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OffsetWindowOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    OffsetWindowOrgEx(hdc.into(), x, y, ::core::mem::transmute(lppt.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PaintDesktop<'a, P0>(hdc: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PaintDesktop(hdc: HDC) -> super::super::Foundation::BOOL;
    }
    PaintDesktop(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PaintRgn<'a, P0, P1>(hdc: P0, hrgn: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PaintRgn(hdc: HDC, hrgn: HRGN) -> super::super::Foundation::BOOL;
    }
    PaintRgn(hdc.into(), hrgn.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PatBlt<'a, P0>(hdc: P0, x: i32, y: i32, w: i32, h: i32, rop: ROP_CODE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PatBlt(hdc: HDC, x: i32, y: i32, w: i32, h: i32, rop: ROP_CODE) -> super::super::Foundation::BOOL;
    }
    PatBlt(hdc.into(), x, y, w, h, rop)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn PathToRegion<'a, P0>(hdc: P0) -> HRGN
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PathToRegion(hdc: HDC) -> HRGN;
    }
    PathToRegion(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Pie<'a, P0>(hdc: P0, left: i32, top: i32, right: i32, bottom: i32, xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn Pie(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32, xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> super::super::Foundation::BOOL;
    }
    Pie(hdc.into(), left, top, right, bottom, xr1, yr1, xr2, yr2)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PlayEnhMetaFile<'a, P0, P1>(hdc: P0, hmf: P1, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HENHMETAFILE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PlayEnhMetaFile(hdc: HDC, hmf: HENHMETAFILE, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    PlayEnhMetaFile(hdc.into(), hmf.into(), ::core::mem::transmute(lprect))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PlayEnhMetaFileRecord<'a, P0>(hdc: P0, pht: &[HANDLETABLE], pmr: *const ENHMETARECORD) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PlayEnhMetaFileRecord(hdc: HDC, pht: *const HANDLETABLE, pmr: *const ENHMETARECORD, cht: u32) -> super::super::Foundation::BOOL;
    }
    PlayEnhMetaFileRecord(hdc.into(), ::core::mem::transmute(pht.as_ptr()), ::core::mem::transmute(pmr), pht.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PlayMetaFile<'a, P0, P1>(hdc: P0, hmf: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HMETAFILE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PlayMetaFile(hdc: HDC, hmf: HMETAFILE) -> super::super::Foundation::BOOL;
    }
    PlayMetaFile(hdc.into(), hmf.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PlayMetaFileRecord<'a, P0>(hdc: P0, lphandletable: &[HANDLETABLE], lpmr: *const METARECORD) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PlayMetaFileRecord(hdc: HDC, lphandletable: *const HANDLETABLE, lpmr: *const METARECORD, noobjs: u32) -> super::super::Foundation::BOOL;
    }
    PlayMetaFileRecord(hdc.into(), ::core::mem::transmute(lphandletable.as_ptr()), ::core::mem::transmute(lpmr), lphandletable.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PlgBlt<'a, P0, P1, P2>(hdcdest: P0, lppoint: &[super::super::Foundation::POINT; 3], hdcsrc: P1, xsrc: i32, ysrc: i32, width: i32, height: i32, hbmmask: P2, xmask: i32, ymask: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HDC>,
    P2: ::std::convert::Into<HBITMAP>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PlgBlt(hdcdest: HDC, lppoint: *const super::super::Foundation::POINT, hdcsrc: HDC, xsrc: i32, ysrc: i32, width: i32, height: i32, hbmmask: HBITMAP, xmask: i32, ymask: i32) -> super::super::Foundation::BOOL;
    }
    PlgBlt(hdcdest.into(), ::core::mem::transmute(lppoint.as_ptr()), hdcsrc.into(), xsrc, ysrc, width, height, hbmmask.into(), xmask, ymask)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PolyBezier<'a, P0>(hdc: P0, apt: &[super::super::Foundation::POINT]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PolyBezier(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: u32) -> super::super::Foundation::BOOL;
    }
    PolyBezier(hdc.into(), ::core::mem::transmute(apt.as_ptr()), apt.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PolyBezierTo<'a, P0>(hdc: P0, apt: &[super::super::Foundation::POINT]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PolyBezierTo(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: u32) -> super::super::Foundation::BOOL;
    }
    PolyBezierTo(hdc.into(), ::core::mem::transmute(apt.as_ptr()), apt.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PolyDraw<'a, P0>(hdc: P0, apt: *const super::super::Foundation::POINT, aj: *const u8, cpt: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PolyDraw(hdc: HDC, apt: *const super::super::Foundation::POINT, aj: *const u8, cpt: i32) -> super::super::Foundation::BOOL;
    }
    PolyDraw(hdc.into(), ::core::mem::transmute(apt), ::core::mem::transmute(aj), cpt)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PolyPolygon<'a, P0>(hdc: P0, apt: *const super::super::Foundation::POINT, asz: &[i32]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PolyPolygon(hdc: HDC, apt: *const super::super::Foundation::POINT, asz: *const i32, csz: i32) -> super::super::Foundation::BOOL;
    }
    PolyPolygon(hdc.into(), ::core::mem::transmute(apt), ::core::mem::transmute(asz.as_ptr()), asz.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PolyPolyline<'a, P0>(hdc: P0, apt: *const super::super::Foundation::POINT, asz: &[u32]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PolyPolyline(hdc: HDC, apt: *const super::super::Foundation::POINT, asz: *const u32, csz: u32) -> super::super::Foundation::BOOL;
    }
    PolyPolyline(hdc.into(), ::core::mem::transmute(apt), ::core::mem::transmute(asz.as_ptr()), asz.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PolyTextOutA<'a, P0>(hdc: P0, ppt: &[POLYTEXTA]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PolyTextOutA(hdc: HDC, ppt: *const POLYTEXTA, nstrings: i32) -> super::super::Foundation::BOOL;
    }
    PolyTextOutA(hdc.into(), ::core::mem::transmute(ppt.as_ptr()), ppt.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PolyTextOutW<'a, P0>(hdc: P0, ppt: &[POLYTEXTW]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PolyTextOutW(hdc: HDC, ppt: *const POLYTEXTW, nstrings: i32) -> super::super::Foundation::BOOL;
    }
    PolyTextOutW(hdc.into(), ::core::mem::transmute(ppt.as_ptr()), ppt.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Polygon<'a, P0>(hdc: P0, apt: &[super::super::Foundation::POINT]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn Polygon(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: i32) -> super::super::Foundation::BOOL;
    }
    Polygon(hdc.into(), ::core::mem::transmute(apt.as_ptr()), apt.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Polyline<'a, P0>(hdc: P0, apt: &[super::super::Foundation::POINT]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn Polyline(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: i32) -> super::super::Foundation::BOOL;
    }
    Polyline(hdc.into(), ::core::mem::transmute(apt.as_ptr()), apt.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PolylineTo<'a, P0>(hdc: P0, apt: &[super::super::Foundation::POINT]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PolylineTo(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: u32) -> super::super::Foundation::BOOL;
    }
    PolylineTo(hdc.into(), ::core::mem::transmute(apt.as_ptr()), apt.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PtInRect(lprc: *const super::super::Foundation::RECT, pt: super::super::Foundation::POINT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PtInRect(lprc: *const super::super::Foundation::RECT, pt: super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    PtInRect(::core::mem::transmute(lprc), ::core::mem::transmute(pt))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PtInRegion<'a, P0>(hrgn: P0, x: i32, y: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PtInRegion(hrgn: HRGN, x: i32, y: i32) -> super::super::Foundation::BOOL;
    }
    PtInRegion(hrgn.into(), x, y)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PtVisible<'a, P0>(hdc: P0, x: i32, y: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PtVisible(hdc: HDC, x: i32, y: i32) -> super::super::Foundation::BOOL;
    }
    PtVisible(hdc.into(), x, y)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn RealizePalette<'a, P0>(hdc: P0) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RealizePalette(hdc: HDC) -> u32;
    }
    RealizePalette(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RectInRegion<'a, P0>(hrgn: P0, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RectInRegion(hrgn: HRGN, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    RectInRegion(hrgn.into(), ::core::mem::transmute(lprect))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RectVisible<'a, P0>(hdc: P0, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RectVisible(hdc: HDC, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    RectVisible(hdc.into(), ::core::mem::transmute(lprect))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Rectangle<'a, P0>(hdc: P0, left: i32, top: i32, right: i32, bottom: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn Rectangle(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> super::super::Foundation::BOOL;
    }
    Rectangle(hdc.into(), left, top, right, bottom)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RedrawWindow<'a, P0, P1>(hwnd: P0, lprcupdate: ::core::option::Option<*const super::super::Foundation::RECT>, hrgnupdate: P1, flags: REDRAW_WINDOW_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RedrawWindow(hwnd: super::super::Foundation::HWND, lprcupdate: *const super::super::Foundation::RECT, hrgnupdate: HRGN, flags: REDRAW_WINDOW_FLAGS) -> super::super::Foundation::BOOL;
    }
    RedrawWindow(hwnd.into(), ::core::mem::transmute(lprcupdate.unwrap_or(::std::ptr::null())), hrgnupdate.into(), flags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReleaseDC<'a, P0, P1>(hwnd: P0, hdc: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ReleaseDC(hwnd: super::super::Foundation::HWND, hdc: HDC) -> i32;
    }
    ReleaseDC(hwnd.into(), hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveFontMemResourceEx<'a, P0>(h: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RemoveFontMemResourceEx(h: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    }
    RemoveFontMemResourceEx(h.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveFontResourceA<'a, P0>(lpfilename: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RemoveFontResourceA(lpfilename: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
    }
    RemoveFontResourceA(lpfilename.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveFontResourceExA<'a, P0>(name: P0, fl: u32, pdv: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RemoveFontResourceExA(name: ::windows::core::PCSTR, fl: u32, pdv: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    RemoveFontResourceExA(name.into(), fl, ::core::mem::transmute(pdv))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveFontResourceExW<'a, P0>(name: P0, fl: u32, pdv: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RemoveFontResourceExW(name: ::windows::core::PCWSTR, fl: u32, pdv: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    RemoveFontResourceExW(name.into(), fl, ::core::mem::transmute(pdv))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveFontResourceW<'a, P0>(lpfilename: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RemoveFontResourceW(lpfilename: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    RemoveFontResourceW(lpfilename.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResetDCA<'a, P0>(hdc: P0, lpdm: *const DEVMODEA) -> HDC
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ResetDCA(hdc: HDC, lpdm: *const DEVMODEA) -> HDC;
    }
    ResetDCA(hdc.into(), ::core::mem::transmute(lpdm))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResetDCW<'a, P0>(hdc: P0, lpdm: *const DEVMODEW) -> HDC
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ResetDCW(hdc: HDC, lpdm: *const DEVMODEW) -> HDC;
    }
    ResetDCW(hdc.into(), ::core::mem::transmute(lpdm))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResizePalette<'a, P0>(hpal: P0, n: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HPALETTE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ResizePalette(hpal: HPALETTE, n: u32) -> super::super::Foundation::BOOL;
    }
    ResizePalette(hpal.into(), n)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RestoreDC<'a, P0>(hdc: P0, nsaveddc: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RestoreDC(hdc: HDC, nsaveddc: i32) -> super::super::Foundation::BOOL;
    }
    RestoreDC(hdc.into(), nsaveddc)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RoundRect<'a, P0>(hdc: P0, left: i32, top: i32, right: i32, bottom: i32, width: i32, height: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RoundRect(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32, width: i32, height: i32) -> super::super::Foundation::BOOL;
    }
    RoundRect(hdc.into(), left, top, right, bottom, width, height)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SaveDC<'a, P0>(hdc: P0) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SaveDC(hdc: HDC) -> i32;
    }
    SaveDC(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ScaleViewportExtEx<'a, P0>(hdc: P0, xn: i32, dx: i32, yn: i32, yd: i32, lpsz: ::core::option::Option<*mut super::super::Foundation::SIZE>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScaleViewportExtEx(hdc: HDC, xn: i32, dx: i32, yn: i32, yd: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    }
    ScaleViewportExtEx(hdc.into(), xn, dx, yn, yd, ::core::mem::transmute(lpsz.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ScaleWindowExtEx<'a, P0>(hdc: P0, xn: i32, xd: i32, yn: i32, yd: i32, lpsz: ::core::option::Option<*mut super::super::Foundation::SIZE>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScaleWindowExtEx(hdc: HDC, xn: i32, xd: i32, yn: i32, yd: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    }
    ScaleWindowExtEx(hdc.into(), xn, xd, yn, yd, ::core::mem::transmute(lpsz.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ScreenToClient<'a, P0>(hwnd: P0, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScreenToClient(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    ScreenToClient(hwnd.into(), ::core::mem::transmute(lppoint))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SelectClipPath<'a, P0>(hdc: P0, mode: RGN_COMBINE_MODE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SelectClipPath(hdc: HDC, mode: RGN_COMBINE_MODE) -> super::super::Foundation::BOOL;
    }
    SelectClipPath(hdc.into(), mode)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SelectClipRgn<'a, P0, P1>(hdc: P0, hrgn: P1) -> GDI_REGION_TYPE
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SelectClipRgn(hdc: HDC, hrgn: HRGN) -> GDI_REGION_TYPE;
    }
    SelectClipRgn(hdc.into(), hrgn.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SelectObject<'a, P0, P1>(hdc: P0, h: P1) -> HGDIOBJ
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HGDIOBJ>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SelectObject(hdc: HDC, h: HGDIOBJ) -> HGDIOBJ;
    }
    SelectObject(hdc.into(), h.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SelectPalette<'a, P0, P1, P2>(hdc: P0, hpal: P1, bforcebkgd: P2) -> HPALETTE
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HPALETTE>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SelectPalette(hdc: HDC, hpal: HPALETTE, bforcebkgd: super::super::Foundation::BOOL) -> HPALETTE;
    }
    SelectPalette(hdc.into(), hpal.into(), bforcebkgd.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetArcDirection<'a, P0>(hdc: P0, dir: ARC_DIRECTION) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetArcDirection(hdc: HDC, dir: ARC_DIRECTION) -> i32;
    }
    SetArcDirection(hdc.into(), dir)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetBitmapBits<'a, P0>(hbm: P0, pvbits: &[u8]) -> i32
where
    P0: ::std::convert::Into<HBITMAP>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetBitmapBits(hbm: HBITMAP, cb: u32, pvbits: *const ::core::ffi::c_void) -> i32;
    }
    SetBitmapBits(hbm.into(), pvbits.len() as _, ::core::mem::transmute(pvbits.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetBitmapDimensionEx<'a, P0>(hbm: P0, w: i32, h: i32, lpsz: ::core::option::Option<*mut super::super::Foundation::SIZE>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HBITMAP>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetBitmapDimensionEx(hbm: HBITMAP, w: i32, h: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    }
    SetBitmapDimensionEx(hbm.into(), w, h, ::core::mem::transmute(lpsz.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetBkColor<'a, P0, P1>(hdc: P0, color: P1) -> super::super::Foundation::COLORREF
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetBkColor(hdc: HDC, color: super::super::Foundation::COLORREF) -> super::super::Foundation::COLORREF;
    }
    SetBkColor(hdc.into(), color.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetBkMode<'a, P0>(hdc: P0, mode: BACKGROUND_MODE) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetBkMode(hdc: HDC, mode: BACKGROUND_MODE) -> i32;
    }
    SetBkMode(hdc.into(), mode)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetBoundsRect<'a, P0>(hdc: P0, lprect: ::core::option::Option<*const super::super::Foundation::RECT>, flags: SET_BOUNDS_RECT_FLAGS) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetBoundsRect(hdc: HDC, lprect: *const super::super::Foundation::RECT, flags: SET_BOUNDS_RECT_FLAGS) -> u32;
    }
    SetBoundsRect(hdc.into(), ::core::mem::transmute(lprect.unwrap_or(::std::ptr::null())), flags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetBrushOrgEx<'a, P0>(hdc: P0, x: i32, y: i32, lppt: ::core::option::Option<*mut super::super::Foundation::POINT>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetBrushOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    SetBrushOrgEx(hdc.into(), x, y, ::core::mem::transmute(lppt.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetColorAdjustment<'a, P0>(hdc: P0, lpca: *const COLORADJUSTMENT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetColorAdjustment(hdc: HDC, lpca: *const COLORADJUSTMENT) -> super::super::Foundation::BOOL;
    }
    SetColorAdjustment(hdc.into(), ::core::mem::transmute(lpca))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDCBrushColor<'a, P0, P1>(hdc: P0, color: P1) -> super::super::Foundation::COLORREF
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetDCBrushColor(hdc: HDC, color: super::super::Foundation::COLORREF) -> super::super::Foundation::COLORREF;
    }
    SetDCBrushColor(hdc.into(), color.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDCPenColor<'a, P0, P1>(hdc: P0, color: P1) -> super::super::Foundation::COLORREF
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetDCPenColor(hdc: HDC, color: super::super::Foundation::COLORREF) -> super::super::Foundation::COLORREF;
    }
    SetDCPenColor(hdc.into(), color.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetDIBColorTable<'a, P0>(hdc: P0, istart: u32, prgbq: &[RGBQUAD]) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetDIBColorTable(hdc: HDC, istart: u32, centries: u32, prgbq: *const RGBQUAD) -> u32;
    }
    SetDIBColorTable(hdc.into(), istart, prgbq.len() as _, ::core::mem::transmute(prgbq.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetDIBits<'a, P0, P1>(hdc: P0, hbm: P1, start: u32, clines: u32, lpbits: *const ::core::ffi::c_void, lpbmi: *const BITMAPINFO, coloruse: DIB_USAGE) -> i32
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HBITMAP>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetDIBits(hdc: HDC, hbm: HBITMAP, start: u32, clines: u32, lpbits: *const ::core::ffi::c_void, lpbmi: *const BITMAPINFO, coloruse: DIB_USAGE) -> i32;
    }
    SetDIBits(hdc.into(), hbm.into(), start, clines, ::core::mem::transmute(lpbits), ::core::mem::transmute(lpbmi), coloruse)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetDIBitsToDevice<'a, P0>(hdc: P0, xdest: i32, ydest: i32, w: u32, h: u32, xsrc: i32, ysrc: i32, startscan: u32, clines: u32, lpvbits: *const ::core::ffi::c_void, lpbmi: *const BITMAPINFO, coloruse: DIB_USAGE) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetDIBitsToDevice(hdc: HDC, xdest: i32, ydest: i32, w: u32, h: u32, xsrc: i32, ysrc: i32, startscan: u32, clines: u32, lpvbits: *const ::core::ffi::c_void, lpbmi: *const BITMAPINFO, coloruse: DIB_USAGE) -> i32;
    }
    SetDIBitsToDevice(hdc.into(), xdest, ydest, w, h, xsrc, ysrc, startscan, clines, ::core::mem::transmute(lpvbits), ::core::mem::transmute(lpbmi), coloruse)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetEnhMetaFileBits(pb: &[u8]) -> HENHMETAFILE {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetEnhMetaFileBits(nsize: u32, pb: *const u8) -> HENHMETAFILE;
    }
    SetEnhMetaFileBits(pb.len() as _, ::core::mem::transmute(pb.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetGraphicsMode<'a, P0>(hdc: P0, imode: GRAPHICS_MODE) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetGraphicsMode(hdc: HDC, imode: GRAPHICS_MODE) -> i32;
    }
    SetGraphicsMode(hdc.into(), imode)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetLayout<'a, P0>(hdc: P0, l: DC_LAYOUT) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetLayout(hdc: HDC, l: DC_LAYOUT) -> u32;
    }
    SetLayout(hdc.into(), l)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetMapMode<'a, P0>(hdc: P0, imode: HDC_MAP_MODE) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetMapMode(hdc: HDC, imode: HDC_MAP_MODE) -> i32;
    }
    SetMapMode(hdc.into(), imode)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetMapperFlags<'a, P0>(hdc: P0, flags: u32) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetMapperFlags(hdc: HDC, flags: u32) -> u32;
    }
    SetMapperFlags(hdc.into(), flags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetMetaFileBitsEx(lpdata: &[u8]) -> HMETAFILE {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetMetaFileBitsEx(cbbuffer: u32, lpdata: *const u8) -> HMETAFILE;
    }
    SetMetaFileBitsEx(lpdata.len() as _, ::core::mem::transmute(lpdata.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetMetaRgn<'a, P0>(hdc: P0) -> GDI_REGION_TYPE
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetMetaRgn(hdc: HDC) -> GDI_REGION_TYPE;
    }
    SetMetaRgn(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMiterLimit<'a, P0>(hdc: P0, limit: f32, old: ::core::option::Option<*mut f32>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetMiterLimit(hdc: HDC, limit: f32, old: *mut f32) -> super::super::Foundation::BOOL;
    }
    SetMiterLimit(hdc.into(), limit, ::core::mem::transmute(old.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetPaletteEntries<'a, P0>(hpal: P0, istart: u32, ppalentries: &[PALETTEENTRY]) -> u32
where
    P0: ::std::convert::Into<HPALETTE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetPaletteEntries(hpal: HPALETTE, istart: u32, centries: u32, ppalentries: *const PALETTEENTRY) -> u32;
    }
    SetPaletteEntries(hpal.into(), istart, ppalentries.len() as _, ::core::mem::transmute(ppalentries.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPixel<'a, P0, P1>(hdc: P0, x: i32, y: i32, color: P1) -> super::super::Foundation::COLORREF
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetPixel(hdc: HDC, x: i32, y: i32, color: super::super::Foundation::COLORREF) -> super::super::Foundation::COLORREF;
    }
    SetPixel(hdc.into(), x, y, color.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPixelV<'a, P0, P1>(hdc: P0, x: i32, y: i32, color: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetPixelV(hdc: HDC, x: i32, y: i32, color: super::super::Foundation::COLORREF) -> super::super::Foundation::BOOL;
    }
    SetPixelV(hdc.into(), x, y, color.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetPolyFillMode<'a, P0>(hdc: P0, mode: CREATE_POLYGON_RGN_MODE) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetPolyFillMode(hdc: HDC, mode: CREATE_POLYGON_RGN_MODE) -> i32;
    }
    SetPolyFillMode(hdc.into(), mode)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetROP2<'a, P0>(hdc: P0, rop2: R2_MODE) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetROP2(hdc: HDC, rop2: R2_MODE) -> i32;
    }
    SetROP2(hdc.into(), rop2)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetRect(lprc: *mut super::super::Foundation::RECT, xleft: i32, ytop: i32, xright: i32, ybottom: i32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetRect(lprc: *mut super::super::Foundation::RECT, xleft: i32, ytop: i32, xright: i32, ybottom: i32) -> super::super::Foundation::BOOL;
    }
    SetRect(::core::mem::transmute(lprc), xleft, ytop, xright, ybottom)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetRectEmpty(lprc: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetRectEmpty(lprc: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    SetRectEmpty(::core::mem::transmute(lprc))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetRectRgn<'a, P0>(hrgn: P0, left: i32, top: i32, right: i32, bottom: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetRectRgn(hrgn: HRGN, left: i32, top: i32, right: i32, bottom: i32) -> super::super::Foundation::BOOL;
    }
    SetRectRgn(hrgn.into(), left, top, right, bottom)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetStretchBltMode<'a, P0>(hdc: P0, mode: STRETCH_BLT_MODE) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetStretchBltMode(hdc: HDC, mode: STRETCH_BLT_MODE) -> i32;
    }
    SetStretchBltMode(hdc.into(), mode)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetSystemPaletteUse<'a, P0>(hdc: P0, r#use: SYSTEM_PALETTE_USE) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetSystemPaletteUse(hdc: HDC, r#use: SYSTEM_PALETTE_USE) -> u32;
    }
    SetSystemPaletteUse(hdc.into(), r#use)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetTextAlign<'a, P0>(hdc: P0, align: TEXT_ALIGN_OPTIONS) -> u32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetTextAlign(hdc: HDC, align: TEXT_ALIGN_OPTIONS) -> u32;
    }
    SetTextAlign(hdc.into(), align)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn SetTextCharacterExtra<'a, P0>(hdc: P0, extra: i32) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetTextCharacterExtra(hdc: HDC, extra: i32) -> i32;
    }
    SetTextCharacterExtra(hdc.into(), extra)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetTextColor<'a, P0, P1>(hdc: P0, color: P1) -> super::super::Foundation::COLORREF
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetTextColor(hdc: HDC, color: super::super::Foundation::COLORREF) -> super::super::Foundation::COLORREF;
    }
    SetTextColor(hdc.into(), color.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetTextJustification<'a, P0>(hdc: P0, extra: i32, count: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetTextJustification(hdc: HDC, extra: i32, count: i32) -> super::super::Foundation::BOOL;
    }
    SetTextJustification(hdc.into(), extra, count)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetViewportExtEx<'a, P0>(hdc: P0, x: i32, y: i32, lpsz: ::core::option::Option<*mut super::super::Foundation::SIZE>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetViewportExtEx(hdc: HDC, x: i32, y: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    }
    SetViewportExtEx(hdc.into(), x, y, ::core::mem::transmute(lpsz.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetViewportOrgEx<'a, P0>(hdc: P0, x: i32, y: i32, lppt: ::core::option::Option<*mut super::super::Foundation::POINT>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetViewportOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    SetViewportOrgEx(hdc.into(), x, y, ::core::mem::transmute(lppt.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowExtEx<'a, P0>(hdc: P0, x: i32, y: i32, lpsz: ::core::option::Option<*mut super::super::Foundation::SIZE>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWindowExtEx(hdc: HDC, x: i32, y: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    }
    SetWindowExtEx(hdc.into(), x, y, ::core::mem::transmute(lpsz.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowOrgEx<'a, P0>(hdc: P0, x: i32, y: i32, lppt: ::core::option::Option<*mut super::super::Foundation::POINT>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWindowOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    SetWindowOrgEx(hdc.into(), x, y, ::core::mem::transmute(lppt.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowRgn<'a, P0, P1, P2>(hwnd: P0, hrgn: P1, bredraw: P2) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<HRGN>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWindowRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN, bredraw: super::super::Foundation::BOOL) -> i32;
    }
    SetWindowRgn(hwnd.into(), hrgn.into(), bredraw.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWorldTransform<'a, P0>(hdc: P0, lpxf: *const XFORM) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWorldTransform(hdc: HDC, lpxf: *const XFORM) -> super::super::Foundation::BOOL;
    }
    SetWorldTransform(hdc.into(), ::core::mem::transmute(lpxf))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StretchBlt<'a, P0, P1>(hdcdest: P0, xdest: i32, ydest: i32, wdest: i32, hdest: i32, hdcsrc: P1, xsrc: i32, ysrc: i32, wsrc: i32, hsrc: i32, rop: ROP_CODE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StretchBlt(hdcdest: HDC, xdest: i32, ydest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xsrc: i32, ysrc: i32, wsrc: i32, hsrc: i32, rop: ROP_CODE) -> super::super::Foundation::BOOL;
    }
    StretchBlt(hdcdest.into(), xdest, ydest, wdest, hdest, hdcsrc.into(), xsrc, ysrc, wsrc, hsrc, rop)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn StretchDIBits<'a, P0>(hdc: P0, xdest: i32, ydest: i32, destwidth: i32, destheight: i32, xsrc: i32, ysrc: i32, srcwidth: i32, srcheight: i32, lpbits: ::core::option::Option<*const ::core::ffi::c_void>, lpbmi: *const BITMAPINFO, iusage: DIB_USAGE, rop: ROP_CODE) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StretchDIBits(hdc: HDC, xdest: i32, ydest: i32, destwidth: i32, destheight: i32, xsrc: i32, ysrc: i32, srcwidth: i32, srcheight: i32, lpbits: *const ::core::ffi::c_void, lpbmi: *const BITMAPINFO, iusage: DIB_USAGE, rop: ROP_CODE) -> i32;
    }
    StretchDIBits(hdc.into(), xdest, ydest, destwidth, destheight, xsrc, ysrc, srcwidth, srcheight, ::core::mem::transmute(lpbits.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpbmi), iusage, rop)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StrokeAndFillPath<'a, P0>(hdc: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StrokeAndFillPath(hdc: HDC) -> super::super::Foundation::BOOL;
    }
    StrokeAndFillPath(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StrokePath<'a, P0>(hdc: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StrokePath(hdc: HDC) -> super::super::Foundation::BOOL;
    }
    StrokePath(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SubtractRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc1: *const super::super::Foundation::RECT, lprcsrc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SubtractRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc1: *const super::super::Foundation::RECT, lprcsrc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    SubtractRect(::core::mem::transmute(lprcdst), ::core::mem::transmute(lprcsrc1), ::core::mem::transmute(lprcsrc2))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn TTCharToUnicode<'a, P0>(hdc: P0, puccharcodes: &[u8], pusshortcodes: &mut [u16], ulflags: u32) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TTCharToUnicode(hdc: HDC, puccharcodes: *const u8, ulcharcodesize: u32, pusshortcodes: *mut u16, ulshortcodesize: u32, ulflags: u32) -> i32;
    }
    TTCharToUnicode(hdc.into(), ::core::mem::transmute(puccharcodes.as_ptr()), puccharcodes.len() as _, ::core::mem::transmute(pusshortcodes.as_ptr()), pusshortcodes.len() as _, ulflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TTDeleteEmbeddedFont<'a, P0>(hfontreference: P0, ulflags: u32, pulstatus: *mut u32) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TTDeleteEmbeddedFont(hfontreference: super::super::Foundation::HANDLE, ulflags: u32, pulstatus: *mut u32) -> i32;
    }
    TTDeleteEmbeddedFont(hfontreference.into(), ulflags, ::core::mem::transmute(pulstatus))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn TTEmbedFont<'a, P0>(hdc: P0, ulflags: TTEMBED_FLAGS, ulcharset: EMBED_FONT_CHARSET, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus: *mut u32, lpfnwritetostream: WRITEEMBEDPROC, lpvwritestream: *const ::core::ffi::c_void, puscharcodeset: &[u16], uslanguage: u16, pttembedinfo: ::core::option::Option<*const TTEMBEDINFO>) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TTEmbedFont(hdc: HDC, ulflags: TTEMBED_FLAGS, ulcharset: EMBED_FONT_CHARSET, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus: *mut u32, lpfnwritetostream: *mut ::core::ffi::c_void, lpvwritestream: *const ::core::ffi::c_void, puscharcodeset: *const u16, uscharcodecount: u16, uslanguage: u16, pttembedinfo: *const TTEMBEDINFO) -> i32;
    }
    TTEmbedFont(hdc.into(), ulflags, ulcharset, ::core::mem::transmute(pulprivstatus), ::core::mem::transmute(pulstatus), ::core::mem::transmute(lpfnwritetostream), ::core::mem::transmute(lpvwritestream), ::core::mem::transmute(puscharcodeset.as_ptr()), puscharcodeset.len() as _, uslanguage, ::core::mem::transmute(pttembedinfo.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn TTEmbedFontEx<'a, P0>(hdc: P0, ulflags: TTEMBED_FLAGS, ulcharset: EMBED_FONT_CHARSET, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus: *mut u32, lpfnwritetostream: WRITEEMBEDPROC, lpvwritestream: *const ::core::ffi::c_void, pulcharcodeset: &[u32], uslanguage: u16, pttembedinfo: ::core::option::Option<*const TTEMBEDINFO>) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TTEmbedFontEx(hdc: HDC, ulflags: TTEMBED_FLAGS, ulcharset: EMBED_FONT_CHARSET, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus: *mut u32, lpfnwritetostream: *mut ::core::ffi::c_void, lpvwritestream: *const ::core::ffi::c_void, pulcharcodeset: *const u32, uscharcodecount: u16, uslanguage: u16, pttembedinfo: *const TTEMBEDINFO) -> i32;
    }
    TTEmbedFontEx(hdc.into(), ulflags, ulcharset, ::core::mem::transmute(pulprivstatus), ::core::mem::transmute(pulstatus), ::core::mem::transmute(lpfnwritetostream), ::core::mem::transmute(lpvwritestream), ::core::mem::transmute(pulcharcodeset.as_ptr()), pulcharcodeset.len() as _, uslanguage, ::core::mem::transmute(pttembedinfo.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn TTEmbedFontFromFileA<'a, P0, P1>(hdc: P0, szfontfilename: P1, usttcindex: u16, ulflags: TTEMBED_FLAGS, ulcharset: EMBED_FONT_CHARSET, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus: *mut u32, lpfnwritetostream: WRITEEMBEDPROC, lpvwritestream: *const ::core::ffi::c_void, puscharcodeset: &[u16], uslanguage: u16, pttembedinfo: ::core::option::Option<*const TTEMBEDINFO>) -> i32
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TTEmbedFontFromFileA(hdc: HDC, szfontfilename: ::windows::core::PCSTR, usttcindex: u16, ulflags: TTEMBED_FLAGS, ulcharset: EMBED_FONT_CHARSET, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus: *mut u32, lpfnwritetostream: *mut ::core::ffi::c_void, lpvwritestream: *const ::core::ffi::c_void, puscharcodeset: *const u16, uscharcodecount: u16, uslanguage: u16, pttembedinfo: *const TTEMBEDINFO) -> i32;
    }
    TTEmbedFontFromFileA(hdc.into(), szfontfilename.into(), usttcindex, ulflags, ulcharset, ::core::mem::transmute(pulprivstatus), ::core::mem::transmute(pulstatus), ::core::mem::transmute(lpfnwritetostream), ::core::mem::transmute(lpvwritestream), ::core::mem::transmute(puscharcodeset.as_ptr()), puscharcodeset.len() as _, uslanguage, ::core::mem::transmute(pttembedinfo.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TTEnableEmbeddingForFacename<'a, P0, P1>(lpszfacename: P0, benable: P1) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TTEnableEmbeddingForFacename(lpszfacename: ::windows::core::PCSTR, benable: super::super::Foundation::BOOL) -> i32;
    }
    TTEnableEmbeddingForFacename(lpszfacename.into(), benable.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn TTGetEmbeddedFontInfo(ulflags: TTEMBED_FLAGS, pulprivstatus: *mut u32, ulprivs: FONT_LICENSE_PRIVS, pulstatus: *mut u32, lpfnreadfromstream: READEMBEDPROC, lpvreadstream: *const ::core::ffi::c_void, pttloadinfo: ::core::option::Option<*const TTLOADINFO>) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TTGetEmbeddedFontInfo(ulflags: TTEMBED_FLAGS, pulprivstatus: *mut u32, ulprivs: FONT_LICENSE_PRIVS, pulstatus: *mut u32, lpfnreadfromstream: *mut ::core::ffi::c_void, lpvreadstream: *const ::core::ffi::c_void, pttloadinfo: *const TTLOADINFO) -> i32;
    }
    TTGetEmbeddedFontInfo(ulflags, ::core::mem::transmute(pulprivstatus), ulprivs, ::core::mem::transmute(pulstatus), ::core::mem::transmute(lpfnreadfromstream), ::core::mem::transmute(lpvreadstream), ::core::mem::transmute(pttloadinfo.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn TTGetEmbeddingType<'a, P0>(hdc: P0, pulembedtype: *mut EMBEDDED_FONT_PRIV_STATUS) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TTGetEmbeddingType(hdc: HDC, pulembedtype: *mut EMBEDDED_FONT_PRIV_STATUS) -> i32;
    }
    TTGetEmbeddingType(hdc.into(), ::core::mem::transmute(pulembedtype))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TTGetNewFontName(phfontreference: *const super::super::Foundation::HANDLE, wzwinfamilyname: &mut [u16], szmacfamilyname: &mut [u8]) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TTGetNewFontName(phfontreference: *const super::super::Foundation::HANDLE, wzwinfamilyname: ::windows::core::PWSTR, cchmaxwinname: i32, szmacfamilyname: ::windows::core::PSTR, cchmaxmacname: i32) -> i32;
    }
    TTGetNewFontName(::core::mem::transmute(phfontreference), ::core::mem::transmute(wzwinfamilyname.as_ptr()), wzwinfamilyname.len() as _, ::core::mem::transmute(szmacfamilyname.as_ptr()), szmacfamilyname.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TTIsEmbeddingEnabled<'a, P0>(hdc: P0, pbenabled: *mut super::super::Foundation::BOOL) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TTIsEmbeddingEnabled(hdc: HDC, pbenabled: *mut super::super::Foundation::BOOL) -> i32;
    }
    TTIsEmbeddingEnabled(hdc.into(), ::core::mem::transmute(pbenabled))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TTIsEmbeddingEnabledForFacename<'a, P0>(lpszfacename: P0, pbenabled: *mut super::super::Foundation::BOOL) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TTIsEmbeddingEnabledForFacename(lpszfacename: ::windows::core::PCSTR, pbenabled: *mut super::super::Foundation::BOOL) -> i32;
    }
    TTIsEmbeddingEnabledForFacename(lpszfacename.into(), ::core::mem::transmute(pbenabled))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TTLoadEmbeddedFont<'a, P0, P1>(phfontreference: *mut super::super::Foundation::HANDLE, ulflags: u32, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, ulprivs: FONT_LICENSE_PRIVS, pulstatus: *mut TTLOAD_EMBEDDED_FONT_STATUS, lpfnreadfromstream: READEMBEDPROC, lpvreadstream: *const ::core::ffi::c_void, szwinfamilyname: P0, szmacfamilyname: P1, pttloadinfo: ::core::option::Option<*const TTLOADINFO>) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TTLoadEmbeddedFont(phfontreference: *mut super::super::Foundation::HANDLE, ulflags: u32, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, ulprivs: FONT_LICENSE_PRIVS, pulstatus: *mut TTLOAD_EMBEDDED_FONT_STATUS, lpfnreadfromstream: *mut ::core::ffi::c_void, lpvreadstream: *const ::core::ffi::c_void, szwinfamilyname: ::windows::core::PCWSTR, szmacfamilyname: ::windows::core::PCSTR, pttloadinfo: *const TTLOADINFO) -> i32;
    }
    TTLoadEmbeddedFont(::core::mem::transmute(phfontreference), ulflags, ::core::mem::transmute(pulprivstatus), ulprivs, ::core::mem::transmute(pulstatus), ::core::mem::transmute(lpfnreadfromstream), ::core::mem::transmute(lpvreadstream), szwinfamilyname.into(), szmacfamilyname.into(), ::core::mem::transmute(pttloadinfo.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn TTRunValidationTests<'a, P0>(hdc: P0, ptestparam: *const TTVALIDATIONTESTSPARAMS) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TTRunValidationTests(hdc: HDC, ptestparam: *const TTVALIDATIONTESTSPARAMS) -> i32;
    }
    TTRunValidationTests(hdc.into(), ::core::mem::transmute(ptestparam))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn TTRunValidationTestsEx<'a, P0>(hdc: P0, ptestparam: *const TTVALIDATIONTESTSPARAMSEX) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TTRunValidationTestsEx(hdc: HDC, ptestparam: *const TTVALIDATIONTESTSPARAMSEX) -> i32;
    }
    TTRunValidationTestsEx(hdc.into(), ::core::mem::transmute(ptestparam))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn TabbedTextOutA<'a, P0>(hdc: P0, x: i32, y: i32, lpstring: &[u8], lpntabstoppositions: ::core::option::Option<&[i32]>, ntaborigin: i32) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TabbedTextOutA(hdc: HDC, x: i32, y: i32, lpstring: ::windows::core::PCSTR, chcount: i32, ntabpositions: i32, lpntabstoppositions: *const i32, ntaborigin: i32) -> i32;
    }
    TabbedTextOutA(hdc.into(), x, y, ::core::mem::transmute(lpstring.as_ptr()), lpstring.len() as _, lpntabstoppositions.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpntabstoppositions.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ntaborigin)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn TabbedTextOutW<'a, P0>(hdc: P0, x: i32, y: i32, lpstring: &[u16], lpntabstoppositions: ::core::option::Option<&[i32]>, ntaborigin: i32) -> i32
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TabbedTextOutW(hdc: HDC, x: i32, y: i32, lpstring: ::windows::core::PCWSTR, chcount: i32, ntabpositions: i32, lpntabstoppositions: *const i32, ntaborigin: i32) -> i32;
    }
    TabbedTextOutW(hdc.into(), x, y, ::core::mem::transmute(lpstring.as_ptr()), lpstring.len() as _, lpntabstoppositions.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpntabstoppositions.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ntaborigin)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TextOutA<'a, P0>(hdc: P0, x: i32, y: i32, lpstring: &[u8]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TextOutA(hdc: HDC, x: i32, y: i32, lpstring: ::windows::core::PCSTR, c: i32) -> super::super::Foundation::BOOL;
    }
    TextOutA(hdc.into(), x, y, ::core::mem::transmute(lpstring.as_ptr()), lpstring.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TextOutW<'a, P0>(hdc: P0, x: i32, y: i32, lpstring: &[u16]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TextOutW(hdc: HDC, x: i32, y: i32, lpstring: ::windows::core::PCWSTR, c: i32) -> super::super::Foundation::BOOL;
    }
    TextOutW(hdc.into(), x, y, ::core::mem::transmute(lpstring.as_ptr()), lpstring.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TransparentBlt<'a, P0, P1>(hdcdest: P0, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: P1, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, crtransparent: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
    P1: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TransparentBlt(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, crtransparent: u32) -> super::super::Foundation::BOOL;
    }
    TransparentBlt(hdcdest.into(), xorigindest, yorigindest, wdest, hdest, hdcsrc.into(), xoriginsrc, yoriginsrc, wsrc, hsrc, crtransparent)
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnionRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc1: *const super::super::Foundation::RECT, lprcsrc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UnionRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc1: *const super::super::Foundation::RECT, lprcsrc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    UnionRect(::core::mem::transmute(lprcdst), ::core::mem::transmute(lprcsrc1), ::core::mem::transmute(lprcsrc2))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnrealizeObject<'a, P0>(h: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HGDIOBJ>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UnrealizeObject(h: HGDIOBJ) -> super::super::Foundation::BOOL;
    }
    UnrealizeObject(h.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateColors<'a, P0>(hdc: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UpdateColors(hdc: HDC) -> super::super::Foundation::BOOL;
    }
    UpdateColors(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateWindow<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UpdateWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    UpdateWindow(hwnd.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ValidateRect<'a, P0>(hwnd: P0, lprect: ::core::option::Option<*const super::super::Foundation::RECT>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ValidateRect(hwnd: super::super::Foundation::HWND, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    ValidateRect(hwnd.into(), ::core::mem::transmute(lprect.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ValidateRgn<'a, P0, P1>(hwnd: P0, hrgn: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<HRGN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ValidateRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN) -> super::super::Foundation::BOOL;
    }
    ValidateRgn(hwnd.into(), hrgn.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WidenPath<'a, P0>(hdc: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WidenPath(hdc: HDC) -> super::super::Foundation::BOOL;
    }
    WidenPath(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WindowFromDC<'a, P0>(hdc: P0) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WindowFromDC(hdc: HDC) -> super::super::Foundation::HWND;
    }
    WindowFromDC(hdc.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[inline]
pub unsafe fn wglSwapMultipleBuffers(param0: u32, param1: *const WGLSWAP) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn wglSwapMultipleBuffers(param0: u32, param1: *const WGLSWAP) -> u32;
    }
    wglSwapMultipleBuffers(param0, ::core::mem::transmute(param1))
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ABORTDOC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ABSOLUTE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const AC_SRC_ALPHA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const AC_SRC_OVER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ASPECT_FILTERING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BANDINFO: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BEGIN_PATH: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BKMODE_LAST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CA_LOG_FILTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CA_NEGATIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CBM_INIT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CCHFORMNAME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_CHORD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_CIRCLES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_ELLIPSES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_INTERIORS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_PIE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_ROUNDRECT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_STYLED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_WIDE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_WIDESTYLED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CHARSET_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CHARSET_GLYPHIDX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CHECKJPEGFORMAT: u32 = 4119u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CHECKPNGFORMAT: u32 = 4120u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLEARTYPE_NATURAL_QUALITY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIP_TO_PATH: u32 = 4097u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLOSECHANNEL: u32 = 4112u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLR_INVALID: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CM_CMYK_COLOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CM_DEVICE_ICM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CM_GAMMA_RAMP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CM_IN_GAMUT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CM_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CM_OUT_OF_GAMUT: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const COLORMATCHTOTARGET_EMBEDED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CP_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CP_RECTANGLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CP_REGION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CREATECOLORSPACE_EMBEDED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCBA_FACEDOWNCENTER: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCBA_FACEDOWNLEFT: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCBA_FACEDOWNNONE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCBA_FACEDOWNRIGHT: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCBA_FACEUPCENTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCBA_FACEUPLEFT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCBA_FACEUPNONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCBA_FACEUPRIGHT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCTT_BITMAP: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCTT_DOWNLOAD: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCTT_DOWNLOAD_OUTLINE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCTT_SUBDEV: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_BINADJUST: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_DATATYPE_PRODUCED: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_EMF_COMPLIANT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_MANUFACTURER: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_MODEL: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DEFAULT_PITCH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DEVICEDATA: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DEVICE_FONTTYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_MAXPATH: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_PATH_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_PATH_CLONE_GROUP_INVALID: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_PATH_DESKTOP_IMAGE_IDX_INVALID: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_PATH_MODE_IDX_INVALID: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_PATH_PREFERRED_UNSCALED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_PATH_SOURCE_MODE_IDX_INVALID: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_PATH_TARGET_MODE_IDX_INVALID: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_PATH_VALID_FLAGS: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_SOURCE_IN_USE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_BOOT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_PATH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_SYSTEM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_TARGET_FORCIBLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_TARGET_IN_USE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_TARGET_IS_HMD: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_ACC_DRIVER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_ATTACHED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_ATTACHED_TO_DESKTOP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_DISCONNECT: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_MIRRORING_DRIVER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_MODESPRUNED: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_MULTI_DRIVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_PRIMARY_DEVICE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_RDPUDD: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_REMOTE: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_REMOVABLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_TS_COMPATIBLE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_UNSAFE_MODES_ON: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_VGA_COMPATIBLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DI_APPBANDING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DI_ROPS_READ_DESTINATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_AUTO: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_CASSETTE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_ENVELOPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_ENVMANUAL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_FORMSOURCE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_LARGECAPACITY: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_LARGEFMT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_LAST: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_LOWER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_MANUAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_MIDDLE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_ONLYONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_SMALLFMT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_TRACTOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_UPPER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_USER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDFO_CENTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDFO_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDFO_STRETCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDISPLAYFLAGS_TEXTMODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_COARSE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_ERRORDIFFUSION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_FINE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_GRAYSCALE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_LINEART: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_RESERVED6: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_RESERVED7: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_RESERVED8: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_RESERVED9: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_USER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDO_180: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDO_270: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDO_90: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDO_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICMMETHOD_DEVICE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICMMETHOD_DRIVER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICMMETHOD_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICMMETHOD_SYSTEM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICMMETHOD_USER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICM_ABS_COLORIMETRIC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICM_COLORIMETRIC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICM_CONTRAST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICM_SATURATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICM_USER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMMEDIA_GLOSSY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMMEDIA_STANDARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMMEDIA_TRANSPARENCY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMMEDIA_USER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMNUP_ONEUP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMNUP_SYSTEM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMORIENT_LANDSCAPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMORIENT_PORTRAIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_10X11: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_10X14: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_11X17: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_12X11: u32 = 90u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_15X11: u32 = 46u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_9X11: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A2: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A3: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A3_EXTRA: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A3_EXTRA_TRANSVERSE: u32 = 68u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A3_ROTATED: u32 = 76u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A3_TRANSVERSE: u32 = 67u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A4: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A4SMALL: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A4_EXTRA: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A4_PLUS: u32 = 60u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A4_ROTATED: u32 = 77u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A4_TRANSVERSE: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A5: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A5_EXTRA: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A5_ROTATED: u32 = 78u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A5_TRANSVERSE: u32 = 61u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A6: u32 = 70u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A6_ROTATED: u32 = 83u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A_PLUS: u32 = 57u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_B4: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_B4_JIS_ROTATED: u32 = 79u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_B5: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_B5_EXTRA: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_B5_JIS_ROTATED: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_B5_TRANSVERSE: u32 = 62u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_B6_JIS: u32 = 88u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_B6_JIS_ROTATED: u32 = 89u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_B_PLUS: u32 = 58u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_CSHEET: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_DBL_JAPANESE_POSTCARD: u32 = 69u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_DBL_JAPANESE_POSTCARD_ROTATED: u32 = 82u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_DSHEET: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_10: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_11: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_12: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_14: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_9: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_B4: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_B5: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_B6: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_C3: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_C4: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_C5: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_C6: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_C65: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_DL: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_INVITE: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_ITALY: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_MONARCH: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_PERSONAL: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ESHEET: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_EXECUTIVE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_FANFOLD_LGL_GERMAN: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_FANFOLD_STD_GERMAN: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_FANFOLD_US: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_FOLIO: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ISO_B4: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JAPANESE_POSTCARD: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JAPANESE_POSTCARD_ROTATED: u32 = 81u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_CHOU3: u32 = 73u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_CHOU3_ROTATED: u32 = 86u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_CHOU4: u32 = 74u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_CHOU4_ROTATED: u32 = 87u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_KAKU2: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_KAKU2_ROTATED: u32 = 84u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_KAKU3: u32 = 72u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_KAKU3_ROTATED: u32 = 85u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_YOU4: u32 = 91u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_YOU4_ROTATED: u32 = 92u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LAST: u32 = 118u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LEDGER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LEGAL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LEGAL_EXTRA: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LETTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LETTERSMALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LETTER_EXTRA: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LETTER_EXTRA_TRANSVERSE: u32 = 56u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LETTER_PLUS: u32 = 59u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LETTER_ROTATED: u32 = 75u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LETTER_TRANSVERSE: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_NOTE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_P16K: u32 = 93u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_P16K_ROTATED: u32 = 106u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_P32K: u32 = 94u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_P32KBIG: u32 = 95u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_P32KBIG_ROTATED: u32 = 108u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_P32K_ROTATED: u32 = 107u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_1: u32 = 96u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_10: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_10_ROTATED: u32 = 118u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_1_ROTATED: u32 = 109u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_2: u32 = 97u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_2_ROTATED: u32 = 110u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_3: u32 = 98u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_3_ROTATED: u32 = 111u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_4: u32 = 99u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_4_ROTATED: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_5: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_5_ROTATED: u32 = 113u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_6: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_6_ROTATED: u32 = 114u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_7: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_7_ROTATED: u32 = 115u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_8: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_8_ROTATED: u32 = 116u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_9: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_9_ROTATED: u32 = 117u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_QUARTO: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_RESERVED_48: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_RESERVED_49: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_STATEMENT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_TABLOID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_TABLOID_EXTRA: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_USER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMRES_DRAFT: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMRES_HIGH: i32 = -4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMRES_LOW: i32 = -2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMRES_MEDIUM: i32 = -3i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DOWNLOADFACE: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DOWNLOADHEADER: u32 = 4111u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DRAFTMODE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DRAWPATTERNRECT: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_CHARSTREAM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_DISPFILE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_METAFILE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_PLOTTER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_RASCAMERA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_RASDISPLAY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_RASPRINTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ELF_CULTURE_LATIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ELF_VENDOR_SIZE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ELF_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENABLEDUPLEX: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENABLEPAIRKERNING: u32 = 769u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENABLERELATIVEWIDTHS: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENCAPSULATED_POSTSCRIPT: u32 = 4116u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENDDOC: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const END_PATH: u32 = 4098u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENHMETA_SIGNATURE: u32 = 1179469088u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENHMETA_STOCK_OBJECT: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENUMPAPERBINS: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENUMPAPERMETRICS: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EPSPRINTING: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EPS_SIGNATURE: u32 = 1179865157u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERROR: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_FORMAT: u32 = 1006u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_GENERIC: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_BASE: u32 = 1085u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_CMAP: u32 = 1060u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_DELTA_FORMAT: u32 = 1013u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_EBLC: u32 = 1086u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_GDEF: u32 = 1083u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_GLYF: u32 = 1061u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_GPOS: u32 = 1082u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_GSUB: u32 = 1081u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_HDMX: u32 = 1089u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_HEAD: u32 = 1062u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_HHEA: u32 = 1063u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_HHEA_OR_VHEA: u32 = 1072u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_HMTX: u32 = 1064u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_HMTX_OR_VMTX: u32 = 1073u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_JSTF: u32 = 1084u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_LOCA: u32 = 1065u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_LTSH: u32 = 1087u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_MAXP: u32 = 1066u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_MERGE_CHECKSUMS: u32 = 1011u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_MERGE_FORMATS: u32 = 1010u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_MERGE_NUMGLYPHS: u32 = 1012u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_NAME: u32 = 1067u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_OS2: u32 = 1069u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_POST: u32 = 1068u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_TTC_INDEX: u32 = 1015u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_TTO: u32 = 1080u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_VDMX: u32 = 1088u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_VHEA: u32 = 1070u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_VMTX: u32 = 1071u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MEM: u32 = 1005u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_CMAP: u32 = 1030u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_EBDT: u32 = 1044u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_GLYF: u32 = 1031u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_HEAD: u32 = 1032u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_HHEA: u32 = 1033u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_HHEA_OR_VHEA: u32 = 1042u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_HMTX: u32 = 1034u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_HMTX_OR_VMTX: u32 = 1043u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_LOCA: u32 = 1035u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_MAXP: u32 = 1036u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_NAME: u32 = 1037u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_OS2: u32 = 1039u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_POST: u32 = 1038u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_VHEA: u32 = 1040u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_VMTX: u32 = 1041u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_NOT_TTC: u32 = 1014u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_NO_GLYPHS: u32 = 1009u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER0: u32 = 1100u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER1: u32 = 1101u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER10: u32 = 1110u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER11: u32 = 1111u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER12: u32 = 1112u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER13: u32 = 1113u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER14: u32 = 1114u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER15: u32 = 1115u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER16: u32 = 1116u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER2: u32 = 1102u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER3: u32 = 1103u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER4: u32 = 1104u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER5: u32 = 1105u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER6: u32 = 1106u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER7: u32 = 1107u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER8: u32 = 1108u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER9: u32 = 1109u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_READCONTROL: u32 = 1003u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_READOUTOFBOUNDS: u32 = 1001u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_VERSION: u32 = 1008u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_WOULD_GROW: u32 = 1007u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_WRITECONTROL: u32 = 1004u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_WRITEOUTOFBOUNDS: u32 = 1002u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EXTTEXTOUT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EXT_DEVICE_CAPS: u32 = 4099u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ADDFONTFAILED: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_API_NOTIMPL: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_CHARCODECOUNTINVALID: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_CHARCODESETINVALID: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_CHARSETINVALID: i32 = 21i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_COULDNTCREATETEMPFILE: i32 = 513i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_DEVICETRUETYPEFONT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORACCESSINGEXCLUDELIST: i32 = 274i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORACCESSINGFACENAME: i32 = 13i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORACCESSINGFONTDATA: i32 = 12i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORCOMPRESSINGFONTDATA: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORCONVERTINGCHARS: i32 = 18i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORCREATINGFONTFILE: i32 = 269i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORDECOMPRESSINGFONTDATA: i32 = 273i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERROREXPANDINGFONTDATA: i32 = 519i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORGETTINGDC: i32 = 520i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORREADINGFONTDATA: i32 = 267i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORUNICODECONVERSION: i32 = 17i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_EXCEPTION: i32 = 19i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_EXCEPTIONINCOMPRESSION: i32 = 522i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_EXCEPTIONINDECOMPRESSION: i32 = 521i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FACENAMEINVALID: i32 = 275i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FILE_NOT_FOUND: i32 = 23i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FLAGSINVALID: i32 = 268i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTALREADYEXISTS: i32 = 270i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTDATAINVALID: i32 = 258i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTFAMILYNAMENOTINFULL: i32 = 285i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTFILECREATEFAILED: i32 = 515i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTFILENOTFOUND: i32 = 517i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTINSTALLFAILED: i32 = 272i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTNAMEALREADYEXISTS: i32 = 271i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTNOTEMBEDDABLE: i32 = 260i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTREFERENCEINVALID: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTVARIATIONSIMULATED: i32 = 283i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_HDCINVALID: i32 = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_INPUTPARAMINVALID: i32 = 25i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_NAMECHANGEFAILED: i32 = 259i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_NOFREEMEMORY: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_NONE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_NOOS2: i32 = 265i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_NOTATRUETYPEFONT: i32 = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_PBENABLEDINVALID: i32 = 280i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_PERMISSIONSINVALID: i32 = 279i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_PRIVSINVALID: i32 = 261i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_PRIVSTATUSINVALID: i32 = 278i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_READFROMSTREAMFAILED: i32 = 263i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_RESERVEDPARAMNOTNULL: i32 = 20i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_RESOURCEFILECREATEFAILED: i32 = 518i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_SAVETOSTREAMFAILED: i32 = 264i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_STATUSINVALID: i32 = 277i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_STREAMINVALID: i32 = 276i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_SUBSETTINGEXCEPTION: i32 = 281i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_SUBSETTINGFAILED: i32 = 262i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_SUBSTRING_TEST_FAIL: i32 = 282i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_T2NOFREEMEMORY: i32 = 266i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_TTC_INDEX_OUT_OF_RANGE: i32 = 24i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_WINDOWSAPI: i32 = 516i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FEATURESETTING_CUSTPAPER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FEATURESETTING_MIRROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FEATURESETTING_NEGATIVE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FEATURESETTING_NUP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FEATURESETTING_OUTPUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FEATURESETTING_PRIVATE_BEGIN: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FEATURESETTING_PRIVATE_END: u32 = 8191u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FEATURESETTING_PROTOCOL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FEATURESETTING_PSLEVEL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FIXED_PITCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FLI_GLYPHS: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FLI_MASK: u32 = 4155u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FLUSHOUTPUT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FONTMAPPER_MAX: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_ARABIC: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_BALTIC: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_CHINESESIMP: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_CHINESETRAD: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_CYRILLIC: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_GREEK: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_HEBREW: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_JISJAPAN: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_JOHAB: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_LATIN1: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_LATIN2: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_SYMBOL: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_THAI: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_TURKISH: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_VIETNAMESE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_WANSUNG: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_ARABIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_HEBREW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_LATIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_LATINNUMBER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_LATINNUMERICSEPARATOR: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_LATINNUMERICTERMINATOR: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_LOCALNUMBER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_NEUTRAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_NUMERICSEPARATOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_POSTBOUNDLTR: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_POSTBOUNDRTL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_PREBOUNDLTR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_PREBOUNDRTL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPGLYPH_LINKAFTER: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPGLYPH_LINKBEFORE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_DBCS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_ERROR: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_JUSTIFYIN: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDICOMMENT_BEGINGROUP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDICOMMENT_ENDGROUP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDICOMMENT_IDENTIFIER: u32 = 1128875079u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDICOMMENT_MULTIFORMATS: u32 = 1073741828u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDICOMMENT_UNICODE_END: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDICOMMENT_UNICODE_STRING: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDICOMMENT_WINDOWS_METAFILE: u32 = 2147483649u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDIPLUS_TS_QUERYVER: u32 = 4122u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDIPLUS_TS_RECORD: u32 = 4123u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDIREGISTERDDRAWPACKETVERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDI_ERROR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETCOLORTABLE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETDEVICEUNITS: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETEXTENDEDTEXTMETRICS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETEXTENTTABLE: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETFACENAME: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETPAIRKERNTABLE: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETPENWIDTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETPHYSPAGESIZE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETPRINTINGOFFSET: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETSCALINGFACTOR: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETSETPAPERBINS: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETSETPAPERMETRICS: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETSETPRINTORIENT: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETSETSCREENPARAMS: u32 = 3072u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETTECHNOLGY: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETTECHNOLOGY: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETTRACKKERNTABLE: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETVECTORBRUSHSIZE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETVECTORPENSIZE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GET_PS_FEATURESETTING: u32 = 4121u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGI_MARK_NONEXISTING_GLYPHS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GM_LAST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GRADIENT_FILL_OP_FLAG: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GS_8BIT_INDICES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HS_API_MAX: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_A: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_B: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_C: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_D50: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_D55: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_D65: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_D75: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_DAYLIGHT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_DEVICE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_F2: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_FLUORESCENT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_MAX_INDEX: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_NTSC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_TUNGSTEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LAYOUT_BTT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LAYOUT_VBH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LCS_CALIBRATED_RGB: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LCS_GM_ABS_COLORIMETRIC: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LCS_GM_BUSINESS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LCS_GM_GRAPHICS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LCS_GM_IMAGES: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LC_INTERIORS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LC_MARKER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LC_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LC_POLYLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LC_POLYMARKER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LC_STYLED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LC_WIDE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LC_WIDESTYLED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LF_FACESIZE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LF_FULLFACESIZE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_DOUBLEBUFFER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_SHARE_ACCUM: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_SHARE_DEPTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_SHARE_STENCIL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_STEREO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_SUPPORT_GDI: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_SUPPORT_OPENGL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_SWAP_COPY: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_SWAP_EXCHANGE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_TRANSPARENT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_TYPE_COLORINDEX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_TYPE_RGBA: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MAXSTRETCHBLTMODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const METAFILE_DRIVER: u32 = 2049u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_ANIMATEPALETTE: u32 = 1078u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_ARC: u32 = 2071u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_BITBLT: u32 = 2338u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_CHORD: u32 = 2096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_CREATEBRUSHINDIRECT: u32 = 764u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_CREATEFONTINDIRECT: u32 = 763u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_CREATEPALETTE: u32 = 247u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_CREATEPATTERNBRUSH: u32 = 505u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_CREATEPENINDIRECT: u32 = 762u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_CREATEREGION: u32 = 1791u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_DELETEOBJECT: u32 = 496u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_DIBBITBLT: u32 = 2368u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_DIBCREATEPATTERNBRUSH: u32 = 322u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_DIBSTRETCHBLT: u32 = 2881u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_ELLIPSE: u32 = 1048u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_ESCAPE: u32 = 1574u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_EXCLUDECLIPRECT: u32 = 1045u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_EXTFLOODFILL: u32 = 1352u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_EXTTEXTOUT: u32 = 2610u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_FILLREGION: u32 = 552u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_FLOODFILL: u32 = 1049u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_FRAMEREGION: u32 = 1065u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_INTERSECTCLIPRECT: u32 = 1046u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_INVERTREGION: u32 = 298u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_LINETO: u32 = 531u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_MOVETO: u32 = 532u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_OFFSETCLIPRGN: u32 = 544u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_OFFSETVIEWPORTORG: u32 = 529u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_OFFSETWINDOWORG: u32 = 527u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_PAINTREGION: u32 = 299u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_PATBLT: u32 = 1565u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_PIE: u32 = 2074u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_POLYGON: u32 = 804u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_POLYLINE: u32 = 805u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_POLYPOLYGON: u32 = 1336u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_REALIZEPALETTE: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_RECTANGLE: u32 = 1051u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_RESIZEPALETTE: u32 = 313u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_RESTOREDC: u32 = 295u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_ROUNDRECT: u32 = 1564u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SAVEDC: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SCALEVIEWPORTEXT: u32 = 1042u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SCALEWINDOWEXT: u32 = 1040u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SELECTCLIPREGION: u32 = 300u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SELECTOBJECT: u32 = 301u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SELECTPALETTE: u32 = 564u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETBKCOLOR: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETBKMODE: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETDIBTODEV: u32 = 3379u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETLAYOUT: u32 = 329u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETMAPMODE: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETMAPPERFLAGS: u32 = 561u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETPALENTRIES: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETPIXEL: u32 = 1055u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETPOLYFILLMODE: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETRELABS: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETROP2: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETSTRETCHBLTMODE: u32 = 263u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETTEXTALIGN: u32 = 302u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETTEXTCHAREXTRA: u32 = 264u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETTEXTCOLOR: u32 = 521u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETTEXTJUSTIFICATION: u32 = 522u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETVIEWPORTEXT: u32 = 526u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETVIEWPORTORG: u32 = 525u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETWINDOWEXT: u32 = 524u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETWINDOWORG: u32 = 523u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_STRETCHBLT: u32 = 2851u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_STRETCHDIB: u32 = 3907u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_TEXTOUT: u32 = 1313u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MFCOMMENT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MILCORE_TS_QUERYVER_RESULT_FALSE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MILCORE_TS_QUERYVER_RESULT_TRUE: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_MAX_AXES_NAMELEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_MAX_NUMAXES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MONO_FONT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MOUSETRAILS: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NEWFRAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NEWTRANSPARENT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NEXTBAND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NTM_BOLD: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NTM_DSIG: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NTM_ITALIC: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NTM_MULTIPLEMASTER: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NTM_NONNEGATIVE_AC: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NTM_PS_OPENTYPE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NTM_REGULAR: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NTM_TT_OPENTYPE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NTM_TYPE1: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OPENCHANNEL: u32 = 4110u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_SCREEN_OUTLINE_PRECIS: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PANOSE_COUNT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_ARMSTYLE_INDEX: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_BENT_ARMS_DOUBLE_SERIF: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_BENT_ARMS_HORZ: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_BENT_ARMS_SINGLE_SERIF: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_BENT_ARMS_VERT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_BENT_ARMS_WEDGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CONTRAST_HIGH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CONTRAST_INDEX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CONTRAST_LOW: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CONTRAST_MEDIUM: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CONTRAST_MEDIUM_HIGH: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CONTRAST_MEDIUM_LOW: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CONTRAST_NONE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CONTRAST_VERY_HIGH: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CONTRAST_VERY_LOW: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CULTURE_LATIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_FAMILYTYPE_INDEX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_FAMILY_DECORATIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_FAMILY_PICTORIAL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_FAMILY_SCRIPT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_FAMILY_TEXT_DISPLAY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETTERFORM_INDEX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_NORMAL_BOXED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_NORMAL_CONTACT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_NORMAL_FLATTENED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_NORMAL_OFF_CENTER: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_NORMAL_ROUNDED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_NORMAL_SQUARE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_NORMAL_WEIGHTED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_OBLIQUE_BOXED: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_OBLIQUE_CONTACT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_OBLIQUE_FLATTENED: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_OBLIQUE_OFF_CENTER: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_OBLIQUE_ROUNDED: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_OBLIQUE_SQUARE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_OBLIQUE_WEIGHTED: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_CONSTANT_POINTED: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_CONSTANT_SERIFED: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_CONSTANT_TRIMMED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_HIGH_POINTED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_HIGH_SERIFED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_HIGH_TRIMMED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_INDEX: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_LOW_POINTED: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_LOW_SERIFED: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_LOW_TRIMMED: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_STANDARD_POINTED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_STANDARD_SERIFED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_STANDARD_TRIMMED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_NO_FIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_PROPORTION_INDEX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_PROP_CONDENSED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_PROP_EVEN_WIDTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_PROP_EXPANDED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_PROP_MODERN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_PROP_MONOSPACED: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_PROP_OLD_STYLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_PROP_VERY_CONDENSED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_PROP_VERY_EXPANDED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIFSTYLE_INDEX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_BONE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_COVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_EXAGGERATED: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_FLARED: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_NORMAL_SANS: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_OBTUSE_COVE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_OBTUSE_SANS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_OBTUSE_SQUARE_COVE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_PERP_SANS: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_ROUNDED: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_SQUARE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_SQUARE_COVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_THIN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_TRIANGLE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STRAIGHT_ARMS_DOUBLE_SERIF: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STRAIGHT_ARMS_HORZ: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STRAIGHT_ARMS_SINGLE_SERIF: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STRAIGHT_ARMS_VERT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STRAIGHT_ARMS_WEDGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STROKEVARIATION_INDEX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STROKE_GRADUAL_DIAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STROKE_GRADUAL_HORZ: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STROKE_GRADUAL_TRAN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STROKE_GRADUAL_VERT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STROKE_INSTANT_VERT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STROKE_RAPID_HORZ: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STROKE_RAPID_VERT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_BLACK: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_BOLD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_BOOK: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_DEMI: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_HEAVY: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_INDEX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_LIGHT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_MEDIUM: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_NORD: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_THIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_VERY_LIGHT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_XHEIGHT_CONSTANT_LARGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_XHEIGHT_CONSTANT_SMALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_XHEIGHT_CONSTANT_STD: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_XHEIGHT_DUCKING_LARGE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_XHEIGHT_DUCKING_SMALL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_XHEIGHT_DUCKING_STD: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_XHEIGHT_INDEX: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PASSTHROUGH: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_EXPLICIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_INTERIORS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_NOCOLLAPSE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_PATHS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_POLYGON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_POLYPOLYGON: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_RECTANGLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_RESERVED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_SCANLINE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_STYLED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_TRAPEZOID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_WIDE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_WIDESTYLED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_WINDPOLYGON: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const POLYFILL_LAST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const POSTSCRIPT_DATA: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const POSTSCRIPT_IDENTIFY: u32 = 4117u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const POSTSCRIPT_IGNORE: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const POSTSCRIPT_INJECTION: u32 = 4118u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const POSTSCRIPT_PASSTHROUGH: u32 = 4115u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PRINTRATEUNIT_CPS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PRINTRATEUNIT_IPM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PRINTRATEUNIT_LPM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PRINTRATEUNIT_PPM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PR_JOBSTATUS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PSIDENT_GDICENTRIC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PSIDENT_PSCENTRIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PSINJECT_DLFONT: u32 = 3722304989u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PSPROTOCOL_ASCII: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PSPROTOCOL_BCP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PSPROTOCOL_BINARY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PSPROTOCOL_TBCP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PT_BEZIERTO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PT_CLOSEFIGURE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PT_LINETO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PT_MOVETO: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDC_ALL_PATHS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDC_DATABASE_CURRENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDC_INCLUDE_HMD: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDC_ONLY_ACTIVE_PATHS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDC_VIRTUAL_MODE_AWARE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDC_VIRTUAL_REFRESH_RATE_AWARE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDI_DIBTOSCREEN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDI_GETDIBITS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDI_SETDIBITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDI_STRETCHDIB: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QUERYDIBSUPPORT: u32 = 3073u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QUERYESCSUPPORT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QUERYROPSUPPORT: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RASTER_FONTTYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_BANDING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_BIGFONT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_BITBLT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_BITMAP64: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_DEVBITS: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_DIBTODEV: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_DI_BITMAP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_FLOODFILL: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_GDI20_OUTPUT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_GDI20_STATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_OP_DX_OUTPUT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_PALETTE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_SAVEBITMAP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_SCALING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_STRETCHBLT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_STRETCHDIB: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDH_RECTANGLES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RELATIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RESTORE_CTM: u32 = 4100u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SAVE_CTM: u32 = 4101u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SB_CONST_ALPHA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SB_GRAD_RECT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SB_GRAD_TRI: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SB_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SB_PIXEL_ALPHA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SB_PREMULT_ALPHA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SC_SCREENSAVE: u32 = 61760u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_ALLOW_CHANGES: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_ALLOW_PATH_ORDER_CHANGES: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_APPLY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_FORCE_MODE_ENUMERATION: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_NO_OPTIMIZATION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_PATH_PERSIST_IF_REQUIRED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_SAVE_TO_DATABASE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_TOPOLOGY_CLONE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_TOPOLOGY_EXTEND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_TOPOLOGY_EXTERNAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_TOPOLOGY_INTERNAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_TOPOLOGY_SUPPLIED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_USE_SUPPLIED_DISPLAY_CONFIG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_VALIDATE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_VIRTUAL_MODE_AWARE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_VIRTUAL_REFRESH_RATE_AWARE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SELECTDIB: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SELECTPAPERSOURCE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETABORTPROC: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETALLJUSTVALUES: u32 = 771u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETCHARSET: u32 = 772u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETCOLORTABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETCOPYCOUNT: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETDIBSCALING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETICMPROFILE_EMBEDED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETKERNTRACK: u32 = 770u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETLINECAP: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETLINEJOIN: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETMITERLIMIT: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SET_ARC_DIRECTION: u32 = 4102u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SET_BACKGROUND_COLOR: u32 = 4103u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SET_BOUNDS: u32 = 4109u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SET_CLIP_BOX: u32 = 4108u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SET_MIRROR_MODE: u32 = 4110u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SET_POLY_MODE: u32 = 4104u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SET_SCREEN_ANGLE: u32 = 4105u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SET_SPREAD: u32 = 4106u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SPCLPASSTHROUGH2: u32 = 4568u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SP_APPABORT: i32 = -2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SP_ERROR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SP_NOTREPORTED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SP_OUTOFDISK: i32 = -4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SP_OUTOFMEMORY: i32 = -5i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SP_USERABORT: i32 = -3i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const STARTDOC: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const STOCK_LAST: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const STRETCHBLT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SYSPAL_ERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SYSRGN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_CP_STROKE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_CR_90: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_CR_ANY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_EA_DOUBLE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_IA_ABLE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_OP_CHARACTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_OP_STROKE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_RA_ABLE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_RESERVED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_SA_CONTIN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_SA_DOUBLE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_SA_INTEGER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_SCROLLBLT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_SF_X_YINDEP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_SO_ABLE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_UA_ABLE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_VA_ABLE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TRANSFORM_CTM: u32 = 4107u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TRUETYPE_FONTTYPE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTDELETE_DONTREMOVEFONT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_EUDCEMBEDDED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_FAILIFVARIATIONSIMULATED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_SUBSETCANCEL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_VARIATIONSIMULATED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_WEBOBJECT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_XORENCRYPTDATA: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_APPLE_PLATFORMID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_DELTA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_DONT_CARE: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_FLAGS_COMPRESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_FLAGS_GLYPHLIST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_FLAGS_SUBSET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_FLAGS_TTC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_LANG_KEEP_ALL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_MS_PLATFORMID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_SUBSET: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_SUBSET1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFMFP_DELTA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFMFP_SUBSET: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFMFP_SUBSET1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTLOAD_EUDC_OVERWRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTLOAD_EUDC_SET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTLOAD_PRIVATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TT_AVAILABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TT_ENABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TT_POLYGON_TYPE: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TT_PRIM_CSPLINE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TT_PRIM_LINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TT_PRIM_QSPLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VARIABLE_PITCH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_FONT_LINES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_FONT_POLYGONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAPMULTIPLE_MAX: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_MAIN_PLANE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY10: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY11: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY12: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY13: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY14: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY15: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY2: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY3: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY4: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY5: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY6: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY7: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY8: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY9: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY1: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY10: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY11: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY12: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY13: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY14: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY15: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY2: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY3: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY4: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY5: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY6: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY7: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY8: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY9: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ARC_DIRECTION(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const AD_COUNTERCLOCKWISE: ARC_DIRECTION = ARC_DIRECTION(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const AD_CLOCKWISE: ARC_DIRECTION = ARC_DIRECTION(2u32);
impl ::core::marker::Copy for ARC_DIRECTION {}
impl ::core::clone::Clone for ARC_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ARC_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ARC_DIRECTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for ARC_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ARC_DIRECTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BACKGROUND_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OPAQUE: BACKGROUND_MODE = BACKGROUND_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TRANSPARENT: BACKGROUND_MODE = BACKGROUND_MODE(1u32);
impl ::core::marker::Copy for BACKGROUND_MODE {}
impl ::core::clone::Clone for BACKGROUND_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BACKGROUND_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BACKGROUND_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for BACKGROUND_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BACKGROUND_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BI_COMPRESSION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BI_RGB: BI_COMPRESSION = BI_COMPRESSION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BI_RLE8: BI_COMPRESSION = BI_COMPRESSION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BI_RLE4: BI_COMPRESSION = BI_COMPRESSION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BI_BITFIELDS: BI_COMPRESSION = BI_COMPRESSION(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BI_JPEG: BI_COMPRESSION = BI_COMPRESSION(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BI_PNG: BI_COMPRESSION = BI_COMPRESSION(5i32);
impl ::core::marker::Copy for BI_COMPRESSION {}
impl ::core::clone::Clone for BI_COMPRESSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BI_COMPRESSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BI_COMPRESSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for BI_COMPRESSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BI_COMPRESSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BRUSH_STYLE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_SOLID: BRUSH_STYLE = BRUSH_STYLE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_NULL: BRUSH_STYLE = BRUSH_STYLE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_HOLLOW: BRUSH_STYLE = BRUSH_STYLE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_HATCHED: BRUSH_STYLE = BRUSH_STYLE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_PATTERN: BRUSH_STYLE = BRUSH_STYLE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_INDEXED: BRUSH_STYLE = BRUSH_STYLE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_DIBPATTERN: BRUSH_STYLE = BRUSH_STYLE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_DIBPATTERNPT: BRUSH_STYLE = BRUSH_STYLE(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_PATTERN8X8: BRUSH_STYLE = BRUSH_STYLE(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_DIBPATTERN8X8: BRUSH_STYLE = BRUSH_STYLE(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_MONOPATTERN: BRUSH_STYLE = BRUSH_STYLE(9u32);
impl ::core::marker::Copy for BRUSH_STYLE {}
impl ::core::clone::Clone for BRUSH_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BRUSH_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BRUSH_STYLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for BRUSH_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BRUSH_STYLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CDS_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_FULLSCREEN: CDS_TYPE = CDS_TYPE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_GLOBAL: CDS_TYPE = CDS_TYPE(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_NORESET: CDS_TYPE = CDS_TYPE(268435456u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_RESET: CDS_TYPE = CDS_TYPE(1073741824u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_SET_PRIMARY: CDS_TYPE = CDS_TYPE(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_TEST: CDS_TYPE = CDS_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_UPDATEREGISTRY: CDS_TYPE = CDS_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_VIDEOPARAMETERS: CDS_TYPE = CDS_TYPE(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_ENABLE_UNSAFE_MODES: CDS_TYPE = CDS_TYPE(256u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_DISABLE_UNSAFE_MODES: CDS_TYPE = CDS_TYPE(512u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_RESET_EX: CDS_TYPE = CDS_TYPE(536870912u32);
impl ::core::marker::Copy for CDS_TYPE {}
impl ::core::clone::Clone for CDS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CDS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CDS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CDS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CDS_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CDS_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CDS_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CDS_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CDS_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CDS_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CREATE_FONT_PACKAGE_SUBSET_ENCODING(pub u16);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_STD_MAC_CHAR_SET: CREATE_FONT_PACKAGE_SUBSET_ENCODING = CREATE_FONT_PACKAGE_SUBSET_ENCODING(0u16);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_SYMBOL_CHAR_SET: CREATE_FONT_PACKAGE_SUBSET_ENCODING = CREATE_FONT_PACKAGE_SUBSET_ENCODING(0u16);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_UNICODE_CHAR_SET: CREATE_FONT_PACKAGE_SUBSET_ENCODING = CREATE_FONT_PACKAGE_SUBSET_ENCODING(1u16);
impl ::core::marker::Copy for CREATE_FONT_PACKAGE_SUBSET_ENCODING {}
impl ::core::clone::Clone for CREATE_FONT_PACKAGE_SUBSET_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREATE_FONT_PACKAGE_SUBSET_ENCODING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CREATE_FONT_PACKAGE_SUBSET_ENCODING {
    type Abi = Self;
}
impl ::core::fmt::Debug for CREATE_FONT_PACKAGE_SUBSET_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_FONT_PACKAGE_SUBSET_ENCODING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CREATE_FONT_PACKAGE_SUBSET_PLATFORM(pub u16);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_UNICODE_PLATFORMID: CREATE_FONT_PACKAGE_SUBSET_PLATFORM = CREATE_FONT_PACKAGE_SUBSET_PLATFORM(0u16);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_ISO_PLATFORMID: CREATE_FONT_PACKAGE_SUBSET_PLATFORM = CREATE_FONT_PACKAGE_SUBSET_PLATFORM(2u16);
impl ::core::marker::Copy for CREATE_FONT_PACKAGE_SUBSET_PLATFORM {}
impl ::core::clone::Clone for CREATE_FONT_PACKAGE_SUBSET_PLATFORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREATE_FONT_PACKAGE_SUBSET_PLATFORM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CREATE_FONT_PACKAGE_SUBSET_PLATFORM {
    type Abi = Self;
}
impl ::core::fmt::Debug for CREATE_FONT_PACKAGE_SUBSET_PLATFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_FONT_PACKAGE_SUBSET_PLATFORM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CREATE_POLYGON_RGN_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ALTERNATE: CREATE_POLYGON_RGN_MODE = CREATE_POLYGON_RGN_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WINDING: CREATE_POLYGON_RGN_MODE = CREATE_POLYGON_RGN_MODE(2u32);
impl ::core::marker::Copy for CREATE_POLYGON_RGN_MODE {}
impl ::core::clone::Clone for CREATE_POLYGON_RGN_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREATE_POLYGON_RGN_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CREATE_POLYGON_RGN_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CREATE_POLYGON_RGN_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_POLYGON_RGN_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DC_LAYOUT(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LAYOUT_BITMAPORIENTATIONPRESERVED: DC_LAYOUT = DC_LAYOUT(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LAYOUT_RTL: DC_LAYOUT = DC_LAYOUT(1u32);
impl ::core::marker::Copy for DC_LAYOUT {}
impl ::core::clone::Clone for DC_LAYOUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DC_LAYOUT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DC_LAYOUT {
    type Abi = Self;
}
impl ::core::fmt::Debug for DC_LAYOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DC_LAYOUT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DC_LAYOUT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DC_LAYOUT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DC_LAYOUT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DC_LAYOUT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DC_LAYOUT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEVMODE_COLLATE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMCOLLATE_FALSE: DEVMODE_COLLATE = DEVMODE_COLLATE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMCOLLATE_TRUE: DEVMODE_COLLATE = DEVMODE_COLLATE(1u32);
impl ::core::marker::Copy for DEVMODE_COLLATE {}
impl ::core::clone::Clone for DEVMODE_COLLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVMODE_COLLATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DEVMODE_COLLATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEVMODE_COLLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVMODE_COLLATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEVMODE_COLOR(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMCOLOR_MONOCHROME: DEVMODE_COLOR = DEVMODE_COLOR(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMCOLOR_COLOR: DEVMODE_COLOR = DEVMODE_COLOR(2u32);
impl ::core::marker::Copy for DEVMODE_COLOR {}
impl ::core::clone::Clone for DEVMODE_COLOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVMODE_COLOR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DEVMODE_COLOR {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEVMODE_COLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVMODE_COLOR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEVMODE_DUPLEX(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDUP_SIMPLEX: DEVMODE_DUPLEX = DEVMODE_DUPLEX(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDUP_VERTICAL: DEVMODE_DUPLEX = DEVMODE_DUPLEX(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDUP_HORIZONTAL: DEVMODE_DUPLEX = DEVMODE_DUPLEX(3u32);
impl ::core::marker::Copy for DEVMODE_DUPLEX {}
impl ::core::clone::Clone for DEVMODE_DUPLEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVMODE_DUPLEX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DEVMODE_DUPLEX {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEVMODE_DUPLEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVMODE_DUPLEX").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEVMODE_FIELD_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_SPECVERSION: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(1025u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_ORIENTATION: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_PAPERSIZE: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_PAPERLENGTH: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_PAPERWIDTH: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_SCALE: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_POSITION: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_NUP: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_DISPLAYORIENTATION: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_COPIES: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_DEFAULTSOURCE: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_PRINTQUALITY: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_COLOR: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_DUPLEX: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_YRESOLUTION: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_TTOPTION: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_COLLATE: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_FORMNAME: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_LOGPIXELS: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_BITSPERPEL: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_PELSWIDTH: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_PELSHEIGHT: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(1048576u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_DISPLAYFLAGS: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(2097152u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_DISPLAYFREQUENCY: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(4194304u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_ICMMETHOD: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(8388608u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_ICMINTENT: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(16777216u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_MEDIATYPE: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(33554432u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_DITHERTYPE: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(67108864u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_PANNINGWIDTH: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(134217728u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_PANNINGHEIGHT: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(268435456u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_DISPLAYFIXEDOUTPUT: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(536870912u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_INTERLACED: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_UPDATE: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_COPY: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_PROMPT: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_MODIFY: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_IN_BUFFER: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_IN_PROMPT: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_OUT_BUFFER: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_OUT_DEFAULT: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(1u32);
impl ::core::marker::Copy for DEVMODE_FIELD_FLAGS {}
impl ::core::clone::Clone for DEVMODE_FIELD_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVMODE_FIELD_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DEVMODE_FIELD_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEVMODE_FIELD_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVMODE_FIELD_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DEVMODE_FIELD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DEVMODE_FIELD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DEVMODE_FIELD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DEVMODE_FIELD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DEVMODE_FIELD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEVMODE_TRUETYPE_OPTION(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMTT_BITMAP: DEVMODE_TRUETYPE_OPTION = DEVMODE_TRUETYPE_OPTION(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMTT_DOWNLOAD: DEVMODE_TRUETYPE_OPTION = DEVMODE_TRUETYPE_OPTION(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMTT_SUBDEV: DEVMODE_TRUETYPE_OPTION = DEVMODE_TRUETYPE_OPTION(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMTT_DOWNLOAD_OUTLINE: DEVMODE_TRUETYPE_OPTION = DEVMODE_TRUETYPE_OPTION(4u32);
impl ::core::marker::Copy for DEVMODE_TRUETYPE_OPTION {}
impl ::core::clone::Clone for DEVMODE_TRUETYPE_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVMODE_TRUETYPE_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DEVMODE_TRUETYPE_OPTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEVMODE_TRUETYPE_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVMODE_TRUETYPE_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DFCS_STATE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_CAPTIONCLOSE: DFCS_STATE = DFCS_STATE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_CAPTIONMIN: DFCS_STATE = DFCS_STATE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_CAPTIONMAX: DFCS_STATE = DFCS_STATE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_CAPTIONRESTORE: DFCS_STATE = DFCS_STATE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_CAPTIONHELP: DFCS_STATE = DFCS_STATE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_MENUARROW: DFCS_STATE = DFCS_STATE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_MENUCHECK: DFCS_STATE = DFCS_STATE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_MENUBULLET: DFCS_STATE = DFCS_STATE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_MENUARROWRIGHT: DFCS_STATE = DFCS_STATE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_SCROLLUP: DFCS_STATE = DFCS_STATE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_SCROLLDOWN: DFCS_STATE = DFCS_STATE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_SCROLLLEFT: DFCS_STATE = DFCS_STATE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_SCROLLRIGHT: DFCS_STATE = DFCS_STATE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_SCROLLCOMBOBOX: DFCS_STATE = DFCS_STATE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_SCROLLSIZEGRIP: DFCS_STATE = DFCS_STATE(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_SCROLLSIZEGRIPRIGHT: DFCS_STATE = DFCS_STATE(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_BUTTONCHECK: DFCS_STATE = DFCS_STATE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_BUTTONRADIOIMAGE: DFCS_STATE = DFCS_STATE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_BUTTONRADIOMASK: DFCS_STATE = DFCS_STATE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_BUTTONRADIO: DFCS_STATE = DFCS_STATE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_BUTTON3STATE: DFCS_STATE = DFCS_STATE(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_BUTTONPUSH: DFCS_STATE = DFCS_STATE(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_INACTIVE: DFCS_STATE = DFCS_STATE(256u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_PUSHED: DFCS_STATE = DFCS_STATE(512u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_CHECKED: DFCS_STATE = DFCS_STATE(1024u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_TRANSPARENT: DFCS_STATE = DFCS_STATE(2048u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_HOT: DFCS_STATE = DFCS_STATE(4096u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_ADJUSTRECT: DFCS_STATE = DFCS_STATE(8192u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_FLAT: DFCS_STATE = DFCS_STATE(16384u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_MONO: DFCS_STATE = DFCS_STATE(32768u32);
impl ::core::marker::Copy for DFCS_STATE {}
impl ::core::clone::Clone for DFCS_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DFCS_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DFCS_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DFCS_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DFCS_STATE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DFCS_STATE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DFCS_STATE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DFCS_STATE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DFCS_STATE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DFCS_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DFC_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFC_CAPTION: DFC_TYPE = DFC_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFC_MENU: DFC_TYPE = DFC_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFC_SCROLL: DFC_TYPE = DFC_TYPE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFC_BUTTON: DFC_TYPE = DFC_TYPE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFC_POPUPMENU: DFC_TYPE = DFC_TYPE(5u32);
impl ::core::marker::Copy for DFC_TYPE {}
impl ::core::clone::Clone for DFC_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DFC_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DFC_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DFC_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DFC_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIB_USAGE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DIB_RGB_COLORS: DIB_USAGE = DIB_USAGE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DIB_PAL_COLORS: DIB_USAGE = DIB_USAGE(1u32);
impl ::core::marker::Copy for DIB_USAGE {}
impl ::core::clone::Clone for DIB_USAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIB_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIB_USAGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIB_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIB_USAGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPLAYCONFIG_COLOR_ENCODING(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_COLOR_ENCODING_RGB: DISPLAYCONFIG_COLOR_ENCODING = DISPLAYCONFIG_COLOR_ENCODING(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_COLOR_ENCODING_YCBCR444: DISPLAYCONFIG_COLOR_ENCODING = DISPLAYCONFIG_COLOR_ENCODING(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_COLOR_ENCODING_YCBCR422: DISPLAYCONFIG_COLOR_ENCODING = DISPLAYCONFIG_COLOR_ENCODING(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_COLOR_ENCODING_YCBCR420: DISPLAYCONFIG_COLOR_ENCODING = DISPLAYCONFIG_COLOR_ENCODING(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_COLOR_ENCODING_INTENSITY: DISPLAYCONFIG_COLOR_ENCODING = DISPLAYCONFIG_COLOR_ENCODING(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_COLOR_ENCODING_FORCE_UINT32: DISPLAYCONFIG_COLOR_ENCODING = DISPLAYCONFIG_COLOR_ENCODING(-1i32);
impl ::core::marker::Copy for DISPLAYCONFIG_COLOR_ENCODING {}
impl ::core::clone::Clone for DISPLAYCONFIG_COLOR_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPLAYCONFIG_COLOR_ENCODING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_COLOR_ENCODING {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPLAYCONFIG_COLOR_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPLAYCONFIG_COLOR_ENCODING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISP_CHANGE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISP_CHANGE_SUCCESSFUL: DISP_CHANGE = DISP_CHANGE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISP_CHANGE_RESTART: DISP_CHANGE = DISP_CHANGE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISP_CHANGE_FAILED: DISP_CHANGE = DISP_CHANGE(-1i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISP_CHANGE_BADMODE: DISP_CHANGE = DISP_CHANGE(-2i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISP_CHANGE_NOTUPDATED: DISP_CHANGE = DISP_CHANGE(-3i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISP_CHANGE_BADFLAGS: DISP_CHANGE = DISP_CHANGE(-4i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISP_CHANGE_BADPARAM: DISP_CHANGE = DISP_CHANGE(-5i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISP_CHANGE_BADDUALVIEW: DISP_CHANGE = DISP_CHANGE(-6i32);
impl ::core::marker::Copy for DISP_CHANGE {}
impl ::core::clone::Clone for DISP_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISP_CHANGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DISP_CHANGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISP_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISP_CHANGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRAWEDGE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BDR_RAISEDOUTER: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BDR_SUNKENOUTER: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BDR_RAISEDINNER: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BDR_SUNKENINNER: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BDR_OUTER: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BDR_INNER: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(12u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BDR_RAISED: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BDR_SUNKEN: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(10u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EDGE_RAISED: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EDGE_SUNKEN: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(10u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EDGE_ETCHED: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EDGE_BUMP: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(9u32);
impl ::core::marker::Copy for DRAWEDGE_FLAGS {}
impl ::core::clone::Clone for DRAWEDGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRAWEDGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRAWEDGE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRAWEDGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAWEDGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DRAWEDGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAWEDGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAWEDGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAWEDGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAWEDGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRAWSTATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DST_COMPLEX: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DST_TEXT: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DST_PREFIXTEXT: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DST_ICON: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DST_BITMAP: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DSS_NORMAL: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DSS_UNION: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DSS_DISABLED: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DSS_MONO: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DSS_HIDEPREFIX: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DSS_PREFIXONLY: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DSS_RIGHT: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(32768u32);
impl ::core::marker::Copy for DRAWSTATE_FLAGS {}
impl ::core::clone::Clone for DRAWSTATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRAWSTATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRAWSTATE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRAWSTATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAWSTATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DRAWSTATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAWSTATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAWSTATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAWSTATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAWSTATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRAW_CAPTION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_ACTIVE: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_BUTTONS: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_GRADIENT: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_ICON: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_INBUTTON: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_SMALLCAP: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_TEXT: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(8u32);
impl ::core::marker::Copy for DRAW_CAPTION_FLAGS {}
impl ::core::clone::Clone for DRAW_CAPTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRAW_CAPTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRAW_CAPTION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRAW_CAPTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAW_CAPTION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DRAW_CAPTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAW_CAPTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAW_CAPTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAW_CAPTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAW_CAPTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRAW_EDGE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_ADJUST: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_BOTTOM: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_BOTTOMLEFT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(9u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_BOTTOMRIGHT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(12u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_DIAGONAL: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_DIAGONAL_ENDBOTTOMLEFT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(25u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_DIAGONAL_ENDBOTTOMRIGHT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(28u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_DIAGONAL_ENDTOPLEFT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(19u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_DIAGONAL_ENDTOPRIGHT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(22u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_FLAT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_LEFT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_MIDDLE: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_MONO: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_RECT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(15u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_RIGHT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_SOFT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_TOP: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_TOPLEFT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_TOPRIGHT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(6u32);
impl ::core::marker::Copy for DRAW_EDGE_FLAGS {}
impl ::core::clone::Clone for DRAW_EDGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRAW_EDGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRAW_EDGE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRAW_EDGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAW_EDGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DRAW_EDGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAW_EDGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAW_EDGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAW_EDGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAW_EDGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRAW_TEXT_FORMAT(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_BOTTOM: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_CALCRECT: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(1024u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_CENTER: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_EDITCONTROL: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(8192u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_END_ELLIPSIS: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(32768u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_EXPANDTABS: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(64u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_EXTERNALLEADING: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(512u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_HIDEPREFIX: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(1048576u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_INTERNAL: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(4096u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_LEFT: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_MODIFYSTRING: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(65536u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_NOCLIP: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(256u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_NOFULLWIDTHCHARBREAK: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(524288u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_NOPREFIX: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(2048u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_PATH_ELLIPSIS: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(16384u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_PREFIXONLY: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(2097152u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_RIGHT: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_RTLREADING: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(131072u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_SINGLELINE: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_TABSTOP: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(128u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_TOP: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_VCENTER: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_WORDBREAK: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_WORD_ELLIPSIS: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(262144u32);
impl ::core::marker::Copy for DRAW_TEXT_FORMAT {}
impl ::core::clone::Clone for DRAW_TEXT_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRAW_TEXT_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRAW_TEXT_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRAW_TEXT_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAW_TEXT_FORMAT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DRAW_TEXT_FORMAT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAW_TEXT_FORMAT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAW_TEXT_FORMAT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAW_TEXT_FORMAT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAW_TEXT_FORMAT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EMBEDDED_FONT_PRIV_STATUS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMBED_PREVIEWPRINT: EMBEDDED_FONT_PRIV_STATUS = EMBEDDED_FONT_PRIV_STATUS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMBED_EDITABLE: EMBEDDED_FONT_PRIV_STATUS = EMBEDDED_FONT_PRIV_STATUS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMBED_INSTALLABLE: EMBEDDED_FONT_PRIV_STATUS = EMBEDDED_FONT_PRIV_STATUS(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMBED_NOEMBEDDING: EMBEDDED_FONT_PRIV_STATUS = EMBEDDED_FONT_PRIV_STATUS(4u32);
impl ::core::marker::Copy for EMBEDDED_FONT_PRIV_STATUS {}
impl ::core::clone::Clone for EMBEDDED_FONT_PRIV_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EMBEDDED_FONT_PRIV_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EMBEDDED_FONT_PRIV_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EMBEDDED_FONT_PRIV_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EMBEDDED_FONT_PRIV_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EMBED_FONT_CHARSET(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CHARSET_UNICODE: EMBED_FONT_CHARSET = EMBED_FONT_CHARSET(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CHARSET_SYMBOL: EMBED_FONT_CHARSET = EMBED_FONT_CHARSET(2u32);
impl ::core::marker::Copy for EMBED_FONT_CHARSET {}
impl ::core::clone::Clone for EMBED_FONT_CHARSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EMBED_FONT_CHARSET {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EMBED_FONT_CHARSET {
    type Abi = Self;
}
impl ::core::fmt::Debug for EMBED_FONT_CHARSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EMBED_FONT_CHARSET").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ENHANCED_METAFILE_RECORD_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_HEADER: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYBEZIER: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYGON: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYLINE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYBEZIERTO: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYLINETO: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYPOLYLINE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYPOLYGON: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETWINDOWEXTEX: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(9u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETWINDOWORGEX: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(10u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETVIEWPORTEXTEX: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(11u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETVIEWPORTORGEX: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(12u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETBRUSHORGEX: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(13u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_EOF: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(14u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETPIXELV: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(15u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETMAPPERFLAGS: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETMAPMODE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(17u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETBKMODE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(18u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETPOLYFILLMODE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(19u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETROP2: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(20u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETSTRETCHBLTMODE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(21u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETTEXTALIGN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(22u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETCOLORADJUSTMENT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(23u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETTEXTCOLOR: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(24u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETBKCOLOR: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(25u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_OFFSETCLIPRGN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(26u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_MOVETOEX: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(27u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETMETARGN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(28u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_EXCLUDECLIPRECT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(29u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_INTERSECTCLIPRECT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(30u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SCALEVIEWPORTEXTEX: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(31u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SCALEWINDOWEXTEX: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SAVEDC: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(33u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESTOREDC: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(34u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETWORLDTRANSFORM: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(35u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_MODIFYWORLDTRANSFORM: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(36u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SELECTOBJECT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(37u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_CREATEPEN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(38u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_CREATEBRUSHINDIRECT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(39u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_DELETEOBJECT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(40u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_ANGLEARC: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(41u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_ELLIPSE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(42u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RECTANGLE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(43u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_ROUNDRECT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(44u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_ARC: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(45u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_CHORD: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(46u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_PIE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(47u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SELECTPALETTE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(48u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_CREATEPALETTE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(49u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETPALETTEENTRIES: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(50u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESIZEPALETTE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(51u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_REALIZEPALETTE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(52u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_EXTFLOODFILL: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(53u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_LINETO: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(54u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_ARCTO: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(55u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYDRAW: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(56u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETARCDIRECTION: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(57u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETMITERLIMIT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(58u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_BEGINPATH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(59u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_ENDPATH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(60u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_CLOSEFIGURE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(61u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_FILLPATH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(62u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_STROKEANDFILLPATH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(63u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_STROKEPATH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(64u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_FLATTENPATH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(65u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_WIDENPATH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(66u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SELECTCLIPPATH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(67u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_ABORTPATH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(68u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_GDICOMMENT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(70u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_FILLRGN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(71u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_FRAMERGN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(72u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_INVERTRGN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(73u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_PAINTRGN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(74u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_EXTSELECTCLIPRGN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(75u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_BITBLT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(76u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_STRETCHBLT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(77u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_MASKBLT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(78u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_PLGBLT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(79u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETDIBITSTODEVICE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(80u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_STRETCHDIBITS: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(81u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_EXTCREATEFONTINDIRECTW: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(82u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_EXTTEXTOUTA: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(83u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_EXTTEXTOUTW: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(84u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYBEZIER16: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(85u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYGON16: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(86u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYLINE16: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(87u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYBEZIERTO16: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(88u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYLINETO16: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(89u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYPOLYLINE16: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(90u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYPOLYGON16: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(91u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYDRAW16: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(92u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_CREATEMONOBRUSH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(93u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_CREATEDIBPATTERNBRUSHPT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(94u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_EXTCREATEPEN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(95u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYTEXTOUTA: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(96u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYTEXTOUTW: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(97u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETICMMODE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(98u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_CREATECOLORSPACE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(99u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETCOLORSPACE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(100u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_DELETECOLORSPACE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(101u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_GLSRECORD: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(102u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_GLSBOUNDEDRECORD: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(103u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_PIXELFORMAT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(104u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESERVED_105: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(105u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESERVED_106: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(106u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESERVED_107: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(107u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESERVED_108: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(108u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESERVED_109: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(109u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESERVED_110: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(110u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_COLORCORRECTPALETTE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(111u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETICMPROFILEA: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(112u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETICMPROFILEW: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(113u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_ALPHABLEND: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(114u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETLAYOUT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(115u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_TRANSPARENTBLT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(116u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESERVED_117: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(117u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_GRADIENTFILL: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(118u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESERVED_119: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(119u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESERVED_120: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(120u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_COLORMATCHTOTARGETW: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(121u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_CREATECOLORSPACEW: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(122u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_MIN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_MAX: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(122u32);
impl ::core::marker::Copy for ENHANCED_METAFILE_RECORD_TYPE {}
impl ::core::clone::Clone for ENHANCED_METAFILE_RECORD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENHANCED_METAFILE_RECORD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ENHANCED_METAFILE_RECORD_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ENHANCED_METAFILE_RECORD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENHANCED_METAFILE_RECORD_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ENUM_DISPLAY_SETTINGS_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENUM_CURRENT_SETTINGS: ENUM_DISPLAY_SETTINGS_MODE = ENUM_DISPLAY_SETTINGS_MODE(4294967295u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENUM_REGISTRY_SETTINGS: ENUM_DISPLAY_SETTINGS_MODE = ENUM_DISPLAY_SETTINGS_MODE(4294967294u32);
impl ::core::marker::Copy for ENUM_DISPLAY_SETTINGS_MODE {}
impl ::core::clone::Clone for ENUM_DISPLAY_SETTINGS_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENUM_DISPLAY_SETTINGS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ENUM_DISPLAY_SETTINGS_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ENUM_DISPLAY_SETTINGS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_DISPLAY_SETTINGS_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ETO_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ETO_OPAQUE: ETO_OPTIONS = ETO_OPTIONS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ETO_CLIPPED: ETO_OPTIONS = ETO_OPTIONS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ETO_GLYPH_INDEX: ETO_OPTIONS = ETO_OPTIONS(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ETO_RTLREADING: ETO_OPTIONS = ETO_OPTIONS(128u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ETO_NUMERICSLOCAL: ETO_OPTIONS = ETO_OPTIONS(1024u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ETO_NUMERICSLATIN: ETO_OPTIONS = ETO_OPTIONS(2048u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ETO_IGNORELANGUAGE: ETO_OPTIONS = ETO_OPTIONS(4096u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ETO_PDY: ETO_OPTIONS = ETO_OPTIONS(8192u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ETO_REVERSE_INDEX_MAP: ETO_OPTIONS = ETO_OPTIONS(65536u32);
impl ::core::marker::Copy for ETO_OPTIONS {}
impl ::core::clone::Clone for ETO_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ETO_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ETO_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for ETO_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ETO_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ETO_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ETO_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ETO_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ETO_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ETO_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EXT_FLOOD_FILL_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FLOODFILLBORDER: EXT_FLOOD_FILL_TYPE = EXT_FLOOD_FILL_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FLOODFILLSURFACE: EXT_FLOOD_FILL_TYPE = EXT_FLOOD_FILL_TYPE(1u32);
impl ::core::marker::Copy for EXT_FLOOD_FILL_TYPE {}
impl ::core::clone::Clone for EXT_FLOOD_FILL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EXT_FLOOD_FILL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EXT_FLOOD_FILL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EXT_FLOOD_FILL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXT_FLOOD_FILL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FONT_CHARSET(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ANSI_CHARSET: FONT_CHARSET = FONT_CHARSET(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DEFAULT_CHARSET: FONT_CHARSET = FONT_CHARSET(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SYMBOL_CHARSET: FONT_CHARSET = FONT_CHARSET(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SHIFTJIS_CHARSET: FONT_CHARSET = FONT_CHARSET(128u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HANGEUL_CHARSET: FONT_CHARSET = FONT_CHARSET(129u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HANGUL_CHARSET: FONT_CHARSET = FONT_CHARSET(129u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GB2312_CHARSET: FONT_CHARSET = FONT_CHARSET(134u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CHINESEBIG5_CHARSET: FONT_CHARSET = FONT_CHARSET(136u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OEM_CHARSET: FONT_CHARSET = FONT_CHARSET(255u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const JOHAB_CHARSET: FONT_CHARSET = FONT_CHARSET(130u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HEBREW_CHARSET: FONT_CHARSET = FONT_CHARSET(177u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ARABIC_CHARSET: FONT_CHARSET = FONT_CHARSET(178u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GREEK_CHARSET: FONT_CHARSET = FONT_CHARSET(161u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TURKISH_CHARSET: FONT_CHARSET = FONT_CHARSET(162u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VIETNAMESE_CHARSET: FONT_CHARSET = FONT_CHARSET(163u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const THAI_CHARSET: FONT_CHARSET = FONT_CHARSET(222u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EASTEUROPE_CHARSET: FONT_CHARSET = FONT_CHARSET(238u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RUSSIAN_CHARSET: FONT_CHARSET = FONT_CHARSET(204u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MAC_CHARSET: FONT_CHARSET = FONT_CHARSET(77u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BALTIC_CHARSET: FONT_CHARSET = FONT_CHARSET(186u32);
impl ::core::marker::Copy for FONT_CHARSET {}
impl ::core::clone::Clone for FONT_CHARSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FONT_CHARSET {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FONT_CHARSET {
    type Abi = Self;
}
impl ::core::fmt::Debug for FONT_CHARSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_CHARSET").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FONT_CLIP_PRECISION(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIP_CHARACTER_PRECIS: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIP_DEFAULT_PRECIS: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIP_DFA_DISABLE: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(64u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIP_EMBEDDED: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(128u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIP_LH_ANGLES: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIP_MASK: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(15u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIP_STROKE_PRECIS: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIP_TT_ALWAYS: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(32u32);
impl ::core::marker::Copy for FONT_CLIP_PRECISION {}
impl ::core::clone::Clone for FONT_CLIP_PRECISION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FONT_CLIP_PRECISION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FONT_CLIP_PRECISION {
    type Abi = Self;
}
impl ::core::fmt::Debug for FONT_CLIP_PRECISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_CLIP_PRECISION").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FONT_CLIP_PRECISION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FONT_CLIP_PRECISION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FONT_CLIP_PRECISION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FONT_CLIP_PRECISION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FONT_CLIP_PRECISION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FONT_LICENSE_PRIVS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LICENSE_PREVIEWPRINT: FONT_LICENSE_PRIVS = FONT_LICENSE_PRIVS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LICENSE_EDITABLE: FONT_LICENSE_PRIVS = FONT_LICENSE_PRIVS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LICENSE_INSTALLABLE: FONT_LICENSE_PRIVS = FONT_LICENSE_PRIVS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LICENSE_NOEMBEDDING: FONT_LICENSE_PRIVS = FONT_LICENSE_PRIVS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LICENSE_DEFAULT: FONT_LICENSE_PRIVS = FONT_LICENSE_PRIVS(0u32);
impl ::core::marker::Copy for FONT_LICENSE_PRIVS {}
impl ::core::clone::Clone for FONT_LICENSE_PRIVS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FONT_LICENSE_PRIVS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FONT_LICENSE_PRIVS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FONT_LICENSE_PRIVS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_LICENSE_PRIVS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FONT_OUTPUT_PRECISION(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_CHARACTER_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_DEFAULT_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_DEVICE_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_OUTLINE_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_PS_ONLY_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(10u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_RASTER_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_STRING_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_STROKE_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_TT_ONLY_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_TT_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(4u32);
impl ::core::marker::Copy for FONT_OUTPUT_PRECISION {}
impl ::core::clone::Clone for FONT_OUTPUT_PRECISION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FONT_OUTPUT_PRECISION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FONT_OUTPUT_PRECISION {
    type Abi = Self;
}
impl ::core::fmt::Debug for FONT_OUTPUT_PRECISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_OUTPUT_PRECISION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FONT_PITCH_AND_FAMILY(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FF_DECORATIVE: FONT_PITCH_AND_FAMILY = FONT_PITCH_AND_FAMILY(80u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FF_DONTCARE: FONT_PITCH_AND_FAMILY = FONT_PITCH_AND_FAMILY(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FF_MODERN: FONT_PITCH_AND_FAMILY = FONT_PITCH_AND_FAMILY(48u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FF_ROMAN: FONT_PITCH_AND_FAMILY = FONT_PITCH_AND_FAMILY(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FF_SCRIPT: FONT_PITCH_AND_FAMILY = FONT_PITCH_AND_FAMILY(64u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FF_SWISS: FONT_PITCH_AND_FAMILY = FONT_PITCH_AND_FAMILY(32u32);
impl ::core::marker::Copy for FONT_PITCH_AND_FAMILY {}
impl ::core::clone::Clone for FONT_PITCH_AND_FAMILY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FONT_PITCH_AND_FAMILY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FONT_PITCH_AND_FAMILY {
    type Abi = Self;
}
impl ::core::fmt::Debug for FONT_PITCH_AND_FAMILY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_PITCH_AND_FAMILY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FONT_QUALITY(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ANTIALIASED_QUALITY: FONT_QUALITY = FONT_QUALITY(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLEARTYPE_QUALITY: FONT_QUALITY = FONT_QUALITY(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DEFAULT_QUALITY: FONT_QUALITY = FONT_QUALITY(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DRAFT_QUALITY: FONT_QUALITY = FONT_QUALITY(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NONANTIALIASED_QUALITY: FONT_QUALITY = FONT_QUALITY(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PROOF_QUALITY: FONT_QUALITY = FONT_QUALITY(2u32);
impl ::core::marker::Copy for FONT_QUALITY {}
impl ::core::clone::Clone for FONT_QUALITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FONT_QUALITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FONT_QUALITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for FONT_QUALITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_QUALITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FONT_RESOURCE_CHARACTERISTICS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FR_PRIVATE: FONT_RESOURCE_CHARACTERISTICS = FONT_RESOURCE_CHARACTERISTICS(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FR_NOT_ENUM: FONT_RESOURCE_CHARACTERISTICS = FONT_RESOURCE_CHARACTERISTICS(32u32);
impl ::core::marker::Copy for FONT_RESOURCE_CHARACTERISTICS {}
impl ::core::clone::Clone for FONT_RESOURCE_CHARACTERISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FONT_RESOURCE_CHARACTERISTICS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FONT_RESOURCE_CHARACTERISTICS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FONT_RESOURCE_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_RESOURCE_CHARACTERISTICS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FONT_WEIGHT(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_DONTCARE: FONT_WEIGHT = FONT_WEIGHT(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_THIN: FONT_WEIGHT = FONT_WEIGHT(100u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_EXTRALIGHT: FONT_WEIGHT = FONT_WEIGHT(200u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_LIGHT: FONT_WEIGHT = FONT_WEIGHT(300u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_NORMAL: FONT_WEIGHT = FONT_WEIGHT(400u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_MEDIUM: FONT_WEIGHT = FONT_WEIGHT(500u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_SEMIBOLD: FONT_WEIGHT = FONT_WEIGHT(600u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_BOLD: FONT_WEIGHT = FONT_WEIGHT(700u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_EXTRABOLD: FONT_WEIGHT = FONT_WEIGHT(800u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_HEAVY: FONT_WEIGHT = FONT_WEIGHT(900u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_ULTRALIGHT: FONT_WEIGHT = FONT_WEIGHT(200u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_REGULAR: FONT_WEIGHT = FONT_WEIGHT(400u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_DEMIBOLD: FONT_WEIGHT = FONT_WEIGHT(600u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_ULTRABOLD: FONT_WEIGHT = FONT_WEIGHT(800u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_BLACK: FONT_WEIGHT = FONT_WEIGHT(900u32);
impl ::core::marker::Copy for FONT_WEIGHT {}
impl ::core::clone::Clone for FONT_WEIGHT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FONT_WEIGHT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FONT_WEIGHT {
    type Abi = Self;
}
impl ::core::fmt::Debug for FONT_WEIGHT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_WEIGHT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GDI_REGION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RGN_ERROR: GDI_REGION_TYPE = GDI_REGION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NULLREGION: GDI_REGION_TYPE = GDI_REGION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SIMPLEREGION: GDI_REGION_TYPE = GDI_REGION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const COMPLEXREGION: GDI_REGION_TYPE = GDI_REGION_TYPE(3i32);
impl ::core::marker::Copy for GDI_REGION_TYPE {}
impl ::core::clone::Clone for GDI_REGION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GDI_REGION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GDI_REGION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for GDI_REGION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GDI_REGION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GET_CHARACTER_PLACEMENT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_CLASSIN: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_DIACRITIC: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_DISPLAYZWG: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(4194304u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_GLYPHSHAPE: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_JUSTIFY: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_KASHIDA: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_LIGATE: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_MAXEXTENT: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(1048576u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_NEUTRALOVERRIDE: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(33554432u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_NUMERICOVERRIDE: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(16777216u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_NUMERICSLATIN: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(67108864u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_NUMERICSLOCAL: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(134217728u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_REORDER: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_SYMSWAPOFF: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(8388608u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_USEKERNING: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(8u32);
impl ::core::marker::Copy for GET_CHARACTER_PLACEMENT_FLAGS {}
impl ::core::clone::Clone for GET_CHARACTER_PLACEMENT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_CHARACTER_PLACEMENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GET_CHARACTER_PLACEMENT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for GET_CHARACTER_PLACEMENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_CHARACTER_PLACEMENT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GET_CHARACTER_PLACEMENT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GET_CHARACTER_PLACEMENT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GET_CHARACTER_PLACEMENT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GET_CHARACTER_PLACEMENT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GET_CHARACTER_PLACEMENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GET_DCX_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_WINDOW: GET_DCX_FLAGS = GET_DCX_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_CACHE: GET_DCX_FLAGS = GET_DCX_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_PARENTCLIP: GET_DCX_FLAGS = GET_DCX_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_CLIPSIBLINGS: GET_DCX_FLAGS = GET_DCX_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_CLIPCHILDREN: GET_DCX_FLAGS = GET_DCX_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_NORESETATTRS: GET_DCX_FLAGS = GET_DCX_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_LOCKWINDOWUPDATE: GET_DCX_FLAGS = GET_DCX_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_EXCLUDERGN: GET_DCX_FLAGS = GET_DCX_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_INTERSECTRGN: GET_DCX_FLAGS = GET_DCX_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_INTERSECTUPDATE: GET_DCX_FLAGS = GET_DCX_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_VALIDATE: GET_DCX_FLAGS = GET_DCX_FLAGS(2097152u32);
impl ::core::marker::Copy for GET_DCX_FLAGS {}
impl ::core::clone::Clone for GET_DCX_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_DCX_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GET_DCX_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for GET_DCX_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_DCX_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GET_DCX_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GET_DCX_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GET_DCX_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GET_DCX_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GET_DCX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GET_DEVICE_CAPS_INDEX(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DRIVERVERSION: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TECHNOLOGY: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HORZSIZE: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VERTSIZE: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HORZRES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VERTRES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(10u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BITSPIXEL: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(12u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PLANES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(14u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NUMBRUSHES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NUMPENS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(18u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NUMMARKERS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(20u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NUMFONTS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(22u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NUMCOLORS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(24u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PDEVICESIZE: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(26u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CURVECAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(28u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LINECAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(30u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const POLYGONALCAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TEXTCAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(34u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIPCAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(36u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RASTERCAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(38u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ASPECTX: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(40u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ASPECTY: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(42u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ASPECTXY: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(44u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LOGPIXELSX: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(88u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LOGPIXELSY: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(90u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SIZEPALETTE: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(104u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NUMRESERVED: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(106u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const COLORRES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(108u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PHYSICALWIDTH: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(110u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PHYSICALHEIGHT: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(111u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PHYSICALOFFSETX: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(112u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PHYSICALOFFSETY: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(113u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SCALINGFACTORX: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(114u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SCALINGFACTORY: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(115u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VREFRESH: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(116u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DESKTOPVERTRES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(117u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DESKTOPHORZRES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(118u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BLTALIGNMENT: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(119u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SHADEBLENDCAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(120u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const COLORMGMTCAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(121u32);
impl ::core::marker::Copy for GET_DEVICE_CAPS_INDEX {}
impl ::core::clone::Clone for GET_DEVICE_CAPS_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_DEVICE_CAPS_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GET_DEVICE_CAPS_INDEX {
    type Abi = Self;
}
impl ::core::fmt::Debug for GET_DEVICE_CAPS_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_DEVICE_CAPS_INDEX").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GET_GLYPH_OUTLINE_FORMAT(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGO_BEZIER: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGO_BITMAP: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGO_GLYPH_INDEX: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(128u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGO_GRAY2_BITMAP: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGO_GRAY4_BITMAP: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGO_GRAY8_BITMAP: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGO_METRICS: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGO_NATIVE: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGO_UNHINTED: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(256u32);
impl ::core::marker::Copy for GET_GLYPH_OUTLINE_FORMAT {}
impl ::core::clone::Clone for GET_GLYPH_OUTLINE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_GLYPH_OUTLINE_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GET_GLYPH_OUTLINE_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for GET_GLYPH_OUTLINE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_GLYPH_OUTLINE_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GET_STOCK_OBJECT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BLACK_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DKGRAY_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(18u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GRAY_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HOLLOW_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LTGRAY_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NULL_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WHITE_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BLACK_PEN: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_PEN: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(19u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NULL_PEN: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WHITE_PEN: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ANSI_FIXED_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(11u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ANSI_VAR_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(12u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DEVICE_DEFAULT_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(14u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DEFAULT_GUI_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(17u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OEM_FIXED_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(10u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SYSTEM_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(13u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SYSTEM_FIXED_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DEFAULT_PALETTE: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(15u32);
impl ::core::marker::Copy for GET_STOCK_OBJECT_FLAGS {}
impl ::core::clone::Clone for GET_STOCK_OBJECT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_STOCK_OBJECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GET_STOCK_OBJECT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for GET_STOCK_OBJECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_STOCK_OBJECT_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GRADIENT_FILL(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GRADIENT_FILL_RECT_H: GRADIENT_FILL = GRADIENT_FILL(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GRADIENT_FILL_RECT_V: GRADIENT_FILL = GRADIENT_FILL(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GRADIENT_FILL_TRIANGLE: GRADIENT_FILL = GRADIENT_FILL(2u32);
impl ::core::marker::Copy for GRADIENT_FILL {}
impl ::core::clone::Clone for GRADIENT_FILL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GRADIENT_FILL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GRADIENT_FILL {
    type Abi = Self;
}
impl ::core::fmt::Debug for GRADIENT_FILL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GRADIENT_FILL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GRAPHICS_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GM_COMPATIBLE: GRAPHICS_MODE = GRAPHICS_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GM_ADVANCED: GRAPHICS_MODE = GRAPHICS_MODE(2u32);
impl ::core::marker::Copy for GRAPHICS_MODE {}
impl ::core::clone::Clone for GRAPHICS_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GRAPHICS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GRAPHICS_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for GRAPHICS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GRAPHICS_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HATCH_BRUSH_STYLE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HS_BDIAGONAL: HATCH_BRUSH_STYLE = HATCH_BRUSH_STYLE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HS_CROSS: HATCH_BRUSH_STYLE = HATCH_BRUSH_STYLE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HS_DIAGCROSS: HATCH_BRUSH_STYLE = HATCH_BRUSH_STYLE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HS_FDIAGONAL: HATCH_BRUSH_STYLE = HATCH_BRUSH_STYLE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HS_HORIZONTAL: HATCH_BRUSH_STYLE = HATCH_BRUSH_STYLE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HS_VERTICAL: HATCH_BRUSH_STYLE = HATCH_BRUSH_STYLE(1u32);
impl ::core::marker::Copy for HATCH_BRUSH_STYLE {}
impl ::core::clone::Clone for HATCH_BRUSH_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HATCH_BRUSH_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HATCH_BRUSH_STYLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HATCH_BRUSH_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HATCH_BRUSH_STYLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HDC_MAP_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_ANISOTROPIC: HDC_MAP_MODE = HDC_MAP_MODE(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_HIENGLISH: HDC_MAP_MODE = HDC_MAP_MODE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_HIMETRIC: HDC_MAP_MODE = HDC_MAP_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_ISOTROPIC: HDC_MAP_MODE = HDC_MAP_MODE(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_LOENGLISH: HDC_MAP_MODE = HDC_MAP_MODE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_LOMETRIC: HDC_MAP_MODE = HDC_MAP_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_TEXT: HDC_MAP_MODE = HDC_MAP_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_TWIPS: HDC_MAP_MODE = HDC_MAP_MODE(6u32);
impl ::core::marker::Copy for HDC_MAP_MODE {}
impl ::core::clone::Clone for HDC_MAP_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HDC_MAP_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HDC_MAP_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HDC_MAP_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDC_MAP_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MODIFY_WORLD_TRANSFORM_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MWT_IDENTITY: MODIFY_WORLD_TRANSFORM_MODE = MODIFY_WORLD_TRANSFORM_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MWT_LEFTMULTIPLY: MODIFY_WORLD_TRANSFORM_MODE = MODIFY_WORLD_TRANSFORM_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MWT_RIGHTMULTIPLY: MODIFY_WORLD_TRANSFORM_MODE = MODIFY_WORLD_TRANSFORM_MODE(3u32);
impl ::core::marker::Copy for MODIFY_WORLD_TRANSFORM_MODE {}
impl ::core::clone::Clone for MODIFY_WORLD_TRANSFORM_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MODIFY_WORLD_TRANSFORM_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MODIFY_WORLD_TRANSFORM_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MODIFY_WORLD_TRANSFORM_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODIFY_WORLD_TRANSFORM_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MONITOR_FROM_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MONITOR_DEFAULTTONEAREST: MONITOR_FROM_FLAGS = MONITOR_FROM_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MONITOR_DEFAULTTONULL: MONITOR_FROM_FLAGS = MONITOR_FROM_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MONITOR_DEFAULTTOPRIMARY: MONITOR_FROM_FLAGS = MONITOR_FROM_FLAGS(1u32);
impl ::core::marker::Copy for MONITOR_FROM_FLAGS {}
impl ::core::clone::Clone for MONITOR_FROM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MONITOR_FROM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MONITOR_FROM_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MONITOR_FROM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MONITOR_FROM_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OBJ_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_PEN: OBJ_TYPE = OBJ_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_BRUSH: OBJ_TYPE = OBJ_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_DC: OBJ_TYPE = OBJ_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_METADC: OBJ_TYPE = OBJ_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_PAL: OBJ_TYPE = OBJ_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_FONT: OBJ_TYPE = OBJ_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_BITMAP: OBJ_TYPE = OBJ_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_REGION: OBJ_TYPE = OBJ_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_METAFILE: OBJ_TYPE = OBJ_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_MEMDC: OBJ_TYPE = OBJ_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_EXTPEN: OBJ_TYPE = OBJ_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_ENHMETADC: OBJ_TYPE = OBJ_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_ENHMETAFILE: OBJ_TYPE = OBJ_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_COLORSPACE: OBJ_TYPE = OBJ_TYPE(14i32);
impl ::core::marker::Copy for OBJ_TYPE {}
impl ::core::clone::Clone for OBJ_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OBJ_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OBJ_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for OBJ_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBJ_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEN_STYLE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_GEOMETRIC: PEN_STYLE = PEN_STYLE(65536u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_COSMETIC: PEN_STYLE = PEN_STYLE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_SOLID: PEN_STYLE = PEN_STYLE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_DASH: PEN_STYLE = PEN_STYLE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_DOT: PEN_STYLE = PEN_STYLE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_DASHDOT: PEN_STYLE = PEN_STYLE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_DASHDOTDOT: PEN_STYLE = PEN_STYLE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_NULL: PEN_STYLE = PEN_STYLE(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_INSIDEFRAME: PEN_STYLE = PEN_STYLE(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_USERSTYLE: PEN_STYLE = PEN_STYLE(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_ALTERNATE: PEN_STYLE = PEN_STYLE(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_STYLE_MASK: PEN_STYLE = PEN_STYLE(15u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_ENDCAP_ROUND: PEN_STYLE = PEN_STYLE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_ENDCAP_SQUARE: PEN_STYLE = PEN_STYLE(256u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_ENDCAP_FLAT: PEN_STYLE = PEN_STYLE(512u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_ENDCAP_MASK: PEN_STYLE = PEN_STYLE(3840u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_JOIN_ROUND: PEN_STYLE = PEN_STYLE(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_JOIN_BEVEL: PEN_STYLE = PEN_STYLE(4096u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_JOIN_MITER: PEN_STYLE = PEN_STYLE(8192u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_JOIN_MASK: PEN_STYLE = PEN_STYLE(61440u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_TYPE_MASK: PEN_STYLE = PEN_STYLE(983040u32);
impl ::core::marker::Copy for PEN_STYLE {}
impl ::core::clone::Clone for PEN_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEN_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PEN_STYLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PEN_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEN_STYLE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PEN_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PEN_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PEN_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PEN_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PEN_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct R2_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_BLACK: R2_MODE = R2_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_NOTMERGEPEN: R2_MODE = R2_MODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_MASKNOTPEN: R2_MODE = R2_MODE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_NOTCOPYPEN: R2_MODE = R2_MODE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_MASKPENNOT: R2_MODE = R2_MODE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_NOT: R2_MODE = R2_MODE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_XORPEN: R2_MODE = R2_MODE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_NOTMASKPEN: R2_MODE = R2_MODE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_MASKPEN: R2_MODE = R2_MODE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_NOTXORPEN: R2_MODE = R2_MODE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_NOP: R2_MODE = R2_MODE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_MERGENOTPEN: R2_MODE = R2_MODE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_COPYPEN: R2_MODE = R2_MODE(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_MERGEPENNOT: R2_MODE = R2_MODE(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_MERGEPEN: R2_MODE = R2_MODE(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_WHITE: R2_MODE = R2_MODE(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_LAST: R2_MODE = R2_MODE(16i32);
impl ::core::marker::Copy for R2_MODE {}
impl ::core::clone::Clone for R2_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for R2_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for R2_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for R2_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("R2_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REDRAW_WINDOW_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_INVALIDATE: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_INTERNALPAINT: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_ERASE: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_VALIDATE: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_NOINTERNALPAINT: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_NOERASE: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_NOCHILDREN: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_ALLCHILDREN: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_UPDATENOW: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_ERASENOW: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_FRAME: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_NOFRAME: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(2048u32);
impl ::core::marker::Copy for REDRAW_WINDOW_FLAGS {}
impl ::core::clone::Clone for REDRAW_WINDOW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REDRAW_WINDOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REDRAW_WINDOW_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for REDRAW_WINDOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REDRAW_WINDOW_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for REDRAW_WINDOW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REDRAW_WINDOW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REDRAW_WINDOW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REDRAW_WINDOW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REDRAW_WINDOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RGN_COMBINE_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RGN_AND: RGN_COMBINE_MODE = RGN_COMBINE_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RGN_OR: RGN_COMBINE_MODE = RGN_COMBINE_MODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RGN_XOR: RGN_COMBINE_MODE = RGN_COMBINE_MODE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RGN_DIFF: RGN_COMBINE_MODE = RGN_COMBINE_MODE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RGN_COPY: RGN_COMBINE_MODE = RGN_COMBINE_MODE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RGN_MIN: RGN_COMBINE_MODE = RGN_COMBINE_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RGN_MAX: RGN_COMBINE_MODE = RGN_COMBINE_MODE(5i32);
impl ::core::marker::Copy for RGN_COMBINE_MODE {}
impl ::core::clone::Clone for RGN_COMBINE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RGN_COMBINE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RGN_COMBINE_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RGN_COMBINE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RGN_COMBINE_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ROP_CODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BLACKNESS: ROP_CODE = ROP_CODE(66u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NOTSRCERASE: ROP_CODE = ROP_CODE(1114278u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NOTSRCCOPY: ROP_CODE = ROP_CODE(3342344u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SRCERASE: ROP_CODE = ROP_CODE(4457256u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DSTINVERT: ROP_CODE = ROP_CODE(5570569u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PATINVERT: ROP_CODE = ROP_CODE(5898313u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SRCINVERT: ROP_CODE = ROP_CODE(6684742u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SRCAND: ROP_CODE = ROP_CODE(8913094u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MERGEPAINT: ROP_CODE = ROP_CODE(12255782u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MERGECOPY: ROP_CODE = ROP_CODE(12583114u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SRCCOPY: ROP_CODE = ROP_CODE(13369376u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SRCPAINT: ROP_CODE = ROP_CODE(15597702u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PATCOPY: ROP_CODE = ROP_CODE(15728673u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PATPAINT: ROP_CODE = ROP_CODE(16452105u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WHITENESS: ROP_CODE = ROP_CODE(16711778u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CAPTUREBLT: ROP_CODE = ROP_CODE(1073741824u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NOMIRRORBITMAP: ROP_CODE = ROP_CODE(2147483648u32);
impl ::core::marker::Copy for ROP_CODE {}
impl ::core::clone::Clone for ROP_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ROP_CODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ROP_CODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ROP_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ROP_CODE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ROP_CODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ROP_CODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ROP_CODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ROP_CODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ROP_CODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SET_BOUNDS_RECT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCB_ACCUMULATE: SET_BOUNDS_RECT_FLAGS = SET_BOUNDS_RECT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCB_DISABLE: SET_BOUNDS_RECT_FLAGS = SET_BOUNDS_RECT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCB_ENABLE: SET_BOUNDS_RECT_FLAGS = SET_BOUNDS_RECT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCB_RESET: SET_BOUNDS_RECT_FLAGS = SET_BOUNDS_RECT_FLAGS(1u32);
impl ::core::marker::Copy for SET_BOUNDS_RECT_FLAGS {}
impl ::core::clone::Clone for SET_BOUNDS_RECT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SET_BOUNDS_RECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SET_BOUNDS_RECT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SET_BOUNDS_RECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SET_BOUNDS_RECT_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STRETCH_BLT_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BLACKONWHITE: STRETCH_BLT_MODE = STRETCH_BLT_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const COLORONCOLOR: STRETCH_BLT_MODE = STRETCH_BLT_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HALFTONE: STRETCH_BLT_MODE = STRETCH_BLT_MODE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const STRETCH_ANDSCANS: STRETCH_BLT_MODE = STRETCH_BLT_MODE(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const STRETCH_DELETESCANS: STRETCH_BLT_MODE = STRETCH_BLT_MODE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const STRETCH_HALFTONE: STRETCH_BLT_MODE = STRETCH_BLT_MODE(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const STRETCH_ORSCANS: STRETCH_BLT_MODE = STRETCH_BLT_MODE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WHITEONBLACK: STRETCH_BLT_MODE = STRETCH_BLT_MODE(2u32);
impl ::core::marker::Copy for STRETCH_BLT_MODE {}
impl ::core::clone::Clone for STRETCH_BLT_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STRETCH_BLT_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STRETCH_BLT_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STRETCH_BLT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STRETCH_BLT_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYSTEM_PALETTE_USE(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SYSPAL_NOSTATIC: SYSTEM_PALETTE_USE = SYSTEM_PALETTE_USE(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SYSPAL_NOSTATIC256: SYSTEM_PALETTE_USE = SYSTEM_PALETTE_USE(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SYSPAL_STATIC: SYSTEM_PALETTE_USE = SYSTEM_PALETTE_USE(1u32);
impl ::core::marker::Copy for SYSTEM_PALETTE_USE {}
impl ::core::clone::Clone for SYSTEM_PALETTE_USE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSTEM_PALETTE_USE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SYSTEM_PALETTE_USE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYSTEM_PALETTE_USE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_PALETTE_USE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TEXT_ALIGN_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_NOUPDATECP: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_UPDATECP: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_LEFT: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_RIGHT: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_CENTER: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_TOP: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_BOTTOM: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_BASELINE: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(24u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_RTLREADING: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(256u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_MASK: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(287u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VTA_BASELINE: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(24u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VTA_LEFT: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VTA_RIGHT: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VTA_CENTER: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VTA_BOTTOM: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VTA_TOP: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(0u32);
impl ::core::marker::Copy for TEXT_ALIGN_OPTIONS {}
impl ::core::clone::Clone for TEXT_ALIGN_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TEXT_ALIGN_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TEXT_ALIGN_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TEXT_ALIGN_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TEXT_ALIGN_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TEXT_ALIGN_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TEXT_ALIGN_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TEXT_ALIGN_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TEXT_ALIGN_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TEXT_ALIGN_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TMPF_FLAGS(pub u8);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TMPF_FIXED_PITCH: TMPF_FLAGS = TMPF_FLAGS(1u8);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TMPF_VECTOR: TMPF_FLAGS = TMPF_FLAGS(2u8);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TMPF_DEVICE: TMPF_FLAGS = TMPF_FLAGS(8u8);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TMPF_TRUETYPE: TMPF_FLAGS = TMPF_FLAGS(4u8);
impl ::core::marker::Copy for TMPF_FLAGS {}
impl ::core::clone::Clone for TMPF_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TMPF_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TMPF_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TMPF_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TMPF_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TMPF_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TMPF_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TMPF_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TMPF_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TMPF_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TTEMBED_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_EMBEDEUDC: TTEMBED_FLAGS = TTEMBED_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_RAW: TTEMBED_FLAGS = TTEMBED_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_SUBSET: TTEMBED_FLAGS = TTEMBED_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_TTCOMPRESSED: TTEMBED_FLAGS = TTEMBED_FLAGS(4u32);
impl ::core::marker::Copy for TTEMBED_FLAGS {}
impl ::core::clone::Clone for TTEMBED_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TTEMBED_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TTEMBED_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TTEMBED_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TTEMBED_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TTEMBED_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TTEMBED_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TTEMBED_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TTEMBED_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TTEMBED_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TTLOAD_EMBEDDED_FONT_STATUS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTLOAD_FONT_SUBSETTED: TTLOAD_EMBEDDED_FONT_STATUS = TTLOAD_EMBEDDED_FONT_STATUS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTLOAD_FONT_IN_SYSSTARTUP: TTLOAD_EMBEDDED_FONT_STATUS = TTLOAD_EMBEDDED_FONT_STATUS(2u32);
impl ::core::marker::Copy for TTLOAD_EMBEDDED_FONT_STATUS {}
impl ::core::clone::Clone for TTLOAD_EMBEDDED_FONT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TTLOAD_EMBEDDED_FONT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TTLOAD_EMBEDDED_FONT_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TTLOAD_EMBEDDED_FONT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TTLOAD_EMBEDDED_FONT_STATUS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TTLOAD_EMBEDDED_FONT_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TTLOAD_EMBEDDED_FONT_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TTLOAD_EMBEDDED_FONT_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TTLOAD_EMBEDDED_FONT_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TTLOAD_EMBEDDED_FONT_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct ABC {
    pub abcA: i32,
    pub abcB: u32,
    pub abcC: i32,
}
impl ::core::marker::Copy for ABC {}
impl ::core::clone::Clone for ABC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ABC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ABC").field("abcA", &self.abcA).field("abcB", &self.abcB).field("abcC", &self.abcC).finish()
    }
}
unsafe impl ::windows::core::Abi for ABC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ABC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ABC>()) == 0 }
    }
}
impl ::core::cmp::Eq for ABC {}
impl ::core::default::Default for ABC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct ABCFLOAT {
    pub abcfA: f32,
    pub abcfB: f32,
    pub abcfC: f32,
}
impl ::core::marker::Copy for ABCFLOAT {}
impl ::core::clone::Clone for ABCFLOAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ABCFLOAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ABCFLOAT").field("abcfA", &self.abcfA).field("abcfB", &self.abcfB).field("abcfC", &self.abcfC).finish()
    }
}
unsafe impl ::windows::core::Abi for ABCFLOAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ABCFLOAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ABCFLOAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for ABCFLOAT {}
impl ::core::default::Default for ABCFLOAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct ABORTPATH {
    pub emr: EMR,
}
impl ::core::marker::Copy for ABORTPATH {}
impl ::core::clone::Clone for ABORTPATH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ABORTPATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ABORTPATH").field("emr", &self.emr).finish()
    }
}
unsafe impl ::windows::core::Abi for ABORTPATH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ABORTPATH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ABORTPATH>()) == 0 }
    }
}
impl ::core::cmp::Eq for ABORTPATH {}
impl ::core::default::Default for ABORTPATH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct AXESLISTA {
    pub axlReserved: u32,
    pub axlNumAxes: u32,
    pub axlAxisInfo: [AXISINFOA; 16],
}
impl ::core::marker::Copy for AXESLISTA {}
impl ::core::clone::Clone for AXESLISTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AXESLISTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AXESLISTA").field("axlReserved", &self.axlReserved).field("axlNumAxes", &self.axlNumAxes).field("axlAxisInfo", &self.axlAxisInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for AXESLISTA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AXESLISTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AXESLISTA>()) == 0 }
    }
}
impl ::core::cmp::Eq for AXESLISTA {}
impl ::core::default::Default for AXESLISTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct AXESLISTW {
    pub axlReserved: u32,
    pub axlNumAxes: u32,
    pub axlAxisInfo: [AXISINFOW; 16],
}
impl ::core::marker::Copy for AXESLISTW {}
impl ::core::clone::Clone for AXESLISTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AXESLISTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AXESLISTW").field("axlReserved", &self.axlReserved).field("axlNumAxes", &self.axlNumAxes).field("axlAxisInfo", &self.axlAxisInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for AXESLISTW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AXESLISTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AXESLISTW>()) == 0 }
    }
}
impl ::core::cmp::Eq for AXESLISTW {}
impl ::core::default::Default for AXESLISTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct AXISINFOA {
    pub axMinValue: i32,
    pub axMaxValue: i32,
    pub axAxisName: [u8; 16],
}
impl ::core::marker::Copy for AXISINFOA {}
impl ::core::clone::Clone for AXISINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AXISINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AXISINFOA").field("axMinValue", &self.axMinValue).field("axMaxValue", &self.axMaxValue).field("axAxisName", &self.axAxisName).finish()
    }
}
unsafe impl ::windows::core::Abi for AXISINFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AXISINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AXISINFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for AXISINFOA {}
impl ::core::default::Default for AXISINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct AXISINFOW {
    pub axMinValue: i32,
    pub axMaxValue: i32,
    pub axAxisName: [u16; 16],
}
impl ::core::marker::Copy for AXISINFOW {}
impl ::core::clone::Clone for AXISINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AXISINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AXISINFOW").field("axMinValue", &self.axMinValue).field("axMaxValue", &self.axMaxValue).field("axAxisName", &self.axAxisName).finish()
    }
}
unsafe impl ::windows::core::Abi for AXISINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AXISINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AXISINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for AXISINFOW {}
impl ::core::default::Default for AXISINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct BITMAP {
    pub bmType: i32,
    pub bmWidth: i32,
    pub bmHeight: i32,
    pub bmWidthBytes: i32,
    pub bmPlanes: u16,
    pub bmBitsPixel: u16,
    pub bmBits: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for BITMAP {}
impl ::core::clone::Clone for BITMAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BITMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAP").field("bmType", &self.bmType).field("bmWidth", &self.bmWidth).field("bmHeight", &self.bmHeight).field("bmWidthBytes", &self.bmWidthBytes).field("bmPlanes", &self.bmPlanes).field("bmBitsPixel", &self.bmBitsPixel).field("bmBits", &self.bmBits).finish()
    }
}
unsafe impl ::windows::core::Abi for BITMAP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BITMAP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BITMAP>()) == 0 }
    }
}
impl ::core::cmp::Eq for BITMAP {}
impl ::core::default::Default for BITMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct BITMAPCOREHEADER {
    pub bcSize: u32,
    pub bcWidth: u16,
    pub bcHeight: u16,
    pub bcPlanes: u16,
    pub bcBitCount: u16,
}
impl ::core::marker::Copy for BITMAPCOREHEADER {}
impl ::core::clone::Clone for BITMAPCOREHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BITMAPCOREHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAPCOREHEADER").field("bcSize", &self.bcSize).field("bcWidth", &self.bcWidth).field("bcHeight", &self.bcHeight).field("bcPlanes", &self.bcPlanes).field("bcBitCount", &self.bcBitCount).finish()
    }
}
unsafe impl ::windows::core::Abi for BITMAPCOREHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BITMAPCOREHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BITMAPCOREHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for BITMAPCOREHEADER {}
impl ::core::default::Default for BITMAPCOREHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct BITMAPCOREINFO {
    pub bmciHeader: BITMAPCOREHEADER,
    pub bmciColors: [RGBTRIPLE; 1],
}
impl ::core::marker::Copy for BITMAPCOREINFO {}
impl ::core::clone::Clone for BITMAPCOREINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BITMAPCOREINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAPCOREINFO").field("bmciHeader", &self.bmciHeader).field("bmciColors", &self.bmciColors).finish()
    }
}
unsafe impl ::windows::core::Abi for BITMAPCOREINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BITMAPCOREINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BITMAPCOREINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for BITMAPCOREINFO {}
impl ::core::default::Default for BITMAPCOREINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct BITMAPFILEHEADER {
    pub bfType: u16,
    pub bfSize: u32,
    pub bfReserved1: u16,
    pub bfReserved2: u16,
    pub bfOffBits: u32,
}
impl ::core::marker::Copy for BITMAPFILEHEADER {}
impl ::core::clone::Clone for BITMAPFILEHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BITMAPFILEHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BITMAPFILEHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BITMAPFILEHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for BITMAPFILEHEADER {}
impl ::core::default::Default for BITMAPFILEHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct BITMAPINFO {
    pub bmiHeader: BITMAPINFOHEADER,
    pub bmiColors: [RGBQUAD; 1],
}
impl ::core::marker::Copy for BITMAPINFO {}
impl ::core::clone::Clone for BITMAPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BITMAPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAPINFO").field("bmiHeader", &self.bmiHeader).field("bmiColors", &self.bmiColors).finish()
    }
}
unsafe impl ::windows::core::Abi for BITMAPINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BITMAPINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BITMAPINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for BITMAPINFO {}
impl ::core::default::Default for BITMAPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct BITMAPINFOHEADER {
    pub biSize: u32,
    pub biWidth: i32,
    pub biHeight: i32,
    pub biPlanes: u16,
    pub biBitCount: u16,
    pub biCompression: BI_COMPRESSION,
    pub biSizeImage: u32,
    pub biXPelsPerMeter: i32,
    pub biYPelsPerMeter: i32,
    pub biClrUsed: u32,
    pub biClrImportant: u32,
}
impl ::core::marker::Copy for BITMAPINFOHEADER {}
impl ::core::clone::Clone for BITMAPINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BITMAPINFOHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAPINFOHEADER").field("biSize", &self.biSize).field("biWidth", &self.biWidth).field("biHeight", &self.biHeight).field("biPlanes", &self.biPlanes).field("biBitCount", &self.biBitCount).field("biCompression", &self.biCompression).field("biSizeImage", &self.biSizeImage).field("biXPelsPerMeter", &self.biXPelsPerMeter).field("biYPelsPerMeter", &self.biYPelsPerMeter).field("biClrUsed", &self.biClrUsed).field("biClrImportant", &self.biClrImportant).finish()
    }
}
unsafe impl ::windows::core::Abi for BITMAPINFOHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BITMAPINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BITMAPINFOHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for BITMAPINFOHEADER {}
impl ::core::default::Default for BITMAPINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct BITMAPV4HEADER {
    pub bV4Size: u32,
    pub bV4Width: i32,
    pub bV4Height: i32,
    pub bV4Planes: u16,
    pub bV4BitCount: u16,
    pub bV4V4Compression: BI_COMPRESSION,
    pub bV4SizeImage: u32,
    pub bV4XPelsPerMeter: i32,
    pub bV4YPelsPerMeter: i32,
    pub bV4ClrUsed: u32,
    pub bV4ClrImportant: u32,
    pub bV4RedMask: u32,
    pub bV4GreenMask: u32,
    pub bV4BlueMask: u32,
    pub bV4AlphaMask: u32,
    pub bV4CSType: u32,
    pub bV4Endpoints: CIEXYZTRIPLE,
    pub bV4GammaRed: u32,
    pub bV4GammaGreen: u32,
    pub bV4GammaBlue: u32,
}
impl ::core::marker::Copy for BITMAPV4HEADER {}
impl ::core::clone::Clone for BITMAPV4HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BITMAPV4HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAPV4HEADER")
            .field("bV4Size", &self.bV4Size)
            .field("bV4Width", &self.bV4Width)
            .field("bV4Height", &self.bV4Height)
            .field("bV4Planes", &self.bV4Planes)
            .field("bV4BitCount", &self.bV4BitCount)
            .field("bV4V4Compression", &self.bV4V4Compression)
            .field("bV4SizeImage", &self.bV4SizeImage)
            .field("bV4XPelsPerMeter", &self.bV4XPelsPerMeter)
            .field("bV4YPelsPerMeter", &self.bV4YPelsPerMeter)
            .field("bV4ClrUsed", &self.bV4ClrUsed)
            .field("bV4ClrImportant", &self.bV4ClrImportant)
            .field("bV4RedMask", &self.bV4RedMask)
            .field("bV4GreenMask", &self.bV4GreenMask)
            .field("bV4BlueMask", &self.bV4BlueMask)
            .field("bV4AlphaMask", &self.bV4AlphaMask)
            .field("bV4CSType", &self.bV4CSType)
            .field("bV4Endpoints", &self.bV4Endpoints)
            .field("bV4GammaRed", &self.bV4GammaRed)
            .field("bV4GammaGreen", &self.bV4GammaGreen)
            .field("bV4GammaBlue", &self.bV4GammaBlue)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for BITMAPV4HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BITMAPV4HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BITMAPV4HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for BITMAPV4HEADER {}
impl ::core::default::Default for BITMAPV4HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct BITMAPV5HEADER {
    pub bV5Size: u32,
    pub bV5Width: i32,
    pub bV5Height: i32,
    pub bV5Planes: u16,
    pub bV5BitCount: u16,
    pub bV5Compression: BI_COMPRESSION,
    pub bV5SizeImage: u32,
    pub bV5XPelsPerMeter: i32,
    pub bV5YPelsPerMeter: i32,
    pub bV5ClrUsed: u32,
    pub bV5ClrImportant: u32,
    pub bV5RedMask: u32,
    pub bV5GreenMask: u32,
    pub bV5BlueMask: u32,
    pub bV5AlphaMask: u32,
    pub bV5CSType: u32,
    pub bV5Endpoints: CIEXYZTRIPLE,
    pub bV5GammaRed: u32,
    pub bV5GammaGreen: u32,
    pub bV5GammaBlue: u32,
    pub bV5Intent: u32,
    pub bV5ProfileData: u32,
    pub bV5ProfileSize: u32,
    pub bV5Reserved: u32,
}
impl ::core::marker::Copy for BITMAPV5HEADER {}
impl ::core::clone::Clone for BITMAPV5HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BITMAPV5HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAPV5HEADER")
            .field("bV5Size", &self.bV5Size)
            .field("bV5Width", &self.bV5Width)
            .field("bV5Height", &self.bV5Height)
            .field("bV5Planes", &self.bV5Planes)
            .field("bV5BitCount", &self.bV5BitCount)
            .field("bV5Compression", &self.bV5Compression)
            .field("bV5SizeImage", &self.bV5SizeImage)
            .field("bV5XPelsPerMeter", &self.bV5XPelsPerMeter)
            .field("bV5YPelsPerMeter", &self.bV5YPelsPerMeter)
            .field("bV5ClrUsed", &self.bV5ClrUsed)
            .field("bV5ClrImportant", &self.bV5ClrImportant)
            .field("bV5RedMask", &self.bV5RedMask)
            .field("bV5GreenMask", &self.bV5GreenMask)
            .field("bV5BlueMask", &self.bV5BlueMask)
            .field("bV5AlphaMask", &self.bV5AlphaMask)
            .field("bV5CSType", &self.bV5CSType)
            .field("bV5Endpoints", &self.bV5Endpoints)
            .field("bV5GammaRed", &self.bV5GammaRed)
            .field("bV5GammaGreen", &self.bV5GammaGreen)
            .field("bV5GammaBlue", &self.bV5GammaBlue)
            .field("bV5Intent", &self.bV5Intent)
            .field("bV5ProfileData", &self.bV5ProfileData)
            .field("bV5ProfileSize", &self.bV5ProfileSize)
            .field("bV5Reserved", &self.bV5Reserved)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for BITMAPV5HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BITMAPV5HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BITMAPV5HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for BITMAPV5HEADER {}
impl ::core::default::Default for BITMAPV5HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct BLENDFUNCTION {
    pub BlendOp: u8,
    pub BlendFlags: u8,
    pub SourceConstantAlpha: u8,
    pub AlphaFormat: u8,
}
impl ::core::marker::Copy for BLENDFUNCTION {}
impl ::core::clone::Clone for BLENDFUNCTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BLENDFUNCTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BLENDFUNCTION").field("BlendOp", &self.BlendOp).field("BlendFlags", &self.BlendFlags).field("SourceConstantAlpha", &self.SourceConstantAlpha).field("AlphaFormat", &self.AlphaFormat).finish()
    }
}
unsafe impl ::windows::core::Abi for BLENDFUNCTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BLENDFUNCTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BLENDFUNCTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for BLENDFUNCTION {}
impl ::core::default::Default for BLENDFUNCTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct CIEXYZ {
    pub ciexyzX: i32,
    pub ciexyzY: i32,
    pub ciexyzZ: i32,
}
impl ::core::marker::Copy for CIEXYZ {}
impl ::core::clone::Clone for CIEXYZ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CIEXYZ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CIEXYZ").field("ciexyzX", &self.ciexyzX).field("ciexyzY", &self.ciexyzY).field("ciexyzZ", &self.ciexyzZ).finish()
    }
}
unsafe impl ::windows::core::Abi for CIEXYZ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CIEXYZ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CIEXYZ>()) == 0 }
    }
}
impl ::core::cmp::Eq for CIEXYZ {}
impl ::core::default::Default for CIEXYZ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct CIEXYZTRIPLE {
    pub ciexyzRed: CIEXYZ,
    pub ciexyzGreen: CIEXYZ,
    pub ciexyzBlue: CIEXYZ,
}
impl ::core::marker::Copy for CIEXYZTRIPLE {}
impl ::core::clone::Clone for CIEXYZTRIPLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CIEXYZTRIPLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CIEXYZTRIPLE").field("ciexyzRed", &self.ciexyzRed).field("ciexyzGreen", &self.ciexyzGreen).field("ciexyzBlue", &self.ciexyzBlue).finish()
    }
}
unsafe impl ::windows::core::Abi for CIEXYZTRIPLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CIEXYZTRIPLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CIEXYZTRIPLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for CIEXYZTRIPLE {}
impl ::core::default::Default for CIEXYZTRIPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct COLORADJUSTMENT {
    pub caSize: u16,
    pub caFlags: u16,
    pub caIlluminantIndex: u16,
    pub caRedGamma: u16,
    pub caGreenGamma: u16,
    pub caBlueGamma: u16,
    pub caReferenceBlack: u16,
    pub caReferenceWhite: u16,
    pub caContrast: i16,
    pub caBrightness: i16,
    pub caColorfulness: i16,
    pub caRedGreenTint: i16,
}
impl ::core::marker::Copy for COLORADJUSTMENT {}
impl ::core::clone::Clone for COLORADJUSTMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COLORADJUSTMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLORADJUSTMENT")
            .field("caSize", &self.caSize)
            .field("caFlags", &self.caFlags)
            .field("caIlluminantIndex", &self.caIlluminantIndex)
            .field("caRedGamma", &self.caRedGamma)
            .field("caGreenGamma", &self.caGreenGamma)
            .field("caBlueGamma", &self.caBlueGamma)
            .field("caReferenceBlack", &self.caReferenceBlack)
            .field("caReferenceWhite", &self.caReferenceWhite)
            .field("caContrast", &self.caContrast)
            .field("caBrightness", &self.caBrightness)
            .field("caColorfulness", &self.caColorfulness)
            .field("caRedGreenTint", &self.caRedGreenTint)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for COLORADJUSTMENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COLORADJUSTMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COLORADJUSTMENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for COLORADJUSTMENT {}
impl ::core::default::Default for COLORADJUSTMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CreatedHDC(pub isize);
impl CreatedHDC {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for CreatedHDC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for CreatedHDC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for CreatedHDC {}
impl ::core::fmt::Debug for CreatedHDC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreatedHDC").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<CreatedHDC>> for CreatedHDC {
    fn from(optional: ::core::option::Option<CreatedHDC>) -> CreatedHDC {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for CreatedHDC {
    type Abi = Self;
}
impl ::core::convert::From<CreatedHDC> for HDC {
    fn from(item: CreatedHDC) -> HDC {
        HDC(item.0)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct DESIGNVECTOR {
    pub dvReserved: u32,
    pub dvNumAxes: u32,
    pub dvValues: [i32; 16],
}
impl ::core::marker::Copy for DESIGNVECTOR {}
impl ::core::clone::Clone for DESIGNVECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DESIGNVECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DESIGNVECTOR").field("dvReserved", &self.dvReserved).field("dvNumAxes", &self.dvNumAxes).field("dvValues", &self.dvValues).finish()
    }
}
unsafe impl ::windows::core::Abi for DESIGNVECTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DESIGNVECTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DESIGNVECTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for DESIGNVECTOR {}
impl ::core::default::Default for DESIGNVECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVMODEA {
    pub dmDeviceName: [u8; 32],
    pub dmSpecVersion: u16,
    pub dmDriverVersion: u16,
    pub dmSize: u16,
    pub dmDriverExtra: u16,
    pub dmFields: DEVMODE_FIELD_FLAGS,
    pub Anonymous1: DEVMODEA_0,
    pub dmColor: DEVMODE_COLOR,
    pub dmDuplex: i16,
    pub dmYResolution: i16,
    pub dmTTOption: DEVMODE_TRUETYPE_OPTION,
    pub dmCollate: DEVMODE_COLLATE,
    pub dmFormName: [u8; 32],
    pub dmLogPixels: u16,
    pub dmBitsPerPel: u32,
    pub dmPelsWidth: u32,
    pub dmPelsHeight: u32,
    pub Anonymous2: DEVMODEA_1,
    pub dmDisplayFrequency: u32,
    pub dmICMMethod: u32,
    pub dmICMIntent: u32,
    pub dmMediaType: u32,
    pub dmDitherType: u32,
    pub dmReserved1: u32,
    pub dmReserved2: u32,
    pub dmPanningWidth: u32,
    pub dmPanningHeight: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVMODEA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVMODEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVMODEA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVMODEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVMODEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DEVMODEA_0 {
    pub Anonymous1: DEVMODEA_0_0,
    pub Anonymous2: DEVMODEA_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVMODEA_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVMODEA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVMODEA_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVMODEA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVMODEA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVMODEA_0_0 {
    pub dmOrientation: i16,
    pub dmPaperSize: i16,
    pub dmPaperLength: i16,
    pub dmPaperWidth: i16,
    pub dmScale: i16,
    pub dmCopies: i16,
    pub dmDefaultSource: i16,
    pub dmPrintQuality: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEA_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVMODEA_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVMODEA_0_0").field("dmOrientation", &self.dmOrientation).field("dmPaperSize", &self.dmPaperSize).field("dmPaperLength", &self.dmPaperLength).field("dmPaperWidth", &self.dmPaperWidth).field("dmScale", &self.dmScale).field("dmCopies", &self.dmCopies).field("dmDefaultSource", &self.dmDefaultSource).field("dmPrintQuality", &self.dmPrintQuality).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVMODEA_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVMODEA_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVMODEA_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVMODEA_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVMODEA_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVMODEA_0_1 {
    pub dmPosition: super::super::Foundation::POINTL,
    pub dmDisplayOrientation: u32,
    pub dmDisplayFixedOutput: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEA_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEA_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVMODEA_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVMODEA_0_1").field("dmPosition", &self.dmPosition).field("dmDisplayOrientation", &self.dmDisplayOrientation).field("dmDisplayFixedOutput", &self.dmDisplayFixedOutput).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVMODEA_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVMODEA_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVMODEA_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVMODEA_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVMODEA_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DEVMODEA_1 {
    pub dmDisplayFlags: u32,
    pub dmNup: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEA_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVMODEA_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVMODEA_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVMODEA_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVMODEA_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVMODEA_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVMODEW {
    pub dmDeviceName: [u16; 32],
    pub dmSpecVersion: u16,
    pub dmDriverVersion: u16,
    pub dmSize: u16,
    pub dmDriverExtra: u16,
    pub dmFields: DEVMODE_FIELD_FLAGS,
    pub Anonymous1: DEVMODEW_0,
    pub dmColor: DEVMODE_COLOR,
    pub dmDuplex: i16,
    pub dmYResolution: i16,
    pub dmTTOption: DEVMODE_TRUETYPE_OPTION,
    pub dmCollate: DEVMODE_COLLATE,
    pub dmFormName: [u16; 32],
    pub dmLogPixels: u16,
    pub dmBitsPerPel: u32,
    pub dmPelsWidth: u32,
    pub dmPelsHeight: u32,
    pub Anonymous2: DEVMODEW_1,
    pub dmDisplayFrequency: u32,
    pub dmICMMethod: u32,
    pub dmICMIntent: u32,
    pub dmMediaType: u32,
    pub dmDitherType: u32,
    pub dmReserved1: u32,
    pub dmReserved2: u32,
    pub dmPanningWidth: u32,
    pub dmPanningHeight: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVMODEW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVMODEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVMODEW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVMODEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVMODEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DEVMODEW_0 {
    pub Anonymous1: DEVMODEW_0_0,
    pub Anonymous2: DEVMODEW_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVMODEW_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVMODEW_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVMODEW_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVMODEW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVMODEW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVMODEW_0_0 {
    pub dmOrientation: i16,
    pub dmPaperSize: i16,
    pub dmPaperLength: i16,
    pub dmPaperWidth: i16,
    pub dmScale: i16,
    pub dmCopies: i16,
    pub dmDefaultSource: i16,
    pub dmPrintQuality: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEW_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEW_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVMODEW_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVMODEW_0_0").field("dmOrientation", &self.dmOrientation).field("dmPaperSize", &self.dmPaperSize).field("dmPaperLength", &self.dmPaperLength).field("dmPaperWidth", &self.dmPaperWidth).field("dmScale", &self.dmScale).field("dmCopies", &self.dmCopies).field("dmDefaultSource", &self.dmDefaultSource).field("dmPrintQuality", &self.dmPrintQuality).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVMODEW_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVMODEW_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVMODEW_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVMODEW_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVMODEW_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVMODEW_0_1 {
    pub dmPosition: super::super::Foundation::POINTL,
    pub dmDisplayOrientation: u32,
    pub dmDisplayFixedOutput: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEW_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEW_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVMODEW_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVMODEW_0_1").field("dmPosition", &self.dmPosition).field("dmDisplayOrientation", &self.dmDisplayOrientation).field("dmDisplayFixedOutput", &self.dmDisplayFixedOutput).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVMODEW_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVMODEW_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVMODEW_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVMODEW_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVMODEW_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DEVMODEW_1 {
    pub dmDisplayFlags: u32,
    pub dmNup: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEW_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEW_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVMODEW_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVMODEW_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVMODEW_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVMODEW_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVMODEW_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DIBSECTION {
    pub dsBm: BITMAP,
    pub dsBmih: BITMAPINFOHEADER,
    pub dsBitfields: [u32; 3],
    pub dshSection: super::super::Foundation::HANDLE,
    pub dsOffset: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIBSECTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIBSECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIBSECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIBSECTION").field("dsBm", &self.dsBm).field("dsBmih", &self.dsBmih).field("dsBitfields", &self.dsBitfields).field("dshSection", &self.dshSection).field("dsOffset", &self.dsOffset).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DIBSECTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIBSECTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIBSECTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIBSECTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIBSECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DISPLAY_DEVICEA {
    pub cb: u32,
    pub DeviceName: [super::super::Foundation::CHAR; 32],
    pub DeviceString: [super::super::Foundation::CHAR; 128],
    pub StateFlags: u32,
    pub DeviceID: [super::super::Foundation::CHAR; 128],
    pub DeviceKey: [super::super::Foundation::CHAR; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DISPLAY_DEVICEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DISPLAY_DEVICEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAY_DEVICEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPLAY_DEVICEA").field("cb", &self.cb).field("DeviceName", &self.DeviceName).field("DeviceString", &self.DeviceString).field("StateFlags", &self.StateFlags).field("DeviceID", &self.DeviceID).field("DeviceKey", &self.DeviceKey).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAY_DEVICEA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAY_DEVICEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISPLAY_DEVICEA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAY_DEVICEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAY_DEVICEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct DISPLAY_DEVICEW {
    pub cb: u32,
    pub DeviceName: [u16; 32],
    pub DeviceString: [u16; 128],
    pub StateFlags: u32,
    pub DeviceID: [u16; 128],
    pub DeviceKey: [u16; 128],
}
impl ::core::marker::Copy for DISPLAY_DEVICEW {}
impl ::core::clone::Clone for DISPLAY_DEVICEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DISPLAY_DEVICEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPLAY_DEVICEW").field("cb", &self.cb).field("DeviceName", &self.DeviceName).field("DeviceString", &self.DeviceString).field("StateFlags", &self.StateFlags).field("DeviceID", &self.DeviceID).field("DeviceKey", &self.DeviceKey).finish()
    }
}
unsafe impl ::windows::core::Abi for DISPLAY_DEVICEW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DISPLAY_DEVICEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISPLAY_DEVICEW>()) == 0 }
    }
}
impl ::core::cmp::Eq for DISPLAY_DEVICEW {}
impl ::core::default::Default for DISPLAY_DEVICEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct DRAWTEXTPARAMS {
    pub cbSize: u32,
    pub iTabLength: i32,
    pub iLeftMargin: i32,
    pub iRightMargin: i32,
    pub uiLengthDrawn: u32,
}
impl ::core::marker::Copy for DRAWTEXTPARAMS {}
impl ::core::clone::Clone for DRAWTEXTPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRAWTEXTPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRAWTEXTPARAMS").field("cbSize", &self.cbSize).field("iTabLength", &self.iTabLength).field("iLeftMargin", &self.iLeftMargin).field("iRightMargin", &self.iRightMargin).field("uiLengthDrawn", &self.uiLengthDrawn).finish()
    }
}
unsafe impl ::windows::core::Abi for DRAWTEXTPARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRAWTEXTPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRAWTEXTPARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRAWTEXTPARAMS {}
impl ::core::default::Default for DRAWTEXTPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMR {
    pub iType: ENHANCED_METAFILE_RECORD_TYPE,
    pub nSize: u32,
}
impl ::core::marker::Copy for EMR {}
impl ::core::clone::Clone for EMR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMR").field("iType", &self.iType).field("nSize", &self.nSize).finish()
    }
}
unsafe impl ::windows::core::Abi for EMR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMR>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMR {}
impl ::core::default::Default for EMR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRALPHABLEND {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::super::Foundation::COLORREF,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub cxSrc: i32,
    pub cySrc: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRALPHABLEND {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRALPHABLEND {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRALPHABLEND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRALPHABLEND")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("xDest", &self.xDest)
            .field("yDest", &self.yDest)
            .field("cxDest", &self.cxDest)
            .field("cyDest", &self.cyDest)
            .field("dwRop", &self.dwRop)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("xformSrc", &self.xformSrc)
            .field("crBkColorSrc", &self.crBkColorSrc)
            .field("iUsageSrc", &self.iUsageSrc)
            .field("offBmiSrc", &self.offBmiSrc)
            .field("cbBmiSrc", &self.cbBmiSrc)
            .field("offBitsSrc", &self.offBitsSrc)
            .field("cbBitsSrc", &self.cbBitsSrc)
            .field("cxSrc", &self.cxSrc)
            .field("cySrc", &self.cySrc)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRALPHABLEND {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRALPHABLEND {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRALPHABLEND>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRALPHABLEND {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRALPHABLEND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRANGLEARC {
    pub emr: EMR,
    pub ptlCenter: super::super::Foundation::POINTL,
    pub nRadius: u32,
    pub eStartAngle: f32,
    pub eSweepAngle: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRANGLEARC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRANGLEARC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRANGLEARC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRANGLEARC").field("emr", &self.emr).field("ptlCenter", &self.ptlCenter).field("nRadius", &self.nRadius).field("eStartAngle", &self.eStartAngle).field("eSweepAngle", &self.eSweepAngle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRANGLEARC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRANGLEARC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRANGLEARC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRANGLEARC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRANGLEARC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRARC {
    pub emr: EMR,
    pub rclBox: super::super::Foundation::RECTL,
    pub ptlStart: super::super::Foundation::POINTL,
    pub ptlEnd: super::super::Foundation::POINTL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRARC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRARC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRARC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRARC").field("emr", &self.emr).field("rclBox", &self.rclBox).field("ptlStart", &self.ptlStart).field("ptlEnd", &self.ptlEnd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRARC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRARC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRARC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRARC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRARC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRBITBLT {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::super::Foundation::COLORREF,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRBITBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRBITBLT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRBITBLT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRBITBLT")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("xDest", &self.xDest)
            .field("yDest", &self.yDest)
            .field("cxDest", &self.cxDest)
            .field("cyDest", &self.cyDest)
            .field("dwRop", &self.dwRop)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("xformSrc", &self.xformSrc)
            .field("crBkColorSrc", &self.crBkColorSrc)
            .field("iUsageSrc", &self.iUsageSrc)
            .field("offBmiSrc", &self.offBmiSrc)
            .field("cbBmiSrc", &self.cbBmiSrc)
            .field("offBitsSrc", &self.offBitsSrc)
            .field("cbBitsSrc", &self.cbBitsSrc)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRBITBLT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRBITBLT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRBITBLT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRBITBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRBITBLT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRCOLORCORRECTPALETTE {
    pub emr: EMR,
    pub ihPalette: u32,
    pub nFirstEntry: u32,
    pub nPalEntries: u32,
    pub nReserved: u32,
}
impl ::core::marker::Copy for EMRCOLORCORRECTPALETTE {}
impl ::core::clone::Clone for EMRCOLORCORRECTPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRCOLORCORRECTPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCOLORCORRECTPALETTE").field("emr", &self.emr).field("ihPalette", &self.ihPalette).field("nFirstEntry", &self.nFirstEntry).field("nPalEntries", &self.nPalEntries).field("nReserved", &self.nReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRCOLORCORRECTPALETTE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRCOLORCORRECTPALETTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRCOLORCORRECTPALETTE>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRCOLORCORRECTPALETTE {}
impl ::core::default::Default for EMRCOLORCORRECTPALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRCOLORMATCHTOTARGET {
    pub emr: EMR,
    pub dwAction: u32,
    pub dwFlags: u32,
    pub cbName: u32,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for EMRCOLORMATCHTOTARGET {}
impl ::core::clone::Clone for EMRCOLORMATCHTOTARGET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRCOLORMATCHTOTARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCOLORMATCHTOTARGET").field("emr", &self.emr).field("dwAction", &self.dwAction).field("dwFlags", &self.dwFlags).field("cbName", &self.cbName).field("cbData", &self.cbData).field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRCOLORMATCHTOTARGET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRCOLORMATCHTOTARGET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRCOLORMATCHTOTARGET>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRCOLORMATCHTOTARGET {}
impl ::core::default::Default for EMRCOLORMATCHTOTARGET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRCREATEBRUSHINDIRECT {
    pub emr: EMR,
    pub ihBrush: u32,
    pub lb: LOGBRUSH32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRCREATEBRUSHINDIRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRCREATEBRUSHINDIRECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRCREATEBRUSHINDIRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATEBRUSHINDIRECT").field("emr", &self.emr).field("ihBrush", &self.ihBrush).field("lb", &self.lb).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRCREATEBRUSHINDIRECT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRCREATEBRUSHINDIRECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRCREATEBRUSHINDIRECT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRCREATEBRUSHINDIRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRCREATEBRUSHINDIRECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRCREATEDIBPATTERNBRUSHPT {
    pub emr: EMR,
    pub ihBrush: u32,
    pub iUsage: u32,
    pub offBmi: u32,
    pub cbBmi: u32,
    pub offBits: u32,
    pub cbBits: u32,
}
impl ::core::marker::Copy for EMRCREATEDIBPATTERNBRUSHPT {}
impl ::core::clone::Clone for EMRCREATEDIBPATTERNBRUSHPT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRCREATEDIBPATTERNBRUSHPT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATEDIBPATTERNBRUSHPT").field("emr", &self.emr).field("ihBrush", &self.ihBrush).field("iUsage", &self.iUsage).field("offBmi", &self.offBmi).field("cbBmi", &self.cbBmi).field("offBits", &self.offBits).field("cbBits", &self.cbBits).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRCREATEDIBPATTERNBRUSHPT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRCREATEDIBPATTERNBRUSHPT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRCREATEDIBPATTERNBRUSHPT>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRCREATEDIBPATTERNBRUSHPT {}
impl ::core::default::Default for EMRCREATEDIBPATTERNBRUSHPT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRCREATEMONOBRUSH {
    pub emr: EMR,
    pub ihBrush: u32,
    pub iUsage: u32,
    pub offBmi: u32,
    pub cbBmi: u32,
    pub offBits: u32,
    pub cbBits: u32,
}
impl ::core::marker::Copy for EMRCREATEMONOBRUSH {}
impl ::core::clone::Clone for EMRCREATEMONOBRUSH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRCREATEMONOBRUSH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATEMONOBRUSH").field("emr", &self.emr).field("ihBrush", &self.ihBrush).field("iUsage", &self.iUsage).field("offBmi", &self.offBmi).field("cbBmi", &self.cbBmi).field("offBits", &self.offBits).field("cbBits", &self.cbBits).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRCREATEMONOBRUSH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRCREATEMONOBRUSH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRCREATEMONOBRUSH>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRCREATEMONOBRUSH {}
impl ::core::default::Default for EMRCREATEMONOBRUSH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRCREATEPALETTE {
    pub emr: EMR,
    pub ihPal: u32,
    pub lgpl: LOGPALETTE,
}
impl ::core::marker::Copy for EMRCREATEPALETTE {}
impl ::core::clone::Clone for EMRCREATEPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRCREATEPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATEPALETTE").field("emr", &self.emr).field("ihPal", &self.ihPal).field("lgpl", &self.lgpl).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRCREATEPALETTE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRCREATEPALETTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRCREATEPALETTE>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRCREATEPALETTE {}
impl ::core::default::Default for EMRCREATEPALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRCREATEPEN {
    pub emr: EMR,
    pub ihPen: u32,
    pub lopn: LOGPEN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRCREATEPEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRCREATEPEN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRCREATEPEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATEPEN").field("emr", &self.emr).field("ihPen", &self.ihPen).field("lopn", &self.lopn).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRCREATEPEN {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRCREATEPEN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRCREATEPEN>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRCREATEPEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRCREATEPEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRELLIPSE {
    pub emr: EMR,
    pub rclBox: super::super::Foundation::RECTL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRELLIPSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRELLIPSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRELLIPSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRELLIPSE").field("emr", &self.emr).field("rclBox", &self.rclBox).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRELLIPSE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRELLIPSE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRELLIPSE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRELLIPSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRELLIPSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMREOF {
    pub emr: EMR,
    pub nPalEntries: u32,
    pub offPalEntries: u32,
    pub nSizeLast: u32,
}
impl ::core::marker::Copy for EMREOF {}
impl ::core::clone::Clone for EMREOF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMREOF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREOF").field("emr", &self.emr).field("nPalEntries", &self.nPalEntries).field("offPalEntries", &self.offPalEntries).field("nSizeLast", &self.nSizeLast).finish()
    }
}
unsafe impl ::windows::core::Abi for EMREOF {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMREOF {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMREOF>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMREOF {}
impl ::core::default::Default for EMREOF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMREXCLUDECLIPRECT {
    pub emr: EMR,
    pub rclClip: super::super::Foundation::RECTL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMREXCLUDECLIPRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMREXCLUDECLIPRECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMREXCLUDECLIPRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXCLUDECLIPRECT").field("emr", &self.emr).field("rclClip", &self.rclClip).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMREXCLUDECLIPRECT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMREXCLUDECLIPRECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMREXCLUDECLIPRECT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMREXCLUDECLIPRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMREXCLUDECLIPRECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMREXTCREATEFONTINDIRECTW {
    pub emr: EMR,
    pub ihFont: u32,
    pub elfw: EXTLOGFONTW,
}
impl ::core::marker::Copy for EMREXTCREATEFONTINDIRECTW {}
impl ::core::clone::Clone for EMREXTCREATEFONTINDIRECTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMREXTCREATEFONTINDIRECTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXTCREATEFONTINDIRECTW").field("emr", &self.emr).field("ihFont", &self.ihFont).field("elfw", &self.elfw).finish()
    }
}
unsafe impl ::windows::core::Abi for EMREXTCREATEFONTINDIRECTW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMREXTCREATEFONTINDIRECTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMREXTCREATEFONTINDIRECTW>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMREXTCREATEFONTINDIRECTW {}
impl ::core::default::Default for EMREXTCREATEFONTINDIRECTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMREXTCREATEPEN {
    pub emr: EMR,
    pub ihPen: u32,
    pub offBmi: u32,
    pub cbBmi: u32,
    pub offBits: u32,
    pub cbBits: u32,
    pub elp: EXTLOGPEN32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMREXTCREATEPEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMREXTCREATEPEN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMREXTCREATEPEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXTCREATEPEN").field("emr", &self.emr).field("ihPen", &self.ihPen).field("offBmi", &self.offBmi).field("cbBmi", &self.cbBmi).field("offBits", &self.offBits).field("cbBits", &self.cbBits).field("elp", &self.elp).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMREXTCREATEPEN {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMREXTCREATEPEN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMREXTCREATEPEN>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMREXTCREATEPEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMREXTCREATEPEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMREXTESCAPE {
    pub emr: EMR,
    pub iEscape: i32,
    pub cbEscData: i32,
    pub EscData: [u8; 1],
}
impl ::core::marker::Copy for EMREXTESCAPE {}
impl ::core::clone::Clone for EMREXTESCAPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMREXTESCAPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXTESCAPE").field("emr", &self.emr).field("iEscape", &self.iEscape).field("cbEscData", &self.cbEscData).field("EscData", &self.EscData).finish()
    }
}
unsafe impl ::windows::core::Abi for EMREXTESCAPE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMREXTESCAPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMREXTESCAPE>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMREXTESCAPE {}
impl ::core::default::Default for EMREXTESCAPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMREXTFLOODFILL {
    pub emr: EMR,
    pub ptlStart: super::super::Foundation::POINTL,
    pub crColor: super::super::Foundation::COLORREF,
    pub iMode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMREXTFLOODFILL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMREXTFLOODFILL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMREXTFLOODFILL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXTFLOODFILL").field("emr", &self.emr).field("ptlStart", &self.ptlStart).field("crColor", &self.crColor).field("iMode", &self.iMode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMREXTFLOODFILL {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMREXTFLOODFILL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMREXTFLOODFILL>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMREXTFLOODFILL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMREXTFLOODFILL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMREXTSELECTCLIPRGN {
    pub emr: EMR,
    pub cbRgnData: u32,
    pub iMode: RGN_COMBINE_MODE,
    pub RgnData: [u8; 1],
}
impl ::core::marker::Copy for EMREXTSELECTCLIPRGN {}
impl ::core::clone::Clone for EMREXTSELECTCLIPRGN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMREXTSELECTCLIPRGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXTSELECTCLIPRGN").field("emr", &self.emr).field("cbRgnData", &self.cbRgnData).field("iMode", &self.iMode).field("RgnData", &self.RgnData).finish()
    }
}
unsafe impl ::windows::core::Abi for EMREXTSELECTCLIPRGN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMREXTSELECTCLIPRGN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMREXTSELECTCLIPRGN>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMREXTSELECTCLIPRGN {}
impl ::core::default::Default for EMREXTSELECTCLIPRGN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMREXTTEXTOUTA {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub iGraphicsMode: u32,
    pub exScale: f32,
    pub eyScale: f32,
    pub emrtext: EMRTEXT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMREXTTEXTOUTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMREXTTEXTOUTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMREXTTEXTOUTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXTTEXTOUTA").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("iGraphicsMode", &self.iGraphicsMode).field("exScale", &self.exScale).field("eyScale", &self.eyScale).field("emrtext", &self.emrtext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMREXTTEXTOUTA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMREXTTEXTOUTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMREXTTEXTOUTA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMREXTTEXTOUTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMREXTTEXTOUTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRFILLPATH {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRFILLPATH {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRFILLPATH {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRFILLPATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRFILLPATH").field("emr", &self.emr).field("rclBounds", &self.rclBounds).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRFILLPATH {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRFILLPATH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRFILLPATH>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRFILLPATH {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRFILLPATH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRFILLRGN {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cbRgnData: u32,
    pub ihBrush: u32,
    pub RgnData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRFILLRGN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRFILLRGN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRFILLRGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRFILLRGN").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("cbRgnData", &self.cbRgnData).field("ihBrush", &self.ihBrush).field("RgnData", &self.RgnData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRFILLRGN {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRFILLRGN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRFILLRGN>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRFILLRGN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRFILLRGN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRFORMAT {
    pub dSignature: u32,
    pub nVersion: u32,
    pub cbData: u32,
    pub offData: u32,
}
impl ::core::marker::Copy for EMRFORMAT {}
impl ::core::clone::Clone for EMRFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRFORMAT").field("dSignature", &self.dSignature).field("nVersion", &self.nVersion).field("cbData", &self.cbData).field("offData", &self.offData).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRFORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRFORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRFORMAT {}
impl ::core::default::Default for EMRFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRFRAMERGN {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cbRgnData: u32,
    pub ihBrush: u32,
    pub szlStroke: super::super::Foundation::SIZE,
    pub RgnData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRFRAMERGN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRFRAMERGN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRFRAMERGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRFRAMERGN").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("cbRgnData", &self.cbRgnData).field("ihBrush", &self.ihBrush).field("szlStroke", &self.szlStroke).field("RgnData", &self.RgnData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRFRAMERGN {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRFRAMERGN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRFRAMERGN>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRFRAMERGN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRFRAMERGN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRGDICOMMENT {
    pub emr: EMR,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for EMRGDICOMMENT {}
impl ::core::clone::Clone for EMRGDICOMMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRGDICOMMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRGDICOMMENT").field("emr", &self.emr).field("cbData", &self.cbData).field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRGDICOMMENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRGDICOMMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRGDICOMMENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRGDICOMMENT {}
impl ::core::default::Default for EMRGDICOMMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRGLSBOUNDEDRECORD {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cbData: u32,
    pub Data: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRGLSBOUNDEDRECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRGLSBOUNDEDRECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRGLSBOUNDEDRECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRGLSBOUNDEDRECORD").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("cbData", &self.cbData).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRGLSBOUNDEDRECORD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRGLSBOUNDEDRECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRGLSBOUNDEDRECORD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRGLSBOUNDEDRECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRGLSBOUNDEDRECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRGLSRECORD {
    pub emr: EMR,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for EMRGLSRECORD {}
impl ::core::clone::Clone for EMRGLSRECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRGLSRECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRGLSRECORD").field("emr", &self.emr).field("cbData", &self.cbData).field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRGLSRECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRGLSRECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRGLSRECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRGLSRECORD {}
impl ::core::default::Default for EMRGLSRECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRGRADIENTFILL {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub nVer: u32,
    pub nTri: u32,
    pub ulMode: GRADIENT_FILL,
    pub Ver: [TRIVERTEX; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRGRADIENTFILL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRGRADIENTFILL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRGRADIENTFILL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRGRADIENTFILL").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("nVer", &self.nVer).field("nTri", &self.nTri).field("ulMode", &self.ulMode).field("Ver", &self.Ver).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRGRADIENTFILL {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRGRADIENTFILL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRGRADIENTFILL>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRGRADIENTFILL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRGRADIENTFILL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRINVERTRGN {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cbRgnData: u32,
    pub RgnData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRINVERTRGN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRINVERTRGN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRINVERTRGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRINVERTRGN").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("cbRgnData", &self.cbRgnData).field("RgnData", &self.RgnData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRINVERTRGN {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRINVERTRGN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRINVERTRGN>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRINVERTRGN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRINVERTRGN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRLINETO {
    pub emr: EMR,
    pub ptl: super::super::Foundation::POINTL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRLINETO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRLINETO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRLINETO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRLINETO").field("emr", &self.emr).field("ptl", &self.ptl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRLINETO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRLINETO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRLINETO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRLINETO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRLINETO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRMASKBLT {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::super::Foundation::COLORREF,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub xMask: i32,
    pub yMask: i32,
    pub iUsageMask: u32,
    pub offBmiMask: u32,
    pub cbBmiMask: u32,
    pub offBitsMask: u32,
    pub cbBitsMask: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRMASKBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRMASKBLT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRMASKBLT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRMASKBLT")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("xDest", &self.xDest)
            .field("yDest", &self.yDest)
            .field("cxDest", &self.cxDest)
            .field("cyDest", &self.cyDest)
            .field("dwRop", &self.dwRop)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("xformSrc", &self.xformSrc)
            .field("crBkColorSrc", &self.crBkColorSrc)
            .field("iUsageSrc", &self.iUsageSrc)
            .field("offBmiSrc", &self.offBmiSrc)
            .field("cbBmiSrc", &self.cbBmiSrc)
            .field("offBitsSrc", &self.offBitsSrc)
            .field("cbBitsSrc", &self.cbBitsSrc)
            .field("xMask", &self.xMask)
            .field("yMask", &self.yMask)
            .field("iUsageMask", &self.iUsageMask)
            .field("offBmiMask", &self.offBmiMask)
            .field("cbBmiMask", &self.cbBmiMask)
            .field("offBitsMask", &self.offBitsMask)
            .field("cbBitsMask", &self.cbBitsMask)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRMASKBLT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRMASKBLT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRMASKBLT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRMASKBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRMASKBLT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRMODIFYWORLDTRANSFORM {
    pub emr: EMR,
    pub xform: XFORM,
    pub iMode: MODIFY_WORLD_TRANSFORM_MODE,
}
impl ::core::marker::Copy for EMRMODIFYWORLDTRANSFORM {}
impl ::core::clone::Clone for EMRMODIFYWORLDTRANSFORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRMODIFYWORLDTRANSFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRMODIFYWORLDTRANSFORM").field("emr", &self.emr).field("xform", &self.xform).field("iMode", &self.iMode).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRMODIFYWORLDTRANSFORM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRMODIFYWORLDTRANSFORM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRMODIFYWORLDTRANSFORM>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRMODIFYWORLDTRANSFORM {}
impl ::core::default::Default for EMRMODIFYWORLDTRANSFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRNAMEDESCAPE {
    pub emr: EMR,
    pub iEscape: i32,
    pub cbDriver: i32,
    pub cbEscData: i32,
    pub EscData: [u8; 1],
}
impl ::core::marker::Copy for EMRNAMEDESCAPE {}
impl ::core::clone::Clone for EMRNAMEDESCAPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRNAMEDESCAPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRNAMEDESCAPE").field("emr", &self.emr).field("iEscape", &self.iEscape).field("cbDriver", &self.cbDriver).field("cbEscData", &self.cbEscData).field("EscData", &self.EscData).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRNAMEDESCAPE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRNAMEDESCAPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRNAMEDESCAPE>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRNAMEDESCAPE {}
impl ::core::default::Default for EMRNAMEDESCAPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMROFFSETCLIPRGN {
    pub emr: EMR,
    pub ptlOffset: super::super::Foundation::POINTL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMROFFSETCLIPRGN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMROFFSETCLIPRGN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMROFFSETCLIPRGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMROFFSETCLIPRGN").field("emr", &self.emr).field("ptlOffset", &self.ptlOffset).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMROFFSETCLIPRGN {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMROFFSETCLIPRGN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMROFFSETCLIPRGN>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMROFFSETCLIPRGN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMROFFSETCLIPRGN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPLGBLT {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub aptlDest: [super::super::Foundation::POINTL; 3],
    pub xSrc: i32,
    pub ySrc: i32,
    pub cxSrc: i32,
    pub cySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::super::Foundation::COLORREF,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub xMask: i32,
    pub yMask: i32,
    pub iUsageMask: u32,
    pub offBmiMask: u32,
    pub cbBmiMask: u32,
    pub offBitsMask: u32,
    pub cbBitsMask: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRPLGBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRPLGBLT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRPLGBLT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPLGBLT")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("aptlDest", &self.aptlDest)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("cxSrc", &self.cxSrc)
            .field("cySrc", &self.cySrc)
            .field("xformSrc", &self.xformSrc)
            .field("crBkColorSrc", &self.crBkColorSrc)
            .field("iUsageSrc", &self.iUsageSrc)
            .field("offBmiSrc", &self.offBmiSrc)
            .field("cbBmiSrc", &self.cbBmiSrc)
            .field("offBitsSrc", &self.offBitsSrc)
            .field("cbBitsSrc", &self.cbBitsSrc)
            .field("xMask", &self.xMask)
            .field("yMask", &self.yMask)
            .field("iUsageMask", &self.iUsageMask)
            .field("offBmiMask", &self.offBmiMask)
            .field("cbBmiMask", &self.cbBmiMask)
            .field("offBitsMask", &self.offBitsMask)
            .field("cbBitsMask", &self.cbBitsMask)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRPLGBLT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRPLGBLT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRPLGBLT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRPLGBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRPLGBLT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYDRAW {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cptl: u32,
    pub aptl: [super::super::Foundation::POINTL; 1],
    pub abTypes: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRPOLYDRAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRPOLYDRAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRPOLYDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYDRAW").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("cptl", &self.cptl).field("aptl", &self.aptl).field("abTypes", &self.abTypes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRPOLYDRAW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRPOLYDRAW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRPOLYDRAW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRPOLYDRAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRPOLYDRAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYDRAW16 {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cpts: u32,
    pub apts: [super::super::Foundation::POINTS; 1],
    pub abTypes: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRPOLYDRAW16 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRPOLYDRAW16 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRPOLYDRAW16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYDRAW16").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("cpts", &self.cpts).field("apts", &self.apts).field("abTypes", &self.abTypes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRPOLYDRAW16 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRPOLYDRAW16 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRPOLYDRAW16>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRPOLYDRAW16 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRPOLYDRAW16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYLINE {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cptl: u32,
    pub aptl: [super::super::Foundation::POINTL; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRPOLYLINE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRPOLYLINE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRPOLYLINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYLINE").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("cptl", &self.cptl).field("aptl", &self.aptl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRPOLYLINE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRPOLYLINE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRPOLYLINE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRPOLYLINE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRPOLYLINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYLINE16 {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cpts: u32,
    pub apts: [super::super::Foundation::POINTS; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRPOLYLINE16 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRPOLYLINE16 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRPOLYLINE16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYLINE16").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("cpts", &self.cpts).field("apts", &self.apts).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRPOLYLINE16 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRPOLYLINE16 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRPOLYLINE16>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRPOLYLINE16 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRPOLYLINE16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYPOLYLINE {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub nPolys: u32,
    pub cptl: u32,
    pub aPolyCounts: [u32; 1],
    pub aptl: [super::super::Foundation::POINTL; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRPOLYPOLYLINE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRPOLYPOLYLINE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRPOLYPOLYLINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYPOLYLINE").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("nPolys", &self.nPolys).field("cptl", &self.cptl).field("aPolyCounts", &self.aPolyCounts).field("aptl", &self.aptl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRPOLYPOLYLINE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRPOLYPOLYLINE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRPOLYPOLYLINE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRPOLYPOLYLINE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRPOLYPOLYLINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYPOLYLINE16 {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub nPolys: u32,
    pub cpts: u32,
    pub aPolyCounts: [u32; 1],
    pub apts: [super::super::Foundation::POINTS; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRPOLYPOLYLINE16 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRPOLYPOLYLINE16 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRPOLYPOLYLINE16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYPOLYLINE16").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("nPolys", &self.nPolys).field("cpts", &self.cpts).field("aPolyCounts", &self.aPolyCounts).field("apts", &self.apts).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRPOLYPOLYLINE16 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRPOLYPOLYLINE16 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRPOLYPOLYLINE16>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRPOLYPOLYLINE16 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRPOLYPOLYLINE16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYTEXTOUTA {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub iGraphicsMode: u32,
    pub exScale: f32,
    pub eyScale: f32,
    pub cStrings: i32,
    pub aemrtext: [EMRTEXT; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRPOLYTEXTOUTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRPOLYTEXTOUTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRPOLYTEXTOUTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYTEXTOUTA").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("iGraphicsMode", &self.iGraphicsMode).field("exScale", &self.exScale).field("eyScale", &self.eyScale).field("cStrings", &self.cStrings).field("aemrtext", &self.aemrtext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRPOLYTEXTOUTA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRPOLYTEXTOUTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRPOLYTEXTOUTA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRPOLYTEXTOUTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRPOLYTEXTOUTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRRESIZEPALETTE {
    pub emr: EMR,
    pub ihPal: u32,
    pub cEntries: u32,
}
impl ::core::marker::Copy for EMRRESIZEPALETTE {}
impl ::core::clone::Clone for EMRRESIZEPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRRESIZEPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRRESIZEPALETTE").field("emr", &self.emr).field("ihPal", &self.ihPal).field("cEntries", &self.cEntries).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRRESIZEPALETTE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRRESIZEPALETTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRRESIZEPALETTE>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRRESIZEPALETTE {}
impl ::core::default::Default for EMRRESIZEPALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRRESTOREDC {
    pub emr: EMR,
    pub iRelative: i32,
}
impl ::core::marker::Copy for EMRRESTOREDC {}
impl ::core::clone::Clone for EMRRESTOREDC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRRESTOREDC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRRESTOREDC").field("emr", &self.emr).field("iRelative", &self.iRelative).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRRESTOREDC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRRESTOREDC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRRESTOREDC>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRRESTOREDC {}
impl ::core::default::Default for EMRRESTOREDC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRROUNDRECT {
    pub emr: EMR,
    pub rclBox: super::super::Foundation::RECTL,
    pub szlCorner: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRROUNDRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRROUNDRECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRROUNDRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRROUNDRECT").field("emr", &self.emr).field("rclBox", &self.rclBox).field("szlCorner", &self.szlCorner).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRROUNDRECT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRROUNDRECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRROUNDRECT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRROUNDRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRROUNDRECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSCALEVIEWPORTEXTEX {
    pub emr: EMR,
    pub xNum: i32,
    pub xDenom: i32,
    pub yNum: i32,
    pub yDenom: i32,
}
impl ::core::marker::Copy for EMRSCALEVIEWPORTEXTEX {}
impl ::core::clone::Clone for EMRSCALEVIEWPORTEXTEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSCALEVIEWPORTEXTEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSCALEVIEWPORTEXTEX").field("emr", &self.emr).field("xNum", &self.xNum).field("xDenom", &self.xDenom).field("yNum", &self.yNum).field("yDenom", &self.yDenom).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRSCALEVIEWPORTEXTEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRSCALEVIEWPORTEXTEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSCALEVIEWPORTEXTEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRSCALEVIEWPORTEXTEX {}
impl ::core::default::Default for EMRSCALEVIEWPORTEXTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSELECTCLIPPATH {
    pub emr: EMR,
    pub iMode: u32,
}
impl ::core::marker::Copy for EMRSELECTCLIPPATH {}
impl ::core::clone::Clone for EMRSELECTCLIPPATH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSELECTCLIPPATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSELECTCLIPPATH").field("emr", &self.emr).field("iMode", &self.iMode).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRSELECTCLIPPATH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRSELECTCLIPPATH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSELECTCLIPPATH>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRSELECTCLIPPATH {}
impl ::core::default::Default for EMRSELECTCLIPPATH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSELECTOBJECT {
    pub emr: EMR,
    pub ihObject: u32,
}
impl ::core::marker::Copy for EMRSELECTOBJECT {}
impl ::core::clone::Clone for EMRSELECTOBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSELECTOBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSELECTOBJECT").field("emr", &self.emr).field("ihObject", &self.ihObject).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRSELECTOBJECT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRSELECTOBJECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSELECTOBJECT>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRSELECTOBJECT {}
impl ::core::default::Default for EMRSELECTOBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSELECTPALETTE {
    pub emr: EMR,
    pub ihPal: u32,
}
impl ::core::marker::Copy for EMRSELECTPALETTE {}
impl ::core::clone::Clone for EMRSELECTPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSELECTPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSELECTPALETTE").field("emr", &self.emr).field("ihPal", &self.ihPal).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRSELECTPALETTE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRSELECTPALETTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSELECTPALETTE>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRSELECTPALETTE {}
impl ::core::default::Default for EMRSELECTPALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSETARCDIRECTION {
    pub emr: EMR,
    pub iArcDirection: u32,
}
impl ::core::marker::Copy for EMRSETARCDIRECTION {}
impl ::core::clone::Clone for EMRSETARCDIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETARCDIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETARCDIRECTION").field("emr", &self.emr).field("iArcDirection", &self.iArcDirection).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRSETARCDIRECTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRSETARCDIRECTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSETARCDIRECTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRSETARCDIRECTION {}
impl ::core::default::Default for EMRSETARCDIRECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSETCOLORADJUSTMENT {
    pub emr: EMR,
    pub ColorAdjustment: COLORADJUSTMENT,
}
impl ::core::marker::Copy for EMRSETCOLORADJUSTMENT {}
impl ::core::clone::Clone for EMRSETCOLORADJUSTMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETCOLORADJUSTMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETCOLORADJUSTMENT").field("emr", &self.emr).field("ColorAdjustment", &self.ColorAdjustment).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRSETCOLORADJUSTMENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRSETCOLORADJUSTMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSETCOLORADJUSTMENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRSETCOLORADJUSTMENT {}
impl ::core::default::Default for EMRSETCOLORADJUSTMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSETCOLORSPACE {
    pub emr: EMR,
    pub ihCS: u32,
}
impl ::core::marker::Copy for EMRSETCOLORSPACE {}
impl ::core::clone::Clone for EMRSETCOLORSPACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETCOLORSPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETCOLORSPACE").field("emr", &self.emr).field("ihCS", &self.ihCS).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRSETCOLORSPACE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRSETCOLORSPACE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSETCOLORSPACE>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRSETCOLORSPACE {}
impl ::core::default::Default for EMRSETCOLORSPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSETDIBITSTODEVICE {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub cxSrc: i32,
    pub cySrc: i32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub iUsageSrc: u32,
    pub iStartScan: u32,
    pub cScans: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRSETDIBITSTODEVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRSETDIBITSTODEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRSETDIBITSTODEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETDIBITSTODEVICE")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("xDest", &self.xDest)
            .field("yDest", &self.yDest)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("cxSrc", &self.cxSrc)
            .field("cySrc", &self.cySrc)
            .field("offBmiSrc", &self.offBmiSrc)
            .field("cbBmiSrc", &self.cbBmiSrc)
            .field("offBitsSrc", &self.offBitsSrc)
            .field("cbBitsSrc", &self.cbBitsSrc)
            .field("iUsageSrc", &self.iUsageSrc)
            .field("iStartScan", &self.iStartScan)
            .field("cScans", &self.cScans)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRSETDIBITSTODEVICE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRSETDIBITSTODEVICE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSETDIBITSTODEVICE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRSETDIBITSTODEVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRSETDIBITSTODEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSETICMPROFILE {
    pub emr: EMR,
    pub dwFlags: u32,
    pub cbName: u32,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for EMRSETICMPROFILE {}
impl ::core::clone::Clone for EMRSETICMPROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETICMPROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETICMPROFILE").field("emr", &self.emr).field("dwFlags", &self.dwFlags).field("cbName", &self.cbName).field("cbData", &self.cbData).field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRSETICMPROFILE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRSETICMPROFILE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSETICMPROFILE>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRSETICMPROFILE {}
impl ::core::default::Default for EMRSETICMPROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSETMAPPERFLAGS {
    pub emr: EMR,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for EMRSETMAPPERFLAGS {}
impl ::core::clone::Clone for EMRSETMAPPERFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETMAPPERFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETMAPPERFLAGS").field("emr", &self.emr).field("dwFlags", &self.dwFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRSETMAPPERFLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRSETMAPPERFLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSETMAPPERFLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRSETMAPPERFLAGS {}
impl ::core::default::Default for EMRSETMAPPERFLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSETMITERLIMIT {
    pub emr: EMR,
    pub eMiterLimit: f32,
}
impl ::core::marker::Copy for EMRSETMITERLIMIT {}
impl ::core::clone::Clone for EMRSETMITERLIMIT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETMITERLIMIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETMITERLIMIT").field("emr", &self.emr).field("eMiterLimit", &self.eMiterLimit).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRSETMITERLIMIT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRSETMITERLIMIT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSETMITERLIMIT>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRSETMITERLIMIT {}
impl ::core::default::Default for EMRSETMITERLIMIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSETPALETTEENTRIES {
    pub emr: EMR,
    pub ihPal: u32,
    pub iStart: u32,
    pub cEntries: u32,
    pub aPalEntries: [PALETTEENTRY; 1],
}
impl ::core::marker::Copy for EMRSETPALETTEENTRIES {}
impl ::core::clone::Clone for EMRSETPALETTEENTRIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETPALETTEENTRIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETPALETTEENTRIES").field("emr", &self.emr).field("ihPal", &self.ihPal).field("iStart", &self.iStart).field("cEntries", &self.cEntries).field("aPalEntries", &self.aPalEntries).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRSETPALETTEENTRIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRSETPALETTEENTRIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSETPALETTEENTRIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRSETPALETTEENTRIES {}
impl ::core::default::Default for EMRSETPALETTEENTRIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSETPIXELV {
    pub emr: EMR,
    pub ptlPixel: super::super::Foundation::POINTL,
    pub crColor: super::super::Foundation::COLORREF,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRSETPIXELV {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRSETPIXELV {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRSETPIXELV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETPIXELV").field("emr", &self.emr).field("ptlPixel", &self.ptlPixel).field("crColor", &self.crColor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRSETPIXELV {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRSETPIXELV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSETPIXELV>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRSETPIXELV {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRSETPIXELV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSETTEXTCOLOR {
    pub emr: EMR,
    pub crColor: super::super::Foundation::COLORREF,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRSETTEXTCOLOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRSETTEXTCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRSETTEXTCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETTEXTCOLOR").field("emr", &self.emr).field("crColor", &self.crColor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRSETTEXTCOLOR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRSETTEXTCOLOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSETTEXTCOLOR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRSETTEXTCOLOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRSETTEXTCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSETVIEWPORTEXTEX {
    pub emr: EMR,
    pub szlExtent: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRSETVIEWPORTEXTEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRSETVIEWPORTEXTEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRSETVIEWPORTEXTEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETVIEWPORTEXTEX").field("emr", &self.emr).field("szlExtent", &self.szlExtent).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRSETVIEWPORTEXTEX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRSETVIEWPORTEXTEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSETVIEWPORTEXTEX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRSETVIEWPORTEXTEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRSETVIEWPORTEXTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSETVIEWPORTORGEX {
    pub emr: EMR,
    pub ptlOrigin: super::super::Foundation::POINTL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRSETVIEWPORTORGEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRSETVIEWPORTORGEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRSETVIEWPORTORGEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETVIEWPORTORGEX").field("emr", &self.emr).field("ptlOrigin", &self.ptlOrigin).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRSETVIEWPORTORGEX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRSETVIEWPORTORGEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSETVIEWPORTORGEX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRSETVIEWPORTORGEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRSETVIEWPORTORGEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSETWORLDTRANSFORM {
    pub emr: EMR,
    pub xform: XFORM,
}
impl ::core::marker::Copy for EMRSETWORLDTRANSFORM {}
impl ::core::clone::Clone for EMRSETWORLDTRANSFORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETWORLDTRANSFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETWORLDTRANSFORM").field("emr", &self.emr).field("xform", &self.xform).finish()
    }
}
unsafe impl ::windows::core::Abi for EMRSETWORLDTRANSFORM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EMRSETWORLDTRANSFORM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSETWORLDTRANSFORM>()) == 0 }
    }
}
impl ::core::cmp::Eq for EMRSETWORLDTRANSFORM {}
impl ::core::default::Default for EMRSETWORLDTRANSFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSTRETCHBLT {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::super::Foundation::COLORREF,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub cxSrc: i32,
    pub cySrc: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRSTRETCHBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRSTRETCHBLT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRSTRETCHBLT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSTRETCHBLT")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("xDest", &self.xDest)
            .field("yDest", &self.yDest)
            .field("cxDest", &self.cxDest)
            .field("cyDest", &self.cyDest)
            .field("dwRop", &self.dwRop)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("xformSrc", &self.xformSrc)
            .field("crBkColorSrc", &self.crBkColorSrc)
            .field("iUsageSrc", &self.iUsageSrc)
            .field("offBmiSrc", &self.offBmiSrc)
            .field("cbBmiSrc", &self.cbBmiSrc)
            .field("offBitsSrc", &self.offBitsSrc)
            .field("cbBitsSrc", &self.cbBitsSrc)
            .field("cxSrc", &self.cxSrc)
            .field("cySrc", &self.cySrc)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRSTRETCHBLT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRSTRETCHBLT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSTRETCHBLT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRSTRETCHBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRSTRETCHBLT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSTRETCHDIBITS {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub cxSrc: i32,
    pub cySrc: i32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub iUsageSrc: u32,
    pub dwRop: u32,
    pub cxDest: i32,
    pub cyDest: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRSTRETCHDIBITS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRSTRETCHDIBITS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRSTRETCHDIBITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSTRETCHDIBITS")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("xDest", &self.xDest)
            .field("yDest", &self.yDest)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("cxSrc", &self.cxSrc)
            .field("cySrc", &self.cySrc)
            .field("offBmiSrc", &self.offBmiSrc)
            .field("cbBmiSrc", &self.cbBmiSrc)
            .field("offBitsSrc", &self.offBitsSrc)
            .field("cbBitsSrc", &self.cbBitsSrc)
            .field("iUsageSrc", &self.iUsageSrc)
            .field("dwRop", &self.dwRop)
            .field("cxDest", &self.cxDest)
            .field("cyDest", &self.cyDest)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRSTRETCHDIBITS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRSTRETCHDIBITS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRSTRETCHDIBITS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRSTRETCHDIBITS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRSTRETCHDIBITS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRTEXT {
    pub ptlReference: super::super::Foundation::POINTL,
    pub nChars: u32,
    pub offString: u32,
    pub fOptions: u32,
    pub rcl: super::super::Foundation::RECTL,
    pub offDx: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRTEXT").field("ptlReference", &self.ptlReference).field("nChars", &self.nChars).field("offString", &self.offString).field("fOptions", &self.fOptions).field("rcl", &self.rcl).field("offDx", &self.offDx).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRTEXT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRTEXT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRTRANSPARENTBLT {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::super::Foundation::COLORREF,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub cxSrc: i32,
    pub cySrc: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRTRANSPARENTBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRTRANSPARENTBLT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRTRANSPARENTBLT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRTRANSPARENTBLT")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("xDest", &self.xDest)
            .field("yDest", &self.yDest)
            .field("cxDest", &self.cxDest)
            .field("cyDest", &self.cyDest)
            .field("dwRop", &self.dwRop)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("xformSrc", &self.xformSrc)
            .field("crBkColorSrc", &self.crBkColorSrc)
            .field("iUsageSrc", &self.iUsageSrc)
            .field("offBmiSrc", &self.offBmiSrc)
            .field("cbBmiSrc", &self.cbBmiSrc)
            .field("offBitsSrc", &self.offBitsSrc)
            .field("cbBitsSrc", &self.cbBitsSrc)
            .field("cxSrc", &self.cxSrc)
            .field("cySrc", &self.cySrc)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EMRTRANSPARENTBLT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRTRANSPARENTBLT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRTRANSPARENTBLT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRTRANSPARENTBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRTRANSPARENTBLT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENHMETAHEADER {
    pub iType: u32,
    pub nSize: u32,
    pub rclBounds: super::super::Foundation::RECTL,
    pub rclFrame: super::super::Foundation::RECTL,
    pub dSignature: u32,
    pub nVersion: u32,
    pub nBytes: u32,
    pub nRecords: u32,
    pub nHandles: u16,
    pub sReserved: u16,
    pub nDescription: u32,
    pub offDescription: u32,
    pub nPalEntries: u32,
    pub szlDevice: super::super::Foundation::SIZE,
    pub szlMillimeters: super::super::Foundation::SIZE,
    pub cbPixelFormat: u32,
    pub offPixelFormat: u32,
    pub bOpenGL: u32,
    pub szlMicrometers: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENHMETAHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENHMETAHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ENHMETAHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENHMETAHEADER")
            .field("iType", &self.iType)
            .field("nSize", &self.nSize)
            .field("rclBounds", &self.rclBounds)
            .field("rclFrame", &self.rclFrame)
            .field("dSignature", &self.dSignature)
            .field("nVersion", &self.nVersion)
            .field("nBytes", &self.nBytes)
            .field("nRecords", &self.nRecords)
            .field("nHandles", &self.nHandles)
            .field("sReserved", &self.sReserved)
            .field("nDescription", &self.nDescription)
            .field("offDescription", &self.offDescription)
            .field("nPalEntries", &self.nPalEntries)
            .field("szlDevice", &self.szlDevice)
            .field("szlMillimeters", &self.szlMillimeters)
            .field("cbPixelFormat", &self.cbPixelFormat)
            .field("offPixelFormat", &self.offPixelFormat)
            .field("bOpenGL", &self.bOpenGL)
            .field("szlMicrometers", &self.szlMicrometers)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ENHMETAHEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENHMETAHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENHMETAHEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENHMETAHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENHMETAHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct ENHMETARECORD {
    pub iType: ENHANCED_METAFILE_RECORD_TYPE,
    pub nSize: u32,
    pub dParm: [u32; 1],
}
impl ::core::marker::Copy for ENHMETARECORD {}
impl ::core::clone::Clone for ENHMETARECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENHMETARECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENHMETARECORD").field("iType", &self.iType).field("nSize", &self.nSize).field("dParm", &self.dParm).finish()
    }
}
unsafe impl ::windows::core::Abi for ENHMETARECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENHMETARECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENHMETARECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENHMETARECORD {}
impl ::core::default::Default for ENHMETARECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENUMLOGFONTA {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENUMLOGFONTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENUMLOGFONTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ENUMLOGFONTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMLOGFONTA").field("elfLogFont", &self.elfLogFont).field("elfFullName", &self.elfFullName).field("elfStyle", &self.elfStyle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ENUMLOGFONTA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENUMLOGFONTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUMLOGFONTA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENUMLOGFONTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENUMLOGFONTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENUMLOGFONTEXA {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
    pub elfScript: [u8; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENUMLOGFONTEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENUMLOGFONTEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ENUMLOGFONTEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMLOGFONTEXA").field("elfLogFont", &self.elfLogFont).field("elfFullName", &self.elfFullName).field("elfStyle", &self.elfStyle).field("elfScript", &self.elfScript).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ENUMLOGFONTEXA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENUMLOGFONTEXA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUMLOGFONTEXA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENUMLOGFONTEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENUMLOGFONTEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENUMLOGFONTEXDVA {
    pub elfEnumLogfontEx: ENUMLOGFONTEXA,
    pub elfDesignVector: DESIGNVECTOR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENUMLOGFONTEXDVA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENUMLOGFONTEXDVA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ENUMLOGFONTEXDVA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMLOGFONTEXDVA").field("elfEnumLogfontEx", &self.elfEnumLogfontEx).field("elfDesignVector", &self.elfDesignVector).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ENUMLOGFONTEXDVA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENUMLOGFONTEXDVA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUMLOGFONTEXDVA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENUMLOGFONTEXDVA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENUMLOGFONTEXDVA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct ENUMLOGFONTEXDVW {
    pub elfEnumLogfontEx: ENUMLOGFONTEXW,
    pub elfDesignVector: DESIGNVECTOR,
}
impl ::core::marker::Copy for ENUMLOGFONTEXDVW {}
impl ::core::clone::Clone for ENUMLOGFONTEXDVW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUMLOGFONTEXDVW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMLOGFONTEXDVW").field("elfEnumLogfontEx", &self.elfEnumLogfontEx).field("elfDesignVector", &self.elfDesignVector).finish()
    }
}
unsafe impl ::windows::core::Abi for ENUMLOGFONTEXDVW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENUMLOGFONTEXDVW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUMLOGFONTEXDVW>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENUMLOGFONTEXDVW {}
impl ::core::default::Default for ENUMLOGFONTEXDVW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct ENUMLOGFONTEXW {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [u16; 64],
    pub elfStyle: [u16; 32],
    pub elfScript: [u16; 32],
}
impl ::core::marker::Copy for ENUMLOGFONTEXW {}
impl ::core::clone::Clone for ENUMLOGFONTEXW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUMLOGFONTEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMLOGFONTEXW").field("elfLogFont", &self.elfLogFont).field("elfFullName", &self.elfFullName).field("elfStyle", &self.elfStyle).field("elfScript", &self.elfScript).finish()
    }
}
unsafe impl ::windows::core::Abi for ENUMLOGFONTEXW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENUMLOGFONTEXW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUMLOGFONTEXW>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENUMLOGFONTEXW {}
impl ::core::default::Default for ENUMLOGFONTEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct ENUMLOGFONTW {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [u16; 64],
    pub elfStyle: [u16; 32],
}
impl ::core::marker::Copy for ENUMLOGFONTW {}
impl ::core::clone::Clone for ENUMLOGFONTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUMLOGFONTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMLOGFONTW").field("elfLogFont", &self.elfLogFont).field("elfFullName", &self.elfFullName).field("elfStyle", &self.elfStyle).finish()
    }
}
unsafe impl ::windows::core::Abi for ENUMLOGFONTW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENUMLOGFONTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUMLOGFONTW>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENUMLOGFONTW {}
impl ::core::default::Default for ENUMLOGFONTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EXTLOGFONTA {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
    pub elfVersion: u32,
    pub elfStyleSize: u32,
    pub elfMatch: u32,
    pub elfReserved: u32,
    pub elfVendorId: [u8; 4],
    pub elfCulture: u32,
    pub elfPanose: PANOSE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EXTLOGFONTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EXTLOGFONTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXTLOGFONTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTLOGFONTA").field("elfLogFont", &self.elfLogFont).field("elfFullName", &self.elfFullName).field("elfStyle", &self.elfStyle).field("elfVersion", &self.elfVersion).field("elfStyleSize", &self.elfStyleSize).field("elfMatch", &self.elfMatch).field("elfReserved", &self.elfReserved).field("elfVendorId", &self.elfVendorId).field("elfCulture", &self.elfCulture).field("elfPanose", &self.elfPanose).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EXTLOGFONTA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXTLOGFONTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EXTLOGFONTA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXTLOGFONTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXTLOGFONTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EXTLOGFONTW {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [u16; 64],
    pub elfStyle: [u16; 32],
    pub elfVersion: u32,
    pub elfStyleSize: u32,
    pub elfMatch: u32,
    pub elfReserved: u32,
    pub elfVendorId: [u8; 4],
    pub elfCulture: u32,
    pub elfPanose: PANOSE,
}
impl ::core::marker::Copy for EXTLOGFONTW {}
impl ::core::clone::Clone for EXTLOGFONTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EXTLOGFONTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTLOGFONTW").field("elfLogFont", &self.elfLogFont).field("elfFullName", &self.elfFullName).field("elfStyle", &self.elfStyle).field("elfVersion", &self.elfVersion).field("elfStyleSize", &self.elfStyleSize).field("elfMatch", &self.elfMatch).field("elfReserved", &self.elfReserved).field("elfVendorId", &self.elfVendorId).field("elfCulture", &self.elfCulture).field("elfPanose", &self.elfPanose).finish()
    }
}
unsafe impl ::windows::core::Abi for EXTLOGFONTW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EXTLOGFONTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EXTLOGFONTW>()) == 0 }
    }
}
impl ::core::cmp::Eq for EXTLOGFONTW {}
impl ::core::default::Default for EXTLOGFONTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EXTLOGPEN {
    pub elpPenStyle: PEN_STYLE,
    pub elpWidth: u32,
    pub elpBrushStyle: u32,
    pub elpColor: super::super::Foundation::COLORREF,
    pub elpHatch: usize,
    pub elpNumEntries: u32,
    pub elpStyleEntry: [u32; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EXTLOGPEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EXTLOGPEN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXTLOGPEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTLOGPEN").field("elpPenStyle", &self.elpPenStyle).field("elpWidth", &self.elpWidth).field("elpBrushStyle", &self.elpBrushStyle).field("elpColor", &self.elpColor).field("elpHatch", &self.elpHatch).field("elpNumEntries", &self.elpNumEntries).field("elpStyleEntry", &self.elpStyleEntry).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EXTLOGPEN {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXTLOGPEN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EXTLOGPEN>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXTLOGPEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXTLOGPEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EXTLOGPEN32 {
    pub elpPenStyle: PEN_STYLE,
    pub elpWidth: u32,
    pub elpBrushStyle: u32,
    pub elpColor: super::super::Foundation::COLORREF,
    pub elpHatch: u32,
    pub elpNumEntries: u32,
    pub elpStyleEntry: [u32; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EXTLOGPEN32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EXTLOGPEN32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXTLOGPEN32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTLOGPEN32").field("elpPenStyle", &self.elpPenStyle).field("elpWidth", &self.elpWidth).field("elpBrushStyle", &self.elpBrushStyle).field("elpColor", &self.elpColor).field("elpHatch", &self.elpHatch).field("elpNumEntries", &self.elpNumEntries).field("elpStyleEntry", &self.elpStyleEntry).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EXTLOGPEN32 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXTLOGPEN32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EXTLOGPEN32>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXTLOGPEN32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXTLOGPEN32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct FIXED {
    pub fract: u16,
    pub value: i16,
}
impl ::core::marker::Copy for FIXED {}
impl ::core::clone::Clone for FIXED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FIXED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FIXED").field("fract", &self.fract).field("value", &self.value).finish()
    }
}
unsafe impl ::windows::core::Abi for FIXED {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FIXED {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FIXED>()) == 0 }
    }
}
impl ::core::cmp::Eq for FIXED {}
impl ::core::default::Default for FIXED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct GCP_RESULTSA {
    pub lStructSize: u32,
    pub lpOutString: ::windows::core::PSTR,
    pub lpOrder: *mut u32,
    pub lpDx: *mut i32,
    pub lpCaretPos: *mut i32,
    pub lpClass: ::windows::core::PSTR,
    pub lpGlyphs: ::windows::core::PWSTR,
    pub nGlyphs: u32,
    pub nMaxFit: i32,
}
impl ::core::marker::Copy for GCP_RESULTSA {}
impl ::core::clone::Clone for GCP_RESULTSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GCP_RESULTSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GCP_RESULTSA").field("lStructSize", &self.lStructSize).field("lpOutString", &self.lpOutString).field("lpOrder", &self.lpOrder).field("lpDx", &self.lpDx).field("lpCaretPos", &self.lpCaretPos).field("lpClass", &self.lpClass).field("lpGlyphs", &self.lpGlyphs).field("nGlyphs", &self.nGlyphs).field("nMaxFit", &self.nMaxFit).finish()
    }
}
unsafe impl ::windows::core::Abi for GCP_RESULTSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GCP_RESULTSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GCP_RESULTSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for GCP_RESULTSA {}
impl ::core::default::Default for GCP_RESULTSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct GCP_RESULTSW {
    pub lStructSize: u32,
    pub lpOutString: ::windows::core::PWSTR,
    pub lpOrder: *mut u32,
    pub lpDx: *mut i32,
    pub lpCaretPos: *mut i32,
    pub lpClass: ::windows::core::PSTR,
    pub lpGlyphs: ::windows::core::PWSTR,
    pub nGlyphs: u32,
    pub nMaxFit: i32,
}
impl ::core::marker::Copy for GCP_RESULTSW {}
impl ::core::clone::Clone for GCP_RESULTSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GCP_RESULTSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GCP_RESULTSW").field("lStructSize", &self.lStructSize).field("lpOutString", &self.lpOutString).field("lpOrder", &self.lpOrder).field("lpDx", &self.lpDx).field("lpCaretPos", &self.lpCaretPos).field("lpClass", &self.lpClass).field("lpGlyphs", &self.lpGlyphs).field("nGlyphs", &self.nGlyphs).field("nMaxFit", &self.nMaxFit).finish()
    }
}
unsafe impl ::windows::core::Abi for GCP_RESULTSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GCP_RESULTSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GCP_RESULTSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for GCP_RESULTSW {}
impl ::core::default::Default for GCP_RESULTSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GLYPHMETRICS {
    pub gmBlackBoxX: u32,
    pub gmBlackBoxY: u32,
    pub gmptGlyphOrigin: super::super::Foundation::POINT,
    pub gmCellIncX: i16,
    pub gmCellIncY: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GLYPHMETRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GLYPHMETRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GLYPHMETRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GLYPHMETRICS").field("gmBlackBoxX", &self.gmBlackBoxX).field("gmBlackBoxY", &self.gmBlackBoxY).field("gmptGlyphOrigin", &self.gmptGlyphOrigin).field("gmCellIncX", &self.gmCellIncX).field("gmCellIncY", &self.gmCellIncY).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GLYPHMETRICS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GLYPHMETRICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GLYPHMETRICS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GLYPHMETRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GLYPHMETRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct GLYPHSET {
    pub cbThis: u32,
    pub flAccel: u32,
    pub cGlyphsSupported: u32,
    pub cRanges: u32,
    pub ranges: [WCRANGE; 1],
}
impl ::core::marker::Copy for GLYPHSET {}
impl ::core::clone::Clone for GLYPHSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GLYPHSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GLYPHSET").field("cbThis", &self.cbThis).field("flAccel", &self.flAccel).field("cGlyphsSupported", &self.cGlyphsSupported).field("cRanges", &self.cRanges).field("ranges", &self.ranges).finish()
    }
}
unsafe impl ::windows::core::Abi for GLYPHSET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GLYPHSET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GLYPHSET>()) == 0 }
    }
}
impl ::core::cmp::Eq for GLYPHSET {}
impl ::core::default::Default for GLYPHSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct GRADIENT_RECT {
    pub UpperLeft: u32,
    pub LowerRight: u32,
}
impl ::core::marker::Copy for GRADIENT_RECT {}
impl ::core::clone::Clone for GRADIENT_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GRADIENT_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GRADIENT_RECT").field("UpperLeft", &self.UpperLeft).field("LowerRight", &self.LowerRight).finish()
    }
}
unsafe impl ::windows::core::Abi for GRADIENT_RECT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GRADIENT_RECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GRADIENT_RECT>()) == 0 }
    }
}
impl ::core::cmp::Eq for GRADIENT_RECT {}
impl ::core::default::Default for GRADIENT_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct GRADIENT_TRIANGLE {
    pub Vertex1: u32,
    pub Vertex2: u32,
    pub Vertex3: u32,
}
impl ::core::marker::Copy for GRADIENT_TRIANGLE {}
impl ::core::clone::Clone for GRADIENT_TRIANGLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GRADIENT_TRIANGLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GRADIENT_TRIANGLE").field("Vertex1", &self.Vertex1).field("Vertex2", &self.Vertex2).field("Vertex3", &self.Vertex3).finish()
    }
}
unsafe impl ::windows::core::Abi for GRADIENT_TRIANGLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GRADIENT_TRIANGLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GRADIENT_TRIANGLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for GRADIENT_TRIANGLE {}
impl ::core::default::Default for GRADIENT_TRIANGLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct HANDLETABLE {
    pub objectHandle: [HGDIOBJ; 1],
}
impl ::core::marker::Copy for HANDLETABLE {}
impl ::core::clone::Clone for HANDLETABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HANDLETABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HANDLETABLE").field("objectHandle", &self.objectHandle).finish()
    }
}
unsafe impl ::windows::core::Abi for HANDLETABLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HANDLETABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HANDLETABLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for HANDLETABLE {}
impl ::core::default::Default for HANDLETABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HBITMAP(pub isize);
impl HBITMAP {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HBITMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HBITMAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HBITMAP {}
impl ::core::fmt::Debug for HBITMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HBITMAP").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HBITMAP>> for HBITMAP {
    fn from(optional: ::core::option::Option<HBITMAP>) -> HBITMAP {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HBITMAP {
    type Abi = Self;
}
impl ::core::convert::From<HBITMAP> for HGDIOBJ {
    fn from(item: HBITMAP) -> HGDIOBJ {
        HGDIOBJ(item.0)
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HBRUSH(pub isize);
impl HBRUSH {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HBRUSH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HBRUSH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HBRUSH {}
impl ::core::fmt::Debug for HBRUSH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HBRUSH").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HBRUSH>> for HBRUSH {
    fn from(optional: ::core::option::Option<HBRUSH>) -> HBRUSH {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HBRUSH {
    type Abi = Self;
}
impl ::core::convert::From<HBRUSH> for HGDIOBJ {
    fn from(item: HBRUSH) -> HGDIOBJ {
        HGDIOBJ(item.0)
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HDC(pub isize);
impl HDC {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HDC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDC {}
impl ::core::fmt::Debug for HDC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDC").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HDC>> for HDC {
    fn from(optional: ::core::option::Option<HDC>) -> HDC {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HDC {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HENHMETAFILE(pub isize);
impl HENHMETAFILE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HENHMETAFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HENHMETAFILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HENHMETAFILE {}
impl ::core::fmt::Debug for HENHMETAFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HENHMETAFILE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HENHMETAFILE>> for HENHMETAFILE {
    fn from(optional: ::core::option::Option<HENHMETAFILE>) -> HENHMETAFILE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HENHMETAFILE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HFONT(pub isize);
impl HFONT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HFONT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HFONT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HFONT {}
impl ::core::fmt::Debug for HFONT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HFONT").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HFONT>> for HFONT {
    fn from(optional: ::core::option::Option<HFONT>) -> HFONT {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HFONT {
    type Abi = Self;
}
impl ::core::convert::From<HFONT> for HGDIOBJ {
    fn from(item: HFONT) -> HGDIOBJ {
        HGDIOBJ(item.0)
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HGDIOBJ(pub isize);
impl HGDIOBJ {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HGDIOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HGDIOBJ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HGDIOBJ {}
impl ::core::fmt::Debug for HGDIOBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HGDIOBJ").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HGDIOBJ>> for HGDIOBJ {
    fn from(optional: ::core::option::Option<HGDIOBJ>) -> HGDIOBJ {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HGDIOBJ {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HMETAFILE(pub isize);
impl HMETAFILE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HMETAFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMETAFILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMETAFILE {}
impl ::core::fmt::Debug for HMETAFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMETAFILE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HMETAFILE>> for HMETAFILE {
    fn from(optional: ::core::option::Option<HMETAFILE>) -> HMETAFILE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HMETAFILE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HMONITOR(pub isize);
impl HMONITOR {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HMONITOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMONITOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMONITOR {}
impl ::core::fmt::Debug for HMONITOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMONITOR").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HMONITOR>> for HMONITOR {
    fn from(optional: ::core::option::Option<HMONITOR>) -> HMONITOR {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HMONITOR {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HPALETTE(pub isize);
impl HPALETTE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HPALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HPALETTE {}
impl ::core::fmt::Debug for HPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HPALETTE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HPALETTE>> for HPALETTE {
    fn from(optional: ::core::option::Option<HPALETTE>) -> HPALETTE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HPALETTE {
    type Abi = Self;
}
impl ::core::convert::From<HPALETTE> for HGDIOBJ {
    fn from(item: HPALETTE) -> HGDIOBJ {
        HGDIOBJ(item.0)
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HPEN(pub isize);
impl HPEN {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HPEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HPEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HPEN {}
impl ::core::fmt::Debug for HPEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HPEN").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HPEN>> for HPEN {
    fn from(optional: ::core::option::Option<HPEN>) -> HPEN {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HPEN {
    type Abi = Self;
}
impl ::core::convert::From<HPEN> for HGDIOBJ {
    fn from(item: HPEN) -> HGDIOBJ {
        HGDIOBJ(item.0)
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HRGN(pub isize);
impl HRGN {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HRGN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HRGN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HRGN {}
impl ::core::fmt::Debug for HRGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HRGN").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HRGN>> for HRGN {
    fn from(optional: ::core::option::Option<HRGN>) -> HRGN {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HRGN {
    type Abi = Self;
}
impl ::core::convert::From<HRGN> for HGDIOBJ {
    fn from(item: HRGN) -> HGDIOBJ {
        HGDIOBJ(item.0)
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HdcMetdataEnhFileHandle(pub isize);
impl HdcMetdataEnhFileHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HdcMetdataEnhFileHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HdcMetdataEnhFileHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HdcMetdataEnhFileHandle {}
impl ::core::fmt::Debug for HdcMetdataEnhFileHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdcMetdataEnhFileHandle").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HdcMetdataEnhFileHandle>> for HdcMetdataEnhFileHandle {
    fn from(optional: ::core::option::Option<HdcMetdataEnhFileHandle>) -> HdcMetdataEnhFileHandle {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HdcMetdataEnhFileHandle {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HdcMetdataFileHandle(pub isize);
impl HdcMetdataFileHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HdcMetdataFileHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HdcMetdataFileHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HdcMetdataFileHandle {}
impl ::core::fmt::Debug for HdcMetdataFileHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdcMetdataFileHandle").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HdcMetdataFileHandle>> for HdcMetdataFileHandle {
    fn from(optional: ::core::option::Option<HdcMetdataFileHandle>) -> HdcMetdataFileHandle {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HdcMetdataFileHandle {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct KERNINGPAIR {
    pub wFirst: u16,
    pub wSecond: u16,
    pub iKernAmount: i32,
}
impl ::core::marker::Copy for KERNINGPAIR {}
impl ::core::clone::Clone for KERNINGPAIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KERNINGPAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERNINGPAIR").field("wFirst", &self.wFirst).field("wSecond", &self.wSecond).field("iKernAmount", &self.iKernAmount).finish()
    }
}
unsafe impl ::windows::core::Abi for KERNINGPAIR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KERNINGPAIR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KERNINGPAIR>()) == 0 }
    }
}
impl ::core::cmp::Eq for KERNINGPAIR {}
impl ::core::default::Default for KERNINGPAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LOGBRUSH {
    pub lbStyle: BRUSH_STYLE,
    pub lbColor: super::super::Foundation::COLORREF,
    pub lbHatch: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LOGBRUSH {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LOGBRUSH {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LOGBRUSH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGBRUSH").field("lbStyle", &self.lbStyle).field("lbColor", &self.lbColor).field("lbHatch", &self.lbHatch).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LOGBRUSH {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOGBRUSH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOGBRUSH>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOGBRUSH {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOGBRUSH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LOGBRUSH32 {
    pub lbStyle: BRUSH_STYLE,
    pub lbColor: super::super::Foundation::COLORREF,
    pub lbHatch: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LOGBRUSH32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LOGBRUSH32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LOGBRUSH32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGBRUSH32").field("lbStyle", &self.lbStyle).field("lbColor", &self.lbColor).field("lbHatch", &self.lbHatch).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LOGBRUSH32 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOGBRUSH32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOGBRUSH32>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOGBRUSH32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOGBRUSH32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LOGFONTA {
    pub lfHeight: i32,
    pub lfWidth: i32,
    pub lfEscapement: i32,
    pub lfOrientation: i32,
    pub lfWeight: FONT_WEIGHT,
    pub lfItalic: u8,
    pub lfUnderline: u8,
    pub lfStrikeOut: u8,
    pub lfCharSet: u8,
    pub lfOutPrecision: FONT_OUTPUT_PRECISION,
    pub lfClipPrecision: FONT_CLIP_PRECISION,
    pub lfQuality: FONT_QUALITY,
    pub lfPitchAndFamily: FONT_PITCH_AND_FAMILY,
    pub lfFaceName: [super::super::Foundation::CHAR; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LOGFONTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LOGFONTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LOGFONTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGFONTA")
            .field("lfHeight", &self.lfHeight)
            .field("lfWidth", &self.lfWidth)
            .field("lfEscapement", &self.lfEscapement)
            .field("lfOrientation", &self.lfOrientation)
            .field("lfWeight", &self.lfWeight)
            .field("lfItalic", &self.lfItalic)
            .field("lfUnderline", &self.lfUnderline)
            .field("lfStrikeOut", &self.lfStrikeOut)
            .field("lfCharSet", &self.lfCharSet)
            .field("lfOutPrecision", &self.lfOutPrecision)
            .field("lfClipPrecision", &self.lfClipPrecision)
            .field("lfQuality", &self.lfQuality)
            .field("lfPitchAndFamily", &self.lfPitchAndFamily)
            .field("lfFaceName", &self.lfFaceName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LOGFONTA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOGFONTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOGFONTA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOGFONTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOGFONTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct LOGFONTW {
    pub lfHeight: i32,
    pub lfWidth: i32,
    pub lfEscapement: i32,
    pub lfOrientation: i32,
    pub lfWeight: FONT_WEIGHT,
    pub lfItalic: u8,
    pub lfUnderline: u8,
    pub lfStrikeOut: u8,
    pub lfCharSet: FONT_CHARSET,
    pub lfOutPrecision: FONT_OUTPUT_PRECISION,
    pub lfClipPrecision: FONT_CLIP_PRECISION,
    pub lfQuality: FONT_QUALITY,
    pub lfPitchAndFamily: FONT_PITCH_AND_FAMILY,
    pub lfFaceName: [u16; 32],
}
impl ::core::marker::Copy for LOGFONTW {}
impl ::core::clone::Clone for LOGFONTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOGFONTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGFONTW")
            .field("lfHeight", &self.lfHeight)
            .field("lfWidth", &self.lfWidth)
            .field("lfEscapement", &self.lfEscapement)
            .field("lfOrientation", &self.lfOrientation)
            .field("lfWeight", &self.lfWeight)
            .field("lfItalic", &self.lfItalic)
            .field("lfUnderline", &self.lfUnderline)
            .field("lfStrikeOut", &self.lfStrikeOut)
            .field("lfCharSet", &self.lfCharSet)
            .field("lfOutPrecision", &self.lfOutPrecision)
            .field("lfClipPrecision", &self.lfClipPrecision)
            .field("lfQuality", &self.lfQuality)
            .field("lfPitchAndFamily", &self.lfPitchAndFamily)
            .field("lfFaceName", &self.lfFaceName)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for LOGFONTW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LOGFONTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOGFONTW>()) == 0 }
    }
}
impl ::core::cmp::Eq for LOGFONTW {}
impl ::core::default::Default for LOGFONTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct LOGPALETTE {
    pub palVersion: u16,
    pub palNumEntries: u16,
    pub palPalEntry: [PALETTEENTRY; 1],
}
impl ::core::marker::Copy for LOGPALETTE {}
impl ::core::clone::Clone for LOGPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOGPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGPALETTE").field("palVersion", &self.palVersion).field("palNumEntries", &self.palNumEntries).field("palPalEntry", &self.palPalEntry).finish()
    }
}
unsafe impl ::windows::core::Abi for LOGPALETTE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LOGPALETTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOGPALETTE>()) == 0 }
    }
}
impl ::core::cmp::Eq for LOGPALETTE {}
impl ::core::default::Default for LOGPALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LOGPEN {
    pub lopnStyle: PEN_STYLE,
    pub lopnWidth: super::super::Foundation::POINT,
    pub lopnColor: super::super::Foundation::COLORREF,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LOGPEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LOGPEN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LOGPEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGPEN").field("lopnStyle", &self.lopnStyle).field("lopnWidth", &self.lopnWidth).field("lopnColor", &self.lopnColor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LOGPEN {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOGPEN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOGPEN>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOGPEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOGPEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct MAT2 {
    pub eM11: FIXED,
    pub eM12: FIXED,
    pub eM21: FIXED,
    pub eM22: FIXED,
}
impl ::core::marker::Copy for MAT2 {}
impl ::core::clone::Clone for MAT2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MAT2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAT2").field("eM11", &self.eM11).field("eM12", &self.eM12).field("eM21", &self.eM21).field("eM22", &self.eM22).finish()
    }
}
unsafe impl ::windows::core::Abi for MAT2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MAT2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MAT2>()) == 0 }
    }
}
impl ::core::cmp::Eq for MAT2 {}
impl ::core::default::Default for MAT2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct METAHEADER {
    pub mtType: u16,
    pub mtHeaderSize: u16,
    pub mtVersion: u16,
    pub mtSize: u32,
    pub mtNoObjects: u16,
    pub mtMaxRecord: u32,
    pub mtNoParameters: u16,
}
impl ::core::marker::Copy for METAHEADER {}
impl ::core::clone::Clone for METAHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for METAHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for METAHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<METAHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for METAHEADER {}
impl ::core::default::Default for METAHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct METARECORD {
    pub rdSize: u32,
    pub rdFunction: u16,
    pub rdParm: [u16; 1],
}
impl ::core::marker::Copy for METARECORD {}
impl ::core::clone::Clone for METARECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for METARECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("METARECORD").field("rdSize", &self.rdSize).field("rdFunction", &self.rdFunction).field("rdParm", &self.rdParm).finish()
    }
}
unsafe impl ::windows::core::Abi for METARECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for METARECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<METARECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for METARECORD {}
impl ::core::default::Default for METARECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MONITORINFO {
    pub cbSize: u32,
    pub rcMonitor: super::super::Foundation::RECT,
    pub rcWork: super::super::Foundation::RECT,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MONITORINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MONITORINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONITORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITORINFO").field("cbSize", &self.cbSize).field("rcMonitor", &self.rcMonitor).field("rcWork", &self.rcWork).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MONITORINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONITORINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MONITORINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONITORINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONITORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MONITORINFOEXA {
    pub monitorInfo: MONITORINFO,
    pub szDevice: [super::super::Foundation::CHAR; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MONITORINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MONITORINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONITORINFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITORINFOEXA").field("monitorInfo", &self.monitorInfo).field("szDevice", &self.szDevice).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MONITORINFOEXA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONITORINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MONITORINFOEXA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONITORINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONITORINFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MONITORINFOEXW {
    pub monitorInfo: MONITORINFO,
    pub szDevice: [u16; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MONITORINFOEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MONITORINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONITORINFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITORINFOEXW").field("monitorInfo", &self.monitorInfo).field("szDevice", &self.szDevice).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MONITORINFOEXW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONITORINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MONITORINFOEXW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONITORINFOEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONITORINFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct NEWTEXTMETRICA {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u8,
    pub tmLastChar: u8,
    pub tmDefaultChar: u8,
    pub tmBreakChar: u8,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: TMPF_FLAGS,
    pub tmCharSet: u8,
    pub ntmFlags: u32,
    pub ntmSizeEM: u32,
    pub ntmCellHeight: u32,
    pub ntmAvgWidth: u32,
}
impl ::core::marker::Copy for NEWTEXTMETRICA {}
impl ::core::clone::Clone for NEWTEXTMETRICA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NEWTEXTMETRICA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NEWTEXTMETRICA")
            .field("tmHeight", &self.tmHeight)
            .field("tmAscent", &self.tmAscent)
            .field("tmDescent", &self.tmDescent)
            .field("tmInternalLeading", &self.tmInternalLeading)
            .field("tmExternalLeading", &self.tmExternalLeading)
            .field("tmAveCharWidth", &self.tmAveCharWidth)
            .field("tmMaxCharWidth", &self.tmMaxCharWidth)
            .field("tmWeight", &self.tmWeight)
            .field("tmOverhang", &self.tmOverhang)
            .field("tmDigitizedAspectX", &self.tmDigitizedAspectX)
            .field("tmDigitizedAspectY", &self.tmDigitizedAspectY)
            .field("tmFirstChar", &self.tmFirstChar)
            .field("tmLastChar", &self.tmLastChar)
            .field("tmDefaultChar", &self.tmDefaultChar)
            .field("tmBreakChar", &self.tmBreakChar)
            .field("tmItalic", &self.tmItalic)
            .field("tmUnderlined", &self.tmUnderlined)
            .field("tmStruckOut", &self.tmStruckOut)
            .field("tmPitchAndFamily", &self.tmPitchAndFamily)
            .field("tmCharSet", &self.tmCharSet)
            .field("ntmFlags", &self.ntmFlags)
            .field("ntmSizeEM", &self.ntmSizeEM)
            .field("ntmCellHeight", &self.ntmCellHeight)
            .field("ntmAvgWidth", &self.ntmAvgWidth)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for NEWTEXTMETRICA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NEWTEXTMETRICA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NEWTEXTMETRICA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NEWTEXTMETRICA {}
impl ::core::default::Default for NEWTEXTMETRICA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct NEWTEXTMETRICW {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u16,
    pub tmLastChar: u16,
    pub tmDefaultChar: u16,
    pub tmBreakChar: u16,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: TMPF_FLAGS,
    pub tmCharSet: u8,
    pub ntmFlags: u32,
    pub ntmSizeEM: u32,
    pub ntmCellHeight: u32,
    pub ntmAvgWidth: u32,
}
impl ::core::marker::Copy for NEWTEXTMETRICW {}
impl ::core::clone::Clone for NEWTEXTMETRICW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NEWTEXTMETRICW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NEWTEXTMETRICW")
            .field("tmHeight", &self.tmHeight)
            .field("tmAscent", &self.tmAscent)
            .field("tmDescent", &self.tmDescent)
            .field("tmInternalLeading", &self.tmInternalLeading)
            .field("tmExternalLeading", &self.tmExternalLeading)
            .field("tmAveCharWidth", &self.tmAveCharWidth)
            .field("tmMaxCharWidth", &self.tmMaxCharWidth)
            .field("tmWeight", &self.tmWeight)
            .field("tmOverhang", &self.tmOverhang)
            .field("tmDigitizedAspectX", &self.tmDigitizedAspectX)
            .field("tmDigitizedAspectY", &self.tmDigitizedAspectY)
            .field("tmFirstChar", &self.tmFirstChar)
            .field("tmLastChar", &self.tmLastChar)
            .field("tmDefaultChar", &self.tmDefaultChar)
            .field("tmBreakChar", &self.tmBreakChar)
            .field("tmItalic", &self.tmItalic)
            .field("tmUnderlined", &self.tmUnderlined)
            .field("tmStruckOut", &self.tmStruckOut)
            .field("tmPitchAndFamily", &self.tmPitchAndFamily)
            .field("tmCharSet", &self.tmCharSet)
            .field("ntmFlags", &self.ntmFlags)
            .field("ntmSizeEM", &self.ntmSizeEM)
            .field("ntmCellHeight", &self.ntmCellHeight)
            .field("ntmAvgWidth", &self.ntmAvgWidth)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for NEWTEXTMETRICW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NEWTEXTMETRICW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NEWTEXTMETRICW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NEWTEXTMETRICW {}
impl ::core::default::Default for NEWTEXTMETRICW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OUTLINETEXTMETRICA {
    pub otmSize: u32,
    pub otmTextMetrics: TEXTMETRICA,
    pub otmFiller: u8,
    pub otmPanoseNumber: PANOSE,
    pub otmfsSelection: u32,
    pub otmfsType: u32,
    pub otmsCharSlopeRise: i32,
    pub otmsCharSlopeRun: i32,
    pub otmItalicAngle: i32,
    pub otmEMSquare: u32,
    pub otmAscent: i32,
    pub otmDescent: i32,
    pub otmLineGap: u32,
    pub otmsCapEmHeight: u32,
    pub otmsXHeight: u32,
    pub otmrcFontBox: super::super::Foundation::RECT,
    pub otmMacAscent: i32,
    pub otmMacDescent: i32,
    pub otmMacLineGap: u32,
    pub otmusMinimumPPEM: u32,
    pub otmptSubscriptSize: super::super::Foundation::POINT,
    pub otmptSubscriptOffset: super::super::Foundation::POINT,
    pub otmptSuperscriptSize: super::super::Foundation::POINT,
    pub otmptSuperscriptOffset: super::super::Foundation::POINT,
    pub otmsStrikeoutSize: u32,
    pub otmsStrikeoutPosition: i32,
    pub otmsUnderscoreSize: i32,
    pub otmsUnderscorePosition: i32,
    pub otmpFamilyName: ::windows::core::PSTR,
    pub otmpFaceName: ::windows::core::PSTR,
    pub otmpStyleName: ::windows::core::PSTR,
    pub otmpFullName: ::windows::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OUTLINETEXTMETRICA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OUTLINETEXTMETRICA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OUTLINETEXTMETRICA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OUTLINETEXTMETRICA")
            .field("otmSize", &self.otmSize)
            .field("otmTextMetrics", &self.otmTextMetrics)
            .field("otmFiller", &self.otmFiller)
            .field("otmPanoseNumber", &self.otmPanoseNumber)
            .field("otmfsSelection", &self.otmfsSelection)
            .field("otmfsType", &self.otmfsType)
            .field("otmsCharSlopeRise", &self.otmsCharSlopeRise)
            .field("otmsCharSlopeRun", &self.otmsCharSlopeRun)
            .field("otmItalicAngle", &self.otmItalicAngle)
            .field("otmEMSquare", &self.otmEMSquare)
            .field("otmAscent", &self.otmAscent)
            .field("otmDescent", &self.otmDescent)
            .field("otmLineGap", &self.otmLineGap)
            .field("otmsCapEmHeight", &self.otmsCapEmHeight)
            .field("otmsXHeight", &self.otmsXHeight)
            .field("otmrcFontBox", &self.otmrcFontBox)
            .field("otmMacAscent", &self.otmMacAscent)
            .field("otmMacDescent", &self.otmMacDescent)
            .field("otmMacLineGap", &self.otmMacLineGap)
            .field("otmusMinimumPPEM", &self.otmusMinimumPPEM)
            .field("otmptSubscriptSize", &self.otmptSubscriptSize)
            .field("otmptSubscriptOffset", &self.otmptSubscriptOffset)
            .field("otmptSuperscriptSize", &self.otmptSuperscriptSize)
            .field("otmptSuperscriptOffset", &self.otmptSuperscriptOffset)
            .field("otmsStrikeoutSize", &self.otmsStrikeoutSize)
            .field("otmsStrikeoutPosition", &self.otmsStrikeoutPosition)
            .field("otmsUnderscoreSize", &self.otmsUnderscoreSize)
            .field("otmsUnderscorePosition", &self.otmsUnderscorePosition)
            .field("otmpFamilyName", &self.otmpFamilyName)
            .field("otmpFaceName", &self.otmpFaceName)
            .field("otmpStyleName", &self.otmpStyleName)
            .field("otmpFullName", &self.otmpFullName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OUTLINETEXTMETRICA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OUTLINETEXTMETRICA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OUTLINETEXTMETRICA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OUTLINETEXTMETRICA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OUTLINETEXTMETRICA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OUTLINETEXTMETRICW {
    pub otmSize: u32,
    pub otmTextMetrics: TEXTMETRICW,
    pub otmFiller: u8,
    pub otmPanoseNumber: PANOSE,
    pub otmfsSelection: u32,
    pub otmfsType: u32,
    pub otmsCharSlopeRise: i32,
    pub otmsCharSlopeRun: i32,
    pub otmItalicAngle: i32,
    pub otmEMSquare: u32,
    pub otmAscent: i32,
    pub otmDescent: i32,
    pub otmLineGap: u32,
    pub otmsCapEmHeight: u32,
    pub otmsXHeight: u32,
    pub otmrcFontBox: super::super::Foundation::RECT,
    pub otmMacAscent: i32,
    pub otmMacDescent: i32,
    pub otmMacLineGap: u32,
    pub otmusMinimumPPEM: u32,
    pub otmptSubscriptSize: super::super::Foundation::POINT,
    pub otmptSubscriptOffset: super::super::Foundation::POINT,
    pub otmptSuperscriptSize: super::super::Foundation::POINT,
    pub otmptSuperscriptOffset: super::super::Foundation::POINT,
    pub otmsStrikeoutSize: u32,
    pub otmsStrikeoutPosition: i32,
    pub otmsUnderscoreSize: i32,
    pub otmsUnderscorePosition: i32,
    pub otmpFamilyName: ::windows::core::PSTR,
    pub otmpFaceName: ::windows::core::PSTR,
    pub otmpStyleName: ::windows::core::PSTR,
    pub otmpFullName: ::windows::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OUTLINETEXTMETRICW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OUTLINETEXTMETRICW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OUTLINETEXTMETRICW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OUTLINETEXTMETRICW")
            .field("otmSize", &self.otmSize)
            .field("otmTextMetrics", &self.otmTextMetrics)
            .field("otmFiller", &self.otmFiller)
            .field("otmPanoseNumber", &self.otmPanoseNumber)
            .field("otmfsSelection", &self.otmfsSelection)
            .field("otmfsType", &self.otmfsType)
            .field("otmsCharSlopeRise", &self.otmsCharSlopeRise)
            .field("otmsCharSlopeRun", &self.otmsCharSlopeRun)
            .field("otmItalicAngle", &self.otmItalicAngle)
            .field("otmEMSquare", &self.otmEMSquare)
            .field("otmAscent", &self.otmAscent)
            .field("otmDescent", &self.otmDescent)
            .field("otmLineGap", &self.otmLineGap)
            .field("otmsCapEmHeight", &self.otmsCapEmHeight)
            .field("otmsXHeight", &self.otmsXHeight)
            .field("otmrcFontBox", &self.otmrcFontBox)
            .field("otmMacAscent", &self.otmMacAscent)
            .field("otmMacDescent", &self.otmMacDescent)
            .field("otmMacLineGap", &self.otmMacLineGap)
            .field("otmusMinimumPPEM", &self.otmusMinimumPPEM)
            .field("otmptSubscriptSize", &self.otmptSubscriptSize)
            .field("otmptSubscriptOffset", &self.otmptSubscriptOffset)
            .field("otmptSuperscriptSize", &self.otmptSuperscriptSize)
            .field("otmptSuperscriptOffset", &self.otmptSuperscriptOffset)
            .field("otmsStrikeoutSize", &self.otmsStrikeoutSize)
            .field("otmsStrikeoutPosition", &self.otmsStrikeoutPosition)
            .field("otmsUnderscoreSize", &self.otmsUnderscoreSize)
            .field("otmsUnderscorePosition", &self.otmsUnderscorePosition)
            .field("otmpFamilyName", &self.otmpFamilyName)
            .field("otmpFaceName", &self.otmpFaceName)
            .field("otmpStyleName", &self.otmpStyleName)
            .field("otmpFullName", &self.otmpFullName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OUTLINETEXTMETRICW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OUTLINETEXTMETRICW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OUTLINETEXTMETRICW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OUTLINETEXTMETRICW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OUTLINETEXTMETRICW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PAINTSTRUCT {
    pub hdc: HDC,
    pub fErase: super::super::Foundation::BOOL,
    pub rcPaint: super::super::Foundation::RECT,
    pub fRestore: super::super::Foundation::BOOL,
    pub fIncUpdate: super::super::Foundation::BOOL,
    pub rgbReserved: [u8; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PAINTSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PAINTSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PAINTSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PAINTSTRUCT").field("hdc", &self.hdc).field("fErase", &self.fErase).field("rcPaint", &self.rcPaint).field("fRestore", &self.fRestore).field("fIncUpdate", &self.fIncUpdate).field("rgbReserved", &self.rgbReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PAINTSTRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PAINTSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PAINTSTRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PAINTSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PAINTSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct PALETTEENTRY {
    pub peRed: u8,
    pub peGreen: u8,
    pub peBlue: u8,
    pub peFlags: u8,
}
impl ::core::marker::Copy for PALETTEENTRY {}
impl ::core::clone::Clone for PALETTEENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PALETTEENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PALETTEENTRY").field("peRed", &self.peRed).field("peGreen", &self.peGreen).field("peBlue", &self.peBlue).field("peFlags", &self.peFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for PALETTEENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PALETTEENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PALETTEENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for PALETTEENTRY {}
impl ::core::default::Default for PALETTEENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct PANOSE {
    pub bFamilyType: u8,
    pub bSerifStyle: u8,
    pub bWeight: u8,
    pub bProportion: u8,
    pub bContrast: u8,
    pub bStrokeVariation: u8,
    pub bArmStyle: u8,
    pub bLetterform: u8,
    pub bMidline: u8,
    pub bXHeight: u8,
}
impl ::core::marker::Copy for PANOSE {}
impl ::core::clone::Clone for PANOSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PANOSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PANOSE").field("bFamilyType", &self.bFamilyType).field("bSerifStyle", &self.bSerifStyle).field("bWeight", &self.bWeight).field("bProportion", &self.bProportion).field("bContrast", &self.bContrast).field("bStrokeVariation", &self.bStrokeVariation).field("bArmStyle", &self.bArmStyle).field("bLetterform", &self.bLetterform).field("bMidline", &self.bMidline).field("bXHeight", &self.bXHeight).finish()
    }
}
unsafe impl ::windows::core::Abi for PANOSE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PANOSE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PANOSE>()) == 0 }
    }
}
impl ::core::cmp::Eq for PANOSE {}
impl ::core::default::Default for PANOSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct PELARRAY {
    pub paXCount: i32,
    pub paYCount: i32,
    pub paXExt: i32,
    pub paYExt: i32,
    pub paRGBs: u8,
}
impl ::core::marker::Copy for PELARRAY {}
impl ::core::clone::Clone for PELARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PELARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PELARRAY").field("paXCount", &self.paXCount).field("paYCount", &self.paYCount).field("paXExt", &self.paXExt).field("paYExt", &self.paYExt).field("paRGBs", &self.paRGBs).finish()
    }
}
unsafe impl ::windows::core::Abi for PELARRAY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PELARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PELARRAY>()) == 0 }
    }
}
impl ::core::cmp::Eq for PELARRAY {}
impl ::core::default::Default for PELARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct POINTFX {
    pub x: FIXED,
    pub y: FIXED,
}
impl ::core::marker::Copy for POINTFX {}
impl ::core::clone::Clone for POINTFX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for POINTFX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTFX").field("x", &self.x).field("y", &self.y).finish()
    }
}
unsafe impl ::windows::core::Abi for POINTFX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for POINTFX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POINTFX>()) == 0 }
    }
}
impl ::core::cmp::Eq for POINTFX {}
impl ::core::default::Default for POINTFX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLYTEXTA {
    pub x: i32,
    pub y: i32,
    pub n: u32,
    pub lpstr: ::windows::core::PCSTR,
    pub uiFlags: u32,
    pub rcl: super::super::Foundation::RECT,
    pub pdx: *mut i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLYTEXTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLYTEXTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLYTEXTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLYTEXTA").field("x", &self.x).field("y", &self.y).field("n", &self.n).field("lpstr", &self.lpstr).field("uiFlags", &self.uiFlags).field("rcl", &self.rcl).field("pdx", &self.pdx).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for POLYTEXTA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLYTEXTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POLYTEXTA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLYTEXTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLYTEXTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLYTEXTW {
    pub x: i32,
    pub y: i32,
    pub n: u32,
    pub lpstr: ::windows::core::PCWSTR,
    pub uiFlags: u32,
    pub rcl: super::super::Foundation::RECT,
    pub pdx: *mut i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLYTEXTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLYTEXTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLYTEXTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLYTEXTW").field("x", &self.x).field("y", &self.y).field("n", &self.n).field("lpstr", &self.lpstr).field("uiFlags", &self.uiFlags).field("rcl", &self.rcl).field("pdx", &self.pdx).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for POLYTEXTW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLYTEXTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POLYTEXTW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLYTEXTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLYTEXTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct RASTERIZER_STATUS {
    pub nSize: i16,
    pub wFlags: i16,
    pub nLanguageID: i16,
}
impl ::core::marker::Copy for RASTERIZER_STATUS {}
impl ::core::clone::Clone for RASTERIZER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RASTERIZER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASTERIZER_STATUS").field("nSize", &self.nSize).field("wFlags", &self.wFlags).field("nLanguageID", &self.nLanguageID).finish()
    }
}
unsafe impl ::windows::core::Abi for RASTERIZER_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASTERIZER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASTERIZER_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASTERIZER_STATUS {}
impl ::core::default::Default for RASTERIZER_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct RGBQUAD {
    pub rgbBlue: u8,
    pub rgbGreen: u8,
    pub rgbRed: u8,
    pub rgbReserved: u8,
}
impl ::core::marker::Copy for RGBQUAD {}
impl ::core::clone::Clone for RGBQUAD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RGBQUAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGBQUAD").field("rgbBlue", &self.rgbBlue).field("rgbGreen", &self.rgbGreen).field("rgbRed", &self.rgbRed).field("rgbReserved", &self.rgbReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for RGBQUAD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RGBQUAD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RGBQUAD>()) == 0 }
    }
}
impl ::core::cmp::Eq for RGBQUAD {}
impl ::core::default::Default for RGBQUAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct RGBTRIPLE {
    pub rgbtBlue: u8,
    pub rgbtGreen: u8,
    pub rgbtRed: u8,
}
impl ::core::marker::Copy for RGBTRIPLE {}
impl ::core::clone::Clone for RGBTRIPLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RGBTRIPLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGBTRIPLE").field("rgbtBlue", &self.rgbtBlue).field("rgbtGreen", &self.rgbtGreen).field("rgbtRed", &self.rgbtRed).finish()
    }
}
unsafe impl ::windows::core::Abi for RGBTRIPLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RGBTRIPLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RGBTRIPLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RGBTRIPLE {}
impl ::core::default::Default for RGBTRIPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RGNDATA {
    pub rdh: RGNDATAHEADER,
    pub Buffer: [super::super::Foundation::CHAR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RGNDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RGNDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RGNDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGNDATA").field("rdh", &self.rdh).field("Buffer", &self.Buffer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RGNDATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RGNDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RGNDATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RGNDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RGNDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RGNDATAHEADER {
    pub dwSize: u32,
    pub iType: u32,
    pub nCount: u32,
    pub nRgnSize: u32,
    pub rcBound: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RGNDATAHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RGNDATAHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RGNDATAHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGNDATAHEADER").field("dwSize", &self.dwSize).field("iType", &self.iType).field("nCount", &self.nCount).field("nRgnSize", &self.nRgnSize).field("rcBound", &self.rcBound).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RGNDATAHEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RGNDATAHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RGNDATAHEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RGNDATAHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RGNDATAHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct TEXTMETRICA {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u8,
    pub tmLastChar: u8,
    pub tmDefaultChar: u8,
    pub tmBreakChar: u8,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: TMPF_FLAGS,
    pub tmCharSet: u8,
}
impl ::core::marker::Copy for TEXTMETRICA {}
impl ::core::clone::Clone for TEXTMETRICA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TEXTMETRICA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TEXTMETRICA")
            .field("tmHeight", &self.tmHeight)
            .field("tmAscent", &self.tmAscent)
            .field("tmDescent", &self.tmDescent)
            .field("tmInternalLeading", &self.tmInternalLeading)
            .field("tmExternalLeading", &self.tmExternalLeading)
            .field("tmAveCharWidth", &self.tmAveCharWidth)
            .field("tmMaxCharWidth", &self.tmMaxCharWidth)
            .field("tmWeight", &self.tmWeight)
            .field("tmOverhang", &self.tmOverhang)
            .field("tmDigitizedAspectX", &self.tmDigitizedAspectX)
            .field("tmDigitizedAspectY", &self.tmDigitizedAspectY)
            .field("tmFirstChar", &self.tmFirstChar)
            .field("tmLastChar", &self.tmLastChar)
            .field("tmDefaultChar", &self.tmDefaultChar)
            .field("tmBreakChar", &self.tmBreakChar)
            .field("tmItalic", &self.tmItalic)
            .field("tmUnderlined", &self.tmUnderlined)
            .field("tmStruckOut", &self.tmStruckOut)
            .field("tmPitchAndFamily", &self.tmPitchAndFamily)
            .field("tmCharSet", &self.tmCharSet)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for TEXTMETRICA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TEXTMETRICA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TEXTMETRICA>()) == 0 }
    }
}
impl ::core::cmp::Eq for TEXTMETRICA {}
impl ::core::default::Default for TEXTMETRICA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct TEXTMETRICW {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u16,
    pub tmLastChar: u16,
    pub tmDefaultChar: u16,
    pub tmBreakChar: u16,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: TMPF_FLAGS,
    pub tmCharSet: u8,
}
impl ::core::marker::Copy for TEXTMETRICW {}
impl ::core::clone::Clone for TEXTMETRICW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TEXTMETRICW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TEXTMETRICW")
            .field("tmHeight", &self.tmHeight)
            .field("tmAscent", &self.tmAscent)
            .field("tmDescent", &self.tmDescent)
            .field("tmInternalLeading", &self.tmInternalLeading)
            .field("tmExternalLeading", &self.tmExternalLeading)
            .field("tmAveCharWidth", &self.tmAveCharWidth)
            .field("tmMaxCharWidth", &self.tmMaxCharWidth)
            .field("tmWeight", &self.tmWeight)
            .field("tmOverhang", &self.tmOverhang)
            .field("tmDigitizedAspectX", &self.tmDigitizedAspectX)
            .field("tmDigitizedAspectY", &self.tmDigitizedAspectY)
            .field("tmFirstChar", &self.tmFirstChar)
            .field("tmLastChar", &self.tmLastChar)
            .field("tmDefaultChar", &self.tmDefaultChar)
            .field("tmBreakChar", &self.tmBreakChar)
            .field("tmItalic", &self.tmItalic)
            .field("tmUnderlined", &self.tmUnderlined)
            .field("tmStruckOut", &self.tmStruckOut)
            .field("tmPitchAndFamily", &self.tmPitchAndFamily)
            .field("tmCharSet", &self.tmCharSet)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for TEXTMETRICW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TEXTMETRICW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TEXTMETRICW>()) == 0 }
    }
}
impl ::core::cmp::Eq for TEXTMETRICW {}
impl ::core::default::Default for TEXTMETRICW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct TRIVERTEX {
    pub x: i32,
    pub y: i32,
    pub Red: u16,
    pub Green: u16,
    pub Blue: u16,
    pub Alpha: u16,
}
impl ::core::marker::Copy for TRIVERTEX {}
impl ::core::clone::Clone for TRIVERTEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRIVERTEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRIVERTEX").field("x", &self.x).field("y", &self.y).field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).field("Alpha", &self.Alpha).finish()
    }
}
unsafe impl ::windows::core::Abi for TRIVERTEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRIVERTEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRIVERTEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRIVERTEX {}
impl ::core::default::Default for TRIVERTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct TTEMBEDINFO {
    pub usStructSize: u16,
    pub usRootStrSize: u16,
    pub pusRootStr: *mut u16,
}
impl ::core::marker::Copy for TTEMBEDINFO {}
impl ::core::clone::Clone for TTEMBEDINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TTEMBEDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTEMBEDINFO").field("usStructSize", &self.usStructSize).field("usRootStrSize", &self.usRootStrSize).field("pusRootStr", &self.pusRootStr).finish()
    }
}
unsafe impl ::windows::core::Abi for TTEMBEDINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TTEMBEDINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TTEMBEDINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for TTEMBEDINFO {}
impl ::core::default::Default for TTEMBEDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct TTLOADINFO {
    pub usStructSize: u16,
    pub usRefStrSize: u16,
    pub pusRefStr: *mut u16,
}
impl ::core::marker::Copy for TTLOADINFO {}
impl ::core::clone::Clone for TTLOADINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TTLOADINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTLOADINFO").field("usStructSize", &self.usStructSize).field("usRefStrSize", &self.usRefStrSize).field("pusRefStr", &self.pusRefStr).finish()
    }
}
unsafe impl ::windows::core::Abi for TTLOADINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TTLOADINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TTLOADINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for TTLOADINFO {}
impl ::core::default::Default for TTLOADINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct TTPOLYCURVE {
    pub wType: u16,
    pub cpfx: u16,
    pub apfx: [POINTFX; 1],
}
impl ::core::marker::Copy for TTPOLYCURVE {}
impl ::core::clone::Clone for TTPOLYCURVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TTPOLYCURVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTPOLYCURVE").field("wType", &self.wType).field("cpfx", &self.cpfx).field("apfx", &self.apfx).finish()
    }
}
unsafe impl ::windows::core::Abi for TTPOLYCURVE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TTPOLYCURVE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TTPOLYCURVE>()) == 0 }
    }
}
impl ::core::cmp::Eq for TTPOLYCURVE {}
impl ::core::default::Default for TTPOLYCURVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct TTPOLYGONHEADER {
    pub cb: u32,
    pub dwType: u32,
    pub pfxStart: POINTFX,
}
impl ::core::marker::Copy for TTPOLYGONHEADER {}
impl ::core::clone::Clone for TTPOLYGONHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TTPOLYGONHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTPOLYGONHEADER").field("cb", &self.cb).field("dwType", &self.dwType).field("pfxStart", &self.pfxStart).finish()
    }
}
unsafe impl ::windows::core::Abi for TTPOLYGONHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TTPOLYGONHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TTPOLYGONHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for TTPOLYGONHEADER {}
impl ::core::default::Default for TTPOLYGONHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct TTVALIDATIONTESTSPARAMS {
    pub ulStructSize: u32,
    pub lTestFromSize: i32,
    pub lTestToSize: i32,
    pub ulCharSet: u32,
    pub usReserved1: u16,
    pub usCharCodeCount: u16,
    pub pusCharCodeSet: *mut u16,
}
impl ::core::marker::Copy for TTVALIDATIONTESTSPARAMS {}
impl ::core::clone::Clone for TTVALIDATIONTESTSPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TTVALIDATIONTESTSPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTVALIDATIONTESTSPARAMS").field("ulStructSize", &self.ulStructSize).field("lTestFromSize", &self.lTestFromSize).field("lTestToSize", &self.lTestToSize).field("ulCharSet", &self.ulCharSet).field("usReserved1", &self.usReserved1).field("usCharCodeCount", &self.usCharCodeCount).field("pusCharCodeSet", &self.pusCharCodeSet).finish()
    }
}
unsafe impl ::windows::core::Abi for TTVALIDATIONTESTSPARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TTVALIDATIONTESTSPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TTVALIDATIONTESTSPARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for TTVALIDATIONTESTSPARAMS {}
impl ::core::default::Default for TTVALIDATIONTESTSPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct TTVALIDATIONTESTSPARAMSEX {
    pub ulStructSize: u32,
    pub lTestFromSize: i32,
    pub lTestToSize: i32,
    pub ulCharSet: u32,
    pub usReserved1: u16,
    pub usCharCodeCount: u16,
    pub pulCharCodeSet: *mut u32,
}
impl ::core::marker::Copy for TTVALIDATIONTESTSPARAMSEX {}
impl ::core::clone::Clone for TTVALIDATIONTESTSPARAMSEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TTVALIDATIONTESTSPARAMSEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTVALIDATIONTESTSPARAMSEX").field("ulStructSize", &self.ulStructSize).field("lTestFromSize", &self.lTestFromSize).field("lTestToSize", &self.lTestToSize).field("ulCharSet", &self.ulCharSet).field("usReserved1", &self.usReserved1).field("usCharCodeCount", &self.usCharCodeCount).field("pulCharCodeSet", &self.pulCharCodeSet).finish()
    }
}
unsafe impl ::windows::core::Abi for TTVALIDATIONTESTSPARAMSEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TTVALIDATIONTESTSPARAMSEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TTVALIDATIONTESTSPARAMSEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for TTVALIDATIONTESTSPARAMSEX {}
impl ::core::default::Default for TTVALIDATIONTESTSPARAMSEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct WCRANGE {
    pub wcLow: u16,
    pub cGlyphs: u16,
}
impl ::core::marker::Copy for WCRANGE {}
impl ::core::clone::Clone for WCRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WCRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCRANGE").field("wcLow", &self.wcLow).field("cGlyphs", &self.cGlyphs).finish()
    }
}
unsafe impl ::windows::core::Abi for WCRANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WCRANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WCRANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WCRANGE {}
impl ::core::default::Default for WCRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct WGLSWAP {
    pub hdc: HDC,
    pub uiFlags: u32,
}
impl ::core::marker::Copy for WGLSWAP {}
impl ::core::clone::Clone for WGLSWAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WGLSWAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WGLSWAP").field("hdc", &self.hdc).field("uiFlags", &self.uiFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for WGLSWAP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WGLSWAP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WGLSWAP>()) == 0 }
    }
}
impl ::core::cmp::Eq for WGLSWAP {}
impl ::core::default::Default for WGLSWAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct XFORM {
    pub eM11: f32,
    pub eM12: f32,
    pub eM21: f32,
    pub eM22: f32,
    pub eDx: f32,
    pub eDy: f32,
}
impl ::core::marker::Copy for XFORM {}
impl ::core::clone::Clone for XFORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XFORM").field("eM11", &self.eM11).field("eM12", &self.eM12).field("eM21", &self.eM21).field("eM22", &self.eM22).field("eDx", &self.eDx).field("eDy", &self.eDy).finish()
    }
}
unsafe impl ::windows::core::Abi for XFORM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XFORM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XFORM>()) == 0 }
    }
}
impl ::core::cmp::Eq for XFORM {}
impl ::core::default::Default for XFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type CFP_ALLOCPROC = ::core::option::Option<unsafe extern "system" fn(param0: usize) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type CFP_FREEPROC = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type CFP_REALLOCPROC = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: usize) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DRAWSTATEPROC = ::core::option::Option<unsafe extern "system" fn(hdc: HDC, ldata: super::super::Foundation::LPARAM, wdata: super::super::Foundation::WPARAM, cx: i32, cy: i32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENHMFENUMPROC = ::core::option::Option<unsafe extern "system" fn(hdc: HDC, lpht: *const HANDLETABLE, lpmr: *const ENHMETARECORD, nhandles: i32, data: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FONTENUMPROCA = ::core::option::Option<unsafe extern "system" fn(param0: *const LOGFONTA, param1: *const TEXTMETRICA, param2: u32, param3: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FONTENUMPROCW = ::core::option::Option<unsafe extern "system" fn(param0: *const LOGFONTW, param1: *const TEXTMETRICW, param2: u32, param3: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type GOBJENUMPROC = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type GRAYSTRINGPROC = ::core::option::Option<unsafe extern "system" fn(param0: HDC, param1: super::super::Foundation::LPARAM, param2: i32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LINEDDAPROC = ::core::option::Option<unsafe extern "system" fn(param0: i32, param1: i32, param2: super::super::Foundation::LPARAM)>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNDEVCAPS = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR, param2: u32, param3: ::windows::core::PCSTR, param4: *mut DEVMODEA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNDEVMODE = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: super::super::Foundation::HINSTANCE, param2: *mut DEVMODEA, param3: ::windows::core::PCSTR, param4: ::windows::core::PCSTR, param5: *mut DEVMODEA, param6: ::windows::core::PCSTR, param7: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type MFENUMPROC = ::core::option::Option<unsafe extern "system" fn(hdc: HDC, lpht: *const HANDLETABLE, lpmr: *const METARECORD, nobj: i32, param4: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type MONITORENUMPROC = ::core::option::Option<unsafe extern "system" fn(param0: HMONITOR, param1: HDC, param2: *mut super::super::Foundation::RECT, param3: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type READEMBEDPROC = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *mut ::core::ffi::c_void, param2: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type WRITEEMBEDPROC = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *const ::core::ffi::c_void, param2: u32) -> u32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
