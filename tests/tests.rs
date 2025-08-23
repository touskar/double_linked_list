use double_linked_list::DoubleRinkedList;
use double_linked_list::double_linked_list::ListError;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_list() -> DoubleRinkedList<i32> {
        let mut list = DoubleRinkedList::new();
        for i in 1..=5 {
            list.push(i).unwrap();
        }
        list
    }

    #[test]
    fn test_construction_and_capacity() {
        let list: DoubleRinkedList<i32> = DoubleRinkedList::new();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
        assert_eq!(list.pool_stats(), None);

        let list_with_pool = DoubleRinkedList::<i32>::with_capacity(10);
        assert_eq!(list_with_pool.len(), 0);
        assert!(list_with_pool.is_empty());
        assert_eq!(list_with_pool.pool_stats(), Some((10, 10)));

        let mut list_capacity = DoubleRinkedList::<i32>::with_capacity(5);
        list_capacity.add_capacity(5);
        assert_eq!(list_capacity.pool_stats(), Some((10, 10)));

        let mut list_set = DoubleRinkedList::<i32>::with_capacity(5);
        assert!(list_set.set_capacity(10).is_ok());
        assert_eq!(list_set.pool_stats(), Some((10, 10)));

        let mut list_fail = DoubleRinkedList::new();
        list_fail.push(1).unwrap();
        list_fail.push(2).unwrap();
        assert!(list_fail.set_capacity(1).is_err());
    }

    #[test]
    fn test_basic_stack_operations() {
        let mut list: DoubleRinkedList<i32> = DoubleRinkedList::new();

        assert_eq!(list.push(1).unwrap(), 1);
        assert_eq!(list.len(), 1);
        assert!(!list.is_empty());

        assert_eq!(list.push(2).unwrap(), 2);
        assert_eq!(list.len(), 2);

        assert_eq!(list.push(3).unwrap(), 3);
        assert_eq!(list.len(), 3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.len(), 2);
        
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.len(), 1);
        
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
        
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_queue_operations() {
        let mut list: DoubleRinkedList<i32> = DoubleRinkedList::new();

        assert_eq!(list.push_front(1).unwrap(), 1);
        assert_eq!(list.push_front(2).unwrap(), 2);
        assert_eq!(list.push_front(3).unwrap(), 3);
        assert_eq!(list.to_vec(), vec![3, 2, 1]);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_element_access() {
        let list = create_test_list();

        assert_eq!(list.get_at_begin(), Some(1));
        assert_eq!(list.get_at_end(), Some(5));
        assert_eq!(list.get_at_index(0).unwrap(), 1);
        assert_eq!(list.get_at_index(2).unwrap(), 3);
        assert_eq!(list.get_at_index(4).unwrap(), 5);
        assert!(list.get_at_index(10).is_err());

        let empty: DoubleRinkedList<i32> = DoubleRinkedList::new();
        assert_eq!(empty.get_at_begin(), None);
        assert_eq!(empty.get_at_end(), None);
        assert!(empty.get_at_index(0).is_err());
    }

    #[test]
    fn test_insertion_operations() {
        let mut list: DoubleRinkedList<i32> = DoubleRinkedList::new();

        assert!(list.insert_at_begin(2).is_ok());
        assert!(list.insert_at_begin(1).is_ok());
        assert_eq!(list.to_vec(), vec![1, 2]);

        assert!(list.insert_at_end(3).is_ok());
        assert!(list.insert_at_end(4).is_ok());
        assert_eq!(list.to_vec(), vec![1, 2, 3, 4]);

        assert!(list.insert_at_index(2, 99).is_ok());
        assert_eq!(list.to_vec(), vec![1, 2, 99, 3, 4]);

        assert!(list.insert_at_index(0, 0).is_ok());
        assert_eq!(list.to_vec(), vec![0, 1, 2, 99, 3, 4]);

        assert!(list.insert_at_index(6, 100).is_ok());
        assert_eq!(list.to_vec(), vec![0, 1, 2, 99, 3, 4, 100]);

        assert!(list.insert_at_index(10, 999).is_err());
    }

    #[test]
    fn test_removal_operations() {
        let mut list = create_test_list();

        assert_eq!(list.remove_at_end().unwrap(), 5);
        assert_eq!(list.to_vec(), vec![1, 2, 3, 4]);

        assert_eq!(list.remove_at_begin().unwrap(), 1);
        assert_eq!(list.to_vec(), vec![2, 3, 4]);

        assert_eq!(list.remove_at_index(1).unwrap(), 3);
        assert_eq!(list.to_vec(), vec![2, 4]);

        assert_eq!(list.remove_at_index(0).unwrap(), 2);
        assert_eq!(list.to_vec(), vec![4]);

        assert_eq!(list.remove_at_index(0).unwrap(), 4);
        assert!(list.is_empty());

        assert!(list.remove_at_end().is_err());
        assert!(list.remove_at_begin().is_err());
        assert!(list.remove_at_index(0).is_err());
    }

    #[test]
    fn test_search_operations() {
        let list = create_test_list();

        assert!(list.contains(&3));
        assert!(!list.contains(&10));
        assert!(list.includes(&4));
        assert!(!list.includes(&0));

        assert_eq!(list.index_of(&1), Some(0));
        assert_eq!(list.index_of(&3), Some(2));
        assert_eq!(list.index_of(&5), Some(4));
        assert_eq!(list.index_of(&10), None);

        assert_eq!(list.find_index(|x| *x > 3), Some(3));
        assert_eq!(list.find_index(|x| *x > 10), None);
        assert_eq!(list.find_index(|x| *x == 2), Some(1));

        let empty: DoubleRinkedList<i32> = DoubleRinkedList::new();
        assert!(!empty.contains(&1));
        assert_eq!(empty.index_of(&1), None);
        assert_eq!(empty.find_index(|x| *x == 1), None);
    }

    #[test]
    fn test_functional_programming() {
        let list = create_test_list();

        let doubled = list.map(|x| x * 2);
        assert_eq!(doubled.to_vec(), vec![2, 4, 6, 8, 10]);

        let evens = list.filter(|x| x % 2 == 0);
        assert_eq!(evens.to_vec(), vec![2, 4]);

        let odds = list.filter(|x| x % 2 == 1);
        assert_eq!(odds.to_vec(), vec![1, 3, 5]);

        let sum = list.reduce(|acc, x| acc + x, 0);
        assert_eq!(sum, 15);

        let product = list.reduce(|acc, x| acc * x, 1);
        assert_eq!(product, 120);

        assert!(list.every(|x| *x > 0));
        assert!(!list.every(|x| *x > 3));

        assert!(list.any(|x| *x > 3));
        assert!(!list.any(|x| *x > 10));
        assert!(list.some(|x| *x == 3));
        assert!(!list.some(|x| *x == 10));

        let empty: DoubleRinkedList<i32> = DoubleRinkedList::new();
        assert_eq!(empty.map(|x| x * 2).len(), 0);
        assert_eq!(empty.filter(|x| *x > 0).len(), 0);
        assert_eq!(empty.reduce(|acc, x| acc + x, 42), 42);
        assert!(empty.every(|x| *x > 0));
        assert!(!empty.any(|x| *x > 0));
    }

    #[test]
    fn test_cursor_positioning() {
        let mut list = create_test_list();

        assert!(list.move_cursor_at_begin().is_ok());
        assert!(list.move_cursor_at_end().is_ok());
        assert!(list.move_cursor_at_index(0).is_ok());
        assert!(list.move_cursor_at_index(2).is_ok());
        assert!(list.move_cursor_at_index(4).is_ok());
        assert!(list.move_cursor_at_index(5).is_ok());
        assert!(list.move_cursor_at_index(10).is_err());

        list.move_cursor_at_begin().unwrap();
        assert!(list.move_cursor_to_next().is_ok());
        assert!(list.move_cursor_to_next().is_ok());
        assert!(list.move_cursor_to_previous().is_ok());
        assert!(list.move_cursor_to_previous().is_ok());

        let mut empty: DoubleRinkedList<i32> = DoubleRinkedList::new();
        assert!(empty.move_cursor_at_begin().is_ok());
        assert!(empty.move_cursor_at_end().is_ok());
        assert!(empty.move_cursor_at_index(0).is_err());
        assert!(empty.move_cursor_to_next().is_err());
        assert!(empty.move_cursor_to_previous().is_err());
    }

    #[test]
    fn test_cursor_operations() {
        let mut list: DoubleRinkedList<i32> = DoubleRinkedList::new();

        assert!(list.insert_after_cursor(1).is_ok());
        assert_eq!(list.to_vec(), vec![1]);

        list.move_cursor_at_begin().unwrap();
        assert!(list.insert_after_cursor(2).is_ok());
        assert_eq!(list.to_vec(), vec![1, 2]);

        list.move_cursor_at_end().unwrap();
        assert!(list.insert_before_cursor(3).is_ok());
        assert_eq!(list.to_vec(), vec![1, 2, 3]);

        list.move_cursor_at_begin().unwrap();
        assert_eq!(list.remove_after_cursor().unwrap(), 2);
        assert_eq!(list.to_vec(), vec![1, 3]);

        list.move_cursor_at_end().unwrap();
        assert_eq!(list.remove_before_cursor().unwrap(), 1);
        assert_eq!(list.to_vec(), vec![3]);

        list.remove_after_cursor().ok();
        list.clear();

        assert!(list.remove_after_cursor().is_err());
        assert!(list.remove_before_cursor().is_err());
    }

    #[test]
    fn test_cursor_management() {
        let mut list = create_test_list();

        assert!(list.validate_cursor());

        list.move_cursor_at_index(2).unwrap();
        let cursor = list.get_cursor();
        
        list.move_cursor_at_begin().unwrap();
        assert!(list.set_cursor(cursor).is_ok());

        list.reset_cursor();
        assert!(list.validate_cursor());

        let mut empty: DoubleRinkedList<i32> = DoubleRinkedList::new();
        assert!(empty.validate_cursor());
        empty.reset_cursor();
        assert!(empty.validate_cursor());
    }

    #[test]
    fn test_display_methods() {
        let list = create_test_list();

        list.display(None);
        list.display(Some(", "));
        list.display(Some(" -> "));

        list.log(None);
        list.log(Some(", "));
        list.log(Some(" -> "));

        let empty: DoubleRinkedList<i32> = DoubleRinkedList::new();
        empty.display(None);
        empty.log(None);
    }

    #[test]
    fn test_utility_methods() {
        let list = create_test_list();

        assert_eq!(list.to_vec(), vec![1, 2, 3, 4, 5]);

        let mut list_to_clear = list.clone();
        list_to_clear.clear();
        assert!(list_to_clear.is_empty());
        assert_eq!(list_to_clear.len(), 0);

        let mut list_to_reverse = list.clone();
        list_to_reverse.reverse();
        assert_eq!(list_to_reverse.to_vec(), vec![5, 4, 3, 2, 1]);

        let mut empty: DoubleRinkedList<i32> = DoubleRinkedList::new();
        assert_eq!(empty.to_vec(), Vec::<i32>::new());
        empty.clear();
        empty.reverse();
        assert!(empty.is_empty());
    }

    #[test]
    fn test_splice_operation() {
        let mut list = create_test_list();

        let removed = list.splice(1, 2, vec![10, 20, 30]).unwrap();
        assert_eq!(removed, vec![2, 3]);
        assert_eq!(list.to_vec(), vec![1, 10, 20, 30, 4, 5]);

        let removed2 = list.splice(2, 2, vec![]).unwrap();
        assert_eq!(removed2, vec![20, 30]);
        assert_eq!(list.to_vec(), vec![1, 10, 4, 5]);

        let removed3 = list.splice(2, 0, vec![100]).unwrap();
        assert_eq!(removed3, Vec::<i32>::new());
        assert_eq!(list.to_vec(), vec![1, 10, 100, 4, 5]);

        assert!(list.splice(10, 1, vec![]).is_err());
    }

    #[test]
    fn test_iterator_implementation() {
        let list = create_test_list();

        let collected: Vec<i32> = list.clone().into_iter().collect();
        assert_eq!(collected, vec![1, 2, 3, 4, 5]);

        let collected_ref: Vec<i32> = (&list).into_iter().collect();
        assert_eq!(collected_ref, vec![1, 2, 3, 4, 5]);

        let empty: DoubleRinkedList<i32> = DoubleRinkedList::new();
        let empty_collected: Vec<i32> = empty.into_iter().collect();
        assert_eq!(empty_collected, Vec::<i32>::new());

        let sum: i32 = create_test_list().into_iter().sum();
        assert_eq!(sum, 15);

        let filtered: Vec<i32> = create_test_list().into_iter().filter(|&x| x % 2 == 0).collect();
        assert_eq!(filtered, vec![2, 4]);
    }

    #[test]
    fn test_from_iterator() {
        let vec = vec![1, 2, 3, 4, 5];
        let list: DoubleRinkedList<i32> = vec.into_iter().collect();
        assert_eq!(list.to_vec(), vec![1, 2, 3, 4, 5]);

        let range_list: DoubleRinkedList<i32> = (1..=5).collect();
        assert_eq!(range_list.to_vec(), vec![1, 2, 3, 4, 5]);

        let empty_vec: Vec<i32> = vec![];
        let empty_list: DoubleRinkedList<i32> = empty_vec.into_iter().collect();
        assert!(empty_list.is_empty());
    }

    #[test]
    fn test_default_trait() {
        let list: DoubleRinkedList<i32> = DoubleRinkedList::default();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_clone_trait() {
        let list = create_test_list();
        let cloned = list.clone();
        
        assert_eq!(list.to_vec(), cloned.to_vec());
        assert_eq!(list.len(), cloned.len());
    }

    #[test]
    fn test_memory_pool_functionality() {
        let mut pooled_list = DoubleRinkedList::<i32>::with_capacity(5);
        
        assert_eq!(pooled_list.pool_stats(), Some((5, 5)));
        
        for i in 1..=3 {
            pooled_list.push(i).unwrap();
        }
        assert_eq!(pooled_list.pool_stats(), Some((2, 5)));
        
        pooled_list.pop();
        pooled_list.pop();
        pooled_list.clear();
        assert!(pooled_list.pool_stats().is_some());
    }

    #[test]
    fn test_thread_safety_ready() {
        let list = Arc::new(Mutex::new(DoubleRinkedList::<i32>::new()));
        
        {
            let mut guard = list.lock().unwrap();
            guard.push(1).unwrap();
            guard.push(2).unwrap();
            assert_eq!(guard.len(), 2);
        }
        
        let guard = list.lock().unwrap();
        assert_eq!(guard.len(), 2);
    }

    #[test]
    fn test_error_handling() {
        let mut list: DoubleRinkedList<i32> = DoubleRinkedList::new();

        match list.get_at_index(0) {
            Err(ListError::IndexOutOfBounds { index, length }) => {
                assert_eq!(index, 0);
                assert_eq!(length, 0);
            }
            _ => panic!("Expected IndexOutOfBounds error"),
        }

        match list.remove_at_end() {
            Err(ListError::EmptyList) => {},
            _ => panic!("Expected EmptyList error"),
        }

        list.push(1).unwrap();
        list.push(2).unwrap();
        match list.set_capacity(1) {
            Err(ListError::InsufficientCapacity { requested, current }) => {
                assert_eq!(requested, 1);
                assert_eq!(current, 2);
            }
            _ => panic!("Expected InsufficientCapacity error"),
        }

        let error = ListError::IndexOutOfBounds { index: 5, length: 3 };
        let error_string = format!("{}", error);
        assert!(error_string.contains("Index 5 out of bounds"));
        assert!(error_string.contains("length: 3"));
    }

    #[test]
    fn test_edge_cases() {
        let mut list: DoubleRinkedList<i32> = DoubleRinkedList::new();

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.get_at_begin(), None);
        assert_eq!(list.get_at_end(), None);
        assert!(list.contains(&1) == false);
        assert_eq!(list.index_of(&1), None);
        assert_eq!(list.to_vec(), vec![]);

        list.push(42).unwrap();
        assert_eq!(list.len(), 1);
        assert!(!list.is_empty());
        assert_eq!(list.get_at_begin(), Some(42));
        assert_eq!(list.get_at_end(), Some(42));
        assert_eq!(list.get_at_index(0).unwrap(), 42);
        assert!(list.contains(&42));
        assert_eq!(list.index_of(&42), Some(0));
        assert_eq!(list.to_vec(), vec![42]);

        assert_eq!(list.pop(), Some(42));
        assert!(list.is_empty());

        list.push(-1).unwrap();
        list.push(-5).unwrap();
        assert_eq!(list.to_vec(), vec![-1, -5]);
        assert!(list.contains(&-1));
        assert!(list.contains(&-5));
    }

    #[test]
    fn test_large_list_operations() {
        let mut list: DoubleRinkedList<i32> = DoubleRinkedList::new();
        
        let size: usize = 1000;
        for i in 0..size as i32 {
            list.push(i).unwrap();
        }
        
        assert_eq!(list.len(), size);
        assert_eq!(list.get_at_begin(), Some(0));
        assert_eq!(list.get_at_end(), Some((size - 1) as i32));
        
        let sum: i32 = list.clone().into_iter().sum();
        let expected_sum: i32 = (0..size as i32).sum();
        assert_eq!(sum, expected_sum);
        
        assert!(list.contains(&500));
        assert_eq!(list.index_of(&500), Some(500));
        assert!(!list.contains(&2000));
        
        for _ in 0..size {
            list.pop();
        }
        assert!(list.is_empty());
    }

    #[test]
    fn test_debug_functionality() {
        let mut list = DoubleRinkedList::new();
        
        assert!(!list.is_debug_enabled());
        
        list.enable_debug();
        assert!(list.is_debug_enabled());
        
        list.push(1).unwrap();
        list.push(2).unwrap();
        list.push(3).unwrap();
        
        list.disable_debug();
        assert!(!list.is_debug_enabled());
        
        list.push(4).unwrap();
        assert_eq!(list.len(), 4);
    }

    #[test]
    fn test_simple_push_performance() {
        println!("Simple Push Performance Test");
        
        let mut list: DoubleRinkedList<i32> = DoubleRinkedList::new();
        
        let start = Instant::now();
        for i in 0..10 {
            match list.push(i) {
                Ok(_) => {},
                Err(e) => {
                    println!("Push failed at {}: {}", i, e);
                    break;
                }
            }
        }
        let duration = start.elapsed();
        println!("10 pushes took: {:?}", duration);
        println!("Final length: {}", list.len());
        
        let vec = list.to_vec();
        println!("Vector: {:?}", vec);
        
        let mut list100: DoubleRinkedList<i32> = DoubleRinkedList::new();
        let start100 = Instant::now();
        
        for i in 0..100 {
            match list100.push(i) {
                Ok(_) => {},
                Err(e) => {
                    println!("Push failed at {}: {}", i, e);
                    break;
                }
            }
        }
        let duration100 = start100.elapsed();
        println!("100 pushes took: {:?}", duration100);
        println!("Final length: {}", list100.len());
        
        println!("Testing to_vec on 100 elements...");
        let vec_start = Instant::now();
        let vec100 = list100.to_vec();
        let vec_duration = Instant::now() - vec_start;
        println!("to_vec took: {:?}, collected {} items", vec_duration, vec100.len());
        
        assert_eq!(vec100.len(), 100);
        assert_eq!(vec100[0], 0);
        assert_eq!(vec100[99], 99);
    }

    #[test]
    fn test_debug_enabled_performance() {
        println!("Debug Performance Test");
        
        let mut list: DoubleRinkedList<i32> = DoubleRinkedList::new();
        list.enable_debug();
        
        println!("Testing with debug enabled (first 3 elements):");
        let start = Instant::now();
        for i in 0..3 {
            list.push(i).unwrap();
        }
        let duration = start.elapsed();
        println!("3 pushes with debug took: {:?}", duration);
        
        list.disable_debug();
        println!("Debug disabled, testing 7 more elements:");
        let start2 = Instant::now();
        for i in 3..10 {
            list.push(i).unwrap();
        }
        let duration2 = start2.elapsed();
        println!("7 pushes without debug took: {:?}", duration2);
        
        let vec = list.to_vec();
        println!("Final vector: {:?}", vec);
    }

    #[test]
    fn test_production_ready_basic() {
        let mut list = DoubleRinkedList::new();
        
        assert!(list.push(1).is_ok());
        assert!(list.push(2).is_ok());
        assert_eq!(list.len(), 2);
        assert_eq!(list.to_vec(), vec![1, 2]);
        
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert!(list.is_empty());
    }

    #[test]
    fn test_error_handling_production() {
        let mut list: DoubleRinkedList<i32> = DoubleRinkedList::new();

        match list.get_at_index(0) {
            Err(ListError::IndexOutOfBounds { index: 0, length: 0 }) => {},
            _ => panic!("Expected specific error"),
        }

        match list.remove_at_end() {
            Err(ListError::EmptyList) => {},
            _ => panic!("Expected EmptyList"),
        }

        list.push(1).unwrap();
        list.push(2).unwrap();
        match list.set_capacity(1) {
            Err(ListError::InsufficientCapacity { requested: 1, current: 2 }) => {},
            _ => panic!("Expected capacity error"),
        }
    }

    #[test]
    fn test_capacity_management() {
        let mut list = DoubleRinkedList::with_capacity(10);
        assert_eq!(list.pool_stats(), Some((10, 10)));

        for i in 0..5 {
            list.push(i).unwrap();
        }
        assert_eq!(list.pool_stats(), Some((5, 10)));

        list.add_capacity(5);
        assert_eq!(list.pool_stats(), Some((10, 15)));

        list.set_capacity(20).unwrap();
        assert_eq!(list.pool_stats(), Some((15, 20)));

        assert!(list.set_capacity(1).is_err());
    }

    #[test]
    fn test_cursor_sequential_navigation() {
        let mut list = DoubleRinkedList::new();
        for i in 0..5 {
            list.push(i).unwrap();
        }

        list.move_cursor_at_begin().unwrap();
        
        let mut values = Vec::new();
        for _ in 0..3 {
            if let Ok(()) = list.move_cursor_to_next() {
                values.push("moved");
            }
        }
        
        assert_eq!(values.len(), 3);
        assert!(list.validate_cursor());
    }

    #[test]
    fn test_comprehensive_functional_operations() {
        let list: DoubleRinkedList<i32> = (1..=10).collect();

        let doubled = list.map(|x| x * 2);
        assert_eq!(doubled.len(), 10);
        assert_eq!(doubled.get_at_index(0).unwrap(), 2);

        let evens = list.filter(|x| x % 2 == 0);
        assert_eq!(evens.len(), 5);
        assert_eq!(evens.to_vec(), vec![2, 4, 6, 8, 10]);

        let sum = list.reduce(|acc, x| acc + x, 0);
        assert_eq!(sum, 55);

        assert!(list.every(|x| *x > 0));
        assert!(!list.every(|x| *x > 5));
        assert!(list.any(|x| *x > 5));
        assert!(!list.any(|x| *x > 10));
    }

    #[test] 
    fn test_splice_comprehensive() {
        let mut list: DoubleRinkedList<i32> = (0..10).collect();

        let removed = list.splice(3, 4, vec![100, 200]).unwrap();
        assert_eq!(removed, vec![3, 4, 5, 6]);
        assert_eq!(list.len(), 8);

        let first_part: Vec<i32> = list.clone().into_iter().take(3).collect();
        assert_eq!(first_part, vec![0, 1, 2]);

        let spliced_part: Vec<i32> = list.clone().into_iter().skip(3).take(2).collect();
        assert_eq!(spliced_part, vec![100, 200]);

        let last_part: Vec<i32> = list.clone().into_iter().skip(5).collect();
        assert_eq!(last_part, vec![7, 8, 9]);
    }

    #[test]
    fn test_large_scale_operations() {
        let mut list = DoubleRinkedList::with_capacity(1000);
        
        for i in 0..1000 {
            list.push(i).unwrap();
        }
        assert_eq!(list.len(), 1000);
        
        let middle_values: Vec<i32> = list.clone().into_iter().skip(400).take(200).collect();
        assert_eq!(middle_values.len(), 200);
        assert_eq!(middle_values[0], 400);
        assert_eq!(middle_values[199], 599);
        
        for _ in 0..500 {
            list.pop();
        }
        assert_eq!(list.len(), 500);
        
        list.clear();
        assert!(list.is_empty());
        assert!(list.pool_stats().is_some());
    }
}