use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn quicksort(arr: *mut c_int, len: usize) {
    if arr.is_null() || len == 0 {
        return;
    }

    let arr_slice = unsafe { std::slice::from_raw_parts_mut(arr, len) };

    quicksort_helper(arr_slice);
}

fn quicksort_helper(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = partition(arr);
    quicksort_helper(&mut arr[..pivot]);
    quicksort_helper(&mut arr[pivot + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr.len() - 1;
    let mut i = 0;

    for j in 0..pivot {
        if arr[j] < arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot);
    i
}
