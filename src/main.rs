use std::collections::HashMap;

fn main() {
    // collect into hash map
    let foo: HashMap<&str, usize> = vec!["this", "is", "a", "test"]
                                    .into_iter()
                                    .enumerate()  //kalau di ts .map((el, i)) kita bisa tahu index
                                                  //di rust harus di enumerate dulu biar bisa nge index pas ngemap
                                                  //enumerate = pair elemen dengan index nya => index, element
                                    //contoh dari destructuring map(|(idx, item)|)
                                    .map(|(idx, item)| (item, idx)) //reverse the order
                                    .collect();
}
