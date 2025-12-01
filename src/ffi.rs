/*use std::os::raw::c_int;
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
*/

use std::os::raw::{c_char, c_double, c_int};

#[repr(C)]
pub struct TreeNDS {
    _private: [u8; 0],
}

unsafe extern "C" {
    // constructeur / destructeur
    pub fn initializationTreeNDS(
        nbCrit: c_int,
        nbVar: c_int,
        maxChildren: c_int,
        maxLocalTabSize: c_int,
    ) -> *mut TreeNDS;

    pub fn deleteTreeNDS(t: *mut TreeNDS) -> *mut TreeNDS;
    pub fn freeTreeNDS(t: *mut TreeNDS);

    // utilitaires
    pub fn max_double(a: c_double, b: c_double) -> c_double;
    pub fn euclideanDistance(obj1: *const c_int, obj2: *const c_int, nbCrit: c_int) -> c_double;

    // accès / affichage
    pub fn displayTree(t: *mut TreeNDS);
    pub fn writeTreeNDS(t: *mut TreeNDS, all: c_int, name: *const c_char);
    //pub fn writeScanTreeNDS(t: *mut TreeNDS, all: c_int, f: *mut FILE);

    // informations
    pub fn totalSizeTree(t: *mut TreeNDS) -> c_int;
    pub fn totalNbLeaves(t: *mut TreeNDS) -> c_int;
    pub fn totalNbInternalNodes(t: *mut TreeNDS) -> c_int;

    // initialisation / mise à jour
    pub fn initializationNadirIdealTreeNDS(
        t: *mut TreeNDS,
        nadir: *const c_int,
        ideal: *const c_int,
    );
    pub fn updateIdealNadir(t: *mut TreeNDS, obj: *const c_int);

    // opérations principales
    pub fn addSolutionTreeFirst(t: *mut TreeNDS, x: *const c_int, obj: *const c_int) -> c_int;
    pub fn addSolutionTree(t: *mut TreeNDS, x: *const c_int, obj: *const c_int) -> c_int;

    // fonctions internes exposées (pointer-to-pointer, etc.)
    pub fn upDateNode(t: *mut *mut TreeNDS, x: *const c_int, obj: *const c_int, added: *mut c_int);
    pub fn insertTree(t: *mut TreeNDS, x: *const c_int, obj: *const c_int);
    pub fn splitTree(t: *mut TreeNDS);
}
