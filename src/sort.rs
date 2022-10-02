pub fn merge_sort<T: Ord + Copy>(v: &mut Vec<T>) {
    let sorted = merge_sort_helper(v.as_slice());
    for i in 0..v.len(){
        v[i] = sorted[i];
    }
}

pub fn quick_sort<T: Ord>(v: &mut Vec<T>) {
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

fn quick_sort_helper<T: Ord>(v: &mut [T], low: usize, high: usize){
    
    if low < high {

        let pivot = quick_sort_pivot(v, low, high);

        quick_sort_helper(v, low, pivot);
        quick_sort_helper(v, pivot+1, high);
    }
        
}

fn quick_sort_pivot<T: Ord>(v: &mut [T], low: usize, high: usize) -> usize {

    let mut piv_ind = (high + low) / 2;

    let mut i = low;
    let mut j = high;

    loop {
        while &v[i] < &v[piv_ind] {
            i += 1;
        }

        while &v[j] > &v[piv_ind] {
            j -= 1;
        }

        if i >= j {
            break;
        }
        if piv_ind == i { piv_ind = j}
        else if piv_ind == j {piv_ind = i}
        v.swap(i, j);

        i += 1;
        j -= 1;

    }

    j

}