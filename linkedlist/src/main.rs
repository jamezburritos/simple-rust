use linkedlist::List;

fn main() {
    let mut test_list: List<i32> = List::new();
    test_list.push(13);
    test_list.push(54);
    test_list.push(4);
    test_list.push(101);

    for item in test_list.iter() {
        println!("{item}");
    }
}
