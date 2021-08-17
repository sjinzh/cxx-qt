mod my_object {
    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;
            #[namespace = ""]
            type QString = cxx_qt_lib::QString;

            #[rust_name = "number"]
            fn getNumber(self: &MyObject) -> i32;
            #[rust_name = "set_number"]
            fn setNumber(self: Pin<&mut MyObject>, value: i32);

            #[rust_name = "string"]
            fn getString(self: &MyObject) -> &QString;
            #[rust_name = "set_string"]
            fn setString(self: Pin<&mut MyObject>, value: &QString);

            #[rust_name = "new_MyObject"]
            fn newMyObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type MyObjectRs;

            #[cxx_name = "createMyObjectRs"]
            fn create_my_object_rs() -> Box<MyObjectRs>;
        }
    }

    pub type CppObj = ffi::MyObject;

    struct MyObjectRs {
        number: i32,
        string: String,
    }

    struct MyObjectWrapper<'a> {
        cpp: std::pin::Pin<&'a mut CppObj>,
    }

    impl<'a> MyObjectWrapper<'a> {
        fn new(cpp: std::pin::Pin<&'a mut CppObj>) -> Self {
            Self { cpp }
        }

        fn number(&self) -> i32 {
            self.cpp.number()
        }

        fn set_number(&mut self, value: i32) {
            self.cpp.as_mut().set_number(value);
        }

        fn string(&self) -> &cxx_qt_lib::QString {
            self.cpp.string()
        }

        fn set_string(&mut self, value: &cxx_qt_lib::QString) {
            self.cpp.as_mut().set_string(value);
        }
    }

    #[derive(Default)]
    struct Data {
        number: i32,
        string: String,
    }

    impl From<Data> for MyObjectRs {
        fn from(value: Data) -> Self {
            Self {
                number: value.number,
                string: value.string,
            }
        }
    }

    impl From<&MyObjectRs> for Data {
        fn from(value: &MyObjectRs) -> Self {
            Self {
                number: value.number.clone(),
                string: value.string.clone(),
            }
        }
    }

    fn create_my_object_rs() -> Box<MyObjectRs> {
        Box::new(Data::default().into())
    }
}
