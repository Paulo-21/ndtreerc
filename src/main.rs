use std::os::raw::c_int;

#[link(name = "ndtree")]
unsafe extern "C" {
    //fn euclideanDistance(obj1: intptr_t, obj2: intptr_t, nbCrit: c_int) -> c_double;
    pub fn initializationTreeNDS(
        nbCrit: c_int,
        nbVar: c_int,
        maxChildren: c_int,
        maxLocalTabSize: c_int,
    ) -> *mut TreeNDS;
    pub fn addSolutionTree(t: *mut TreeNDS, x: *const i32, obj: *const i32) -> i32;
}
#[repr(C)]
pub struct TreeNDS {
    _private: [u8; 0], // structure opaque
}
fn main() {
    let obj1 = [0, 2, 4, 5];
    let obj2 = [0, 2, 99, 3];
    //let x = unsafe { euclideanDistance(&obj1, &obj2, 4) };
    let x = unsafe { initializationTreeNDS(4, 10, 20, 7) };
}
