pub mod sort;

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
        let mut vec: Vec<i32> = vec![0; 500];

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
        let mut vec: Vec<i32> = vec![0; 10];

        for i in &mut vec {
            *i = rng.gen::<i32>();
        }
        
        sort::quick_sort(&mut vec);

        for i in 0..vec.len()-1{
            assert!(&vec[i] <= &vec[i+1]);
        }
    }
}
