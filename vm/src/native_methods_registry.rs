use std::{collections::HashMap, fmt, fmt::Formatter};

use crate::{
    call_stack::CallStack,
    class_and_method::ClassAndMethod,
    value::{ObjectRef, Value},
    vm::Vm,
    vm_error::VmError,
};

pub type NativeCallback<'a> = fn(
    &mut Vm<'a>,
    &mut CallStack<'a>,
    Option<ObjectRef<'a>>,
    Vec<Value<'a>>,
) -> Result<Option<Value<'a>>, VmError>;

#[derive(Default)]
pub struct NativeMethodsRegistry<'a> {
    methods: HashMap<ClassMethodAndDescriptor, NativeCallback<'a>>,

    // Hack for checking that integration tests can actually print the correct values:
    // this just stores the values printed by a method named `tempPrint` into an array
    // in the Vm object
    temp_print_callback: Option<NativeCallback<'a>>,
}

impl<'a> fmt::Debug for NativeMethodsRegistry<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "NativeMethodsRegistry={:?}", self.methods.keys())
    }
}

impl<'a> NativeMethodsRegistry<'a> {
    pub fn register(
        &mut self,
        class_name: &str,
        method_name: &str,
        type_descriptor: &str,
        callback: NativeCallback<'a>,
    ) {
        self.methods.insert(
            ClassMethodAndDescriptor {
                class: class_name.to_string(),
                method: method_name.to_string(),
                descriptor: type_descriptor.to_string(),
            },
            callback,
        );
    }

    pub fn register_temp_print(&mut self, callback: NativeCallback<'a>) {
        self.temp_print_callback = Some(callback);
    }

    pub fn get_method(&self, class_and_method: &ClassAndMethod) -> Option<NativeCallback<'a>> {
        self.get(
            &class_and_method.class.name,
            &class_and_method.method.name,
            &class_and_method.method.type_descriptor,
        )
    }

    pub fn get(
        &self,
        class_name: &str,
        method_name: &str,
        type_descriptor: &str,
    ) -> Option<NativeCallback<'a>> {
        if class_name.starts_with("rjvm/") && method_name == "tempPrint" {
            self.temp_print_callback
        } else {
            self.methods
                .get(&ClassMethodAndDescriptor {
                    class: class_name.to_string(),
                    method: method_name.to_string(),
                    descriptor: type_descriptor.to_string(),
                })
                .cloned()
        }
    }
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct ClassMethodAndDescriptor {
    class: String,
    method: String,
    descriptor: String,
}