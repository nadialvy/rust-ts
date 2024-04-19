fn main() {
    let foo: Vec<_> = vec![1, 2, 3] //bikin 3 belt
        .iter() //hire pegawai untuk meletakkan setiap belt di conveyor
        .map(|x| x + 1) //beli mesin untuk mengubah belt satu persatu
        .collect(); //kumpulkan semua belt menjadi 1 container

    // kenapa butuh collect. kalau berhenti di map doang, itu bentuknya masih iter dan bukan vector dan bukan list
    // bentuknya masih iterator
    // collect digunakan untuk mengubah dari iterator menjadi vector

    // intinya = mengembalikan bentuk dari iterator menjadi sesuatu (di case ini vector)

    println!("{:?}", foo);
}
