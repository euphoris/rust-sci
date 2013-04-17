extern mod std;

#[cfg(test)]
mod test {
    use dist::*;
    use random::*;

    fn approx<T: NumCast, U: NumCast>(x: T, y: U) -> bool {
        let x1 = x.to_f64();
        let y1 = y.to_f64();
        y1-0.001 < x1 && x1 < y1+0.001
    }

    #[test]
    fn test_fdist() {
        let x = 2.0;
        let f = Fdist {df1: 1.0, df2: 3.0};
        assert!(approx(f.d(x), 0.09356362));
        assert!(approx(f.p(x), 0.7477845));
        assert!(approx(f.q(x), 1.0-f.p(x)));

        assert!(approx(f.pinv(f.p(x)), x));
        assert!(approx(f.qinv(f.q(x)), x));
    }


    #[test]
    fn test_random(){
        unsafe {
            let rng = alloc(borosh13());
            free(rng);
        }
    }
}
