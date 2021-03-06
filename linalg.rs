use core::libc::{c_double, c_int, c_void, size_t};
use sci::{matrix, gsl_vector, gsl_matrix, vector};


pub type gsl_permutation = c_void;


extern mod gsl {
    fn gsl_permutation_calloc(n: size_t) -> *gsl_permutation;
    fn gsl_permutation_free(p: *gsl_permutation);

    // LU decomposition
    fn gsl_linalg_LU_decomp (A: *gsl_matrix, p: *gsl_permutation, signum: *mut c_int) -> c_int;
    fn gsl_linalg_LU_solve (LU: *gsl_matrix, p: *gsl_permutation, b: *gsl_vector, x: *gsl_vector) -> c_int;
    fn gsl_linalg_LU_svx (LU: *gsl_matrix,  p: *gsl_permutation, x: *gsl_vector) -> c_int;
    fn gsl_linalg_LU_refine (A: *gsl_matrix, LU: *gsl_matrix, p: *gsl_permutation, b: *gsl_vector, x: *mut gsl_vector, residual: *mut gsl_vector) -> c_int;
    fn gsl_linalg_LU_det (LU: *mut gsl_matrix, signum: c_int) -> c_double;
    fn gsl_linalg_LU_lndet (LU: *mut gsl_matrix) -> c_double;
    fn gsl_linalg_LU_sgndet (LU: *mut gsl_matrix, signum: c_int) -> c_int;

    // Singular Value Decomposition
    fn gsl_linalg_SV_decomp (A: *gsl_matrix, V: *gsl_matrix, S: *gsl_vector, work: *gsl_vector) -> c_int;
    fn gsl_linalg_SV_decomp_mod (A: *gsl_matrix, X: *gsl_matrix, V: *gsl_matrix, S: *gsl_vector, work: *gsl_vector) -> c_int;
    fn gsl_linalg_SV_decomp_jacobi (A: *gsl_matrix, V: *gsl_matrix, S: *gsl_vector) -> c_int;
    fn gsl_linalg_SV_solve (U: *gsl_matrix, V: *gsl_matrix, S: *gsl_vector, b: *gsl_vector, x: *gsl_vector) -> c_int;
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
    fn decomp(m: &matrix) -> LU {
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


pub struct SV {
    U: matrix,
    S: vector,
    V: matrix
}


pub impl SV {
    fn decomp(m: &matrix) -> SV {
        let mut U = m.clone();
        let (_, n) = U.size;
        let mut S = vector::zeros(n);
        let mut V = matrix::zeros(n, n);
        let mut work = vector::zeros(n);

        unsafe {
            gsl::gsl_linalg_SV_decomp(U.ptr, V.ptr, S.ptr, work.ptr);
        }

        SV { U: U, S: S, V: V }
    }
}
