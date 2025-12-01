use crate::ffi;
use std::ffi::CString;
use std::os::raw::c_int;
//#[derive(Debug)]
pub struct NdsTree {
    ptr: *mut ffi::TreeNDS,
}

impl NdsTree {
    pub fn new(nb_crit: i32, nb_var: i32, max_children: i32, max_local: i32) -> Self {
        let ptr = unsafe {
            ffi::initializationTreeNDS(
                nb_crit as c_int,
                nb_var as c_int,
                max_children as c_int,
                max_local as c_int,
            )
        };
        if ptr.is_null() {
            panic!("initializationTreeNDS returned NULL");
        }
        Self { ptr }
    }

    /// ajoute une solution : x et obj doivent avoir les bonnes tailles (nbVar / nbCrit)
    pub fn add(&self, x: &[i32], obj: &[i32]) -> i32 {
        assert!(!self.ptr.is_null());
        unsafe {
            ffi::addSolutionTree(
                self.ptr,
                x.as_ptr() as *const c_int,
                obj.as_ptr() as *const c_int,
            )
        }
    }

    /// affiche (wrapper pratique)
    pub fn display(&self) {
        unsafe { ffi::displayTree(self.ptr) }
    }

    /// write to a file by name
    pub fn write_to_file(&self, all: i32, name: &str) {
        let c = CString::new(name).expect("CString::new failed");
        unsafe { ffi::writeTreeNDS(self.ptr, all as c_int, c.as_ptr()) }
    }

    pub fn total_size(&self) -> i32 {
        unsafe { ffi::totalSizeTree(self.ptr) }
    }
}

impl Drop for NdsTree {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let _ = ffi::deleteTreeNDS(self.ptr);
            }
        }
    }
}
