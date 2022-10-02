pub mod sort;
pub mod fft;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn merge_sort_test() {
        use rand::Rng;
        use crate::sort;

        let mut rng = rand::thread_rng();
        let mut vec: Vec<i32> = vec![0; 10000];

        for i in &mut vec {
            *i = rng.gen::<i32>();
        }
        
        sort::merge_sort(&mut vec);

        for i in 0..vec.len()-1{
            assert!(&vec[i] <= &vec[i+1]);
        }
    }

    #[test]
    fn quick_sort_test() {
        use rand::Rng;
        use crate::sort;

        let mut rng = rand::thread_rng();
        let mut vec: Vec<i32> = vec![0; 10000];

        for i in &mut vec {
            *i = rng.gen::<i32>();
        }
        
        sort::quick_sort(&mut vec);

        for i in 0..vec.len()-1{
            assert!(&vec[i] <= &vec[i+1]);
        }
    }

    #[test]
    fn fft_test() {

        use crate::fft;

        let mut vec: Vec<fft::Complex> = Vec::new();
        vec.push(fft::Complex{real: 1.0, imag: 0.0,});
        vec.push(fft::Complex{real: 2.0, imag: 0.0,});
        vec.push(fft::Complex{real: 3.0, imag: 0.0,});
        vec.push(fft::Complex{real: 4.0, imag: 0.0,});
        vec.push(fft::Complex{real: 5.0, imag: 0.0,});
        

        let A = fft::fft(&vec);
        println!("{:?}", A);
    }

}
