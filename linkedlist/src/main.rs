use linkedlist::List;

fn main() {
    let mut test_list = List::new();
    test_list.push(13);
    test_list.push(54);
    test_list.push(4);
    test_list.push(101);

    while let Some(value) = test_list.pop() {
        println!("{value}");
    }
}
