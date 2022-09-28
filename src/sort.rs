use std::fmt::Debug;

pub fn merge_sort<T: Ord + Copy>(v: &mut Vec<T>) {
    let sorted = merge_sort_helper(v.as_slice());
    for i in 0..v.len(){
        v[i] = sorted[i];
    }
}

pub fn quick_sort<T: Ord + Copy + Debug>(v: &mut Vec<T>) {
    let len = v.len();
    quick_sort_helper(v.as_mut_slice(), 0, len - 1);
}

fn merge_sort_helper<T: Ord + Copy>(v: &[T]) -> Vec<T>{

    if v.len() <= 1 {
        return vec![v[0]];
    }

    let mut ret: Vec<T> = Vec::new();

    let vl = merge_sort_helper(&v[..v.len() / 2]);
    let vr = merge_sort_helper(&v[v.len() / 2..]);

    let mut pl: usize = 0;
    let mut pr: usize = 0;

    while pl < vl.len() && pr < vr.len(){

        if &vl[pl] > &vr[pr]{
            ret.push(vr[pr]);
            pr += 1;
        }
        else {
            ret.push(vl[pl]);
            pl += 1;
        }
    }

    while pl < vl.len() {
        ret.push(vl[pl]);
        pl += 1;
    }

    while pr < vr.len() {
        ret.push(vr[pr]);
        pr += 1;
    }

    ret
}

fn quick_sort_helper<T: Ord + Copy + Debug>(v: &mut [T], low: usize, high: usize){
    
    if low < high {

        println!("{:?}",&v[low..=high]);

        let pivot = quick_sort_pivot(v, low, high);
        println!("{}", pivot);

        quick_sort_helper(v, low, pivot);
        quick_sort_helper(v, pivot+1, high);

        println!("{:?}",&v[low..=high]);
    }
        
}

fn quick_sort_pivot<T: Ord + Copy>(v: &mut [T], low: usize, high: usize) -> usize {

    let pivot = v[(high + low) / 2];
    let p_ref = &pivot;

    let mut i = low;
    let mut j = high;

    loop {
        while &v[i] < p_ref {
            i += 1;
        }

        while &v[j] > p_ref {
            j -= 1;
        }

        if i >= j {
            break;
        }

        let tmp = v[i];
        v[i] = v[j];
        v[j] = tmp;

        i += 1;
        j -= 1;

    }

    j

}