use crate::{
    error::{to_ruby_err, TypeError},
    prelude::*,
};
use rutie::{AnyException, AnyObject, Array, Boolean, Integer, NilClass, Object, RString};
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Type {
    I32 = 1,
    I64 = 2,
    F32 = 3,
    F64 = 4,
    V128 = 5,
    ExternRef = 6,
    FuncRef = 7,
}

impl Type {
    fn to_integer(&self) -> Integer {
        match self {
            Self::I32 => Integer::new(1),
            Self::I64 => Integer::new(2),
            Self::F32 => Integer::new(3),
            Self::F64 => Integer::new(4),
            Self::V128 => Integer::new(5),
            Self::ExternRef => Integer::new(6),
            Self::FuncRef => Integer::new(7),
        }
    }
}

impl From<&wasmer::Type> for Type {
    fn from(value: &wasmer::Type) -> Self {
        match value {
            wasmer::Type::I32 => Self::I32,
            wasmer::Type::I64 => Self::I64,
            wasmer::Type::F32 => Self::F32,
            wasmer::Type::F64 => Self::F64,
            wasmer::Type::V128 => Self::V128,
            wasmer::Type::ExternRef => Self::ExternRef,
            wasmer::Type::FuncRef => Self::FuncRef,
        }
    }
}

impl Into<wasmer::Type> for Type {
    fn into(self) -> wasmer::Type {
        match self {
            Self::I32 => wasmer::Type::I32,
            Self::I64 => wasmer::Type::I64,
            Self::F32 => wasmer::Type::F32,
            Self::F64 => wasmer::Type::F64,
            Self::V128 => wasmer::Type::V128,
            Self::ExternRef => wasmer::Type::ExternRef,
            Self::FuncRef => wasmer::Type::FuncRef,
        }
    }
}

impl TryFrom<&Integer> for Type {
    type Error = &'static str;

    fn try_from(value: &Integer) -> Result<Self, Self::Error> {
        Ok(match value.to_i32() {
            1 => Type::I32,
            2 => Type::I64,
            3 => Type::F32,
            4 => Type::F64,
            5 => Type::V128,
            6 => Type::ExternRef,
            7 => Type::FuncRef,
            _ => return Err("Unrecognized type"),
        })
    }
}

#[rubyclass(module = "Wasmer")]
pub struct FunctionType {
    pub params: Vec<Type>,
    pub results: Vec<Type>,
}

#[rubymethods]
impl FunctionType {
    pub fn new(params: &Array, results: &Array) -> RubyResult<AnyObject> {
        let params = unsafe { params.to_any_object().to::<Array>() }
            .into_iter()
            .map(|param| {
                param
                    .try_convert_to::<Integer>()
                    .and_then(|param| Type::try_from(&param).map_err(to_ruby_err::<TypeError, _>))
            })
            .collect::<Result<Vec<Type>, AnyException>>()?;
        let results = unsafe { results.to_any_object().to::<Array>() }
            .into_iter()
            .map(|result| {
                result
                    .try_convert_to::<Integer>()
                    .and_then(|result| Type::try_from(&result).map_err(to_ruby_err::<TypeError, _>))
            })
            .collect::<Result<Vec<Type>, AnyException>>()?;

        Ok(FunctionType::ruby_new(FunctionType { params, results }))
    }

    pub fn params(&self) -> RubyResult<Array> {
        Ok(self
            .params
            .iter()
            .map(|ty| Type::to_integer(ty).to_any_object())
            .collect())
    }

    pub fn results(&self) -> RubyResult<Array> {
        Ok(self
            .results
            .iter()
            .map(|ty| Type::to_integer(ty).to_any_object())
            .collect())
    }
}

impl From<&wasmer::FunctionType> for FunctionType {
    fn from(value: &wasmer::FunctionType) -> Self {
        Self {
            params: value.params().iter().map(Into::into).collect(),
            results: value.results().iter().map(Into::into).collect(),
        }
    }
}

impl Into<wasmer::FunctionType> for &FunctionType {
    fn into(self) -> wasmer::FunctionType {
        wasmer::FunctionType::new(
            self.params
                .iter()
                .cloned()
                .map(Into::into)
                .collect::<Vec<_>>(),
            self.results
                .iter()
                .cloned()
                .map(Into::into)
                .collect::<Vec<_>>(),
        )
    }
}

#[rubyclass(module = "Wasmer")]
pub struct MemoryType {
    pub minimum: u32,
    pub maximum: Option<u32>,
    pub shared: bool,
}

#[rubymethods]
impl MemoryType {
    pub fn new(minimum: &Integer, maximum: &AnyObject, shared: &Boolean) -> RubyResult<AnyObject> {
        Ok(MemoryType::ruby_new(MemoryType {
            minimum: minimum.to_u64() as _,
            maximum: if maximum.is_nil() {
                None
            } else {
                Some(maximum.try_convert_to::<Integer>()?.to_u64() as _)
            },
            shared: shared.to_bool(),
        }))
    }

    pub fn minimum(&self) -> RubyResult<Integer> {
        Ok(Integer::new(self.minimum.into()))
    }

    pub fn maximum(&self) -> RubyResult<AnyObject> {
        Ok(match self.maximum {
            Some(maximum) => Integer::new(maximum.into()).to_any_object(),
            None => NilClass::new().to_any_object(),
        })
    }

    pub fn shared(&self) -> RubyResult<Boolean> {
        Ok(Boolean::new(self.shared))
    }
}

impl From<wasmer::MemoryType> for MemoryType {
    fn from(value: wasmer::MemoryType) -> Self {
        Self::from(&value)
    }
}

impl From<&wasmer::MemoryType> for MemoryType {
    fn from(value: &wasmer::MemoryType) -> Self {
        Self {
            minimum: value.minimum.0,
            maximum: value.maximum.map(|pages| pages.0),
            shared: value.shared,
        }
    }
}

impl Into<wasmer::MemoryType> for &MemoryType {
    fn into(self) -> wasmer::MemoryType {
        wasmer::MemoryType::new(self.minimum, self.maximum, self.shared)
    }
}

#[rubyclass(module = "Wasmer")]
pub struct GlobalType {
    pub ty: Type,
    pub mutable: bool,
}

#[rubymethods]
impl GlobalType {
    pub fn new(ty: &Integer, mutable: &Boolean) -> RubyResult<AnyObject> {
        Ok(GlobalType::ruby_new(GlobalType {
            ty: Type::try_from(ty).map_err(to_ruby_err::<TypeError, _>)?,
            mutable: mutable.to_bool(),
        }))
    }

    pub fn r#type(&self) -> RubyResult<Integer> {
        Ok(self.ty.to_integer())
    }

    pub fn mutable(&self) -> RubyResult<Boolean> {
        Ok(Boolean::new(self.mutable))
    }
}

impl From<&wasmer::GlobalType> for GlobalType {
    fn from(value: &wasmer::GlobalType) -> Self {
        Self {
            ty: (&value.ty).into(),
            mutable: value.mutability.is_mutable(),
        }
    }
}

#[rubyclass(module = "Wasmer")]
pub struct TableType {
    pub ty: Type,
    pub minimum: u32,
    pub maximum: Option<u32>,
}

#[rubymethods]
impl TableType {
    pub fn new(ty: &Integer, minimum: &Integer, maximum: &AnyObject) -> RubyResult<AnyObject> {
        Ok(TableType::ruby_new(TableType {
            ty: Type::try_from(ty).map_err(to_ruby_err::<TypeError, _>)?,
            minimum: minimum.to_u64() as _,
            maximum: if maximum.is_nil() {
                None
            } else {
                Some(maximum.try_convert_to::<Integer>()?.to_u64() as _)
            },
        }))
    }

    pub fn r#type(&self) -> RubyResult<Integer> {
        Ok(self.ty.to_integer())
    }

    pub fn minimum(&self) -> RubyResult<Integer> {
        Ok(Integer::new(self.minimum.into()))
    }

    pub fn maximum(&self) -> RubyResult<AnyObject> {
        Ok(match self.maximum {
            Some(maximum) => Integer::new(maximum.into()).to_any_object(),
            None => NilClass::new().to_any_object(),
        })
    }
}

impl From<&wasmer::TableType> for TableType {
    fn from(value: &wasmer::TableType) -> Self {
        Self {
            ty: (&value.ty).into(),
            minimum: value.minimum,
            maximum: value.maximum,
        }
    }
}

impl Into<wasmer::TableType> for &TableType {
    fn into(self) -> wasmer::TableType {
        wasmer::TableType::new(self.ty.into(), self.minimum, self.maximum)
    }
}

#[rubyclass(module = "Wasmer")]
pub struct ExportType {
    pub name: String,
    pub ty: AnyObject,
}

#[rubymethods]
impl ExportType {
    pub fn new(name: &RString, ty: &AnyObject) -> RubyResult<AnyObject> {
        Ok(ExportType::ruby_new(ExportType {
            name: name.to_string(),
            ty: if ty.try_convert_to::<RubyFunctionType>().is_ok()
                || ty.try_convert_to::<RubyMemoryType>().is_ok()
                || ty.try_convert_to::<RubyGlobalType>().is_ok()
                || ty.try_convert_to::<RubyTableType>().is_ok()
            {
                unsafe { ty.to::<AnyObject>() }
            } else {
                return Err(to_ruby_err::<TypeError, _>("Argument #2 of `ExportType.new` must be of kind `FunctionType`, `MemoryType`, `GlobalType` or `TableType`"));
            },
        }))
    }

    pub fn name(&self) -> RubyResult<RString> {
        Ok(RString::new_utf8(&self.name))
    }

    pub fn r#type(&self) -> RubyResult<AnyObject> {
        Ok(self.ty.clone())
    }
}

impl TryFrom<wasmer::ExportType> for ExportType {
    type Error = AnyException;

    fn try_from(value: wasmer::ExportType) -> Result<Self, Self::Error> {
        Ok(ExportType {
            name: value.name().to_string(),
            ty: extern_type_to_ruby_any_object(value.ty()),
        })
    }
}

#[rubyclass(module = "Wasmer")]
pub struct ImportType {
    pub module: String,
    pub name: String,
    pub ty: AnyObject,
}

#[rubymethods]
impl ImportType {
    pub fn new(module: &RString, name: &RString, ty: &AnyObject) -> RubyResult<AnyObject> {
        Ok(ImportType::ruby_new(ImportType {
            module: module.to_string(),
            name: name.to_string(),
            ty: if ty.try_convert_to::<RubyFunctionType>().is_ok()
                || ty.try_convert_to::<RubyMemoryType>().is_ok()
                || ty.try_convert_to::<RubyGlobalType>().is_ok()
                || ty.try_convert_to::<RubyTableType>().is_ok()
            {
                unsafe { ty.to::<AnyObject>() }
            } else {
                return Err(to_ruby_err::<TypeError, _>("Argument #3 of `ImportType.new` must be of kind `FunctionType`, `MemoryType`, `GlobalType` or `TableType`"));
            },
        }))
    }

    pub fn module(&self) -> RubyResult<RString> {
        Ok(RString::new_utf8(&self.module))
    }

    pub fn name(&self) -> RubyResult<RString> {
        Ok(RString::new_utf8(&self.name))
    }

    pub fn r#type(&self) -> RubyResult<AnyObject> {
        Ok(self.ty.clone())
    }
}

impl TryFrom<wasmer::ImportType> for ImportType {
    type Error = AnyException;

    fn try_from(value: wasmer::ImportType) -> Result<Self, Self::Error> {
        Ok(ImportType {
            module: value.module().to_string(),
            name: value.name().to_string(),
            ty: extern_type_to_ruby_any_object(value.ty()),
        })
    }
}

fn extern_type_to_ruby_any_object(value: &wasmer::ExternType) -> AnyObject {
    match value {
        wasmer::ExternType::Function(t) => FunctionType::ruby_new(FunctionType::from(t)),
        wasmer::ExternType::Memory(t) => MemoryType::ruby_new(MemoryType::from(t)),
        wasmer::ExternType::Global(t) => GlobalType::ruby_new(GlobalType::from(t)),
        wasmer::ExternType::Table(t) => TableType::ruby_new(TableType::from(t)),
    }
}
