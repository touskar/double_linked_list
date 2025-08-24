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
    println!("Testing insert_many_at_index...");
    debug_push_front();

    let mut list = DoubleRinkedList::<i32>::with_capacity(100);
    
    // Start with some initial values
    let _ = list.push(1);
    let _ = list.push(2);
    let _ = list.push(5);
    
    println!("Initial list:");
    list.log(Some(" -> "));
    
    // Insert multiple values at index 2 (after the 5)
    let values = vec![3, 4];
    match list.insert_many_at_index(2, values) {
        Ok(count) => println!("Successfully inserted {} items", count),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("After inserting [3, 4] at index 2:");
    list.log(Some(" -> "));
    
    // Test with iterator
    let more_values = 10..13; // [10, 11, 12]
    match list.insert_many_at_index(0, more_values) {
        Ok(count) => println!("Successfully inserted {} items at beginning", count),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("After inserting [10, 11, 12] at index 0:");
    list.log(Some(" -> "));

    println!("Done!");
}