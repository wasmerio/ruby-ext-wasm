#![deny(warnings)]

//! # The `wasmer` Ruby package
//!
//! A complete and mature WebAssembly runtime for Ruby based on [Wasmer].
//!
//! ## Features
//!
//! * **Easy to use**: The `wasmer` API mimics the standard
//!   WebAssembly API,
//! * **Fast**: `wasmer` executes the WebAssembly modules as fast as
//!   possible,
//! * **Safe**: All calls to WebAssembly will be fast, but more
//!   importantly, complete safe and sandboxed.
//!
//! [Wasmer]: https://github.com/wasmerio/wasmer

#[cfg(doc)]
mod doc;
mod error;
mod exports;
mod externals;
mod import_object;
mod instance;
mod memory;
mod module;
mod prelude;
mod store;
mod types;
mod values;
mod wasi;
mod wat;

#[cfg(doc)]
pub use doc::*;

use crate::memory::views::{
    Int16Array, Int32Array, Int8Array, Uint16Array, Uint32Array, Uint8Array,
};
use rutie::{Class, Integer, Module, Object, RString};

macro_rules! ruby_define {
    (in $module:ident
     $( class ( $( $class_rust_module:path ),+ ) $class_name:ident
        $( include $include_name:ident )*
        {
           $( @const $constant_name:ident = $constant_value:expr; )*
           $( $ruby_definition:ident ($method_rust_name:ident) $method_name:expr; )*
        };
     )*

     $( function ( $function_rust_name:path ) $function_name:expr; )*
    ) => {
        $(
            {
                let data_class = Class::from_existing("Object");

                $module
                    .define_nested_class(stringify!($class_name), Some(&data_class))
                    .define(|this| {
                        $(
                            #[allow(unused_imports)]
                            use $class_rust_module::*;
                        )+

                        $(
                            this.$ruby_definition($method_name, $method_rust_name);
                        )*

                        $(
                            this.const_set(stringify!($constant_name), &$constant_value);
                        )*
                    })
                    $( .include( stringify!($include_name) ) )*
                    ;
            }
        )*

        $( $module.define_module_function($function_name, $function_rust_name); )*
    }
}

#[doc(hidden)]
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn init() {
    let mut wasmer_module = Module::from_existing("Wasmer");

    wasmer_module.const_set("VERSION", &RString::new_utf8(env!("CARGO_PKG_VERSION")));

    ruby_define! {
        in wasmer_module
            class (store::ruby_store) Store {
                def_self (new) "new";
            };

            class (module::ruby_module) Module {
                def_self (validate) "validate";
                def_self (new) "new";
                def (set_name) "name=";
                def (get_name) "name";
                def (exports) "exports";
                def (imports) "imports";
                def (custom_sections) "custom_sections";
                def (serialize) "serialize";
                def_self (deserialize) "deserialize";
            };

            class (instance::ruby_instance) Instance {
                def_self (new) "new";
                def (exports) "exports";
            };

            class (exports::ruby_exports, exports::ruby_exports_extra) Exports {
                def (respond_to_missing) "respond_to_missing?";
                def (method_missing) "method_missing";
                def (length) "length";
            };

            class (import_object::ruby_importobject) ImportObject {
                def_self (new) "new";
                def (contains_namespace) "contains_namespace?";
                def (register) "register";
            };

            class (externals::function::ruby_function, externals::function::ruby_function_extra) Function {
                def_self (new) "new";
                def (call) "call";
                def (r#type) "type";
            };

            class (externals::memory::ruby_memory) Memory {
                def_self (new) "new";
                def (r#type) "type";
                def (size) "size";
                def (data_size) "data_size";
                def (grow) "grow";
                def (uint8_view) "uint8_view";
                def (int8_view) "int8_view";
                def (uint16_view) "uint16_view";
                def (int16_view) "int16_view";
                def (uint32_view) "uint32_view";
                def (int32_view) "int32_view";
            };

            class (memory::views::ruby_uint8array) Uint8Array
            include Enumerable
            {
                @const BYTES_PER_ELEMENT = Integer::from(Uint8Array::BYTES_PER_ELEMENT);
                def (length) "length";
                def (set) "[]=";
                def (get) "[]";
                def (each) "each";
            };

            class (memory::views::ruby_int8array) Int8Array
            include Enumerable
            {
                @const BYTES_PER_ELEMENT = Integer::from(Int8Array::BYTES_PER_ELEMENT);
                def (length) "length";
                def (set) "[]=";
                def (get) "[]";
                def (each) "each";
            };

            class (memory::views::ruby_uint16array) Uint16Array
            include Enumerable
            {
                @const BYTES_PER_ELEMENT = Integer::from(Uint16Array::BYTES_PER_ELEMENT);
                def (length) "length";
                def (set) "[]=";
                def (get) "[]";
                def (each) "each";
            };

            class (memory::views::ruby_int16array) Int16Array
            include Enumerable
            {
                @const BYTES_PER_ELEMENT = Integer::from(Int16Array::BYTES_PER_ELEMENT);
                def (length) "length";
                def (set) "[]=";
                def (get) "[]";
                def (each) "each";
            };

            class (memory::views::ruby_uint32array) Uint32Array
            include Enumerable
            {
                @const BYTES_PER_ELEMENT = Integer::from(Uint32Array::BYTES_PER_ELEMENT);
                def (length) "length";
                def (set) "[]=";
                def (get) "[]";
                def (each) "each";
            };

            class (memory::views::ruby_int32array) Int32Array
            include Enumerable
            {
                @const BYTES_PER_ELEMENT = Integer::from(Int32Array::BYTES_PER_ELEMENT);
                def (length) "length";
                def (set) "[]=";
                def (get) "[]";
                def (each) "each";
            };

            class (externals::global::ruby_global) Global {
                def_self (new) "new";
                def (mutable) "mutable?";
                def (get_value) "value";
                def (set_value) "value=";
                def (r#type) "type";
            };

            class (externals::table::ruby_table) Table {
                def_self (new) "new";
            };

            class (types) Type {
                @const I32 = Integer::new(1);
                @const I64 = Integer::new(2);
                @const F32 = Integer::new(3);
                @const F64 = Integer::new(4);
                @const V128 = Integer::new(5);
                @const EXTERN_REF = Integer::new(6);
                @const FUNC_REF = Integer::new(7);
            };

            class (types::ruby_functiontype) FunctionType {
                def_self (new) "new";
                def (params) "params";
                def (results) "results";
            };

            class (types::ruby_memorytype) MemoryType {
                def_self (new) "new";
                def (minimum) "minimum";
                def (maximum) "maximum";
                def (shared) "shared?";
            };

            class (types::ruby_globaltype) GlobalType {
                def_self (new) "new";
                def (r#type) "type";
                def (mutable) "mutable?";
            };

            class (types::ruby_tabletype) TableType {
                def_self (new) "new";
                def (r#type) "type";
                def (minimum) "minimum";
                def (maximum) "maximum";
            };

            class (types::ruby_exporttype) ExportType {
                def_self (new) "new";
                def (name) "name";
                def (r#type) "type";
            };

            class (types::ruby_importtype) ImportType {
                def_self (new) "new";
                def (module) "module";
                def (name) "name";
                def (r#type) "type";
            };

            class (values::ruby_value) Value {
                def_self (i32) "i32";
                def_self (i64) "i64";
                def_self (f32) "f32";
                def_self (f64) "f64";
            };

            function (wat::wat2wasm) "wat2wasm";
            function (wat::wasm2wat) "wasm2wat";
    };

    let mut wasmer_wasi_module = wasmer_module.define_nested_module("Wasi");

    ruby_define! {
        in wasmer_wasi_module
            class (wasi) Version {
                @const LATEST = Integer::new(1);
                @const SNAPSHOT0 = Integer::new(2);
                @const SNAPSHOT1 = Integer::new(3);
            };

            class (wasi::ruby_statebuilder) StateBuilder {
                def_self (new) "new";
                def (arguments) "arguments";
                def (argument) "argument";
                def (environments) "environments";
                def (environment) "environment";
                def (preopen_directories) "preopen_directories";
                def (preopen_directory) "preopen_directory";
                def (map_directories) "map_directories";
                def (map_directory) "map_directory";
                def (finalize) "finalize";
            };

            class (wasi::ruby_environment) Environment {
                def (generate_import_object) "generate_import_object";
            };

            function (wasi::get_version) "get_version";
    };
}
