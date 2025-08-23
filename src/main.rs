use double_linked_list::DoubleRinkedList;

fn debug_push_front() {
    let mut list = DoubleRinkedList::<i32>::new();
    
    for i in 0..5 {
        println!("Iteration {}", i);
        match list.push_front(i) {
            Ok(len) => {
                println!(" push_front({}) réussi, longueur: {}", i, len);
                let contents = list.to_vec();
                println!("  Contenu: {:?}", contents);
            }
            Err(e) => {
                println!(" push_front({}) échoué: {}", i, e);
                break;
            }
        }
    }
}

fn main() {
    println!("Testing debug push_front...");
    debug_push_front();
    println!("Done!");
}