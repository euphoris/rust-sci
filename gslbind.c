#include <stdio.h>
#include <gsl/gsl_blas.h>
#include <gsl/gsl_matrix.h> 
#include <gsl/gsl_rng.h>

const gsl_rng_type *rng_borosh13(){ return gsl_rng_borosh13; }
const gsl_rng_type *rng_coveyou(){ return gsl_rng_coveyou; }
const gsl_rng_type *rng_cmrg(){ return gsl_rng_cmrg; }
const gsl_rng_type *rng_fishman18(){ return gsl_rng_fishman18; }
const gsl_rng_type *rng_fishman20(){ return gsl_rng_fishman20; }
const gsl_rng_type *rng_fishman2x(){ return gsl_rng_fishman2x; }
const gsl_rng_type *rng_gfsr4(){ return gsl_rng_gfsr4; }
const gsl_rng_type *rng_knuthran(){ return gsl_rng_knuthran; }
const gsl_rng_type *rng_knuthran2(){ return gsl_rng_knuthran2; }
const gsl_rng_type *rng_knuthran2002(){ return gsl_rng_knuthran2002; }
const gsl_rng_type *rng_lecuyer21(){ return gsl_rng_lecuyer21; }
const gsl_rng_type *rng_minstd(){ return gsl_rng_minstd; }
const gsl_rng_type *rng_mrg(){ return gsl_rng_mrg; }
const gsl_rng_type *rng_mt19937(){ return gsl_rng_mt19937; }
const gsl_rng_type *rng_mt19937_1999(){ return gsl_rng_mt19937_1999; }
const gsl_rng_type *rng_mt19937_1998(){ return gsl_rng_mt19937_1998; }
const gsl_rng_type *rng_r250(){ return gsl_rng_r250; }
const gsl_rng_type *rng_ran0(){ return gsl_rng_ran0; }
const gsl_rng_type *rng_ran1(){ return gsl_rng_ran1; }
const gsl_rng_type *rng_ran2(){ return gsl_rng_ran2; }
const gsl_rng_type *rng_ran3(){ return gsl_rng_ran3; }
const gsl_rng_type *rng_rand(){ return gsl_rng_rand; }
const gsl_rng_type *rng_rand48(){ return gsl_rng_rand48; }
const gsl_rng_type *rng_random128_bsd(){ return gsl_rng_random128_bsd; }
const gsl_rng_type *rng_random128_glibc2(){ return gsl_rng_random128_glibc2; }
const gsl_rng_type *rng_random128_libc5(){ return gsl_rng_random128_libc5; }
const gsl_rng_type *rng_random256_bsd(){ return gsl_rng_random256_bsd; }
const gsl_rng_type *rng_random256_glibc2(){ return gsl_rng_random256_glibc2; }
const gsl_rng_type *rng_random256_libc5(){ return gsl_rng_random256_libc5; }
const gsl_rng_type *rng_random32_bsd(){ return gsl_rng_random32_bsd; }
const gsl_rng_type *rng_random32_glibc2(){ return gsl_rng_random32_glibc2; }
const gsl_rng_type *rng_random32_libc5(){ return gsl_rng_random32_libc5; }
const gsl_rng_type *rng_random64_bsd(){ return gsl_rng_random64_bsd; }
const gsl_rng_type *rng_random64_glibc2(){ return gsl_rng_random64_glibc2; }
const gsl_rng_type *rng_random64_libc5(){ return gsl_rng_random64_libc5; }
const gsl_rng_type *rng_random8_bsd(){ return gsl_rng_random8_bsd; }
const gsl_rng_type *rng_random8_glibc2(){ return gsl_rng_random8_glibc2; }
const gsl_rng_type *rng_random8_libc5(){ return gsl_rng_random8_libc5; }
const gsl_rng_type *rng_random_bsd(){ return gsl_rng_random_bsd; }
const gsl_rng_type *rng_random_glibc2(){ return gsl_rng_random_glibc2; }
const gsl_rng_type *rng_random_libc5(){ return gsl_rng_random_libc5; }
const gsl_rng_type *rng_randu(){ return gsl_rng_randu; }
const gsl_rng_type *rng_ranf(){ return gsl_rng_ranf; }
const gsl_rng_type *rng_ranlux(){ return gsl_rng_ranlux; }
const gsl_rng_type *rng_ranlux389(){ return gsl_rng_ranlux389; }
const gsl_rng_type *rng_ranlxd1(){ return gsl_rng_ranlxd1; }
const gsl_rng_type *rng_ranlxd2(){ return gsl_rng_ranlxd2; }
const gsl_rng_type *rng_ranlxs0(){ return gsl_rng_ranlxs0; }
const gsl_rng_type *rng_ranlxs1(){ return gsl_rng_ranlxs1; }
const gsl_rng_type *rng_ranlxs2(){ return gsl_rng_ranlxs2; }
const gsl_rng_type *rng_ranmar(){ return gsl_rng_ranmar; }
const gsl_rng_type *rng_slatec(){ return gsl_rng_slatec; }
const gsl_rng_type *rng_taus(){ return gsl_rng_taus; }
const gsl_rng_type *rng_taus2(){ return gsl_rng_taus2; }
const gsl_rng_type *rng_taus113(){ return gsl_rng_taus113; }
const gsl_rng_type *rng_transputer(){ return gsl_rng_transputer; }
const gsl_rng_type *rng_tt800(){ return gsl_rng_tt800; }
const gsl_rng_type *rng_uni(){ return gsl_rng_uni; }
const gsl_rng_type *rng_uni32(){ return gsl_rng_uni32; }
const gsl_rng_type *rng_vax(){ return gsl_rng_vax; }
const gsl_rng_type *rng_waterman14(){ return gsl_rng_waterman14; }
const gsl_rng_type *rng_zuf(){ return gsl_rng_zuf; }

const gsl_rng_type *rng_default(){ return gsl_rng_default; }
/* unsigned long int gsl_rng_default_seed; */

void mul_matrix(gsl_matrix* A, gsl_matrix* B, gsl_matrix* C){
    gsl_blas_dgemm (CblasNoTrans, CblasNoTrans, 1.0, A, B, 0.0, C);
}
