fn main() {}

pub fn run() {
    let mut array_to_sort = [5, 18, 3, 27, 12, 9, 6, 21, 15, 8, 34, 1, 29, 7, 11];
    let mut i = 0;
    let mut v = 0;
    while v + 1 < array_to_sort.len() {
        while i + 1 < array_to_sort.len() {
            if array_to_sort[i] > array_to_sort[i + 1] {
                let j = array_to_sort[i + 1];
                array_to_sort[i + 1] = array_to_sort[i];
                array_to_sort[i] = j;
            }
            i += 1;
        }
        i = 0;
        v += 1;
    }
    for x in array_to_sort {
        println!("{}", x);
    }
}
