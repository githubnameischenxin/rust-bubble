fn simple_bubble_sort(list: &mut [i32; 5]) {
    let length= list.len();
    for i in 0..length {
        for j in 0..length - i - 1 {
            if list[j + 1] > list[j] {
                let tmp = list[j];
                list[j] = list[j + 1];
                list[j + 1] = tmp;
            }
        }
    }
}

fn generics_bubble_sort<T: PartialOrd>(list: &mut [T]) {
    let length= list.len();
    for i in 0..length {
        for j in 0..length - i - 1 {
            if list[j + 1] > list[j] {
                list.swap(j + 1, j);
            }
        }
    }
}

fn main() {
    let mut list = [3, 4, 2, 1, 5];
    println!("原始数组1: {:?}", list);
    simple_bubble_sort(&mut list);
    println!("排序后1: {:?}", list);

    let mut list_str = ["c", "d", "b", "a", "e"];
    println!("原始数组2: {:?}", list_str);
    generics_bubble_sort(&mut list_str);
    println!("排序后2: {:?}", list_str);

    let mut list_str = vec!["c", "d", "b", "a", "e"];
    println!("原始数组3: {:?}", list_str);
    generics_bubble_sort(&mut list_str);
    println!("排序后3: {:?}", list_str);

    let mut list = [3, 4, 2, 1, 5];
    println!("原始数组4: {:?}", list);
    generics_bubble_sort(&mut list);
    println!("排序后4: {:?}", list);
}