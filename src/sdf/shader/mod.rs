use std::{any::type_name, borrow::Cow, ops::Deref};

pub trait GpuSdf {
    fn emit_gpu_fn(&self) -> String;
    fn gpu_fn_name(&self) -> Cow<'static, str>;
}

impl<T, D: GpuSdf> GpuSdf for T
where
    T: Deref<Target = D>,
{
    #[inline]
    fn emit_gpu_fn(&self) -> String {
        self.deref().emit_gpu_fn()
    }

    #[inline]
    fn gpu_fn_name(&self) -> Cow<'static, str> {
        let original_name = self.deref().gpu_fn_name();
        Cow::Owned(format!("{original_name}_{}", type_name::<T>()))
    }
}
