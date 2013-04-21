use core::libc::{c_int, c_void, size_t};
use sci::{matrix, gsl_matrix};


type gsl_permutation = c_void;


extern mod gsl {
    fn gsl_permutation_calloc(n: size_t) -> *gsl_permutation;
    fn gsl_permutation_free(p: *gsl_permutation);

    fn gsl_linalg_LU_decomp (A: *gsl_matrix, p: *gsl_permutation, signum: *mut c_int) -> c_int;
}


struct permutation {
    ptr: *gsl_permutation
}


impl permutation {
    fn new(n: u64) -> permutation {
        unsafe { permutation { ptr: gsl::gsl_permutation_calloc(n) } }
    }
}


impl Drop for permutation {
    fn finalize(&self){
        unsafe { gsl::gsl_permutation_free(self.ptr); }
    }
}


pub struct LU {
    mat: matrix,
    p: permutation,
    signum: i32
}


pub impl LU {
    fn decomp(m: matrix) -> LU {
        let mut mat = m.clone();
        let (n, _) = mat.size;
        let mut p = permutation::new(n);
        let mut signum = ~1;

        unsafe {
            gsl::gsl_linalg_LU_decomp(mat.ptr, p.ptr, ptr::to_mut_unsafe_ptr(signum));
        }

        LU { mat: mat, p: p, signum: *signum }
    }
}



