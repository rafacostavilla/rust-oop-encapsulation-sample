use rust_oop_encapsulation_sample::AveragedCollection;

fn main() {
    let mut avg_list = AveragedCollection::new();

    avg_list.add(1);
    avg_list.add(2);
    avg_list.add(3);
    avg_list.add(4);

    // This should return 2.5 
    assert_eq!(2.5, avg_list.average());
    println!("{}", avg_list.average());
}
