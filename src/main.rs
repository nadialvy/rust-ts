use std::collections::HashMap;

fn main() {
    //  * digunakan untuk mengambil nilai dari reference, karena x ini masih reference bukan nilai integers
    let value: usize = vec![1, 2, 3].iter().filter(|x| *x % 2 == 0).count();
    println!("{:?}", value);

    // skip(2) artinya skip sampai nilai 2
    let value2: usize = vec![1, 2, 3].iter().skip(2).count();
    println!("{:?}", value2);

    let map = HashMap::from([("foo", 1), ("bar", 2), ("baz", 3)]);

    map.iter().for_each(|(k, v)| println!("{} : {}", k, v))
}
