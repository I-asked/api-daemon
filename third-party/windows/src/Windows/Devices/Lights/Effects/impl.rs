pub trait ILampArrayEffect_Impl: Sized {
    fn ZIndex(&mut self) -> ::windows::core::Result<i32>;
    fn SetZIndex(&mut self, value: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ILampArrayEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayEffect";
}
impl ILampArrayEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayEffect_Impl, const OFFSET: isize>() -> ILampArrayEffect_Vtbl {
        unsafe extern "system" fn ZIndex<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ZIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZIndex<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetZIndex(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampArrayEffect, OFFSET>(),
            ZIndex: ZIndex::<Identity, Impl, OFFSET>,
            SetZIndex: SetZIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampArrayEffect as ::windows::core::Interface>::IID
    }
}
