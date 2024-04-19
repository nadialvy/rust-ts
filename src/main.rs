fn main() {
    let data = vec![1, 2, 3];
    let mut foo = data //live long enough dan bisa di refer selamanya
        .iter() //iter itu merefer pada vector, maka dari itu kita harus meng-assign-kan vector di variable lain
        .map(|x| x + 1);

    // APA YANG TERJADI DI BALIK COLLECT
    let mut new_vector = vec![];

    // Some dipakai biar kalau null dia ga error kalau next nya undefined
    while let Some(x) = foo.next() {
        new_vector.push(x);
    }

    println!("{:?}", new_vector);
}
