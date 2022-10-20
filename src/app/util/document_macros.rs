#[macro_export]
macro_rules! doc_object {
    
    ($expression:expr) => {
        doc_add_group(EMPTY_CSTR);
        $expression
        doc_close_group();
    };

    ($prop_name:literal, $expression:expr) => {
        doc_add_group(to_c_string($prop_name));
        $expression
        doc_close_group();
    };
} 

#[macro_export]
macro_rules! doc_array {
    ($expression:expr) => {
        doc_add_array(EMPTY_CSTR);
        $expression
        doc_close_array();
    };

    ($prop_name:literal, $expression:expr) => {
        doc_add_array(to_c_string($prop_name));
        $expression
        doc_close_array();
    };
} 


pub use doc_object;
pub use doc_array;

/*
fn t() {
    obj_write!({
        obj_write!({
            let i = 1;
        });
    });

    crate::app::safe::doc_add_group(crate::common::extensions::to_c_string("test\0"));
    
    doc_close_group();
} */