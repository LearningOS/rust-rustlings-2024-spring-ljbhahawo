/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


use core::slice;

fn sort<T: std::cmp::PartialOrd>(array: &mut [T]){
	//TODO
    if array.len() < 2{
        return;
    }
    let mut sort_len = 0;
    
    while sort_len < array.len()-1{
        sort_len+=1;
        let mut r = sort_len;


        for _ in 0..r{
            if array[r] < array[r-1]{
                array.swap(r-1, r);
                r-=1;

            }else {
                break;
            }
        }
        
    }

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}