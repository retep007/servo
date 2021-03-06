/* THIS FILE IS AUTOGENERATED - DO NOT EDIT */

#![allow(non_camel_case_types,non_upper_case_globals,unused_mut, unused_imports)]
use dom;
use dom::bindings;
use dom::bindings::callback::CallSetup;
use dom::bindings::callback::CallbackContainer;
use dom::bindings::callback::CallbackFunction;
use dom::bindings::callback::CallbackInterface;
use dom::bindings::callback::CallbackObject;
use dom::bindings::callback::ExceptionHandling;
use dom::bindings::callback::wrap_call_this_object;
use dom::bindings::codegen::InterfaceObjectMap;
use dom::bindings::codegen::PrototypeList;
use dom::bindings::codegen::RegisterBindings;
use dom::bindings::codegen::UnionTypes;
use dom::bindings::constant::ConstantSpec;
use dom::bindings::constant::ConstantVal;
use dom::bindings::conversions::ConversionBehavior;
use dom::bindings::conversions::ConversionResult;
use dom::bindings::conversions::DOM_OBJECT_SLOT;
use dom::bindings::conversions::FromJSValConvertible;
use dom::bindings::conversions::IDLInterface;
use dom::bindings::conversions::StringificationBehavior;
use dom::bindings::conversions::ToJSValConvertible;
use dom::bindings::conversions::is_array_like;
use dom::bindings::conversions::jsid_to_string;
use dom::bindings::conversions::native_from_handlevalue;
use dom::bindings::conversions::native_from_object;
use dom::bindings::conversions::private_from_object;
use dom::bindings::conversions::root_from_handleobject;
use dom::bindings::conversions::root_from_handlevalue;
use dom::bindings::conversions::root_from_object;
use dom::bindings::error::Error;
use dom::bindings::error::Error::JSFailed;
use dom::bindings::error::ErrorResult;
use dom::bindings::error::Fallible;
use dom::bindings::error::throw_dom_exception;
use dom::bindings::guard::Condition;
use dom::bindings::guard::Guard;
use dom::bindings::inheritance::Castable;
use dom::bindings::interface::ConstructorClassHook;
use dom::bindings::interface::InterfaceConstructorBehavior;
use dom::bindings::interface::NonCallbackInterfaceObjectClass;
use dom::bindings::interface::create_callback_interface_object;
use dom::bindings::interface::create_global_object;
use dom::bindings::interface::create_interface_prototype_object;
use dom::bindings::interface::create_named_constructors;
use dom::bindings::interface::create_noncallback_interface_object;
use dom::bindings::interface::define_guarded_constants;
use dom::bindings::interface::define_guarded_methods;
use dom::bindings::interface::define_guarded_properties;
use dom::bindings::interface::is_exposed_in;
use dom::bindings::num::Finite;
use dom::bindings::proxyhandler;
use dom::bindings::proxyhandler::ensure_expando_object;
use dom::bindings::proxyhandler::fill_property_descriptor;
use dom::bindings::proxyhandler::get_expando_object;
use dom::bindings::proxyhandler::get_property_descriptor;
use dom::bindings::reflector::DomObject;
use dom::bindings::reflector::MutDomObject;
use dom::bindings::root::Dom;
use dom::bindings::root::DomRoot;
use dom::bindings::root::OptionalHeapSetter;
use dom::bindings::root::RootedReference;
use dom::bindings::str::ByteString;
use dom::bindings::str::DOMString;
use dom::bindings::str::USVString;
use dom::bindings::trace::JSTraceable;
use dom::bindings::trace::RootedTraceable;
use dom::bindings::trace::RootedTraceableBox;
use dom::bindings::utils::AsVoidPtr;
use dom::bindings::utils::DOMClass;
use dom::bindings::utils::DOMJSClass;
use dom::bindings::utils::DOM_PROTO_UNFORGEABLE_HOLDER_SLOT;
use dom::bindings::utils::JSCLASS_DOM_GLOBAL;
use dom::bindings::utils::ProtoOrIfaceArray;
use dom::bindings::utils::enumerate_global;
use dom::bindings::utils::finalize_global;
use dom::bindings::utils::find_enum_value;
use dom::bindings::utils::generic_getter;
use dom::bindings::utils::generic_lenient_getter;
use dom::bindings::utils::generic_lenient_setter;
use dom::bindings::utils::generic_method;
use dom::bindings::utils::generic_setter;
use dom::bindings::utils::get_array_index_from_id;
use dom::bindings::utils::get_dictionary_property;
use dom::bindings::utils::get_property_on_prototype;
use dom::bindings::utils::get_proto_or_iface_array;
use dom::bindings::utils::has_property_on_prototype;
use dom::bindings::utils::is_platform_object;
use dom::bindings::utils::resolve_global;
use dom::bindings::utils::set_dictionary_property;
use dom::bindings::utils::trace_global;
use dom::bindings::weakref::DOM_WEAK_SLOT;
use dom::bindings::weakref::WeakBox;
use dom::bindings::weakref::WeakReferenceable;
use dom::globalscope::GlobalScope;
use dom::windowproxy::WindowProxy;
use js;
use js::JSCLASS_GLOBAL_SLOT_COUNT;
use js::JSCLASS_IS_DOMJSCLASS;
use js::JSCLASS_IS_GLOBAL;
use js::JSCLASS_RESERVED_SLOTS_MASK;
use js::JS_CALLEE;
use js::error::throw_internal_error;
use js::error::throw_type_error;
use js::glue::AppendToAutoIdVector;
use js::glue::CallJitGetterOp;
use js::glue::CallJitMethodOp;
use js::glue::CallJitSetterOp;
use js::glue::CreateProxyHandler;
use js::glue::GetProxyPrivate;
use js::glue::NewProxyObject;
use js::glue::ProxyTraps;
use js::glue::RUST_JSID_IS_INT;
use js::glue::RUST_JSID_IS_STRING;
use js::glue::RUST_SYMBOL_TO_JSID;
use js::glue::UnwrapObject;
use js::glue::int_to_jsid;
use js::jsapi::AutoIdVector;
use js::jsapi::Call;
use js::jsapi::CallArgs;
use js::jsapi::CurrentGlobalOrNull;
use js::jsapi::FreeOp;
use js::jsapi::GetPropertyKeys;
use js::jsapi::GetWellKnownSymbol;
use js::jsapi::Handle;
use js::jsapi::HandleId;
use js::jsapi::HandleObject;
use js::jsapi::HandleValue;
use js::jsapi::HandleValueArray;
use js::jsapi::Heap;
use js::jsapi::INTERNED_STRING_TO_JSID;
use js::jsapi::IsCallable;
use js::jsapi::JSAutoCompartment;
use js::jsapi::JSCLASS_RESERVED_SLOTS_SHIFT;
use js::jsapi::JSClass;
use js::jsapi::JSContext;
use js::jsapi::JSFreeOp;
use js::jsapi::JSFunctionSpec;
use js::jsapi::JSITER_HIDDEN;
use js::jsapi::JSITER_OWNONLY;
use js::jsapi::JSITER_SYMBOLS;
use js::jsapi::JSJitGetterCallArgs;
use js::jsapi::JSJitInfo;
use js::jsapi::JSJitInfo_AliasSet;
use js::jsapi::JSJitInfo_ArgType;
use js::jsapi::JSJitInfo_OpType;
use js::jsapi::JSJitMethodCallArgs;
use js::jsapi::JSJitSetterCallArgs;
use js::jsapi::JSNative;
use js::jsapi::JSNativeWrapper;
use js::jsapi::JSObject;
use js::jsapi::JSPROP_ENUMERATE;
use js::jsapi::JSPROP_PERMANENT;
use js::jsapi::JSPROP_READONLY;
use js::jsapi::JSPROP_SHARED;
use js::jsapi::JSPropertySpec;
use js::jsapi::JSString;
use js::jsapi::JSTracer;
use js::jsapi::JSType;
use js::jsapi::JSTypedMethodJitInfo;
use js::jsapi::JSValueType;
use js::jsapi::JS_AtomizeAndPinString;
use js::jsapi::JS_CallFunctionValue;
use js::jsapi::JS_CopyPropertiesFrom;
use js::jsapi::JS_DefineProperty;
use js::jsapi::JS_DefinePropertyById2;
use js::jsapi::JS_ForwardGetPropertyTo;
use js::jsapi::JS_GetErrorPrototype;
use js::jsapi::JS_GetFunctionPrototype;
use js::jsapi::JS_GetGlobalForObject;
use js::jsapi::JS_GetIteratorPrototype;
use js::jsapi::JS_GetObjectPrototype;
use js::jsapi::JS_GetProperty;
use js::jsapi::JS_GetPropertyById;
use js::jsapi::JS_GetPropertyDescriptorById;
use js::jsapi::JS_GetReservedSlot;
use js::jsapi::JS_HasProperty;
use js::jsapi::JS_HasPropertyById;
use js::jsapi::JS_InitializePropertiesFromCompatibleNativeObject;
use js::jsapi::JS_NewObject;
use js::jsapi::JS_NewObjectWithGivenProto;
use js::jsapi::JS_NewObjectWithoutMetadata;
use js::jsapi::JS_ObjectIsDate;
use js::jsapi::JS_SetImmutablePrototype;
use js::jsapi::JS_SetProperty;
use js::jsapi::JS_SetPrototype;
use js::jsapi::JS_SetReservedSlot;
use js::jsapi::JS_SplicePrototype;
use js::jsapi::JS_WrapObject;
use js::jsapi::JS_WrapValue;
use js::jsapi::MutableHandle;
use js::jsapi::MutableHandleObject;
use js::jsapi::MutableHandleValue;
use js::jsapi::ObjectOpResult;
use js::jsapi::PropertyDescriptor;
use js::jsapi::RootedId;
use js::jsapi::RootedObject;
use js::jsapi::RootedString;
use js::jsapi::SymbolCode;
use js::jsapi::jsid;
use js::jsval::JSVal;
use js::jsval::NullValue;
use js::jsval::ObjectOrNullValue;
use js::jsval::ObjectValue;
use js::jsval::PrivateValue;
use js::jsval::UndefinedValue;
use js::panic::maybe_resume_unwind;
use js::panic::wrap_panic;
use js::rust::CustomAutoRooterGuard;
use js::rust::GCMethods;
use js::rust::define_methods;
use js::rust::define_properties;
use js::rust::get_object_class;
use libc;
use mem::malloc_size_of_including_raw_self;
use servo_config::prefs::PREFS;
use std::borrow::ToOwned;
use std::cmp;
use std::default::Default;
use std::ffi::CString;
use std::mem;
use std::num;
use std::os;
use std::panic;
use std::ptr;
use std::ptr::NonNull;
use std::rc;
use std::rc::Rc;
use std::str;

pub use self::AttrBinding::{Wrap, AttrMethods, GetProtoObject, DefineDOMInterface};
pub mod AttrBinding {
#![allow(non_camel_case_types,non_upper_case_globals,unused_imports,unused_variables,unused_assignments,unused_mut)]
use dom;
use dom::bindings;
use dom::bindings::callback::CallSetup;
use dom::bindings::callback::CallbackContainer;
use dom::bindings::callback::CallbackFunction;
use dom::bindings::callback::CallbackInterface;
use dom::bindings::callback::CallbackObject;
use dom::bindings::callback::ExceptionHandling;
use dom::bindings::callback::wrap_call_this_object;
use dom::bindings::codegen::InterfaceObjectMap;
use dom::bindings::codegen::PrototypeList;
use dom::bindings::codegen::RegisterBindings;
use dom::bindings::codegen::UnionTypes;
use dom::bindings::constant::ConstantSpec;
use dom::bindings::constant::ConstantVal;
use dom::bindings::conversions::ConversionBehavior;
use dom::bindings::conversions::ConversionResult;
use dom::bindings::conversions::DOM_OBJECT_SLOT;
use dom::bindings::conversions::FromJSValConvertible;
use dom::bindings::conversions::IDLInterface;
use dom::bindings::conversions::StringificationBehavior;
use dom::bindings::conversions::ToJSValConvertible;
use dom::bindings::conversions::is_array_like;
use dom::bindings::conversions::jsid_to_string;
use dom::bindings::conversions::native_from_handlevalue;
use dom::bindings::conversions::native_from_object;
use dom::bindings::conversions::private_from_object;
use dom::bindings::conversions::root_from_handleobject;
use dom::bindings::conversions::root_from_handlevalue;
use dom::bindings::conversions::root_from_object;
use dom::bindings::error::Error;
use dom::bindings::error::Error::JSFailed;
use dom::bindings::error::ErrorResult;
use dom::bindings::error::Fallible;
use dom::bindings::error::throw_dom_exception;
use dom::bindings::guard::Condition;
use dom::bindings::guard::Guard;
use dom::bindings::inheritance::Castable;
use dom::bindings::interface::ConstructorClassHook;
use dom::bindings::interface::InterfaceConstructorBehavior;
use dom::bindings::interface::NonCallbackInterfaceObjectClass;
use dom::bindings::interface::create_callback_interface_object;
use dom::bindings::interface::create_global_object;
use dom::bindings::interface::create_interface_prototype_object;
use dom::bindings::interface::create_named_constructors;
use dom::bindings::interface::create_noncallback_interface_object;
use dom::bindings::interface::define_guarded_constants;
use dom::bindings::interface::define_guarded_methods;
use dom::bindings::interface::define_guarded_properties;
use dom::bindings::interface::is_exposed_in;
use dom::bindings::num::Finite;
use dom::bindings::proxyhandler;
use dom::bindings::proxyhandler::ensure_expando_object;
use dom::bindings::proxyhandler::fill_property_descriptor;
use dom::bindings::proxyhandler::get_expando_object;
use dom::bindings::proxyhandler::get_property_descriptor;
use dom::bindings::reflector::DomObject;
use dom::bindings::reflector::MutDomObject;
use dom::bindings::root::Dom;
use dom::bindings::root::DomRoot;
use dom::bindings::root::OptionalHeapSetter;
use dom::bindings::root::RootedReference;
use dom::bindings::str::ByteString;
use dom::bindings::str::DOMString;
use dom::bindings::str::USVString;
use dom::bindings::trace::JSTraceable;
use dom::bindings::trace::RootedTraceable;
use dom::bindings::trace::RootedTraceableBox;
use dom::bindings::utils::AsVoidPtr;
use dom::bindings::utils::DOMClass;
use dom::bindings::utils::DOMJSClass;
use dom::bindings::utils::DOM_PROTO_UNFORGEABLE_HOLDER_SLOT;
use dom::bindings::utils::JSCLASS_DOM_GLOBAL;
use dom::bindings::utils::ProtoOrIfaceArray;
use dom::bindings::utils::enumerate_global;
use dom::bindings::utils::finalize_global;
use dom::bindings::utils::find_enum_value;
use dom::bindings::utils::generic_getter;
use dom::bindings::utils::generic_lenient_getter;
use dom::bindings::utils::generic_lenient_setter;
use dom::bindings::utils::generic_method;
use dom::bindings::utils::generic_setter;
use dom::bindings::utils::get_array_index_from_id;
use dom::bindings::utils::get_dictionary_property;
use dom::bindings::utils::get_property_on_prototype;
use dom::bindings::utils::get_proto_or_iface_array;
use dom::bindings::utils::has_property_on_prototype;
use dom::bindings::utils::is_platform_object;
use dom::bindings::utils::resolve_global;
use dom::bindings::utils::set_dictionary_property;
use dom::bindings::utils::trace_global;
use dom::bindings::weakref::DOM_WEAK_SLOT;
use dom::bindings::weakref::WeakBox;
use dom::bindings::weakref::WeakReferenceable;
use dom::globalscope::GlobalScope;
use dom::types::Attr;
use dom::types::Element;
use dom::windowproxy::WindowProxy;
use js;
use js::JSCLASS_GLOBAL_SLOT_COUNT;
use js::JSCLASS_IS_DOMJSCLASS;
use js::JSCLASS_IS_GLOBAL;
use js::JSCLASS_RESERVED_SLOTS_MASK;
use js::JS_CALLEE;
use js::error::throw_internal_error;
use js::error::throw_type_error;
use js::glue::AppendToAutoIdVector;
use js::glue::CallJitGetterOp;
use js::glue::CallJitMethodOp;
use js::glue::CallJitSetterOp;
use js::glue::CreateProxyHandler;
use js::glue::GetProxyPrivate;
use js::glue::NewProxyObject;
use js::glue::ProxyTraps;
use js::glue::RUST_JSID_IS_INT;
use js::glue::RUST_JSID_IS_STRING;
use js::glue::RUST_SYMBOL_TO_JSID;
use js::glue::UnwrapObject;
use js::glue::int_to_jsid;
use js::jsapi::AutoIdVector;
use js::jsapi::Call;
use js::jsapi::CallArgs;
use js::jsapi::CurrentGlobalOrNull;
use js::jsapi::FreeOp;
use js::jsapi::GetPropertyKeys;
use js::jsapi::GetWellKnownSymbol;
use js::jsapi::Handle;
use js::jsapi::HandleId;
use js::jsapi::HandleObject;
use js::jsapi::HandleValue;
use js::jsapi::HandleValueArray;
use js::jsapi::Heap;
use js::jsapi::INTERNED_STRING_TO_JSID;
use js::jsapi::IsCallable;
use js::jsapi::JSAutoCompartment;
use js::jsapi::JSCLASS_RESERVED_SLOTS_SHIFT;
use js::jsapi::JSClass;
use js::jsapi::JSContext;
use js::jsapi::JSFreeOp;
use js::jsapi::JSFunctionSpec;
use js::jsapi::JSITER_HIDDEN;
use js::jsapi::JSITER_OWNONLY;
use js::jsapi::JSITER_SYMBOLS;
use js::jsapi::JSJitGetterCallArgs;
use js::jsapi::JSJitInfo;
use js::jsapi::JSJitInfo_AliasSet;
use js::jsapi::JSJitInfo_ArgType;
use js::jsapi::JSJitInfo_OpType;
use js::jsapi::JSJitMethodCallArgs;
use js::jsapi::JSJitSetterCallArgs;
use js::jsapi::JSNative;
use js::jsapi::JSNativeWrapper;
use js::jsapi::JSObject;
use js::jsapi::JSPROP_ENUMERATE;
use js::jsapi::JSPROP_PERMANENT;
use js::jsapi::JSPROP_READONLY;
use js::jsapi::JSPROP_SHARED;
use js::jsapi::JSPropertySpec;
use js::jsapi::JSString;
use js::jsapi::JSTracer;
use js::jsapi::JSType;
use js::jsapi::JSTypedMethodJitInfo;
use js::jsapi::JSValueType;
use js::jsapi::JS_AtomizeAndPinString;
use js::jsapi::JS_CallFunctionValue;
use js::jsapi::JS_CopyPropertiesFrom;
use js::jsapi::JS_DefineProperty;
use js::jsapi::JS_DefinePropertyById2;
use js::jsapi::JS_ForwardGetPropertyTo;
use js::jsapi::JS_GetErrorPrototype;
use js::jsapi::JS_GetFunctionPrototype;
use js::jsapi::JS_GetGlobalForObject;
use js::jsapi::JS_GetIteratorPrototype;
use js::jsapi::JS_GetObjectPrototype;
use js::jsapi::JS_GetProperty;
use js::jsapi::JS_GetPropertyById;
use js::jsapi::JS_GetPropertyDescriptorById;
use js::jsapi::JS_GetReservedSlot;
use js::jsapi::JS_HasProperty;
use js::jsapi::JS_HasPropertyById;
use js::jsapi::JS_InitializePropertiesFromCompatibleNativeObject;
use js::jsapi::JS_NewObject;
use js::jsapi::JS_NewObjectWithGivenProto;
use js::jsapi::JS_NewObjectWithoutMetadata;
use js::jsapi::JS_ObjectIsDate;
use js::jsapi::JS_SetImmutablePrototype;
use js::jsapi::JS_SetProperty;
use js::jsapi::JS_SetPrototype;
use js::jsapi::JS_SetReservedSlot;
use js::jsapi::JS_SplicePrototype;
use js::jsapi::JS_WrapObject;
use js::jsapi::JS_WrapValue;
use js::jsapi::MutableHandle;
use js::jsapi::MutableHandleObject;
use js::jsapi::MutableHandleValue;
use js::jsapi::ObjectOpResult;
use js::jsapi::PropertyDescriptor;
use js::jsapi::RootedId;
use js::jsapi::RootedObject;
use js::jsapi::RootedString;
use js::jsapi::SymbolCode;
use js::jsapi::jsid;
use js::jsval::JSVal;
use js::jsval::NullValue;
use js::jsval::ObjectOrNullValue;
use js::jsval::ObjectValue;
use js::jsval::PrivateValue;
use js::jsval::UndefinedValue;
use js::panic::maybe_resume_unwind;
use js::panic::wrap_panic;
use js::rust::CustomAutoRooterGuard;
use js::rust::GCMethods;
use js::rust::define_methods;
use js::rust::define_properties;
use js::rust::get_object_class;
use libc;
use mem::malloc_size_of_including_raw_self;
use servo_config::prefs::PREFS;
use std::borrow::ToOwned;
use std::cmp;
use std::default::Default;
use std::ffi::CString;
use std::mem;
use std::num;
use std::os;
use std::panic;
use std::ptr;
use std::ptr::NonNull;
use std::rc;
use std::rc::Rc;
use std::str;

unsafe extern fn get_namespaceURI(cx: *mut JSContext, _obj: HandleObject, this: *const Attr, args: JSJitGetterCallArgs) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let this = &*this;
        let result: Option<DOMString> = this.GetNamespaceURI();

        (result).to_jsval(cx, args.rval());
        return true;
    }), false);
}


const namespaceURI_getterinfo: JSJitInfo = JSJitInfo {
    call: get_namespaceURI as *const os::raw::c_void,
    protoID: PrototypeList::ID::Attr as u16,
    depth: 0,
    _bitfield_1: new_jsjitinfo_bitfield_1!(
        JSJitInfo_OpType::Getter as u8,
        JSJitInfo_AliasSet::AliasNone as u8,
        JSValueType::JSVAL_TYPE_UNKNOWN as u8,
        true,
        true,
        false,
        false,
        false,
        false,
        0,
    ),
};

unsafe extern fn get_prefix(cx: *mut JSContext, _obj: HandleObject, this: *const Attr, args: JSJitGetterCallArgs) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let this = &*this;
        let result: Option<DOMString> = this.GetPrefix();

        (result).to_jsval(cx, args.rval());
        return true;
    }), false);
}


const prefix_getterinfo: JSJitInfo = JSJitInfo {
    call: get_prefix as *const os::raw::c_void,
    protoID: PrototypeList::ID::Attr as u16,
    depth: 0,
    _bitfield_1: new_jsjitinfo_bitfield_1!(
        JSJitInfo_OpType::Getter as u8,
        JSJitInfo_AliasSet::AliasNone as u8,
        JSValueType::JSVAL_TYPE_UNKNOWN as u8,
        true,
        true,
        false,
        false,
        false,
        false,
        0,
    ),
};

unsafe extern fn get_localName(cx: *mut JSContext, _obj: HandleObject, this: *const Attr, args: JSJitGetterCallArgs) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let this = &*this;
        let result: DOMString = this.LocalName();

        (result).to_jsval(cx, args.rval());
        return true;
    }), false);
}


const localName_getterinfo: JSJitInfo = JSJitInfo {
    call: get_localName as *const os::raw::c_void,
    protoID: PrototypeList::ID::Attr as u16,
    depth: 0,
    _bitfield_1: new_jsjitinfo_bitfield_1!(
        JSJitInfo_OpType::Getter as u8,
        JSJitInfo_AliasSet::AliasNone as u8,
        JSValueType::JSVAL_TYPE_STRING as u8,
        true,
        true,
        false,
        false,
        false,
        false,
        0,
    ),
};

unsafe extern fn get_name(cx: *mut JSContext, _obj: HandleObject, this: *const Attr, args: JSJitGetterCallArgs) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let this = &*this;
        let result: DOMString = this.Name();

        (result).to_jsval(cx, args.rval());
        return true;
    }), false);
}


const name_getterinfo: JSJitInfo = JSJitInfo {
    call: get_name as *const os::raw::c_void,
    protoID: PrototypeList::ID::Attr as u16,
    depth: 0,
    _bitfield_1: new_jsjitinfo_bitfield_1!(
        JSJitInfo_OpType::Getter as u8,
        JSJitInfo_AliasSet::AliasNone as u8,
        JSValueType::JSVAL_TYPE_STRING as u8,
        true,
        true,
        false,
        false,
        false,
        false,
        0,
    ),
};

unsafe extern fn get_nodeName(cx: *mut JSContext, _obj: HandleObject, this: *const Attr, args: JSJitGetterCallArgs) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let this = &*this;
        let result: DOMString = this.NodeName();

        (result).to_jsval(cx, args.rval());
        return true;
    }), false);
}


const nodeName_getterinfo: JSJitInfo = JSJitInfo {
    call: get_nodeName as *const os::raw::c_void,
    protoID: PrototypeList::ID::Attr as u16,
    depth: 0,
    _bitfield_1: new_jsjitinfo_bitfield_1!(
        JSJitInfo_OpType::Getter as u8,
        JSJitInfo_AliasSet::AliasNone as u8,
        JSValueType::JSVAL_TYPE_STRING as u8,
        true,
        true,
        false,
        false,
        false,
        false,
        0,
    ),
};

unsafe extern fn get_value(cx: *mut JSContext, _obj: HandleObject, this: *const Attr, args: JSJitGetterCallArgs) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let this = &*this;

        let result: DOMString = this.Value();


        (result).to_jsval(cx, args.rval());
        return true;
    }), false);
}

unsafe extern fn set_value(cx: *mut JSContext, obj: HandleObject, this: *const Attr, args: JSJitSetterCallArgs) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let this = &*this;
        let arg0: DOMString = match FromJSValConvertible::from_jsval(cx, args.get(0), StringificationBehavior::Default) {
            Ok(ConversionResult::Success(strval)) => strval,
            Ok(ConversionResult::Failure(error)) => {
                throw_type_error(cx, &error);
                return false;

            }
            _ => { return false;
         },
        };

        let result: () = this.SetValue(arg0);


        return true;
    }), false);
}


const value_getterinfo: JSJitInfo = JSJitInfo {
    call: get_value as *const os::raw::c_void,
    protoID: PrototypeList::ID::Attr as u16,
    depth: 0,
    _bitfield_1: new_jsjitinfo_bitfield_1!(
        JSJitInfo_OpType::Getter as u8,
        JSJitInfo_AliasSet::AliasDOMSets as u8,
        JSValueType::JSVAL_TYPE_STRING as u8,
        true,
        true,
        false,
        false,
        false,
        false,
        0,
    ),
};

const value_setterinfo: JSJitInfo = JSJitInfo {
    call: set_value as *const os::raw::c_void,
    protoID: PrototypeList::ID::Attr as u16,
    depth: 0,
    _bitfield_1: new_jsjitinfo_bitfield_1!(
        JSJitInfo_OpType::Setter as u8,
        JSJitInfo_AliasSet::AliasEverything as u8,
        JSValueType::JSVAL_TYPE_UNDEFINED as u8,
        false,
        false,
        false,
        false,
        false,
        false,
        0,
    ),
};

unsafe extern fn get_textContent(cx: *mut JSContext, _obj: HandleObject, this: *const Attr, args: JSJitGetterCallArgs) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let this = &*this;

        let result: DOMString = this.TextContent();


        (result).to_jsval(cx, args.rval());
        return true;
    }), false);
}

unsafe extern fn set_textContent(cx: *mut JSContext, obj: HandleObject, this: *const Attr, args: JSJitSetterCallArgs) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let this = &*this;
        let arg0: DOMString = match FromJSValConvertible::from_jsval(cx, args.get(0), StringificationBehavior::Default) {
            Ok(ConversionResult::Success(strval)) => strval,
            Ok(ConversionResult::Failure(error)) => {
                throw_type_error(cx, &error);
                return false;

            }
            _ => { return false;
         },
        };

        let result: () = this.SetTextContent(arg0);


        return true;
    }), false);
}


const textContent_getterinfo: JSJitInfo = JSJitInfo {
    call: get_textContent as *const os::raw::c_void,
    protoID: PrototypeList::ID::Attr as u16,
    depth: 0,
    _bitfield_1: new_jsjitinfo_bitfield_1!(
        JSJitInfo_OpType::Getter as u8,
        JSJitInfo_AliasSet::AliasDOMSets as u8,
        JSValueType::JSVAL_TYPE_STRING as u8,
        true,
        true,
        false,
        false,
        false,
        false,
        0,
    ),
};

const textContent_setterinfo: JSJitInfo = JSJitInfo {
    call: set_textContent as *const os::raw::c_void,
    protoID: PrototypeList::ID::Attr as u16,
    depth: 0,
    _bitfield_1: new_jsjitinfo_bitfield_1!(
        JSJitInfo_OpType::Setter as u8,
        JSJitInfo_AliasSet::AliasEverything as u8,
        JSValueType::JSVAL_TYPE_UNDEFINED as u8,
        false,
        false,
        false,
        false,
        false,
        false,
        0,
    ),
};

unsafe extern fn get_nodeValue(cx: *mut JSContext, _obj: HandleObject, this: *const Attr, args: JSJitGetterCallArgs) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let this = &*this;

        let result: DOMString = this.NodeValue();


        (result).to_jsval(cx, args.rval());
        return true;
    }), false);
}

unsafe extern fn set_nodeValue(cx: *mut JSContext, obj: HandleObject, this: *const Attr, args: JSJitSetterCallArgs) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let this = &*this;
        let arg0: DOMString = match FromJSValConvertible::from_jsval(cx, args.get(0), StringificationBehavior::Default) {
            Ok(ConversionResult::Success(strval)) => strval,
            Ok(ConversionResult::Failure(error)) => {
                throw_type_error(cx, &error);
                return false;

            }
            _ => { return false;
         },
        };

        let result: () = this.SetNodeValue(arg0);



        return true;
    }), false);
}


const nodeValue_getterinfo: JSJitInfo = JSJitInfo {
    call: get_nodeValue as *const os::raw::c_void,
    protoID: PrototypeList::ID::Attr as u16,
    depth: 0,
    _bitfield_1: new_jsjitinfo_bitfield_1!(
        JSJitInfo_OpType::Getter as u8,
        JSJitInfo_AliasSet::AliasDOMSets as u8,
        JSValueType::JSVAL_TYPE_STRING as u8,
        true,
        true,
        false,
        false,
        false,
        false,
        0,
    ),
};

const nodeValue_setterinfo: JSJitInfo = JSJitInfo {
    call: set_nodeValue as *const os::raw::c_void,
    protoID: PrototypeList::ID::Attr as u16,
    depth: 0,
    _bitfield_1: new_jsjitinfo_bitfield_1!(
        JSJitInfo_OpType::Setter as u8,
        JSJitInfo_AliasSet::AliasEverything as u8,
        JSValueType::JSVAL_TYPE_UNDEFINED as u8,
        false,
        false,
        false,
        false,
        false,
        false,
        0,
    ),
};

unsafe extern fn get_ownerElement(cx: *mut JSContext, _obj: HandleObject, this: *const Attr, args: JSJitGetterCallArgs) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let this = &*this;
        let result: Option<DomRoot<Element>> = this.GetOwnerElement();

        (result).to_jsval(cx, args.rval());
        return true;
    }), false);
}


const ownerElement_getterinfo: JSJitInfo = JSJitInfo {
    call: get_ownerElement as *const os::raw::c_void,
    protoID: PrototypeList::ID::Attr as u16,
    depth: 0,
    _bitfield_1: new_jsjitinfo_bitfield_1!(
        JSJitInfo_OpType::Getter as u8,
        JSJitInfo_AliasSet::AliasDOMSets as u8,
        JSValueType::JSVAL_TYPE_UNKNOWN as u8,
        true,
        true,
        false,
        false,
        false,
        false,
        0,
    ),
};

unsafe extern fn get_specified(cx: *mut JSContext, _obj: HandleObject, this: *const Attr, args: JSJitGetterCallArgs) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let this = &*this;
        let result: bool = this.Specified();

        (result).to_jsval(cx, args.rval());
        return true;
    }), false);
}


const specified_getterinfo: JSJitInfo = JSJitInfo {
    call: get_specified as *const os::raw::c_void,
    protoID: PrototypeList::ID::Attr as u16,
    depth: 0,
    _bitfield_1: new_jsjitinfo_bitfield_1!(
        JSJitInfo_OpType::Getter as u8,
        JSJitInfo_AliasSet::AliasNone as u8,
        JSValueType::JSVAL_TYPE_BOOLEAN as u8,
        true,
        true,
        false,
        false,
        false,
        false,
        0,
    ),
};

unsafe extern fn _finalize(_fop: *mut JSFreeOp, obj: *mut JSObject) {
    return wrap_panic(panic::AssertUnwindSafe(|| {

        let this = native_from_object::<Attr>(obj).unwrap();
        if !this.is_null() {
            // The pointer can be null if the object is the unforgeable holder of that interface.
            let _ = Box::from_raw(this as *mut Attr);
        }
        debug!("Attr finalize: {:p}", this);
    }), ());
}

unsafe extern fn _trace(trc: *mut JSTracer, obj: *mut JSObject) {
    return wrap_panic(panic::AssertUnwindSafe(|| {

        let this = native_from_object::<Attr>(obj).unwrap();
        if this.is_null() { return; } // GC during obj creation
        (*this).trace(trc);
    }), ());
}

static CLASS_OPS: js::jsapi::JSClassOps = js::jsapi::JSClassOps {
    addProperty: None,
    delProperty: None,
    getProperty: None,
    setProperty: None,
    enumerate: None,
    resolve: None,
    mayResolve: None,
    finalize: Some(_finalize),
    call: None,
    hasInstance: None,
    construct: None,
    trace: Some(_trace),
};

static Class: DOMJSClass = DOMJSClass {
    base: js::jsapi::JSClass {
        name: b"Attr\0" as *const u8 as *const libc::c_char,
        flags: JSCLASS_IS_DOMJSCLASS | 0 |
               (((1) & JSCLASS_RESERVED_SLOTS_MASK) << JSCLASS_RESERVED_SLOTS_SHIFT)
               /* JSCLASS_HAS_RESERVED_SLOTS(1) */,
        cOps: &CLASS_OPS,
        reserved: [0 as *mut _; 3],
    },
    dom_class: DOMClass {
    interface_chain: [ PrototypeList::ID::Attr, PrototypeList::ID::Last, PrototypeList::ID::Last, PrototypeList::ID::Last, PrototypeList::ID::Last, PrototypeList::ID::Last ],
    type_id: ::dom::bindings::codegen::InheritTypes::TopTypeId { alone: () },
    malloc_size_of: malloc_size_of_including_raw_self::<Attr> as unsafe fn(&mut _, _) -> _,
    global: InterfaceObjectMap::Globals::EMPTY,
}
};
pub unsafe fn Wrap(cx: *mut JSContext, scope: &GlobalScope, object: Box<Attr>) -> DomRoot<Attr> {
    let scope = scope.reflector().get_jsobject();
    assert!(!scope.get().is_null());
    assert!(((*get_object_class(scope.get())).flags & JSCLASS_IS_GLOBAL) != 0);

    rooted!(in(cx) let mut proto = ptr::null_mut::<JSObject>());
    let _ac = JSAutoCompartment::new(cx, scope.get());
    GetProtoObject(cx, scope, proto.handle_mut());
    assert!(!proto.is_null());

    let raw = Box::into_raw(object);
    let _rt = RootedTraceable::new(&*raw);
    rooted!(in(cx) let obj = JS_NewObjectWithGivenProto(
        cx, &Class.base as *const JSClass, proto.handle()));
    assert!(!obj.is_null());

    JS_SetReservedSlot(obj.get(), DOM_OBJECT_SLOT,
                       PrivateValue(raw as *const libc::c_void));


    (*raw).init_reflector(obj.get());

    DomRoot::from_ref(&*raw)
}

impl IDLInterface for Attr {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        class as *const _ == &Class.dom_class as *const _
    }
}

impl PartialEq for Attr {
    fn eq(&self, other: &Attr) -> bool {
        self as *const Attr == &*other
    }
}

pub trait AttrMethods {
    fn GetNamespaceURI(&self) -> Option<DOMString>;
    fn GetPrefix(&self) -> Option<DOMString>;
    fn LocalName(&self) -> DOMString;
    fn Name(&self) -> DOMString;
    fn NodeName(&self) -> DOMString;
    fn Value(&self) -> DOMString;
    fn SetValue(&self, value: DOMString) -> ();
    fn TextContent(&self) -> DOMString;
    fn SetTextContent(&self, value: DOMString) -> ();
    fn NodeValue(&self) -> DOMString;
    fn SetNodeValue(&self, value: DOMString) -> ();
    fn GetOwnerElement(&self) -> Option<DomRoot<Element>>;
    fn Specified(&self) -> bool;
}
const sAttributes_specs: &'static [&'static[JSPropertySpec]] = &[
&[
    JSPropertySpec {
        name: b"namespaceURI\0" as *const u8 as *const libc::c_char,
        flags: (JSPROP_ENUMERATE | JSPROP_SHARED) as u8,
        getter: JSNativeWrapper { op: Some(generic_getter), info: &namespaceURI_getterinfo },
        setter: JSNativeWrapper { op: None, info: 0 as *const JSJitInfo }
    },
    JSPropertySpec {
        name: b"prefix\0" as *const u8 as *const libc::c_char,
        flags: (JSPROP_ENUMERATE | JSPROP_SHARED) as u8,
        getter: JSNativeWrapper { op: Some(generic_getter), info: &prefix_getterinfo },
        setter: JSNativeWrapper { op: None, info: 0 as *const JSJitInfo }
    },
    JSPropertySpec {
        name: b"localName\0" as *const u8 as *const libc::c_char,
        flags: (JSPROP_ENUMERATE | JSPROP_SHARED) as u8,
        getter: JSNativeWrapper { op: Some(generic_getter), info: &localName_getterinfo },
        setter: JSNativeWrapper { op: None, info: 0 as *const JSJitInfo }
    },
    JSPropertySpec {
        name: b"name\0" as *const u8 as *const libc::c_char,
        flags: (JSPROP_ENUMERATE | JSPROP_SHARED) as u8,
        getter: JSNativeWrapper { op: Some(generic_getter), info: &name_getterinfo },
        setter: JSNativeWrapper { op: None, info: 0 as *const JSJitInfo }
    },
    JSPropertySpec {
        name: b"nodeName\0" as *const u8 as *const libc::c_char,
        flags: (JSPROP_ENUMERATE | JSPROP_SHARED) as u8,
        getter: JSNativeWrapper { op: Some(generic_getter), info: &nodeName_getterinfo },
        setter: JSNativeWrapper { op: None, info: 0 as *const JSJitInfo }
    },
    JSPropertySpec {
        name: b"value\0" as *const u8 as *const libc::c_char,
        flags: (JSPROP_ENUMERATE | JSPROP_SHARED) as u8,
        getter: JSNativeWrapper { op: Some(generic_getter), info: &value_getterinfo },
        setter: JSNativeWrapper { op: Some(generic_setter), info: &value_setterinfo }
    },
    JSPropertySpec {
        name: b"textContent\0" as *const u8 as *const libc::c_char,
        flags: (JSPROP_ENUMERATE | JSPROP_SHARED) as u8,
        getter: JSNativeWrapper { op: Some(generic_getter), info: &textContent_getterinfo },
        setter: JSNativeWrapper { op: Some(generic_setter), info: &textContent_setterinfo }
    },
    JSPropertySpec {
        name: b"nodeValue\0" as *const u8 as *const libc::c_char,
        flags: (JSPROP_ENUMERATE | JSPROP_SHARED) as u8,
        getter: JSNativeWrapper { op: Some(generic_getter), info: &nodeValue_getterinfo },
        setter: JSNativeWrapper { op: Some(generic_setter), info: &nodeValue_setterinfo }
    },
    JSPropertySpec {
        name: b"ownerElement\0" as *const u8 as *const libc::c_char,
        flags: (JSPROP_ENUMERATE | JSPROP_SHARED) as u8,
        getter: JSNativeWrapper { op: Some(generic_getter), info: &ownerElement_getterinfo },
        setter: JSNativeWrapper { op: None, info: 0 as *const JSJitInfo }
    },
    JSPropertySpec {
        name: b"specified\0" as *const u8 as *const libc::c_char,
        flags: (JSPROP_ENUMERATE | JSPROP_SHARED) as u8,
        getter: JSNativeWrapper { op: Some(generic_getter), info: &specified_getterinfo },
        setter: JSNativeWrapper { op: None, info: 0 as *const JSJitInfo }
    },
    JSPropertySpec {
        name: 0 as *const libc::c_char,
        flags: 0,
        getter: JSNativeWrapper { op: None, info: 0 as *const JSJitInfo },
        setter: JSNativeWrapper { op: None, info: 0 as *const JSJitInfo }
    }]

];
const sAttributes: &'static [Guard<&'static [JSPropertySpec]>] = &[
    Guard::new(Condition::Satisfied, sAttributes_specs[0])
];

pub unsafe fn GetProtoObject(cx: *mut JSContext, global: HandleObject, rval: MutableHandleObject) {
    /* Get the interface prototype object for this class.  This will create the
       object as needed. */
    assert!(((*get_object_class(global.get())).flags & JSCLASS_DOM_GLOBAL) != 0);

    /* Check to see whether the interface objects are already installed */
    let proto_or_iface_array = get_proto_or_iface_array(global.get());
    rval.set((*proto_or_iface_array)[PrototypeList::ID::Attr as usize]);
    if !rval.get().is_null() {
        return;
    }

    CreateInterfaceObjects(cx, global, proto_or_iface_array);
    rval.set((*proto_or_iface_array)[PrototypeList::ID::Attr as usize]);
    assert!(!rval.get().is_null());

}

static PrototypeClass: JSClass = JSClass {
    name: b"AttrPrototype\0" as *const u8 as *const libc::c_char,
    flags:
        // JSCLASS_HAS_RESERVED_SLOTS(0)
        (0 & JSCLASS_RESERVED_SLOTS_MASK) << JSCLASS_RESERVED_SLOTS_SHIFT,
    cOps: 0 as *const _,
    reserved: [0 as *mut os::raw::c_void; 3]
};

static INTERFACE_OBJECT_CLASS: NonCallbackInterfaceObjectClass =
    NonCallbackInterfaceObjectClass::new(
        &InterfaceConstructorBehavior::throw(),
        b"function Attr() {\n    [native code]\n}",
        PrototypeList::ID::Attr,
        0);

pub unsafe fn DefineDOMInterface(cx: *mut JSContext, global: HandleObject) {
    assert!(!global.get().is_null());

    if !ConstructorEnabled(cx, global) {
        return;
    }

    rooted!(in(cx) let mut proto = ptr::null_mut::<JSObject>());
    GetProtoObject(cx, global, proto.handle_mut());
    assert!(!proto.is_null());
}

unsafe fn ConstructorEnabled(aCx: *mut JSContext, aObj: HandleObject) -> bool {
    is_exposed_in(aObj, InterfaceObjectMap::Globals::WINDOW)
}

unsafe fn CreateInterfaceObjects(cx: *mut JSContext, global: HandleObject, cache: *mut ProtoOrIfaceArray) {
    rooted!(in(cx) let mut prototype_proto = ptr::null_mut::<JSObject>());
    prototype_proto.set(JS_GetObjectPrototype(cx, global));
    assert!(!prototype_proto.is_null());

    rooted!(in(cx) let mut prototype = ptr::null_mut::<JSObject>());
    create_interface_prototype_object(cx,
                                      prototype_proto.handle(),
                                      &PrototypeClass,
                                      &[],
                                      sAttributes,
                                      &[],
                                      &[],
                                      prototype.handle_mut());
    assert!(!prototype.is_null());
    assert!((*cache)[PrototypeList::ID::Attr as usize].is_null());
    (*cache)[PrototypeList::ID::Attr as usize] = prototype.get();
    <*mut JSObject>::post_barrier((*cache).as_mut_ptr().offset(PrototypeList::ID::Attr as isize),
                                  ptr::null_mut(),
                                  prototype.get());


    rooted!(in(cx) let interface_proto = JS_GetFunctionPrototype(cx, global));
    assert!(!interface_proto.is_null());

    rooted!(in(cx) let mut interface = ptr::null_mut::<JSObject>());
    create_noncallback_interface_object(cx,
                                        global,
                                        interface_proto.handle(),
                                        &INTERFACE_OBJECT_CLASS,
                                        &[],
                                        &[],
                                        &[],
                                        prototype.handle(),
                                        b"Attr\0",
                                        0,
                                        interface.handle_mut());
    assert!(!interface.is_null());
}
} // mod AttrBinding


