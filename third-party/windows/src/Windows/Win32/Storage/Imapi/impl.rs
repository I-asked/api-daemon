#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DDiscFormat2DataEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&mut self, object: &::core::option::Option<super::super::System::Com::IDispatch>, progress: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DDiscFormat2DataEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2DataEvents_Impl, const OFFSET: isize>() -> DDiscFormat2DataEvents_Vtbl {
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2DataEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, progress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Update(::core::mem::transmute(&object), ::core::mem::transmute(&progress)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Update: Update::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DDiscFormat2DataEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DDiscFormat2EraseEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&mut self, object: &::core::option::Option<super::super::System::Com::IDispatch>, elapsedseconds: i32, estimatedtotalseconds: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DDiscFormat2EraseEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2EraseEvents_Impl, const OFFSET: isize>() -> DDiscFormat2EraseEvents_Vtbl {
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2EraseEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, elapsedseconds: i32, estimatedtotalseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Update(::core::mem::transmute(&object), ::core::mem::transmute_copy(&elapsedseconds), ::core::mem::transmute_copy(&estimatedtotalseconds)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Update: Update::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DDiscFormat2EraseEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DDiscFormat2RawCDEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&mut self, object: &::core::option::Option<super::super::System::Com::IDispatch>, progress: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DDiscFormat2RawCDEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2RawCDEvents_Impl, const OFFSET: isize>() -> DDiscFormat2RawCDEvents_Vtbl {
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2RawCDEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, progress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Update(::core::mem::transmute(&object), ::core::mem::transmute(&progress)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Update: Update::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DDiscFormat2RawCDEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DDiscFormat2TrackAtOnceEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&mut self, object: &::core::option::Option<super::super::System::Com::IDispatch>, progress: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DDiscFormat2TrackAtOnceEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2TrackAtOnceEvents_Impl, const OFFSET: isize>() -> DDiscFormat2TrackAtOnceEvents_Vtbl {
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2TrackAtOnceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, progress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Update(::core::mem::transmute(&object), ::core::mem::transmute(&progress)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Update: Update::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DDiscFormat2TrackAtOnceEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DDiscMaster2Events_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn NotifyDeviceAdded(&mut self, object: &::core::option::Option<super::super::System::Com::IDispatch>, uniqueid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NotifyDeviceRemoved(&mut self, object: &::core::option::Option<super::super::System::Com::IDispatch>, uniqueid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DDiscMaster2Events_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscMaster2Events_Impl, const OFFSET: isize>() -> DDiscMaster2Events_Vtbl {
        unsafe extern "system" fn NotifyDeviceAdded<Identity: ::windows::core::IUnknownImpl, Impl: DDiscMaster2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, uniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyDeviceAdded(::core::mem::transmute(&object), ::core::mem::transmute_copy(&uniqueid)).into()
        }
        unsafe extern "system" fn NotifyDeviceRemoved<Identity: ::windows::core::IUnknownImpl, Impl: DDiscMaster2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, uniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyDeviceRemoved(::core::mem::transmute(&object), ::core::mem::transmute_copy(&uniqueid)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            NotifyDeviceAdded: NotifyDeviceAdded::<Identity, Impl, OFFSET>,
            NotifyDeviceRemoved: NotifyDeviceRemoved::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DDiscMaster2Events as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DFileSystemImageEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&mut self, object: &::core::option::Option<super::super::System::Com::IDispatch>, currentfile: &super::super::Foundation::BSTR, copiedsectors: i32, totalsectors: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DFileSystemImageEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DFileSystemImageEvents_Impl, const OFFSET: isize>() -> DFileSystemImageEvents_Vtbl {
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: DFileSystemImageEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, currentfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, copiedsectors: i32, totalsectors: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Update(::core::mem::transmute(&object), ::core::mem::transmute_copy(&currentfile), ::core::mem::transmute_copy(&copiedsectors), ::core::mem::transmute_copy(&totalsectors)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Update: Update::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DFileSystemImageEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DFileSystemImageImportEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UpdateImport(&mut self, object: &::core::option::Option<super::super::System::Com::IDispatch>, filesystem: FsiFileSystems, currentitem: &super::super::Foundation::BSTR, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DFileSystemImageImportEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DFileSystemImageImportEvents_Impl, const OFFSET: isize>() -> DFileSystemImageImportEvents_Vtbl {
        unsafe extern "system" fn UpdateImport<Identity: ::windows::core::IUnknownImpl, Impl: DFileSystemImageImportEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, filesystem: FsiFileSystems, currentitem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateImport(::core::mem::transmute(&object), ::core::mem::transmute_copy(&filesystem), ::core::mem::transmute_copy(&currentitem), ::core::mem::transmute_copy(&importeddirectoryitems), ::core::mem::transmute_copy(&totaldirectoryitems), ::core::mem::transmute_copy(&importedfileitems), ::core::mem::transmute_copy(&totalfileitems)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), UpdateImport: UpdateImport::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DFileSystemImageImportEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DWriteEngine2Events_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&mut self, object: &::core::option::Option<super::super::System::Com::IDispatch>, progress: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DWriteEngine2Events_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DWriteEngine2Events_Impl, const OFFSET: isize>() -> DWriteEngine2Events_Vtbl {
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: DWriteEngine2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, progress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Update(::core::mem::transmute(&object), ::core::mem::transmute(&progress)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Update: Update::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DWriteEngine2Events as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBlockRange_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartLba(&mut self) -> ::windows::core::Result<i32>;
    fn EndLba(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBlockRange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBlockRange_Impl, const OFFSET: isize>() -> IBlockRange_Vtbl {
        unsafe extern "system" fn StartLba<Identity: ::windows::core::IUnknownImpl, Impl: IBlockRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartLba() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndLba<Identity: ::windows::core::IUnknownImpl, Impl: IBlockRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EndLba() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StartLba: StartLba::<Identity, Impl, OFFSET>,
            EndLba: EndLba::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBlockRange as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBlockRangeList_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn BlockRanges(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBlockRangeList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBlockRangeList_Impl, const OFFSET: isize>() -> IBlockRangeList_Vtbl {
        unsafe extern "system" fn BlockRanges<Identity: ::windows::core::IUnknownImpl, Impl: IBlockRangeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BlockRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), BlockRanges: BlockRanges::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBlockRangeList as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBootOptions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn BootImage(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn Manufacturer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetManufacturer(&mut self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlatformId(&mut self) -> ::windows::core::Result<PlatformId>;
    fn SetPlatformId(&mut self, newval: PlatformId) -> ::windows::core::Result<()>;
    fn Emulation(&mut self) -> ::windows::core::Result<EmulationType>;
    fn SetEmulation(&mut self, newval: EmulationType) -> ::windows::core::Result<()>;
    fn ImageSize(&mut self) -> ::windows::core::Result<u32>;
    fn AssignBootImage(&mut self, newval: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBootOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBootOptions_Impl, const OFFSET: isize>() -> IBootOptions_Vtbl {
        unsafe extern "system" fn BootImage<Identity: ::windows::core::IUnknownImpl, Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BootImage() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Manufacturer<Identity: ::windows::core::IUnknownImpl, Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Manufacturer() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManufacturer<Identity: ::windows::core::IUnknownImpl, Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetManufacturer(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn PlatformId<Identity: ::windows::core::IUnknownImpl, Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut PlatformId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PlatformId() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlatformId<Identity: ::windows::core::IUnknownImpl, Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: PlatformId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPlatformId(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Emulation<Identity: ::windows::core::IUnknownImpl, Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut EmulationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Emulation() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmulation<Identity: ::windows::core::IUnknownImpl, Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: EmulationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEmulation(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ImageSize<Identity: ::windows::core::IUnknownImpl, Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ImageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssignBootImage<Identity: ::windows::core::IUnknownImpl, Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AssignBootImage(::core::mem::transmute(&newval)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BootImage: BootImage::<Identity, Impl, OFFSET>,
            Manufacturer: Manufacturer::<Identity, Impl, OFFSET>,
            SetManufacturer: SetManufacturer::<Identity, Impl, OFFSET>,
            PlatformId: PlatformId::<Identity, Impl, OFFSET>,
            SetPlatformId: SetPlatformId::<Identity, Impl, OFFSET>,
            Emulation: Emulation::<Identity, Impl, OFFSET>,
            SetEmulation: SetEmulation::<Identity, Impl, OFFSET>,
            ImageSize: ImageSize::<Identity, Impl, OFFSET>,
            AssignBootImage: AssignBootImage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBootOptions as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IBurnVerification_Impl: Sized {
    fn SetBurnVerificationLevel(&mut self, value: IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows::core::Result<()>;
    fn BurnVerificationLevel(&mut self) -> ::windows::core::Result<IMAPI_BURN_VERIFICATION_LEVEL>;
}
impl IBurnVerification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBurnVerification_Impl, const OFFSET: isize>() -> IBurnVerification_Vtbl {
        unsafe extern "system" fn SetBurnVerificationLevel<Identity: ::windows::core::IUnknownImpl, Impl: IBurnVerification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBurnVerificationLevel(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BurnVerificationLevel<Identity: ::windows::core::IUnknownImpl, Impl: IBurnVerification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BurnVerificationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetBurnVerificationLevel: SetBurnVerificationLevel::<Identity, Impl, OFFSET>,
            BurnVerificationLevel: BurnVerificationLevel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBurnVerification as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscFormat2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IsRecorderSupported(&mut self, recorder: &::core::option::Option<IDiscRecorder2>) -> ::windows::core::Result<i16>;
    fn IsCurrentMediaSupported(&mut self, recorder: &::core::option::Option<IDiscRecorder2>) -> ::windows::core::Result<i16>;
    fn MediaPhysicallyBlank(&mut self) -> ::windows::core::Result<i16>;
    fn MediaHeuristicallyBlank(&mut self) -> ::windows::core::Result<i16>;
    fn SupportedMediaTypes(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2_Impl, const OFFSET: isize>() -> IDiscFormat2_Vtbl {
        unsafe extern "system" fn IsRecorderSupported<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recorder: ::windows::core::RawPtr, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsRecorderSupported(::core::mem::transmute(&recorder)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentMediaSupported<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recorder: ::windows::core::RawPtr, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsCurrentMediaSupported(::core::mem::transmute(&recorder)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaPhysicallyBlank<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MediaPhysicallyBlank() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaHeuristicallyBlank<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MediaHeuristicallyBlank() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedMediaTypes<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedMediaTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsRecorderSupported: IsRecorderSupported::<Identity, Impl, OFFSET>,
            IsCurrentMediaSupported: IsCurrentMediaSupported::<Identity, Impl, OFFSET>,
            MediaPhysicallyBlank: MediaPhysicallyBlank::<Identity, Impl, OFFSET>,
            MediaHeuristicallyBlank: MediaHeuristicallyBlank::<Identity, Impl, OFFSET>,
            SupportedMediaTypes: SupportedMediaTypes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscFormat2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscFormat2Data_Impl: Sized + super::super::System::Com::IDispatch_Impl + IDiscFormat2_Impl {
    fn SetRecorder(&mut self, value: &::core::option::Option<IDiscRecorder2>) -> ::windows::core::Result<()>;
    fn Recorder(&mut self) -> ::windows::core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn BufferUnderrunFreeDisabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetPostgapAlreadyInImage(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn PostgapAlreadyInImage(&mut self) -> ::windows::core::Result<i16>;
    fn CurrentMediaStatus(&mut self) -> ::windows::core::Result<IMAPI_FORMAT2_DATA_MEDIA_STATE>;
    fn WriteProtectStatus(&mut self) -> ::windows::core::Result<IMAPI_MEDIA_WRITE_PROTECT_STATE>;
    fn TotalSectorsOnMedia(&mut self) -> ::windows::core::Result<i32>;
    fn FreeSectorsOnMedia(&mut self) -> ::windows::core::Result<i32>;
    fn NextWritableAddress(&mut self) -> ::windows::core::Result<i32>;
    fn StartAddressOfPreviousSession(&mut self) -> ::windows::core::Result<i32>;
    fn LastWrittenAddressOfPreviousSession(&mut self) -> ::windows::core::Result<i32>;
    fn SetForceMediaToBeClosed(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn ForceMediaToBeClosed(&mut self) -> ::windows::core::Result<i16>;
    fn SetDisableConsumerDvdCompatibilityMode(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn DisableConsumerDvdCompatibilityMode(&mut self) -> ::windows::core::Result<i16>;
    fn CurrentPhysicalMediaType(&mut self) -> ::windows::core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(&mut self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ClientName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RequestedWriteSpeed(&mut self) -> ::windows::core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(&mut self) -> ::windows::core::Result<i16>;
    fn CurrentWriteSpeed(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(&mut self) -> ::windows::core::Result<i16>;
    fn SupportedWriteSpeeds(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetForceOverwrite(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn ForceOverwrite(&mut self) -> ::windows::core::Result<i16>;
    fn MultisessionInterfaces(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn Write(&mut self, data: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn CancelWrite(&mut self) -> ::windows::core::Result<()>;
    fn SetWriteSpeed(&mut self, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2Data_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>() -> IDiscFormat2Data_Vtbl {
        unsafe extern "system" fn SetRecorder<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRecorder(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Recorder<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Recorder() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBufferUnderrunFreeDisabled(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BufferUnderrunFreeDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostgapAlreadyInImage<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPostgapAlreadyInImage(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn PostgapAlreadyInImage<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PostgapAlreadyInImage() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMediaStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_MEDIA_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentMediaStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteProtectStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_WRITE_PROTECT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WriteProtectStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FreeSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextWritableAddress<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NextWritableAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAddressOfPreviousSession<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartAddressOfPreviousSession() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenAddressOfPreviousSession<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastWrittenAddressOfPreviousSession() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceMediaToBeClosed<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetForceMediaToBeClosed(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ForceMediaToBeClosed<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ForceMediaToBeClosed() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableConsumerDvdCompatibilityMode<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisableConsumerDvdCompatibilityMode(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn DisableConsumerDvdCompatibilityMode<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisableConsumerDvdCompatibilityMode() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentPhysicalMediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClientName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ClientName<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClientName() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequestedWriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequestedRotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentWriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentRotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedWriteSpeeds() {
                ::core::result::Result::Ok(ok__) => {
                    *supportedspeeds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedWriteSpeedDescriptors() {
                ::core::result::Result::Ok(ok__) => {
                    *supportedspeeddescriptors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceOverwrite<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetForceOverwrite(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ForceOverwrite<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ForceOverwrite() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MultisessionInterfaces<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MultisessionInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Write(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn CancelWrite<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelWrite().into()
        }
        unsafe extern "system" fn SetWriteSpeed<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWriteSpeed(::core::mem::transmute_copy(&requestedsectorspersecond), ::core::mem::transmute_copy(&rotationtypeispurecav)).into()
        }
        Self {
            base: IDiscFormat2_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetRecorder: SetRecorder::<Identity, Impl, OFFSET>,
            Recorder: Recorder::<Identity, Impl, OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Identity, Impl, OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Identity, Impl, OFFSET>,
            SetPostgapAlreadyInImage: SetPostgapAlreadyInImage::<Identity, Impl, OFFSET>,
            PostgapAlreadyInImage: PostgapAlreadyInImage::<Identity, Impl, OFFSET>,
            CurrentMediaStatus: CurrentMediaStatus::<Identity, Impl, OFFSET>,
            WriteProtectStatus: WriteProtectStatus::<Identity, Impl, OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Identity, Impl, OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Identity, Impl, OFFSET>,
            NextWritableAddress: NextWritableAddress::<Identity, Impl, OFFSET>,
            StartAddressOfPreviousSession: StartAddressOfPreviousSession::<Identity, Impl, OFFSET>,
            LastWrittenAddressOfPreviousSession: LastWrittenAddressOfPreviousSession::<Identity, Impl, OFFSET>,
            SetForceMediaToBeClosed: SetForceMediaToBeClosed::<Identity, Impl, OFFSET>,
            ForceMediaToBeClosed: ForceMediaToBeClosed::<Identity, Impl, OFFSET>,
            SetDisableConsumerDvdCompatibilityMode: SetDisableConsumerDvdCompatibilityMode::<Identity, Impl, OFFSET>,
            DisableConsumerDvdCompatibilityMode: DisableConsumerDvdCompatibilityMode::<Identity, Impl, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, Impl, OFFSET>,
            SetClientName: SetClientName::<Identity, Impl, OFFSET>,
            ClientName: ClientName::<Identity, Impl, OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Identity, Impl, OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Identity, Impl, OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Identity, Impl, OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Identity, Impl, OFFSET>,
            SetForceOverwrite: SetForceOverwrite::<Identity, Impl, OFFSET>,
            ForceOverwrite: ForceOverwrite::<Identity, Impl, OFFSET>,
            MultisessionInterfaces: MultisessionInterfaces::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            CancelWrite: CancelWrite::<Identity, Impl, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscFormat2Data as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IDiscFormat2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscFormat2DataEventArgs_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWriteEngine2EventArgs_Impl {
    fn ElapsedTime(&mut self) -> ::windows::core::Result<i32>;
    fn RemainingTime(&mut self) -> ::windows::core::Result<i32>;
    fn TotalTime(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentAction(&mut self) -> ::windows::core::Result<IMAPI_FORMAT2_DATA_WRITE_ACTION>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2DataEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>() -> IDiscFormat2DataEventArgs_Vtbl {
        unsafe extern "system" fn ElapsedTime<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ElapsedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingTime<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemainingTime() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalTime<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalTime() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAction<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_WRITE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentAction() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWriteEngine2EventArgs_Vtbl::new::<Identity, Impl, OFFSET>(),
            ElapsedTime: ElapsedTime::<Identity, Impl, OFFSET>,
            RemainingTime: RemainingTime::<Identity, Impl, OFFSET>,
            TotalTime: TotalTime::<Identity, Impl, OFFSET>,
            CurrentAction: CurrentAction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscFormat2DataEventArgs as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWriteEngine2EventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscFormat2Erase_Impl: Sized + super::super::System::Com::IDispatch_Impl + IDiscFormat2_Impl {
    fn SetRecorder(&mut self, value: &::core::option::Option<IDiscRecorder2>) -> ::windows::core::Result<()>;
    fn Recorder(&mut self) -> ::windows::core::Result<IDiscRecorder2>;
    fn SetFullErase(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn FullErase(&mut self) -> ::windows::core::Result<i16>;
    fn CurrentPhysicalMediaType(&mut self) -> ::windows::core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(&mut self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ClientName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EraseMedia(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2Erase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>() -> IDiscFormat2Erase_Vtbl {
        unsafe extern "system" fn SetRecorder<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRecorder(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Recorder<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Recorder() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFullErase<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFullErase(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn FullErase<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FullErase() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentPhysicalMediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClientName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ClientName<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClientName() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EraseMedia<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EraseMedia().into()
        }
        Self {
            base: IDiscFormat2_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetRecorder: SetRecorder::<Identity, Impl, OFFSET>,
            Recorder: Recorder::<Identity, Impl, OFFSET>,
            SetFullErase: SetFullErase::<Identity, Impl, OFFSET>,
            FullErase: FullErase::<Identity, Impl, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, Impl, OFFSET>,
            SetClientName: SetClientName::<Identity, Impl, OFFSET>,
            ClientName: ClientName::<Identity, Impl, OFFSET>,
            EraseMedia: EraseMedia::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscFormat2Erase as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IDiscFormat2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscFormat2RawCD_Impl: Sized + super::super::System::Com::IDispatch_Impl + IDiscFormat2_Impl {
    fn PrepareMedia(&mut self) -> ::windows::core::Result<()>;
    fn WriteMedia(&mut self, data: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn WriteMedia2(&mut self, data: &::core::option::Option<super::super::System::Com::IStream>, streamleadinsectors: i32) -> ::windows::core::Result<()>;
    fn CancelWrite(&mut self) -> ::windows::core::Result<()>;
    fn ReleaseMedia(&mut self) -> ::windows::core::Result<()>;
    fn SetWriteSpeed(&mut self, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::Result<()>;
    fn SetRecorder(&mut self, value: &::core::option::Option<IDiscRecorder2>) -> ::windows::core::Result<()>;
    fn Recorder(&mut self) -> ::windows::core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn BufferUnderrunFreeDisabled(&mut self) -> ::windows::core::Result<i16>;
    fn StartOfNextSession(&mut self) -> ::windows::core::Result<i32>;
    fn LastPossibleStartOfLeadout(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentPhysicalMediaType(&mut self) -> ::windows::core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SupportedSectorTypes(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetRequestedSectorType(&mut self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::Result<()>;
    fn RequestedSectorType(&mut self) -> ::windows::core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>;
    fn SetClientName(&mut self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ClientName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RequestedWriteSpeed(&mut self) -> ::windows::core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(&mut self) -> ::windows::core::Result<i16>;
    fn CurrentWriteSpeed(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(&mut self) -> ::windows::core::Result<i16>;
    fn SupportedWriteSpeeds(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2RawCD_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>() -> IDiscFormat2RawCD_Vtbl {
        unsafe extern "system" fn PrepareMedia<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PrepareMedia().into()
        }
        unsafe extern "system" fn WriteMedia<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteMedia(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn WriteMedia2<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, streamleadinsectors: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteMedia2(::core::mem::transmute(&data), ::core::mem::transmute_copy(&streamleadinsectors)).into()
        }
        unsafe extern "system" fn CancelWrite<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelWrite().into()
        }
        unsafe extern "system" fn ReleaseMedia<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseMedia().into()
        }
        unsafe extern "system" fn SetWriteSpeed<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWriteSpeed(::core::mem::transmute_copy(&requestedsectorspersecond), ::core::mem::transmute_copy(&rotationtypeispurecav)).into()
        }
        unsafe extern "system" fn SetRecorder<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRecorder(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Recorder<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Recorder() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBufferUnderrunFreeDisabled(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BufferUnderrunFreeDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartOfNextSession<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartOfNextSession() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastPossibleStartOfLeadout<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastPossibleStartOfLeadout() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentPhysicalMediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedSectorTypes<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedSectorTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestedSectorType<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRequestedSectorType(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn RequestedSectorType<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequestedSectorType() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClientName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ClientName<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClientName() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequestedWriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequestedRotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentWriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentRotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedWriteSpeeds() {
                ::core::result::Result::Ok(ok__) => {
                    *supportedspeeds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedWriteSpeedDescriptors() {
                ::core::result::Result::Ok(ok__) => {
                    *supportedspeeddescriptors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDiscFormat2_Vtbl::new::<Identity, Impl, OFFSET>(),
            PrepareMedia: PrepareMedia::<Identity, Impl, OFFSET>,
            WriteMedia: WriteMedia::<Identity, Impl, OFFSET>,
            WriteMedia2: WriteMedia2::<Identity, Impl, OFFSET>,
            CancelWrite: CancelWrite::<Identity, Impl, OFFSET>,
            ReleaseMedia: ReleaseMedia::<Identity, Impl, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, Impl, OFFSET>,
            SetRecorder: SetRecorder::<Identity, Impl, OFFSET>,
            Recorder: Recorder::<Identity, Impl, OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Identity, Impl, OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Identity, Impl, OFFSET>,
            StartOfNextSession: StartOfNextSession::<Identity, Impl, OFFSET>,
            LastPossibleStartOfLeadout: LastPossibleStartOfLeadout::<Identity, Impl, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, Impl, OFFSET>,
            SupportedSectorTypes: SupportedSectorTypes::<Identity, Impl, OFFSET>,
            SetRequestedSectorType: SetRequestedSectorType::<Identity, Impl, OFFSET>,
            RequestedSectorType: RequestedSectorType::<Identity, Impl, OFFSET>,
            SetClientName: SetClientName::<Identity, Impl, OFFSET>,
            ClientName: ClientName::<Identity, Impl, OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Identity, Impl, OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Identity, Impl, OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Identity, Impl, OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscFormat2RawCD as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IDiscFormat2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscFormat2RawCDEventArgs_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWriteEngine2EventArgs_Impl {
    fn CurrentAction(&mut self) -> ::windows::core::Result<IMAPI_FORMAT2_RAW_CD_WRITE_ACTION>;
    fn ElapsedTime(&mut self) -> ::windows::core::Result<i32>;
    fn RemainingTime(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2RawCDEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>() -> IDiscFormat2RawCDEventArgs_Vtbl {
        unsafe extern "system" fn CurrentAction<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentAction() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElapsedTime<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ElapsedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingTime<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemainingTime() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWriteEngine2EventArgs_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentAction: CurrentAction::<Identity, Impl, OFFSET>,
            ElapsedTime: ElapsedTime::<Identity, Impl, OFFSET>,
            RemainingTime: RemainingTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscFormat2RawCDEventArgs as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWriteEngine2EventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscFormat2TrackAtOnce_Impl: Sized + super::super::System::Com::IDispatch_Impl + IDiscFormat2_Impl {
    fn PrepareMedia(&mut self) -> ::windows::core::Result<()>;
    fn AddAudioTrack(&mut self, data: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn CancelAddTrack(&mut self) -> ::windows::core::Result<()>;
    fn ReleaseMedia(&mut self) -> ::windows::core::Result<()>;
    fn SetWriteSpeed(&mut self, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::Result<()>;
    fn SetRecorder(&mut self, value: &::core::option::Option<IDiscRecorder2>) -> ::windows::core::Result<()>;
    fn Recorder(&mut self) -> ::windows::core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn BufferUnderrunFreeDisabled(&mut self) -> ::windows::core::Result<i16>;
    fn NumberOfExistingTracks(&mut self) -> ::windows::core::Result<i32>;
    fn TotalSectorsOnMedia(&mut self) -> ::windows::core::Result<i32>;
    fn FreeSectorsOnMedia(&mut self) -> ::windows::core::Result<i32>;
    fn UsedSectorsOnMedia(&mut self) -> ::windows::core::Result<i32>;
    fn SetDoNotFinalizeMedia(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn DoNotFinalizeMedia(&mut self) -> ::windows::core::Result<i16>;
    fn ExpectedTableOfContents(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentPhysicalMediaType(&mut self) -> ::windows::core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(&mut self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ClientName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RequestedWriteSpeed(&mut self) -> ::windows::core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(&mut self) -> ::windows::core::Result<i16>;
    fn CurrentWriteSpeed(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(&mut self) -> ::windows::core::Result<i16>;
    fn SupportedWriteSpeeds(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2TrackAtOnce_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>() -> IDiscFormat2TrackAtOnce_Vtbl {
        unsafe extern "system" fn PrepareMedia<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PrepareMedia().into()
        }
        unsafe extern "system" fn AddAudioTrack<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAudioTrack(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn CancelAddTrack<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelAddTrack().into()
        }
        unsafe extern "system" fn ReleaseMedia<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseMedia().into()
        }
        unsafe extern "system" fn SetWriteSpeed<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWriteSpeed(::core::mem::transmute_copy(&requestedsectorspersecond), ::core::mem::transmute_copy(&rotationtypeispurecav)).into()
        }
        unsafe extern "system" fn SetRecorder<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRecorder(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Recorder<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Recorder() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBufferUnderrunFreeDisabled(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BufferUnderrunFreeDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfExistingTracks<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumberOfExistingTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FreeSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsedSectorsOnMedia<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UsedSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoNotFinalizeMedia<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDoNotFinalizeMedia(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn DoNotFinalizeMedia<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DoNotFinalizeMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpectedTableOfContents<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExpectedTableOfContents() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentPhysicalMediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClientName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ClientName<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClientName() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequestedWriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequestedRotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentWriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentRotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedWriteSpeeds() {
                ::core::result::Result::Ok(ok__) => {
                    *supportedspeeds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedWriteSpeedDescriptors() {
                ::core::result::Result::Ok(ok__) => {
                    *supportedspeeddescriptors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDiscFormat2_Vtbl::new::<Identity, Impl, OFFSET>(),
            PrepareMedia: PrepareMedia::<Identity, Impl, OFFSET>,
            AddAudioTrack: AddAudioTrack::<Identity, Impl, OFFSET>,
            CancelAddTrack: CancelAddTrack::<Identity, Impl, OFFSET>,
            ReleaseMedia: ReleaseMedia::<Identity, Impl, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, Impl, OFFSET>,
            SetRecorder: SetRecorder::<Identity, Impl, OFFSET>,
            Recorder: Recorder::<Identity, Impl, OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Identity, Impl, OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Identity, Impl, OFFSET>,
            NumberOfExistingTracks: NumberOfExistingTracks::<Identity, Impl, OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Identity, Impl, OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Identity, Impl, OFFSET>,
            UsedSectorsOnMedia: UsedSectorsOnMedia::<Identity, Impl, OFFSET>,
            SetDoNotFinalizeMedia: SetDoNotFinalizeMedia::<Identity, Impl, OFFSET>,
            DoNotFinalizeMedia: DoNotFinalizeMedia::<Identity, Impl, OFFSET>,
            ExpectedTableOfContents: ExpectedTableOfContents::<Identity, Impl, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, Impl, OFFSET>,
            SetClientName: SetClientName::<Identity, Impl, OFFSET>,
            ClientName: ClientName::<Identity, Impl, OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Identity, Impl, OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Identity, Impl, OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Identity, Impl, OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscFormat2TrackAtOnce as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IDiscFormat2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscFormat2TrackAtOnceEventArgs_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWriteEngine2EventArgs_Impl {
    fn CurrentTrackNumber(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentAction(&mut self) -> ::windows::core::Result<IMAPI_FORMAT2_TAO_WRITE_ACTION>;
    fn ElapsedTime(&mut self) -> ::windows::core::Result<i32>;
    fn RemainingTime(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2TrackAtOnceEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>() -> IDiscFormat2TrackAtOnceEventArgs_Vtbl {
        unsafe extern "system" fn CurrentTrackNumber<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentTrackNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAction<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_TAO_WRITE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentAction() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElapsedTime<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ElapsedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingTime<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemainingTime() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWriteEngine2EventArgs_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentTrackNumber: CurrentTrackNumber::<Identity, Impl, OFFSET>,
            CurrentAction: CurrentAction::<Identity, Impl, OFFSET>,
            ElapsedTime: ElapsedTime::<Identity, Impl, OFFSET>,
            RemainingTime: RemainingTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscFormat2TrackAtOnceEventArgs as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWriteEngine2EventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IDiscMaster_Impl: Sized {
    fn Open(&mut self) -> ::windows::core::Result<()>;
    fn EnumDiscMasterFormats(&mut self) -> ::windows::core::Result<IEnumDiscMasterFormats>;
    fn GetActiveDiscMasterFormat(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetActiveDiscMasterFormat(&mut self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn EnumDiscRecorders(&mut self) -> ::windows::core::Result<IEnumDiscRecorders>;
    fn GetActiveDiscRecorder(&mut self) -> ::windows::core::Result<IDiscRecorder>;
    fn SetActiveDiscRecorder(&mut self, precorder: &::core::option::Option<IDiscRecorder>) -> ::windows::core::Result<()>;
    fn ClearFormatContent(&mut self) -> ::windows::core::Result<()>;
    fn ProgressAdvise(&mut self, pevents: &::core::option::Option<IDiscMasterProgressEvents>) -> ::windows::core::Result<usize>;
    fn ProgressUnadvise(&mut self, vcookie: usize) -> ::windows::core::Result<()>;
    fn RecordDisc(&mut self, bsimulate: u8, bejectafterburn: u8) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
impl IDiscMaster_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster_Impl, const OFFSET: isize>() -> IDiscMaster_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open().into()
        }
        unsafe extern "system" fn EnumDiscMasterFormats<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumDiscMasterFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveDiscMasterFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetActiveDiscMasterFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *lpiid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveDiscMasterFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetActiveDiscMasterFormat(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn EnumDiscRecorders<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumDiscRecorders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveDiscRecorder<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecorder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetActiveDiscRecorder() {
                ::core::result::Result::Ok(ok__) => {
                    *pprecorder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveDiscRecorder<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precorder: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetActiveDiscRecorder(::core::mem::transmute(&precorder)).into()
        }
        unsafe extern "system" fn ClearFormatContent<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearFormatContent().into()
        }
        unsafe extern "system" fn ProgressAdvise<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevents: ::windows::core::RawPtr, pvcookie: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProgressAdvise(::core::mem::transmute(&pevents)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressUnadvise<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcookie: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProgressUnadvise(::core::mem::transmute_copy(&vcookie)).into()
        }
        unsafe extern "system" fn RecordDisc<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsimulate: u8, bejectafterburn: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RecordDisc(::core::mem::transmute_copy(&bsimulate), ::core::mem::transmute_copy(&bejectafterburn)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            EnumDiscMasterFormats: EnumDiscMasterFormats::<Identity, Impl, OFFSET>,
            GetActiveDiscMasterFormat: GetActiveDiscMasterFormat::<Identity, Impl, OFFSET>,
            SetActiveDiscMasterFormat: SetActiveDiscMasterFormat::<Identity, Impl, OFFSET>,
            EnumDiscRecorders: EnumDiscRecorders::<Identity, Impl, OFFSET>,
            GetActiveDiscRecorder: GetActiveDiscRecorder::<Identity, Impl, OFFSET>,
            SetActiveDiscRecorder: SetActiveDiscRecorder::<Identity, Impl, OFFSET>,
            ClearFormatContent: ClearFormatContent::<Identity, Impl, OFFSET>,
            ProgressAdvise: ProgressAdvise::<Identity, Impl, OFFSET>,
            ProgressUnadvise: ProgressUnadvise::<Identity, Impl, OFFSET>,
            RecordDisc: RecordDisc::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscMaster as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscMaster2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn IsSupportedEnvironment(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscMaster2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster2_Impl, const OFFSET: isize>() -> IDiscMaster2_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupportedEnvironment<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsSupportedEnvironment() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            IsSupportedEnvironment: IsSupportedEnvironment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscMaster2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IDiscMasterProgressEvents_Impl: Sized {
    fn QueryCancel(&mut self) -> ::windows::core::Result<u8>;
    fn NotifyPnPActivity(&mut self) -> ::windows::core::Result<()>;
    fn NotifyAddProgress(&mut self, ncompletedsteps: i32, ntotalsteps: i32) -> ::windows::core::Result<()>;
    fn NotifyBlockProgress(&mut self, ncompleted: i32, ntotal: i32) -> ::windows::core::Result<()>;
    fn NotifyTrackProgress(&mut self, ncurrenttrack: i32, ntotaltracks: i32) -> ::windows::core::Result<()>;
    fn NotifyPreparingBurn(&mut self, nestimatedseconds: i32) -> ::windows::core::Result<()>;
    fn NotifyClosingDisc(&mut self, nestimatedseconds: i32) -> ::windows::core::Result<()>;
    fn NotifyBurnComplete(&mut self, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn NotifyEraseComplete(&mut self, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IDiscMasterProgressEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>() -> IDiscMasterProgressEvents_Vtbl {
        unsafe extern "system" fn QueryCancel<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcancel: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryCancel() {
                ::core::result::Result::Ok(ok__) => {
                    *pbcancel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyPnPActivity<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyPnPActivity().into()
        }
        unsafe extern "system" fn NotifyAddProgress<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncompletedsteps: i32, ntotalsteps: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyAddProgress(::core::mem::transmute_copy(&ncompletedsteps), ::core::mem::transmute_copy(&ntotalsteps)).into()
        }
        unsafe extern "system" fn NotifyBlockProgress<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncompleted: i32, ntotal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyBlockProgress(::core::mem::transmute_copy(&ncompleted), ::core::mem::transmute_copy(&ntotal)).into()
        }
        unsafe extern "system" fn NotifyTrackProgress<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncurrenttrack: i32, ntotaltracks: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyTrackProgress(::core::mem::transmute_copy(&ncurrenttrack), ::core::mem::transmute_copy(&ntotaltracks)).into()
        }
        unsafe extern "system" fn NotifyPreparingBurn<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nestimatedseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyPreparingBurn(::core::mem::transmute_copy(&nestimatedseconds)).into()
        }
        unsafe extern "system" fn NotifyClosingDisc<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nestimatedseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyClosingDisc(::core::mem::transmute_copy(&nestimatedseconds)).into()
        }
        unsafe extern "system" fn NotifyBurnComplete<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyBurnComplete(::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn NotifyEraseComplete<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyEraseComplete(::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueryCancel: QueryCancel::<Identity, Impl, OFFSET>,
            NotifyPnPActivity: NotifyPnPActivity::<Identity, Impl, OFFSET>,
            NotifyAddProgress: NotifyAddProgress::<Identity, Impl, OFFSET>,
            NotifyBlockProgress: NotifyBlockProgress::<Identity, Impl, OFFSET>,
            NotifyTrackProgress: NotifyTrackProgress::<Identity, Impl, OFFSET>,
            NotifyPreparingBurn: NotifyPreparingBurn::<Identity, Impl, OFFSET>,
            NotifyClosingDisc: NotifyClosingDisc::<Identity, Impl, OFFSET>,
            NotifyBurnComplete: NotifyBurnComplete::<Identity, Impl, OFFSET>,
            NotifyEraseComplete: NotifyEraseComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscMasterProgressEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IDiscRecorder_Impl: Sized {
    fn Init(&mut self, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> ::windows::core::Result<()>;
    fn GetRecorderGUID(&mut self, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> ::windows::core::Result<()>;
    fn GetRecorderType(&mut self) -> ::windows::core::Result<RECORDER_TYPES>;
    fn GetDisplayNames(&mut self, pbstrvendorid: *mut super::super::Foundation::BSTR, pbstrproductid: *mut super::super::Foundation::BSTR, pbstrrevision: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetBasePnPID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetRecorderProperties(&mut self) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage>;
    fn SetRecorderProperties(&mut self, ppropstg: &::core::option::Option<super::super::System::Com::StructuredStorage::IPropertyStorage>) -> ::windows::core::Result<()>;
    fn GetRecorderState(&mut self) -> ::windows::core::Result<DISC_RECORDER_STATE_FLAGS>;
    fn OpenExclusive(&mut self) -> ::windows::core::Result<()>;
    fn QueryMediaType(&mut self, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> ::windows::core::Result<()>;
    fn QueryMediaInfo(&mut self, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> ::windows::core::Result<()>;
    fn Eject(&mut self) -> ::windows::core::Result<()>;
    fn Erase(&mut self, bfullerase: u8) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IDiscRecorder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder_Impl, const OFFSET: isize>() -> IDiscRecorder_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&pbyuniqueid), ::core::mem::transmute_copy(&nulidsize), ::core::mem::transmute_copy(&nuldrivenumber)).into()
        }
        unsafe extern "system" fn GetRecorderGUID<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRecorderGUID(::core::mem::transmute_copy(&pbyuniqueid), ::core::mem::transmute_copy(&ulbuffersize), ::core::mem::transmute_copy(&pulreturnsizerequired)).into()
        }
        unsafe extern "system" fn GetRecorderType<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftypecode: *mut RECORDER_TYPES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRecorderType() {
                ::core::result::Result::Ok(ok__) => {
                    *ftypecode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayNames<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrvendorid: *mut super::super::Foundation::BSTR, pbstrproductid: *mut super::super::Foundation::BSTR, pbstrrevision: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplayNames(::core::mem::transmute_copy(&pbstrvendorid), ::core::mem::transmute_copy(&pbstrproductid), ::core::mem::transmute_copy(&pbstrrevision)).into()
        }
        unsafe extern "system" fn GetBasePnPID<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbasepnpid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBasePnPID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrbasepnpid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPath<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecorderProperties<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRecorderProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropstg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecorderProperties<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRecorderProperties(::core::mem::transmute(&ppropstg)).into()
        }
        unsafe extern "system" fn GetRecorderState<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puldevstateflags: *mut DISC_RECORDER_STATE_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRecorderState() {
                ::core::result::Result::Ok(ok__) => {
                    *puldevstateflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenExclusive<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenExclusive().into()
        }
        unsafe extern "system" fn QueryMediaType<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryMediaType(::core::mem::transmute_copy(&fmediatype), ::core::mem::transmute_copy(&fmediaflags)).into()
        }
        unsafe extern "system" fn QueryMediaInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryMediaInfo(::core::mem::transmute_copy(&pbsessions), ::core::mem::transmute_copy(&pblasttrack), ::core::mem::transmute_copy(&ulstartaddress), ::core::mem::transmute_copy(&ulnextwritable), ::core::mem::transmute_copy(&ulfreeblocks)).into()
        }
        unsafe extern "system" fn Eject<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Eject().into()
        }
        unsafe extern "system" fn Erase<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullerase: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Erase(::core::mem::transmute_copy(&bfullerase)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            GetRecorderGUID: GetRecorderGUID::<Identity, Impl, OFFSET>,
            GetRecorderType: GetRecorderType::<Identity, Impl, OFFSET>,
            GetDisplayNames: GetDisplayNames::<Identity, Impl, OFFSET>,
            GetBasePnPID: GetBasePnPID::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetRecorderProperties: GetRecorderProperties::<Identity, Impl, OFFSET>,
            SetRecorderProperties: SetRecorderProperties::<Identity, Impl, OFFSET>,
            GetRecorderState: GetRecorderState::<Identity, Impl, OFFSET>,
            OpenExclusive: OpenExclusive::<Identity, Impl, OFFSET>,
            QueryMediaType: QueryMediaType::<Identity, Impl, OFFSET>,
            QueryMediaInfo: QueryMediaInfo::<Identity, Impl, OFFSET>,
            Eject: Eject::<Identity, Impl, OFFSET>,
            Erase: Erase::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscRecorder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscRecorder2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EjectMedia(&mut self) -> ::windows::core::Result<()>;
    fn CloseTray(&mut self) -> ::windows::core::Result<()>;
    fn AcquireExclusiveAccess(&mut self, force: i16, __midl__idiscrecorder20000: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ReleaseExclusiveAccess(&mut self) -> ::windows::core::Result<()>;
    fn DisableMcn(&mut self) -> ::windows::core::Result<()>;
    fn EnableMcn(&mut self) -> ::windows::core::Result<()>;
    fn InitializeDiscRecorder(&mut self, recorderuniqueid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ActiveDiscRecorder(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn VendorId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ProductId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ProductRevision(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn VolumeName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn VolumePathNames(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn DeviceCanLoadMedia(&mut self) -> ::windows::core::Result<i16>;
    fn LegacyDeviceNumber(&mut self) -> ::windows::core::Result<i32>;
    fn SupportedFeaturePages(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentFeaturePages(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedProfiles(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentProfiles(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedModePages(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ExclusiveAccessOwner(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscRecorder2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>() -> IDiscRecorder2_Vtbl {
        unsafe extern "system" fn EjectMedia<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EjectMedia().into()
        }
        unsafe extern "system" fn CloseTray<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CloseTray().into()
        }
        unsafe extern "system" fn AcquireExclusiveAccess<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, force: i16, __midl__idiscrecorder20000: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AcquireExclusiveAccess(::core::mem::transmute_copy(&force), ::core::mem::transmute_copy(&__midl__idiscrecorder20000)).into()
        }
        unsafe extern "system" fn ReleaseExclusiveAccess<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseExclusiveAccess().into()
        }
        unsafe extern "system" fn DisableMcn<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisableMcn().into()
        }
        unsafe extern "system" fn EnableMcn<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableMcn().into()
        }
        unsafe extern "system" fn InitializeDiscRecorder<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recorderuniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeDiscRecorder(::core::mem::transmute_copy(&recorderuniqueid)).into()
        }
        unsafe extern "system" fn ActiveDiscRecorder<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ActiveDiscRecorder() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VendorId<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VendorId() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductId<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProductId() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductRevision<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProductRevision() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeName<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VolumeName() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumePathNames<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VolumePathNames() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceCanLoadMedia<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeviceCanLoadMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LegacyDeviceNumber<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, legacydevicenumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LegacyDeviceNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *legacydevicenumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedFeaturePages<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedFeaturePages() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFeaturePages<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentFeaturePages() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedProfiles<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentProfiles<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedModePages<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedModePages() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExclusiveAccessOwner<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExclusiveAccessOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EjectMedia: EjectMedia::<Identity, Impl, OFFSET>,
            CloseTray: CloseTray::<Identity, Impl, OFFSET>,
            AcquireExclusiveAccess: AcquireExclusiveAccess::<Identity, Impl, OFFSET>,
            ReleaseExclusiveAccess: ReleaseExclusiveAccess::<Identity, Impl, OFFSET>,
            DisableMcn: DisableMcn::<Identity, Impl, OFFSET>,
            EnableMcn: EnableMcn::<Identity, Impl, OFFSET>,
            InitializeDiscRecorder: InitializeDiscRecorder::<Identity, Impl, OFFSET>,
            ActiveDiscRecorder: ActiveDiscRecorder::<Identity, Impl, OFFSET>,
            VendorId: VendorId::<Identity, Impl, OFFSET>,
            ProductId: ProductId::<Identity, Impl, OFFSET>,
            ProductRevision: ProductRevision::<Identity, Impl, OFFSET>,
            VolumeName: VolumeName::<Identity, Impl, OFFSET>,
            VolumePathNames: VolumePathNames::<Identity, Impl, OFFSET>,
            DeviceCanLoadMedia: DeviceCanLoadMedia::<Identity, Impl, OFFSET>,
            LegacyDeviceNumber: LegacyDeviceNumber::<Identity, Impl, OFFSET>,
            SupportedFeaturePages: SupportedFeaturePages::<Identity, Impl, OFFSET>,
            CurrentFeaturePages: CurrentFeaturePages::<Identity, Impl, OFFSET>,
            SupportedProfiles: SupportedProfiles::<Identity, Impl, OFFSET>,
            CurrentProfiles: CurrentProfiles::<Identity, Impl, OFFSET>,
            SupportedModePages: SupportedModePages::<Identity, Impl, OFFSET>,
            ExclusiveAccessOwner: ExclusiveAccessOwner::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscRecorder2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDiscRecorder2Ex_Impl: Sized {
    fn SendCommandNoData(&mut self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> ::windows::core::Result<()>;
    fn SendCommandSendDataToDevice(&mut self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> ::windows::core::Result<()>;
    fn SendCommandGetDataFromDevice(&mut self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> ::windows::core::Result<()>;
    fn ReadDvdStructure(&mut self, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()>;
    fn SendDvdStructure(&mut self, format: u32, data: *const u8, count: u32) -> ::windows::core::Result<()>;
    fn GetAdapterDescriptor(&mut self, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::Result<()>;
    fn GetDeviceDescriptor(&mut self, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::Result<()>;
    fn GetDiscInformation(&mut self, discinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::Result<()>;
    fn GetTrackInformation(&mut self, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::Result<()>;
    fn GetFeaturePage(&mut self, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::Result<()>;
    fn GetModePage(&mut self, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::Result<()>;
    fn SetModePage(&mut self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> ::windows::core::Result<()>;
    fn GetSupportedFeaturePages(&mut self, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> ::windows::core::Result<()>;
    fn GetSupportedProfiles(&mut self, currentonly: super::super::Foundation::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> ::windows::core::Result<()>;
    fn GetSupportedModePages(&mut self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> ::windows::core::Result<()>;
    fn GetByteAlignmentMask(&mut self) -> ::windows::core::Result<u32>;
    fn GetMaximumNonPageAlignedTransferSize(&mut self) -> ::windows::core::Result<u32>;
    fn GetMaximumPageAlignedTransferSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDiscRecorder2Ex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>() -> IDiscRecorder2Ex_Vtbl {
        unsafe extern "system" fn SendCommandNoData<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendCommandNoData(::core::mem::transmute_copy(&cdb), ::core::mem::transmute_copy(&cdbsize), ::core::mem::transmute_copy(&sensebuffer), ::core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn SendCommandSendDataToDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendCommandSendDataToDevice(::core::mem::transmute_copy(&cdb), ::core::mem::transmute_copy(&cdbsize), ::core::mem::transmute_copy(&sensebuffer), ::core::mem::transmute_copy(&timeout), ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&buffersize)).into()
        }
        unsafe extern "system" fn SendCommandGetDataFromDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendCommandGetDataFromDevice(::core::mem::transmute_copy(&cdb), ::core::mem::transmute_copy(&cdbsize), ::core::mem::transmute_copy(&sensebuffer), ::core::mem::transmute_copy(&timeout), ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&buffersize), ::core::mem::transmute_copy(&bufferfetched)).into()
        }
        unsafe extern "system" fn ReadDvdStructure<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReadDvdStructure(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&layer), ::core::mem::transmute_copy(&agid), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SendDvdStructure<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: u32, data: *const u8, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendDvdStructure(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetAdapterDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAdapterDescriptor(::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetDeviceDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceDescriptor(::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetDiscInformation<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDiscInformation(::core::mem::transmute_copy(&discinformation), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetTrackInformation<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTrackInformation(::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&addresstype), ::core::mem::transmute_copy(&trackinformation), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetFeaturePage<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFeaturePage(::core::mem::transmute_copy(&requestedfeature), ::core::mem::transmute_copy(&currentfeatureonly), ::core::mem::transmute_copy(&featuredata), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetModePage<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetModePage(::core::mem::transmute_copy(&requestedmodepage), ::core::mem::transmute_copy(&requesttype), ::core::mem::transmute_copy(&modepagedata), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn SetModePage<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetModePage(::core::mem::transmute_copy(&requesttype), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetSupportedFeaturePages<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSupportedFeaturePages(::core::mem::transmute_copy(&currentfeatureonly), ::core::mem::transmute_copy(&featuredata), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetSupportedProfiles<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentonly: super::super::Foundation::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSupportedProfiles(::core::mem::transmute_copy(&currentonly), ::core::mem::transmute_copy(&profiletypes), ::core::mem::transmute_copy(&validprofiles)).into()
        }
        unsafe extern "system" fn GetSupportedModePages<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSupportedModePages(::core::mem::transmute_copy(&requesttype), ::core::mem::transmute_copy(&modepagetypes), ::core::mem::transmute_copy(&validpages)).into()
        }
        unsafe extern "system" fn GetByteAlignmentMask<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetByteAlignmentMask() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumNonPageAlignedTransferSize<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaximumNonPageAlignedTransferSize() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumPageAlignedTransferSize<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaximumPageAlignedTransferSize() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SendCommandNoData: SendCommandNoData::<Identity, Impl, OFFSET>,
            SendCommandSendDataToDevice: SendCommandSendDataToDevice::<Identity, Impl, OFFSET>,
            SendCommandGetDataFromDevice: SendCommandGetDataFromDevice::<Identity, Impl, OFFSET>,
            ReadDvdStructure: ReadDvdStructure::<Identity, Impl, OFFSET>,
            SendDvdStructure: SendDvdStructure::<Identity, Impl, OFFSET>,
            GetAdapterDescriptor: GetAdapterDescriptor::<Identity, Impl, OFFSET>,
            GetDeviceDescriptor: GetDeviceDescriptor::<Identity, Impl, OFFSET>,
            GetDiscInformation: GetDiscInformation::<Identity, Impl, OFFSET>,
            GetTrackInformation: GetTrackInformation::<Identity, Impl, OFFSET>,
            GetFeaturePage: GetFeaturePage::<Identity, Impl, OFFSET>,
            GetModePage: GetModePage::<Identity, Impl, OFFSET>,
            SetModePage: SetModePage::<Identity, Impl, OFFSET>,
            GetSupportedFeaturePages: GetSupportedFeaturePages::<Identity, Impl, OFFSET>,
            GetSupportedProfiles: GetSupportedProfiles::<Identity, Impl, OFFSET>,
            GetSupportedModePages: GetSupportedModePages::<Identity, Impl, OFFSET>,
            GetByteAlignmentMask: GetByteAlignmentMask::<Identity, Impl, OFFSET>,
            GetMaximumNonPageAlignedTransferSize: GetMaximumNonPageAlignedTransferSize::<Identity, Impl, OFFSET>,
            GetMaximumPageAlignedTransferSize: GetMaximumPageAlignedTransferSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscRecorder2Ex as ::windows::core::Interface>::IID
    }
}
pub trait IEnumDiscMasterFormats_Impl: Sized {
    fn Next(&mut self, cformats: u32, lpiidformatid: *mut ::windows::core::GUID, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cformats: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumDiscMasterFormats>;
}
impl IEnumDiscMasterFormats_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDiscMasterFormats_Impl, const OFFSET: isize>() -> IEnumDiscMasterFormats_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32, lpiidformatid: *mut ::windows::core::GUID, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cformats), ::core::mem::transmute_copy(&lpiidformatid), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cformats)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDiscMasterFormats as ::windows::core::Interface>::IID
    }
}
pub trait IEnumDiscRecorders_Impl: Sized {
    fn Next(&mut self, crecorders: u32, pprecorder: *mut ::core::option::Option<IDiscRecorder>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, crecorders: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumDiscRecorders>;
}
impl IEnumDiscRecorders_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDiscRecorders_Impl, const OFFSET: isize>() -> IEnumDiscRecorders_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crecorders: u32, pprecorder: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&crecorders), ::core::mem::transmute_copy(&pprecorder), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crecorders: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&crecorders)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDiscRecorders as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumFsiItems_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IFsiItem>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumFsiItems>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumFsiItems_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumFsiItems_Impl, const OFFSET: isize>() -> IEnumFsiItems_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumFsiItems as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumProgressItems_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IProgressItem>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumProgressItems>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumProgressItems_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumProgressItems_Impl, const OFFSET: isize>() -> IEnumProgressItems_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumProgressItems as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFileSystemImage_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Root(&mut self) -> ::windows::core::Result<IFsiDirectoryItem>;
    fn SessionStartBlock(&mut self) -> ::windows::core::Result<i32>;
    fn SetSessionStartBlock(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn FreeMediaBlocks(&mut self) -> ::windows::core::Result<i32>;
    fn SetFreeMediaBlocks(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn SetMaxMediaBlocksFromDevice(&mut self, discrecorder: &::core::option::Option<IDiscRecorder2>) -> ::windows::core::Result<()>;
    fn UsedBlocks(&mut self) -> ::windows::core::Result<i32>;
    fn VolumeName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetVolumeName(&mut self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ImportedVolumeName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BootImageOptions(&mut self) -> ::windows::core::Result<IBootOptions>;
    fn SetBootImageOptions(&mut self, newval: &::core::option::Option<IBootOptions>) -> ::windows::core::Result<()>;
    fn FileCount(&mut self) -> ::windows::core::Result<i32>;
    fn DirectoryCount(&mut self) -> ::windows::core::Result<i32>;
    fn WorkingDirectory(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetWorkingDirectory(&mut self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ChangePoint(&mut self) -> ::windows::core::Result<i32>;
    fn StrictFileSystemCompliance(&mut self) -> ::windows::core::Result<i16>;
    fn SetStrictFileSystemCompliance(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn UseRestrictedCharacterSet(&mut self) -> ::windows::core::Result<i16>;
    fn SetUseRestrictedCharacterSet(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn FileSystemsToCreate(&mut self) -> ::windows::core::Result<FsiFileSystems>;
    fn SetFileSystemsToCreate(&mut self, newval: FsiFileSystems) -> ::windows::core::Result<()>;
    fn FileSystemsSupported(&mut self) -> ::windows::core::Result<FsiFileSystems>;
    fn SetUDFRevision(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn UDFRevision(&mut self) -> ::windows::core::Result<i32>;
    fn UDFRevisionsSupported(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ChooseImageDefaults(&mut self, discrecorder: &::core::option::Option<IDiscRecorder2>) -> ::windows::core::Result<()>;
    fn ChooseImageDefaultsForMediaType(&mut self, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::Result<()>;
    fn SetISO9660InterchangeLevel(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn ISO9660InterchangeLevel(&mut self) -> ::windows::core::Result<i32>;
    fn ISO9660InterchangeLevelsSupported(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CreateResultImage(&mut self) -> ::windows::core::Result<IFileSystemImageResult>;
    fn Exists(&mut self, fullpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<FsiItemType>;
    fn CalculateDiscIdentifier(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IdentifyFileSystemsOnDisc(&mut self, discrecorder: &::core::option::Option<IDiscRecorder2>) -> ::windows::core::Result<FsiFileSystems>;
    fn GetDefaultFileSystemForImport(&mut self, filesystems: FsiFileSystems) -> ::windows::core::Result<FsiFileSystems>;
    fn ImportFileSystem(&mut self) -> ::windows::core::Result<FsiFileSystems>;
    fn ImportSpecificFileSystem(&mut self, filesystemtouse: FsiFileSystems) -> ::windows::core::Result<()>;
    fn RollbackToChangePoint(&mut self, changepoint: i32) -> ::windows::core::Result<()>;
    fn LockInChangePoint(&mut self) -> ::windows::core::Result<()>;
    fn CreateDirectoryItem(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsiDirectoryItem>;
    fn CreateFileItem(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsiFileItem>;
    fn VolumeNameUDF(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn VolumeNameJoliet(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn VolumeNameISO9660(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn StageFiles(&mut self) -> ::windows::core::Result<i16>;
    fn SetStageFiles(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn MultisessionInterfaces(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetMultisessionInterfaces(&mut self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFileSystemImage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>() -> IFileSystemImage_Vtbl {
        unsafe extern "system" fn Root<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Root() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionStartBlock<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionStartBlock() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionStartBlock<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSessionStartBlock(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn FreeMediaBlocks<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FreeMediaBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFreeMediaBlocks<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFreeMediaBlocks(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SetMaxMediaBlocksFromDevice<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discrecorder: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxMediaBlocksFromDevice(::core::mem::transmute(&discrecorder)).into()
        }
        unsafe extern "system" fn UsedBlocks<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UsedBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeName<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VolumeName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolumeName<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVolumeName(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ImportedVolumeName<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ImportedVolumeName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BootImageOptions<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BootImageOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBootImageOptions<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBootImageOptions(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn FileCount<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FileCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectoryCount<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DirectoryCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WorkingDirectory<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WorkingDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWorkingDirectory(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ChangePoint<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ChangePoint() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrictFileSystemCompliance<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StrictFileSystemCompliance() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrictFileSystemCompliance<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStrictFileSystemCompliance(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn UseRestrictedCharacterSet<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UseRestrictedCharacterSet() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseRestrictedCharacterSet<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUseRestrictedCharacterSet(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn FileSystemsToCreate<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FileSystemsToCreate() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileSystemsToCreate<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFileSystemsToCreate(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn FileSystemsSupported<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FileSystemsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUDFRevision<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUDFRevision(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn UDFRevision<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UDFRevision() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UDFRevisionsSupported<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UDFRevisionsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChooseImageDefaults<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discrecorder: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChooseImageDefaults(::core::mem::transmute(&discrecorder)).into()
        }
        unsafe extern "system" fn ChooseImageDefaultsForMediaType<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChooseImageDefaultsForMediaType(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetISO9660InterchangeLevel<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetISO9660InterchangeLevel(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ISO9660InterchangeLevel<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ISO9660InterchangeLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ISO9660InterchangeLevelsSupported<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ISO9660InterchangeLevelsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResultImage<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateResultImage() {
                ::core::result::Result::Ok(ok__) => {
                    *resultstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Exists<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemtype: *mut FsiItemType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Exists(::core::mem::transmute_copy(&fullpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *itemtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalculateDiscIdentifier<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discidentifier: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CalculateDiscIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *discidentifier = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdentifyFileSystemsOnDisc<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discrecorder: ::windows::core::RawPtr, filesystems: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IdentifyFileSystemsOnDisc(::core::mem::transmute(&discrecorder)) {
                ::core::result::Result::Ok(ok__) => {
                    *filesystems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultFileSystemForImport<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystems: FsiFileSystems, importdefault: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDefaultFileSystemForImport(::core::mem::transmute_copy(&filesystems)) {
                ::core::result::Result::Ok(ok__) => {
                    *importdefault = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportFileSystem<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, importedfilesystem: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ImportFileSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *importedfilesystem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportSpecificFileSystem<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystemtouse: FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ImportSpecificFileSystem(::core::mem::transmute_copy(&filesystemtouse)).into()
        }
        unsafe extern "system" fn RollbackToChangePoint<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changepoint: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RollbackToChangePoint(::core::mem::transmute_copy(&changepoint)).into()
        }
        unsafe extern "system" fn LockInChangePoint<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LockInChangePoint().into()
        }
        unsafe extern "system" fn CreateDirectoryItem<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDirectoryItem(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *newitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileItem<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFileItem(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *newitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeNameUDF<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VolumeNameUDF() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeNameJoliet<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VolumeNameJoliet() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeNameISO9660<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VolumeNameISO9660() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StageFiles<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StageFiles() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStageFiles<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStageFiles(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn MultisessionInterfaces<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MultisessionInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultisessionInterfaces<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMultisessionInterfaces(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Root: Root::<Identity, Impl, OFFSET>,
            SessionStartBlock: SessionStartBlock::<Identity, Impl, OFFSET>,
            SetSessionStartBlock: SetSessionStartBlock::<Identity, Impl, OFFSET>,
            FreeMediaBlocks: FreeMediaBlocks::<Identity, Impl, OFFSET>,
            SetFreeMediaBlocks: SetFreeMediaBlocks::<Identity, Impl, OFFSET>,
            SetMaxMediaBlocksFromDevice: SetMaxMediaBlocksFromDevice::<Identity, Impl, OFFSET>,
            UsedBlocks: UsedBlocks::<Identity, Impl, OFFSET>,
            VolumeName: VolumeName::<Identity, Impl, OFFSET>,
            SetVolumeName: SetVolumeName::<Identity, Impl, OFFSET>,
            ImportedVolumeName: ImportedVolumeName::<Identity, Impl, OFFSET>,
            BootImageOptions: BootImageOptions::<Identity, Impl, OFFSET>,
            SetBootImageOptions: SetBootImageOptions::<Identity, Impl, OFFSET>,
            FileCount: FileCount::<Identity, Impl, OFFSET>,
            DirectoryCount: DirectoryCount::<Identity, Impl, OFFSET>,
            WorkingDirectory: WorkingDirectory::<Identity, Impl, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, Impl, OFFSET>,
            ChangePoint: ChangePoint::<Identity, Impl, OFFSET>,
            StrictFileSystemCompliance: StrictFileSystemCompliance::<Identity, Impl, OFFSET>,
            SetStrictFileSystemCompliance: SetStrictFileSystemCompliance::<Identity, Impl, OFFSET>,
            UseRestrictedCharacterSet: UseRestrictedCharacterSet::<Identity, Impl, OFFSET>,
            SetUseRestrictedCharacterSet: SetUseRestrictedCharacterSet::<Identity, Impl, OFFSET>,
            FileSystemsToCreate: FileSystemsToCreate::<Identity, Impl, OFFSET>,
            SetFileSystemsToCreate: SetFileSystemsToCreate::<Identity, Impl, OFFSET>,
            FileSystemsSupported: FileSystemsSupported::<Identity, Impl, OFFSET>,
            SetUDFRevision: SetUDFRevision::<Identity, Impl, OFFSET>,
            UDFRevision: UDFRevision::<Identity, Impl, OFFSET>,
            UDFRevisionsSupported: UDFRevisionsSupported::<Identity, Impl, OFFSET>,
            ChooseImageDefaults: ChooseImageDefaults::<Identity, Impl, OFFSET>,
            ChooseImageDefaultsForMediaType: ChooseImageDefaultsForMediaType::<Identity, Impl, OFFSET>,
            SetISO9660InterchangeLevel: SetISO9660InterchangeLevel::<Identity, Impl, OFFSET>,
            ISO9660InterchangeLevel: ISO9660InterchangeLevel::<Identity, Impl, OFFSET>,
            ISO9660InterchangeLevelsSupported: ISO9660InterchangeLevelsSupported::<Identity, Impl, OFFSET>,
            CreateResultImage: CreateResultImage::<Identity, Impl, OFFSET>,
            Exists: Exists::<Identity, Impl, OFFSET>,
            CalculateDiscIdentifier: CalculateDiscIdentifier::<Identity, Impl, OFFSET>,
            IdentifyFileSystemsOnDisc: IdentifyFileSystemsOnDisc::<Identity, Impl, OFFSET>,
            GetDefaultFileSystemForImport: GetDefaultFileSystemForImport::<Identity, Impl, OFFSET>,
            ImportFileSystem: ImportFileSystem::<Identity, Impl, OFFSET>,
            ImportSpecificFileSystem: ImportSpecificFileSystem::<Identity, Impl, OFFSET>,
            RollbackToChangePoint: RollbackToChangePoint::<Identity, Impl, OFFSET>,
            LockInChangePoint: LockInChangePoint::<Identity, Impl, OFFSET>,
            CreateDirectoryItem: CreateDirectoryItem::<Identity, Impl, OFFSET>,
            CreateFileItem: CreateFileItem::<Identity, Impl, OFFSET>,
            VolumeNameUDF: VolumeNameUDF::<Identity, Impl, OFFSET>,
            VolumeNameJoliet: VolumeNameJoliet::<Identity, Impl, OFFSET>,
            VolumeNameISO9660: VolumeNameISO9660::<Identity, Impl, OFFSET>,
            StageFiles: StageFiles::<Identity, Impl, OFFSET>,
            SetStageFiles: SetStageFiles::<Identity, Impl, OFFSET>,
            MultisessionInterfaces: MultisessionInterfaces::<Identity, Impl, OFFSET>,
            SetMultisessionInterfaces: SetMultisessionInterfaces::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSystemImage as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFileSystemImage2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFileSystemImage_Impl {
    fn BootImageOptionsArray(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetBootImageOptionsArray(&mut self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFileSystemImage2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage2_Impl, const OFFSET: isize>() -> IFileSystemImage2_Vtbl {
        unsafe extern "system" fn BootImageOptionsArray<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BootImageOptionsArray() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBootImageOptionsArray<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBootImageOptionsArray(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base: IFileSystemImage_Vtbl::new::<Identity, Impl, OFFSET>(),
            BootImageOptionsArray: BootImageOptionsArray::<Identity, Impl, OFFSET>,
            SetBootImageOptionsArray: SetBootImageOptionsArray::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSystemImage2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IFileSystemImage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFileSystemImage3_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFileSystemImage_Impl + IFileSystemImage2_Impl {
    fn CreateRedundantUdfMetadataFiles(&mut self) -> ::windows::core::Result<i16>;
    fn SetCreateRedundantUdfMetadataFiles(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn ProbeSpecificFileSystem(&mut self, filesystemtoprobe: FsiFileSystems) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFileSystemImage3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage3_Impl, const OFFSET: isize>() -> IFileSystemImage3_Vtbl {
        unsafe extern "system" fn CreateRedundantUdfMetadataFiles<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRedundantUdfMetadataFiles() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreateRedundantUdfMetadataFiles<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCreateRedundantUdfMetadataFiles(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ProbeSpecificFileSystem<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystemtoprobe: FsiFileSystems, isappendable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProbeSpecificFileSystem(::core::mem::transmute_copy(&filesystemtoprobe)) {
                ::core::result::Result::Ok(ok__) => {
                    *isappendable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IFileSystemImage2_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateRedundantUdfMetadataFiles: CreateRedundantUdfMetadataFiles::<Identity, Impl, OFFSET>,
            SetCreateRedundantUdfMetadataFiles: SetCreateRedundantUdfMetadataFiles::<Identity, Impl, OFFSET>,
            ProbeSpecificFileSystem: ProbeSpecificFileSystem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSystemImage3 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IFileSystemImage as ::windows::core::Interface>::IID || iid == &<IFileSystemImage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFileSystemImageResult_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ImageStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn ProgressItems(&mut self) -> ::windows::core::Result<IProgressItems>;
    fn TotalBlocks(&mut self) -> ::windows::core::Result<i32>;
    fn BlockSize(&mut self) -> ::windows::core::Result<i32>;
    fn DiscId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFileSystemImageResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImageResult_Impl, const OFFSET: isize>() -> IFileSystemImageResult_Vtbl {
        unsafe extern "system" fn ImageStream<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ImageStream() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressItems<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProgressItems() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalBlocks<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockSize<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BlockSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscId<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DiscId() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ImageStream: ImageStream::<Identity, Impl, OFFSET>,
            ProgressItems: ProgressItems::<Identity, Impl, OFFSET>,
            TotalBlocks: TotalBlocks::<Identity, Impl, OFFSET>,
            BlockSize: BlockSize::<Identity, Impl, OFFSET>,
            DiscId: DiscId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSystemImageResult as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFileSystemImageResult2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFileSystemImageResult_Impl {
    fn ModifiedBlocks(&mut self) -> ::windows::core::Result<IBlockRangeList>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFileSystemImageResult2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImageResult2_Impl, const OFFSET: isize>() -> IFileSystemImageResult2_Vtbl {
        unsafe extern "system" fn ModifiedBlocks<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImageResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ModifiedBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IFileSystemImageResult_Vtbl::new::<Identity, Impl, OFFSET>(), ModifiedBlocks: ModifiedBlocks::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSystemImageResult2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IFileSystemImageResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsiDirectoryItem_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsiItem_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Item(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsiItem>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn EnumFsiItems(&mut self) -> ::windows::core::Result<IEnumFsiItems>;
    fn AddDirectory(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddFile(&mut self, path: &super::super::Foundation::BSTR, filedata: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn AddTree(&mut self, sourcedirectory: &super::super::Foundation::BSTR, includebasedirectory: i16) -> ::windows::core::Result<()>;
    fn Add(&mut self, item: &::core::option::Option<IFsiItem>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveTree(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsiDirectoryItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>() -> IFsiDirectoryItem_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFsiItems<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumFsiItems() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDirectory<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDirectory(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn AddFile<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filedata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddFile(::core::mem::transmute_copy(&path), ::core::mem::transmute(&filedata)).into()
        }
        unsafe extern "system" fn AddTree<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, includebasedirectory: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddTree(::core::mem::transmute_copy(&sourcedirectory), ::core::mem::transmute_copy(&includebasedirectory)).into()
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&item)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn RemoveTree<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveTree(::core::mem::transmute_copy(&path)).into()
        }
        Self {
            base: IFsiItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            EnumFsiItems: EnumFsiItems::<Identity, Impl, OFFSET>,
            AddDirectory: AddDirectory::<Identity, Impl, OFFSET>,
            AddFile: AddFile::<Identity, Impl, OFFSET>,
            AddTree: AddTree::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveTree: RemoveTree::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsiDirectoryItem as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IFsiItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsiDirectoryItem2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsiItem_Impl + IFsiDirectoryItem_Impl {
    fn AddTreeWithNamedStreams(&mut self, sourcedirectory: &super::super::Foundation::BSTR, includebasedirectory: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsiDirectoryItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItem2_Impl, const OFFSET: isize>() -> IFsiDirectoryItem2_Vtbl {
        unsafe extern "system" fn AddTreeWithNamedStreams<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, includebasedirectory: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddTreeWithNamedStreams(::core::mem::transmute_copy(&sourcedirectory), ::core::mem::transmute_copy(&includebasedirectory)).into()
        }
        Self { base: IFsiDirectoryItem_Vtbl::new::<Identity, Impl, OFFSET>(), AddTreeWithNamedStreams: AddTreeWithNamedStreams::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsiDirectoryItem2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IFsiItem as ::windows::core::Interface>::IID || iid == &<IFsiDirectoryItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsiFileItem_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsiItem_Impl {
    fn DataSize(&mut self) -> ::windows::core::Result<i64>;
    fn DataSize32BitLow(&mut self) -> ::windows::core::Result<i32>;
    fn DataSize32BitHigh(&mut self) -> ::windows::core::Result<i32>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetData(&mut self, newval: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsiFileItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItem_Impl, const OFFSET: isize>() -> IFsiFileItem_Vtbl {
        unsafe extern "system" fn DataSize<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DataSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataSize32BitLow<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DataSize32BitLow() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataSize32BitHigh<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DataSize32BitHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetData(::core::mem::transmute(&newval)).into()
        }
        Self {
            base: IFsiItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            DataSize: DataSize::<Identity, Impl, OFFSET>,
            DataSize32BitLow: DataSize32BitLow::<Identity, Impl, OFFSET>,
            DataSize32BitHigh: DataSize32BitHigh::<Identity, Impl, OFFSET>,
            Data: Data::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsiFileItem as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IFsiItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsiFileItem2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsiItem_Impl + IFsiFileItem_Impl {
    fn FsiNamedStreams(&mut self) -> ::windows::core::Result<IFsiNamedStreams>;
    fn IsNamedStream(&mut self) -> ::windows::core::Result<i16>;
    fn AddStream(&mut self, name: &super::super::Foundation::BSTR, streamdata: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn RemoveStream(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsRealTime(&mut self) -> ::windows::core::Result<i16>;
    fn SetIsRealTime(&mut self, newval: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsiFileItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItem2_Impl, const OFFSET: isize>() -> IFsiFileItem2_Vtbl {
        unsafe extern "system" fn FsiNamedStreams<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FsiNamedStreams() {
                ::core::result::Result::Ok(ok__) => {
                    *streams = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNamedStream<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsNamedStream() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStream<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, streamdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddStream(::core::mem::transmute_copy(&name), ::core::mem::transmute(&streamdata)).into()
        }
        unsafe extern "system" fn RemoveStream<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveStream(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn IsRealTime<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsRealTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRealTime<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIsRealTime(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base: IFsiFileItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            FsiNamedStreams: FsiNamedStreams::<Identity, Impl, OFFSET>,
            IsNamedStream: IsNamedStream::<Identity, Impl, OFFSET>,
            AddStream: AddStream::<Identity, Impl, OFFSET>,
            RemoveStream: RemoveStream::<Identity, Impl, OFFSET>,
            IsRealTime: IsRealTime::<Identity, Impl, OFFSET>,
            SetIsRealTime: SetIsRealTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsiFileItem2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IFsiItem as ::windows::core::Interface>::IID || iid == &<IFsiFileItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsiItem_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn FullPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreationTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetCreationTime(&mut self, newval: f64) -> ::windows::core::Result<()>;
    fn LastAccessedTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetLastAccessedTime(&mut self, newval: f64) -> ::windows::core::Result<()>;
    fn LastModifiedTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetLastModifiedTime(&mut self, newval: f64) -> ::windows::core::Result<()>;
    fn IsHidden(&mut self) -> ::windows::core::Result<i16>;
    fn SetIsHidden(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn FileSystemName(&mut self, filesystem: FsiFileSystems) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn FileSystemPath(&mut self, filesystem: FsiFileSystems) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsiItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiItem_Impl, const OFFSET: isize>() -> IFsiItem_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FullPath<Identity: ::windows::core::IUnknownImpl, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FullPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreationTime<Identity: ::windows::core::IUnknownImpl, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreationTime<Identity: ::windows::core::IUnknownImpl, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCreationTime(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn LastAccessedTime<Identity: ::windows::core::IUnknownImpl, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastAccessedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastAccessedTime<Identity: ::windows::core::IUnknownImpl, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLastAccessedTime(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn LastModifiedTime<Identity: ::windows::core::IUnknownImpl, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastModifiedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastModifiedTime<Identity: ::windows::core::IUnknownImpl, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLastModifiedTime(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn IsHidden<Identity: ::windows::core::IUnknownImpl, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsHidden() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHidden<Identity: ::windows::core::IUnknownImpl, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIsHidden(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn FileSystemName<Identity: ::windows::core::IUnknownImpl, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FileSystemName(::core::mem::transmute_copy(&filesystem)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSystemPath<Identity: ::windows::core::IUnknownImpl, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FileSystemPath(::core::mem::transmute_copy(&filesystem)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            FullPath: FullPath::<Identity, Impl, OFFSET>,
            CreationTime: CreationTime::<Identity, Impl, OFFSET>,
            SetCreationTime: SetCreationTime::<Identity, Impl, OFFSET>,
            LastAccessedTime: LastAccessedTime::<Identity, Impl, OFFSET>,
            SetLastAccessedTime: SetLastAccessedTime::<Identity, Impl, OFFSET>,
            LastModifiedTime: LastModifiedTime::<Identity, Impl, OFFSET>,
            SetLastModifiedTime: SetLastModifiedTime::<Identity, Impl, OFFSET>,
            IsHidden: IsHidden::<Identity, Impl, OFFSET>,
            SetIsHidden: SetIsHidden::<Identity, Impl, OFFSET>,
            FileSystemName: FileSystemName::<Identity, Impl, OFFSET>,
            FileSystemPath: FileSystemPath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsiItem as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsiNamedStreams_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IFsiFileItem2>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn EnumNamedStreams(&mut self) -> ::windows::core::Result<IEnumFsiItems>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsiNamedStreams_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiNamedStreams_Impl, const OFFSET: isize>() -> IFsiNamedStreams_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumNamedStreams<Identity: ::windows::core::IUnknownImpl, Impl: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumNamedStreams() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            EnumNamedStreams: EnumNamedStreams::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsiNamedStreams as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IIsoImageManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Stream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetPath(&mut self, val: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetStream(&mut self, data: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Validate(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IIsoImageManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIsoImageManager_Impl, const OFFSET: isize>() -> IIsoImageManager_Vtbl {
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Identity: ::windows::core::IUnknownImpl, Impl: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Stream() {
                ::core::result::Result::Ok(ok__) => {
                    *data = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Identity: ::windows::core::IUnknownImpl, Impl: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPath(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn SetStream<Identity: ::windows::core::IUnknownImpl, Impl: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStream(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn Validate<Identity: ::windows::core::IUnknownImpl, Impl: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Validate().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Path: Path::<Identity, Impl, OFFSET>,
            Stream: Stream::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
            SetStream: SetStream::<Identity, Impl, OFFSET>,
            Validate: Validate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIsoImageManager as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IJolietDiscMaster_Impl: Sized {
    fn GetTotalDataBlocks(&mut self) -> ::windows::core::Result<i32>;
    fn GetUsedDataBlocks(&mut self) -> ::windows::core::Result<i32>;
    fn GetDataBlockSize(&mut self) -> ::windows::core::Result<i32>;
    fn AddData(&mut self, pstorage: &::core::option::Option<super::super::System::Com::StructuredStorage::IStorage>, lfileoverwrite: i32) -> ::windows::core::Result<()>;
    fn GetJolietProperties(&mut self) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage>;
    fn SetJolietProperties(&mut self, ppropstg: &::core::option::Option<super::super::System::Com::StructuredStorage::IPropertyStorage>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IJolietDiscMaster_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJolietDiscMaster_Impl, const OFFSET: isize>() -> IJolietDiscMaster_Vtbl {
        unsafe extern "system" fn GetTotalDataBlocks<Identity: ::windows::core::IUnknownImpl, Impl: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTotalDataBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *pnblocks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsedDataBlocks<Identity: ::windows::core::IUnknownImpl, Impl: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUsedDataBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *pnblocks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataBlockSize<Identity: ::windows::core::IUnknownImpl, Impl: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblockbytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDataBlockSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pnblockbytes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddData<Identity: ::windows::core::IUnknownImpl, Impl: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstorage: ::windows::core::RawPtr, lfileoverwrite: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddData(::core::mem::transmute(&pstorage), ::core::mem::transmute_copy(&lfileoverwrite)).into()
        }
        unsafe extern "system" fn GetJolietProperties<Identity: ::windows::core::IUnknownImpl, Impl: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetJolietProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropstg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJolietProperties<Identity: ::windows::core::IUnknownImpl, Impl: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetJolietProperties(::core::mem::transmute(&ppropstg)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetTotalDataBlocks: GetTotalDataBlocks::<Identity, Impl, OFFSET>,
            GetUsedDataBlocks: GetUsedDataBlocks::<Identity, Impl, OFFSET>,
            GetDataBlockSize: GetDataBlockSize::<Identity, Impl, OFFSET>,
            AddData: AddData::<Identity, Impl, OFFSET>,
            GetJolietProperties: GetJolietProperties::<Identity, Impl, OFFSET>,
            SetJolietProperties: SetJolietProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJolietDiscMaster as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMultisession_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IsSupportedOnCurrentMediaState(&mut self) -> ::windows::core::Result<i16>;
    fn SetInUse(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn InUse(&mut self) -> ::windows::core::Result<i16>;
    fn ImportRecorder(&mut self) -> ::windows::core::Result<IDiscRecorder2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMultisession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultisession_Impl, const OFFSET: isize>() -> IMultisession_Vtbl {
        unsafe extern "system" fn IsSupportedOnCurrentMediaState<Identity: ::windows::core::IUnknownImpl, Impl: IMultisession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsSupportedOnCurrentMediaState() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInUse<Identity: ::windows::core::IUnknownImpl, Impl: IMultisession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInUse(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn InUse<Identity: ::windows::core::IUnknownImpl, Impl: IMultisession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InUse() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportRecorder<Identity: ::windows::core::IUnknownImpl, Impl: IMultisession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ImportRecorder() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsSupportedOnCurrentMediaState: IsSupportedOnCurrentMediaState::<Identity, Impl, OFFSET>,
            SetInUse: SetInUse::<Identity, Impl, OFFSET>,
            InUse: InUse::<Identity, Impl, OFFSET>,
            ImportRecorder: ImportRecorder::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultisession as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMultisessionRandomWrite_Impl: Sized + super::super::System::Com::IDispatch_Impl + IMultisession_Impl {
    fn WriteUnitSize(&mut self) -> ::windows::core::Result<i32>;
    fn LastWrittenAddress(&mut self) -> ::windows::core::Result<i32>;
    fn TotalSectorsOnMedia(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMultisessionRandomWrite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionRandomWrite_Impl, const OFFSET: isize>() -> IMultisessionRandomWrite_Vtbl {
        unsafe extern "system" fn WriteUnitSize<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WriteUnitSize() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenAddress<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastWrittenAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMultisession_Vtbl::new::<Identity, Impl, OFFSET>(),
            WriteUnitSize: WriteUnitSize::<Identity, Impl, OFFSET>,
            LastWrittenAddress: LastWrittenAddress::<Identity, Impl, OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultisessionRandomWrite as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IMultisession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMultisessionSequential_Impl: Sized + super::super::System::Com::IDispatch_Impl + IMultisession_Impl {
    fn IsFirstDataSession(&mut self) -> ::windows::core::Result<i16>;
    fn StartAddressOfPreviousSession(&mut self) -> ::windows::core::Result<i32>;
    fn LastWrittenAddressOfPreviousSession(&mut self) -> ::windows::core::Result<i32>;
    fn NextWritableAddress(&mut self) -> ::windows::core::Result<i32>;
    fn FreeSectorsOnMedia(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMultisessionSequential_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionSequential_Impl, const OFFSET: isize>() -> IMultisessionSequential_Vtbl {
        unsafe extern "system" fn IsFirstDataSession<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsFirstDataSession() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAddressOfPreviousSession<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartAddressOfPreviousSession() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenAddressOfPreviousSession<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastWrittenAddressOfPreviousSession() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextWritableAddress<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NextWritableAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FreeSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMultisession_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsFirstDataSession: IsFirstDataSession::<Identity, Impl, OFFSET>,
            StartAddressOfPreviousSession: StartAddressOfPreviousSession::<Identity, Impl, OFFSET>,
            LastWrittenAddressOfPreviousSession: LastWrittenAddressOfPreviousSession::<Identity, Impl, OFFSET>,
            NextWritableAddress: NextWritableAddress::<Identity, Impl, OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultisessionSequential as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IMultisession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMultisessionSequential2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IMultisession_Impl + IMultisessionSequential_Impl {
    fn WriteUnitSize(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMultisessionSequential2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionSequential2_Impl, const OFFSET: isize>() -> IMultisessionSequential2_Vtbl {
        unsafe extern "system" fn WriteUnitSize<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionSequential2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WriteUnitSize() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IMultisessionSequential_Vtbl::new::<Identity, Impl, OFFSET>(), WriteUnitSize: WriteUnitSize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultisessionSequential2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IMultisession as ::windows::core::Interface>::IID || iid == &<IMultisessionSequential as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IProgressItem_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn FirstBlock(&mut self) -> ::windows::core::Result<u32>;
    fn LastBlock(&mut self) -> ::windows::core::Result<u32>;
    fn BlockCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IProgressItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressItem_Impl, const OFFSET: isize>() -> IProgressItem_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IProgressItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *desc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstBlock<Identity: ::windows::core::IUnknownImpl, Impl: IProgressItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FirstBlock() {
                ::core::result::Result::Ok(ok__) => {
                    *block = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBlock<Identity: ::windows::core::IUnknownImpl, Impl: IProgressItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastBlock() {
                ::core::result::Result::Ok(ok__) => {
                    *block = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockCount<Identity: ::windows::core::IUnknownImpl, Impl: IProgressItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blocks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BlockCount() {
                ::core::result::Result::Ok(ok__) => {
                    *blocks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            FirstBlock: FirstBlock::<Identity, Impl, OFFSET>,
            LastBlock: LastBlock::<Identity, Impl, OFFSET>,
            BlockCount: BlockCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProgressItem as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IProgressItems_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IProgressItem>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn ProgressItemFromBlock(&mut self, block: u32) -> ::windows::core::Result<IProgressItem>;
    fn ProgressItemFromDescription(&mut self, description: &super::super::Foundation::BSTR) -> ::windows::core::Result<IProgressItem>;
    fn EnumProgressItems(&mut self) -> ::windows::core::Result<IEnumProgressItems>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IProgressItems_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressItems_Impl, const OFFSET: isize>() -> IProgressItems_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressItemFromBlock<Identity: ::windows::core::IUnknownImpl, Impl: IProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: u32, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProgressItemFromBlock(::core::mem::transmute_copy(&block)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressItemFromDescription<Identity: ::windows::core::IUnknownImpl, Impl: IProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProgressItemFromDescription(::core::mem::transmute_copy(&description)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumProgressItems<Identity: ::windows::core::IUnknownImpl, Impl: IProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumProgressItems() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            ProgressItemFromBlock: ProgressItemFromBlock::<Identity, Impl, OFFSET>,
            ProgressItemFromDescription: ProgressItemFromDescription::<Identity, Impl, OFFSET>,
            EnumProgressItems: EnumProgressItems::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProgressItems as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRawCDImageCreator_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CreateResultImage(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn AddTrack(&mut self, datatype: IMAPI_CD_SECTOR_TYPE, data: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<i32>;
    fn AddSpecialPregap(&mut self, data: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn AddSubcodeRWGenerator(&mut self, subcode: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn SetResultingImageType(&mut self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::Result<()>;
    fn ResultingImageType(&mut self) -> ::windows::core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>;
    fn StartOfLeadout(&mut self) -> ::windows::core::Result<i32>;
    fn SetStartOfLeadoutLimit(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn StartOfLeadoutLimit(&mut self) -> ::windows::core::Result<i32>;
    fn SetDisableGaplessAudio(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn DisableGaplessAudio(&mut self) -> ::windows::core::Result<i16>;
    fn SetMediaCatalogNumber(&mut self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MediaCatalogNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetStartingTrackNumber(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn StartingTrackNumber(&mut self) -> ::windows::core::Result<i32>;
    fn TrackInfo(&mut self, trackindex: i32) -> ::windows::core::Result<IRawCDImageTrackInfo>;
    fn NumberOfExistingTracks(&mut self) -> ::windows::core::Result<i32>;
    fn LastUsedUserSectorInImage(&mut self) -> ::windows::core::Result<i32>;
    fn ExpectedTableOfContents(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRawCDImageCreator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>() -> IRawCDImageCreator_Vtbl {
        unsafe extern "system" fn CreateResultImage<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateResultImage() {
                ::core::result::Result::Ok(ok__) => {
                    *resultstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrack<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: IMAPI_CD_SECTOR_TYPE, data: ::windows::core::RawPtr, trackindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddTrack(::core::mem::transmute_copy(&datatype), ::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    *trackindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSpecialPregap<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddSpecialPregap(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn AddSubcodeRWGenerator<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subcode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddSubcodeRWGenerator(::core::mem::transmute(&subcode)).into()
        }
        unsafe extern "system" fn SetResultingImageType<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetResultingImageType(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ResultingImageType<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResultingImageType() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartOfLeadout<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartOfLeadout() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartOfLeadoutLimit<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStartOfLeadoutLimit(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn StartOfLeadoutLimit<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartOfLeadoutLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableGaplessAudio<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisableGaplessAudio(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn DisableGaplessAudio<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisableGaplessAudio() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaCatalogNumber<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMediaCatalogNumber(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn MediaCatalogNumber<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MediaCatalogNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartingTrackNumber<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStartingTrackNumber(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn StartingTrackNumber<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartingTrackNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackInfo<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackindex: i32, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TrackInfo(::core::mem::transmute_copy(&trackindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfExistingTracks<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumberOfExistingTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastUsedUserSectorInImage<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastUsedUserSectorInImage() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpectedTableOfContents<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExpectedTableOfContents() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateResultImage: CreateResultImage::<Identity, Impl, OFFSET>,
            AddTrack: AddTrack::<Identity, Impl, OFFSET>,
            AddSpecialPregap: AddSpecialPregap::<Identity, Impl, OFFSET>,
            AddSubcodeRWGenerator: AddSubcodeRWGenerator::<Identity, Impl, OFFSET>,
            SetResultingImageType: SetResultingImageType::<Identity, Impl, OFFSET>,
            ResultingImageType: ResultingImageType::<Identity, Impl, OFFSET>,
            StartOfLeadout: StartOfLeadout::<Identity, Impl, OFFSET>,
            SetStartOfLeadoutLimit: SetStartOfLeadoutLimit::<Identity, Impl, OFFSET>,
            StartOfLeadoutLimit: StartOfLeadoutLimit::<Identity, Impl, OFFSET>,
            SetDisableGaplessAudio: SetDisableGaplessAudio::<Identity, Impl, OFFSET>,
            DisableGaplessAudio: DisableGaplessAudio::<Identity, Impl, OFFSET>,
            SetMediaCatalogNumber: SetMediaCatalogNumber::<Identity, Impl, OFFSET>,
            MediaCatalogNumber: MediaCatalogNumber::<Identity, Impl, OFFSET>,
            SetStartingTrackNumber: SetStartingTrackNumber::<Identity, Impl, OFFSET>,
            StartingTrackNumber: StartingTrackNumber::<Identity, Impl, OFFSET>,
            TrackInfo: TrackInfo::<Identity, Impl, OFFSET>,
            NumberOfExistingTracks: NumberOfExistingTracks::<Identity, Impl, OFFSET>,
            LastUsedUserSectorInImage: LastUsedUserSectorInImage::<Identity, Impl, OFFSET>,
            ExpectedTableOfContents: ExpectedTableOfContents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawCDImageCreator as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRawCDImageTrackInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartingLba(&mut self) -> ::windows::core::Result<i32>;
    fn SectorCount(&mut self) -> ::windows::core::Result<i32>;
    fn TrackNumber(&mut self) -> ::windows::core::Result<i32>;
    fn SectorType(&mut self) -> ::windows::core::Result<IMAPI_CD_SECTOR_TYPE>;
    fn ISRC(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetISRC(&mut self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DigitalAudioCopySetting(&mut self) -> ::windows::core::Result<IMAPI_CD_TRACK_DIGITAL_COPY_SETTING>;
    fn SetDigitalAudioCopySetting(&mut self, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows::core::Result<()>;
    fn AudioHasPreemphasis(&mut self) -> ::windows::core::Result<i16>;
    fn SetAudioHasPreemphasis(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn TrackIndexes(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn AddTrackIndex(&mut self, lbaoffset: i32) -> ::windows::core::Result<()>;
    fn ClearTrackIndex(&mut self, lbaoffset: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRawCDImageTrackInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>() -> IRawCDImageTrackInfo_Vtbl {
        unsafe extern "system" fn StartingLba<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartingLba() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SectorCount<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SectorCount() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackNumber<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TrackNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SectorType<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_CD_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SectorType() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ISRC<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ISRC() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetISRC<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetISRC(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn DigitalAudioCopySetting<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DigitalAudioCopySetting() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDigitalAudioCopySetting<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDigitalAudioCopySetting(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn AudioHasPreemphasis<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AudioHasPreemphasis() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioHasPreemphasis<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAudioHasPreemphasis(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn TrackIndexes<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TrackIndexes() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrackIndex<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbaoffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddTrackIndex(::core::mem::transmute_copy(&lbaoffset)).into()
        }
        unsafe extern "system" fn ClearTrackIndex<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbaoffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearTrackIndex(::core::mem::transmute_copy(&lbaoffset)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StartingLba: StartingLba::<Identity, Impl, OFFSET>,
            SectorCount: SectorCount::<Identity, Impl, OFFSET>,
            TrackNumber: TrackNumber::<Identity, Impl, OFFSET>,
            SectorType: SectorType::<Identity, Impl, OFFSET>,
            ISRC: ISRC::<Identity, Impl, OFFSET>,
            SetISRC: SetISRC::<Identity, Impl, OFFSET>,
            DigitalAudioCopySetting: DigitalAudioCopySetting::<Identity, Impl, OFFSET>,
            SetDigitalAudioCopySetting: SetDigitalAudioCopySetting::<Identity, Impl, OFFSET>,
            AudioHasPreemphasis: AudioHasPreemphasis::<Identity, Impl, OFFSET>,
            SetAudioHasPreemphasis: SetAudioHasPreemphasis::<Identity, Impl, OFFSET>,
            TrackIndexes: TrackIndexes::<Identity, Impl, OFFSET>,
            AddTrackIndex: AddTrackIndex::<Identity, Impl, OFFSET>,
            ClearTrackIndex: ClearTrackIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawCDImageTrackInfo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IRedbookDiscMaster_Impl: Sized {
    fn GetTotalAudioTracks(&mut self) -> ::windows::core::Result<i32>;
    fn GetTotalAudioBlocks(&mut self) -> ::windows::core::Result<i32>;
    fn GetUsedAudioBlocks(&mut self) -> ::windows::core::Result<i32>;
    fn GetAvailableAudioTrackBlocks(&mut self) -> ::windows::core::Result<i32>;
    fn GetAudioBlockSize(&mut self) -> ::windows::core::Result<i32>;
    fn CreateAudioTrack(&mut self, nblocks: i32) -> ::windows::core::Result<()>;
    fn AddAudioTrackBlocks(&mut self, pby: *const u8, cb: i32) -> ::windows::core::Result<()>;
    fn CloseAudioTrack(&mut self) -> ::windows::core::Result<()>;
}
impl IRedbookDiscMaster_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>() -> IRedbookDiscMaster_Vtbl {
        unsafe extern "system" fn GetTotalAudioTracks<Identity: ::windows::core::IUnknownImpl, Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pntracks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTotalAudioTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *pntracks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTotalAudioBlocks<Identity: ::windows::core::IUnknownImpl, Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTotalAudioBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *pnblocks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsedAudioBlocks<Identity: ::windows::core::IUnknownImpl, Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUsedAudioBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *pnblocks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableAudioTrackBlocks<Identity: ::windows::core::IUnknownImpl, Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAvailableAudioTrackBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *pnblocks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioBlockSize<Identity: ::windows::core::IUnknownImpl, Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblockbytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAudioBlockSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pnblockbytes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAudioTrack<Identity: ::windows::core::IUnknownImpl, Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nblocks: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateAudioTrack(::core::mem::transmute_copy(&nblocks)).into()
        }
        unsafe extern "system" fn AddAudioTrackBlocks<Identity: ::windows::core::IUnknownImpl, Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pby: *const u8, cb: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAudioTrackBlocks(::core::mem::transmute_copy(&pby), ::core::mem::transmute_copy(&cb)).into()
        }
        unsafe extern "system" fn CloseAudioTrack<Identity: ::windows::core::IUnknownImpl, Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CloseAudioTrack().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetTotalAudioTracks: GetTotalAudioTracks::<Identity, Impl, OFFSET>,
            GetTotalAudioBlocks: GetTotalAudioBlocks::<Identity, Impl, OFFSET>,
            GetUsedAudioBlocks: GetUsedAudioBlocks::<Identity, Impl, OFFSET>,
            GetAvailableAudioTrackBlocks: GetAvailableAudioTrackBlocks::<Identity, Impl, OFFSET>,
            GetAudioBlockSize: GetAudioBlockSize::<Identity, Impl, OFFSET>,
            CreateAudioTrack: CreateAudioTrack::<Identity, Impl, OFFSET>,
            AddAudioTrackBlocks: AddAudioTrackBlocks::<Identity, Impl, OFFSET>,
            CloseAudioTrack: CloseAudioTrack::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRedbookDiscMaster as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IStreamConcatenate_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl {
    fn Initialize(&mut self, stream1: &::core::option::Option<super::super::System::Com::IStream>, stream2: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Initialize2(&mut self, streams: *const ::core::option::Option<super::super::System::Com::IStream>, streamcount: u32) -> ::windows::core::Result<()>;
    fn Append(&mut self, stream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Append2(&mut self, streams: *const ::core::option::Option<super::super::System::Com::IStream>, streamcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IStreamConcatenate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamConcatenate_Impl, const OFFSET: isize>() -> IStreamConcatenate_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream1: ::windows::core::RawPtr, stream2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&stream1), ::core::mem::transmute(&stream2)).into()
        }
        unsafe extern "system" fn Initialize2<Identity: ::windows::core::IUnknownImpl, Impl: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *const ::windows::core::RawPtr, streamcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize2(::core::mem::transmute_copy(&streams), ::core::mem::transmute_copy(&streamcount)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows::core::IUnknownImpl, Impl: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Append(::core::mem::transmute(&stream)).into()
        }
        unsafe extern "system" fn Append2<Identity: ::windows::core::IUnknownImpl, Impl: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *const ::windows::core::RawPtr, streamcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Append2(::core::mem::transmute_copy(&streams), ::core::mem::transmute_copy(&streamcount)).into()
        }
        Self {
            base: super::super::System::Com::IStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Initialize2: Initialize2::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            Append2: Append2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamConcatenate as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IStreamInterleave_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl {
    fn Initialize(&mut self, streams: *const ::core::option::Option<super::super::System::Com::IStream>, interleavesizes: *const u32, streamcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IStreamInterleave_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamInterleave_Impl, const OFFSET: isize>() -> IStreamInterleave_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IStreamInterleave_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *const ::windows::core::RawPtr, interleavesizes: *const u32, streamcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&streams), ::core::mem::transmute_copy(&interleavesizes), ::core::mem::transmute_copy(&streamcount)).into()
        }
        Self { base: super::super::System::Com::IStream_Vtbl::new::<Identity, Impl, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamInterleave as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IStreamPseudoRandomBased_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl {
    fn SetSeed(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn Seed(&mut self) -> ::windows::core::Result<u32>;
    fn SetExtendedSeed(&mut self, values: *const u32, ecount: u32) -> ::windows::core::Result<()>;
    fn ExtendedSeed(&mut self, values: *mut *mut u32, ecount: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IStreamPseudoRandomBased_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamPseudoRandomBased_Impl, const OFFSET: isize>() -> IStreamPseudoRandomBased_Vtbl {
        unsafe extern "system" fn SetSeed<Identity: ::windows::core::IUnknownImpl, Impl: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSeed(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Seed<Identity: ::windows::core::IUnknownImpl, Impl: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Seed() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtendedSeed<Identity: ::windows::core::IUnknownImpl, Impl: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: *const u32, ecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExtendedSeed(::core::mem::transmute_copy(&values), ::core::mem::transmute_copy(&ecount)).into()
        }
        unsafe extern "system" fn ExtendedSeed<Identity: ::windows::core::IUnknownImpl, Impl: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: *mut *mut u32, ecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExtendedSeed(::core::mem::transmute_copy(&values), ::core::mem::transmute_copy(&ecount)).into()
        }
        Self {
            base: super::super::System::Com::IStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetSeed: SetSeed::<Identity, Impl, OFFSET>,
            Seed: Seed::<Identity, Impl, OFFSET>,
            SetExtendedSeed: SetExtendedSeed::<Identity, Impl, OFFSET>,
            ExtendedSeed: ExtendedSeed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamPseudoRandomBased as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWriteEngine2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn WriteSection(&mut self, data: &::core::option::Option<super::super::System::Com::IStream>, startingblockaddress: i32, numberofblocks: i32) -> ::windows::core::Result<()>;
    fn CancelWrite(&mut self) -> ::windows::core::Result<()>;
    fn SetRecorder(&mut self, value: &::core::option::Option<IDiscRecorder2Ex>) -> ::windows::core::Result<()>;
    fn Recorder(&mut self) -> ::windows::core::Result<IDiscRecorder2Ex>;
    fn SetUseStreamingWrite12(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn UseStreamingWrite12(&mut self) -> ::windows::core::Result<i16>;
    fn SetStartingSectorsPerSecond(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn StartingSectorsPerSecond(&mut self) -> ::windows::core::Result<i32>;
    fn SetEndingSectorsPerSecond(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn EndingSectorsPerSecond(&mut self) -> ::windows::core::Result<i32>;
    fn SetBytesPerSector(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn BytesPerSector(&mut self) -> ::windows::core::Result<i32>;
    fn WriteInProgress(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWriteEngine2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2_Impl, const OFFSET: isize>() -> IWriteEngine2_Vtbl {
        unsafe extern "system" fn WriteSection<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, startingblockaddress: i32, numberofblocks: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteSection(::core::mem::transmute(&data), ::core::mem::transmute_copy(&startingblockaddress), ::core::mem::transmute_copy(&numberofblocks)).into()
        }
        unsafe extern "system" fn CancelWrite<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelWrite().into()
        }
        unsafe extern "system" fn SetRecorder<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRecorder(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Recorder<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Recorder() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseStreamingWrite12<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUseStreamingWrite12(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn UseStreamingWrite12<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UseStreamingWrite12() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartingSectorsPerSecond<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStartingSectorsPerSecond(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn StartingSectorsPerSecond<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartingSectorsPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndingSectorsPerSecond<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEndingSectorsPerSecond(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn EndingSectorsPerSecond<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EndingSectorsPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBytesPerSector<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBytesPerSector(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BytesPerSector<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BytesPerSector() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteInProgress<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WriteInProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            WriteSection: WriteSection::<Identity, Impl, OFFSET>,
            CancelWrite: CancelWrite::<Identity, Impl, OFFSET>,
            SetRecorder: SetRecorder::<Identity, Impl, OFFSET>,
            Recorder: Recorder::<Identity, Impl, OFFSET>,
            SetUseStreamingWrite12: SetUseStreamingWrite12::<Identity, Impl, OFFSET>,
            UseStreamingWrite12: UseStreamingWrite12::<Identity, Impl, OFFSET>,
            SetStartingSectorsPerSecond: SetStartingSectorsPerSecond::<Identity, Impl, OFFSET>,
            StartingSectorsPerSecond: StartingSectorsPerSecond::<Identity, Impl, OFFSET>,
            SetEndingSectorsPerSecond: SetEndingSectorsPerSecond::<Identity, Impl, OFFSET>,
            EndingSectorsPerSecond: EndingSectorsPerSecond::<Identity, Impl, OFFSET>,
            SetBytesPerSector: SetBytesPerSector::<Identity, Impl, OFFSET>,
            BytesPerSector: BytesPerSector::<Identity, Impl, OFFSET>,
            WriteInProgress: WriteInProgress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWriteEngine2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWriteEngine2EventArgs_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartLba(&mut self) -> ::windows::core::Result<i32>;
    fn SectorCount(&mut self) -> ::windows::core::Result<i32>;
    fn LastReadLba(&mut self) -> ::windows::core::Result<i32>;
    fn LastWrittenLba(&mut self) -> ::windows::core::Result<i32>;
    fn TotalSystemBuffer(&mut self) -> ::windows::core::Result<i32>;
    fn UsedSystemBuffer(&mut self) -> ::windows::core::Result<i32>;
    fn FreeSystemBuffer(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWriteEngine2EventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>() -> IWriteEngine2EventArgs_Vtbl {
        unsafe extern "system" fn StartLba<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartLba() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SectorCount<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SectorCount() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastReadLba<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastReadLba() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenLba<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastWrittenLba() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSystemBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalSystemBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsedSystemBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UsedSystemBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSystemBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FreeSystemBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StartLba: StartLba::<Identity, Impl, OFFSET>,
            SectorCount: SectorCount::<Identity, Impl, OFFSET>,
            LastReadLba: LastReadLba::<Identity, Impl, OFFSET>,
            LastWrittenLba: LastWrittenLba::<Identity, Impl, OFFSET>,
            TotalSystemBuffer: TotalSystemBuffer::<Identity, Impl, OFFSET>,
            UsedSystemBuffer: UsedSystemBuffer::<Identity, Impl, OFFSET>,
            FreeSystemBuffer: FreeSystemBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWriteEngine2EventArgs as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWriteSpeedDescriptor_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MediaType(&mut self) -> ::windows::core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn RotationTypeIsPureCAV(&mut self) -> ::windows::core::Result<i16>;
    fn WriteSpeed(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWriteSpeedDescriptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWriteSpeedDescriptor_Impl, const OFFSET: isize>() -> IWriteSpeedDescriptor_Vtbl {
        unsafe extern "system" fn MediaType<Identity: ::windows::core::IUnknownImpl, Impl: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationTypeIsPureCAV<Identity: ::windows::core::IUnknownImpl, Impl: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteSpeed<Identity: ::windows::core::IUnknownImpl, Impl: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            MediaType: MediaType::<Identity, Impl, OFFSET>,
            RotationTypeIsPureCAV: RotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            WriteSpeed: WriteSpeed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWriteSpeedDescriptor as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
