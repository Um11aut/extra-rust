mod modules;

use modules::list;

fn main() {
    let mut list = list::List::new(1);
    list.add(2);
    list.add(3);
    list.add(4);
    list.add(5);

    list.print();

    list.dublicate_values();
    list.print();
}
