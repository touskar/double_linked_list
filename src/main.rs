use double_rinked_list::DoubleRinkedList;

fn main() {
    println!("DoubleRinkedList Demo");
    
    let mut list = DoubleRinkedList::new();
    let _ = list.push(1);
    let _ = list.push(2);
    let _ = list.push(3);
    
    println!("Basic usage:");
    list.log(None);
    println!("Length: {}", list.len());
    
    let doubled = list.map(|x| x * 2);
    println!("Doubled: {:?}", doubled.to_vec());
    
    let mut pooled = DoubleRinkedList::with_capacity(5);
    for i in 0..10 {
        let _ = pooled.push(i);
    }
    
    pooled.log(Some(" -> "));
    if let Some((available, total)) = pooled.pool_stats() {
        println!("Pool: {}/{} available", available, total);
    }
    
    println!("\nRun 'cargo bench' to see performance comparisons!");
}