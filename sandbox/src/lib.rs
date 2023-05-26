use variant_common_data::with_common_variant_data;

/**
Expands to:

```
#[derive(Debug, Clone)]
struct EnumWithCommonData<T, K> {
    common: String,
    variant: EnumWithCommonDataVariants<T, K>,
}

#[derive(Debug, Clone)]
enum EnumWithCommonDataVariants<T, K> {
    VarTuple(String, u8, T),
    VarStruct {
        str: String,
        num: i32,
        arr: [f32; 5],
        k: K,
    },
    VarEmpty,
}

impl<T, K> EnumWithCommonData<T, K> {
    fn new(common: String, variant: EnumWithCommonDataVariants<T, K>) -> Self { Self { common, variant } }
    fn common(&self) -> &String { &self.common }
    fn common_mut(&mut self) -> &mut String { &mut self.common }
    fn variant(&self) -> &EnumWithCommonDataVariants<T, K> {
        &self.variant
    }
    fn variant_mut(&mut self) -> &mut EnumWithCommonDataVariants<T, K> {
        &mut self.variant
    }
    fn set_common(&mut self, common: String) { self.common = common; }
    fn set_variant(&mut self, variant: EnumWithCommonDataVariants<T, K>) {
        self.variant = variant;
    }
    fn as_ref_mut(&mut self) -> &mut EnumWithCommonDataVariants<T, K> {
        &mut self.variant
    }
}

impl<T, K> AsRef<EnumWithCommonDataVariants<T, K>> for EnumWithCommonData<T, K> {
    fn as_ref(&self) -> &EnumWithCommonDataVariants<T, K> {
        &self.variant
    }
}

impl<T, K> std::ops::Deref for EnumWithCommonData<T, K> {
    type Target = EnumWithCommonDataVariants<T, K>;
    fn deref(&self) -> &Self::Target { &self.variant }
}

impl<T, K> std::ops::DerefMut for EnumWithCommonData<T, K> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.variant }
}
```
*/

#[with_common_variant_data]
#[common_data_type = String]
#[derive(Debug, Clone)]
enum EnumWithCommonData<T, K> {
    VarTuple(String, u8, T),
    VarStruct {
        str: String,
        num: i32,
        arr: [f32; 5],
        k: K
    },
    VarEmpty,
}
