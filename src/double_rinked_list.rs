use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fmt::{Debug, Display};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListError {
    IndexOutOfBounds { index: usize, length: usize },
    InsufficientCapacity { requested: usize, current: usize },
    EmptyList,
    InvalidCursor,
}

impl std::fmt::Display for ListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListError::IndexOutOfBounds { index, length } => {
                write!(f, "Index {} out of bounds (length: {})", index, length)
            }
            ListError::InsufficientCapacity { requested, current } => {
                write!(f, "Cannot set capacity {} below current length {}", requested, current)
            }
            ListError::EmptyList => write!(f, "List is empty"),
            ListError::InvalidCursor => write!(f, "Invalid cursor state"),
        }
    }
}

impl std::error::Error for ListError {}

pub type Result<T> = std::result::Result<T, ListError>;

macro_rules! debug_print {
    ($self:expr, $($args:tt)*) => {
        if $self.debug_enabled {
            eprintln!("[DEBUG] {}", format!($($args)*));
        }
    };
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: Option<T>,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub previous: Option<Weak<RefCell<Node<T>>>>,
    pub is_root: bool,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value: Some(value),
            next: None,
            previous: Option::None,
            is_root: false,
        }
    }

    fn new_sentinel() -> Self {
        Node {
            value: Option::None,
            next: None,
            previous: None,
            is_root: true,
        }
    }

    fn reset(&mut self) {
        self.value = None;
        self.next = None;
        self.previous = None;
        self.is_root = false;
    }
}

#[derive(Debug, Clone)]
pub struct Cursor<T> {
    pub after: Option<Weak<RefCell<Node<T>>>>,
    pub before: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Cursor<T> {
    pub fn new() -> Self {
        Cursor {
            after: Option::None,
            before: Option::None,
        }
    }
}

impl<T> Default for Cursor<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
struct NodePool<T> {
    pool: VecDeque<Rc<RefCell<Node<T>>>>,
    capacity: usize,
    total_allocated: usize,
}

impl<T> NodePool<T> {
    fn new(capacity: usize) -> Self {
        let mut pool = VecDeque::with_capacity(capacity);
        for _ in 0..capacity {
            pool.push_back(Rc::new(RefCell::new(Node::new_sentinel())));
        }
        NodePool {
            pool,
            capacity,
            total_allocated: capacity,
        }
    }

    fn get_node(&mut self, value: T) -> Rc<RefCell<Node<T>>> {
        if let Some(node_ptr) = self.pool.pop_front() {
            {
                let mut node = node_ptr.borrow_mut();
                node.reset();
                node.value = Some(value);
            }
            node_ptr
        } else {
            self.total_allocated += 1;
            Rc::new(RefCell::new(Node::new(value)))
        }
    }

    fn return_node(&mut self, node_ptr: Rc<RefCell<Node<T>>>) {
        if self.pool.len() < self.capacity && Rc::strong_count(&node_ptr) == 1 {
            {
                let mut node = node_ptr.borrow_mut();
                node.reset();
            }
            self.pool.push_back(node_ptr);
        }
    }

    fn stats(&self) -> (usize, usize) {
        (self.pool.len(), self.total_allocated)
    }

    fn ensure_capacity(&mut self, new_capacity: usize) -> Result<()> {
        if new_capacity > self.capacity {
            let nodes_to_add = new_capacity - self.capacity;
            self.capacity = new_capacity;
            for _ in 0..nodes_to_add {
                self.pool.push_back(Rc::new(RefCell::new(Node::new_sentinel())));
                self.total_allocated += 1;
            }
        } else {
            self.capacity = new_capacity;
            while self.pool.len() > new_capacity {
                self.pool.pop_back();
            }
        }
        Ok(())
    }

    fn add_nodes(&mut self, count: usize) {
        self.capacity += count;
        for _ in 0..count {
            self.pool.push_back(Rc::new(RefCell::new(Node::new_sentinel())));
            self.total_allocated += 1;
        }
    }
}

#[derive(Debug, Clone)]
pub struct DoubleRinkedList<T>
where
    T: Clone + Debug,
{
    pub root: Rc<RefCell<Node<T>>>,
    pub cursor: Cursor<T>,
    pub length: usize,
    pool: Option<NodePool<T>>,
    debug_enabled: bool,
}

impl<T> DoubleRinkedList<T>
where
    T: Clone + Debug,
{
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(Node::new_sentinel()));
        
        {
            let mut root_ref = root.borrow_mut();
            root_ref.next = Some(root.clone());
            root_ref.previous = Some(Rc::downgrade(&root));
        }
        
        let root_weak = Rc::downgrade(&root);

        DoubleRinkedList {
            root: root.clone(),
            cursor: Cursor {
                before: Some(root_weak.clone()),
                after: Some(root_weak),
            },
            length: 0,
            pool: None,
            debug_enabled: false,
        }
    }

    pub fn with_capacity(pool_capacity: usize) -> Self {
        let root = Rc::new(RefCell::new(Node::new_sentinel()));
        
        {
            let mut root_ref = root.borrow_mut();
            root_ref.next = Some(root.clone());
            root_ref.previous = Some(Rc::downgrade(&root));
        }
        
        let root_weak = Rc::downgrade(&root);

        DoubleRinkedList {
            root: root.clone(),
            cursor: Cursor {
                before: Some(root_weak.clone()),
                after: Some(root_weak),
            },
            length: 0,
            pool: Some(NodePool::new(pool_capacity)),
            debug_enabled: false,
        }
    }

    pub fn pool_stats(&self) -> Option<(usize, usize)> {
        self.pool.as_ref().map(|pool| pool.stats())
    }

    pub fn enable_debug(&mut self) {
        self.debug_enabled = true;
        debug_print!(self, "Debug mode ENABLED");
    }

    pub fn disable_debug(&mut self) {
        debug_print!(self, "Debug mode DISABLED");
        self.debug_enabled = false;
    }

    pub fn is_debug_enabled(&self) -> bool {
        self.debug_enabled
    }

    pub fn add_capacity(&mut self, additional: usize) {
        if let Some(ref mut pool) = self.pool {
            pool.add_nodes(additional);
        } else {
            self.pool = Some(NodePool::new(additional));
        }
    }

    pub fn set_capacity(&mut self, new_capacity: usize) -> Result<()> {
        if let Some(ref mut pool) = self.pool {
            pool.ensure_capacity(new_capacity)?;
        } else {
            self.pool = Some(NodePool::new(new_capacity));
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn push(&mut self, value: T) -> Result<usize> {
        debug_print!(self, "push({:?}) called, current length: {}", value, self.length);
        let result = self.insert_at_end(value);
        match result {
            Ok(()) => {
                debug_print!(self, "push successful, new length: {}", self.length);
                Ok(self.length)
            }
            Err(e) => {
                debug_print!(self, "push failed: {}", e);
                Err(e)
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.remove_at_end().ok()
    }

    pub fn push_front(&mut self, value: T) -> Result<usize> {
        self.insert_at_begin(value)?;
        Ok(self.length)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.remove_at_begin().ok()
    }

    pub fn insert_at_begin(&mut self, value: T) -> Result<()> {
        self.move_cursor_at_begin()?;
        self.insert_after_cursor(value)
    }

    pub fn insert_at_end(&mut self, value: T) -> Result<()> {
        debug_print!(self, "insert_at_end({:?}) called, length: {}", value, self.length);
        debug_print!(self, "Moving cursor to end...");
        self.move_cursor_at_end()?;
        debug_print!(self, "Cursor moved, inserting after cursor...");
        let result = self.insert_after_cursor(value);
        debug_print!(self, "insert_at_end result: {:?}", result);
        result
    }

    pub fn remove_at_end(&mut self) -> Result<T> {
        if self.length == 0 {
            return Err(ListError::EmptyList);
        }

        self.move_cursor_at_end()?;
        self.remove_before_cursor()
    }

    pub fn remove_at_begin(&mut self) -> Result<T> {
        if self.length == 0 {
            return Err(ListError::EmptyList);
        }

        self.move_cursor_at_begin()?;
        self.remove_after_cursor()
    }

    pub fn insert_at_index(&mut self, index: usize, value: T) -> Result<()> {
        if index > self.length {
            return Err(ListError::IndexOutOfBounds {
                index,
                length: self.length
            });
        }

        self.move_cursor_at_index(index)?;
        self.insert_after_cursor(value)
    }

    pub fn remove_at_index(&mut self, index: usize) -> Result<T> {
        if index >= self.length {
            return Err(ListError::IndexOutOfBounds {
                index,
                length: self.length
            });
        }

        self.move_cursor_at_index(index)?;
        self.remove_after_cursor()
    }

    pub fn get_at_begin(&self) -> Option<T> {
        if self.length == 0 {
            return None;
        }

        let root_ref = self.root.borrow();
        if let Some(first_node) = &root_ref.next {
            first_node.borrow().value.clone()
        } else {
            None
        }
    }

    pub fn get_at_end(&self) -> Option<T> {
        if self.length == 0 {
            return None;
        }

        let root_ref = self.root.borrow();
        if let Some(last_weak) = &root_ref.previous {
            if let Some(last_node) = last_weak.upgrade() {
                last_node.borrow().value.clone()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_at_index(&self, index: usize) -> Result<T> {
        if index >= self.length || self.length == 0 {
            return Err(ListError::IndexOutOfBounds {
                index,
                length: self.length
            });
        }

        let mut current = {
            let root_ref = self.root.borrow();
            root_ref.next.as_ref()
                .ok_or(ListError::EmptyList)?
                .clone()
        };

        for _ in 0..index {
            let next = {
                let node_ref = current.borrow();
                if node_ref.is_root {
                    return Err(ListError::IndexOutOfBounds {
                        index,
                        length: self.length
                    });
                }
                node_ref.next.as_ref()
                    .ok_or(ListError::IndexOutOfBounds {
                        index,
                        length: self.length
                    })?
                    .clone()
            };
            current = next;
        }

        let node_ref = current.borrow();
        if node_ref.is_root {
            Err(ListError::IndexOutOfBounds {
                index,
                length: self.length
            })
        } else {
            node_ref.value.clone()
                .ok_or(ListError::IndexOutOfBounds {
                    index,
                    length: self.length
                })
        }
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.length);
        
        if self.length == 0 {
            return result;
        }

        let mut current = {
            let root_ref = self.root.borrow();
            root_ref.next.as_ref().cloned()
        };
        
        let mut count = 0;
        let max_iterations = self.length + 1;
        
        while let Some(node) = current {
            count += 1;
            
            if count > max_iterations {
                eprintln!("ERREUR: Boucle infinie détectée dans DoubleRinkedList!");
                break;
            }
            
            let node_ref = node.borrow();
            
            if node_ref.is_root {
                break;
            }
            
            if let Some(value) = &node_ref.value {
                result.push(value.clone());
            }
            
            current = node_ref.next.as_ref().cloned();
        }

        result
    }

    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.index_of(value).is_some()
    }

    pub fn includes(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.contains(value)
    }

    pub fn index_of(&self, value: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        if self.length == 0 {
            return None;
        }

        let mut current = {
            let root_ref = self.root.borrow();
            root_ref.next.as_ref().cloned()
        };

        let mut index = 0;
        while let Some(node) = current {
            let node_ref = node.borrow();
            if node_ref.is_root {
                break;
            }
            if let Some(node_value) = &node_ref.value {
                if node_value == value {
                    return Some(index);
                }
            }
            current = node_ref.next.as_ref().cloned();
            index += 1;
        }

        None
    }

    pub fn find_index<F>(&self, mut predicate: F) -> Option<usize>
    where
        F: FnMut(&T) -> bool,
    {
        if self.length == 0 {
            return None;
        }

        let mut current = {
            let root_ref = self.root.borrow();
            root_ref.next.as_ref().cloned()
        };

        let mut index = 0;
        while let Some(node) = current {
            let node_ref = node.borrow();
            if node_ref.is_root {
                break;
            }
            if let Some(node_value) = &node_ref.value {
                if predicate(node_value) {
                    return Some(index);
                }
            }
            current = node_ref.next.as_ref().cloned();
            index += 1;
        }

        None
    }

    pub fn map<U, F>(&self, mut f: F) -> DoubleRinkedList<U>
    where
        U: Clone + Debug,
        F: FnMut(&T) -> U,
    {
        let mut result = DoubleRinkedList::new();

        if self.length == 0 {
            return result;
        }

        let mut current = {
            let root_ref = self.root.borrow();
            root_ref.next.as_ref().cloned()
        };

        while let Some(node) = current {
            let node_ref = node.borrow();
            if node_ref.is_root {
                break;
            }
            if let Some(node_value) = &node_ref.value {
                let mapped_value = f(node_value);
                let _ = result.insert_at_end(mapped_value);
            }
            current = node_ref.next.as_ref().cloned();
        }

        result
    }

    pub fn filter<F>(&self, mut predicate: F) -> Self
    where
        F: FnMut(&T) -> bool,
    {
        let mut result = DoubleRinkedList::new();

        if self.length == 0 {
            return result;
        }

        let mut current = {
            let root_ref = self.root.borrow();
            root_ref.next.as_ref().cloned()
        };

        while let Some(node) = current {
            let node_ref = node.borrow();
            if node_ref.is_root {
                break;
            }
            if let Some(node_value) = &node_ref.value {
                if predicate(node_value) {
                    let _ = result.insert_at_end(node_value.clone()); // Safe because we're building the list
                }
            }
            current = node_ref.next.as_ref().cloned();
        }

        result
    }

    pub fn reduce<U, F>(&self, mut f: F, initial: U) -> U
    where
        F: FnMut(U, &T) -> U,
    {
        let mut accumulator = initial;

        if self.length == 0 {
            return accumulator;
        }

        let mut current = {
            let root_ref = self.root.borrow();
            root_ref.next.as_ref().cloned()
        };

        while let Some(node) = current {
            let node_ref = node.borrow();
            if node_ref.is_root {
                break;
            }
            if let Some(node_value) = &node_ref.value {
                accumulator = f(accumulator, node_value);
            }
            current = node_ref.next.as_ref().cloned();
        }

        accumulator
    }

    pub fn every<F>(&self, mut predicate: F) -> bool
    where
        F: FnMut(&T) -> bool,
    {
        if self.length == 0 {
            return true;
        }

        let mut current = {
            let root_ref = self.root.borrow();
            root_ref.next.as_ref().cloned()
        };

        while let Some(node) = current {
            let node_ref = node.borrow();
            if node_ref.is_root {
                break;
            }
            if let Some(node_value) = &node_ref.value {
                if !predicate(node_value) {
                    return false;
                }
            }
            current = node_ref.next.as_ref().cloned();
        }

        true
    }

    pub fn any<F>(&self, mut predicate: F) -> bool
    where
        F: FnMut(&T) -> bool,
    {
        if self.length == 0 {
            return false;
        }

        let mut current = {
            let root_ref = self.root.borrow();
            root_ref.next.as_ref().cloned()
        };

        while let Some(node) = current {
            let node_ref = node.borrow();
            if node_ref.is_root {
                break;
            }
            if let Some(node_value) = &node_ref.value {
                if predicate(node_value) {
                    return true;
                }
            }
            current = node_ref.next.as_ref().cloned();
        }

        false
    }

    pub fn some<F>(&self, predicate: F) -> bool
    where
        F: FnMut(&T) -> bool,
    {
        self.any(predicate)
    }

    pub fn display(&self, separator: Option<&str>)
    where
        T: Display,
    {
        let sep = separator.unwrap_or(", ");

        if self.length == 0 {
            println!("[empty list]");
            return;
        }

        let mut output = String::new();
        let mut current = {
            let root_ref = self.root.borrow();
            root_ref.next.as_ref().cloned()
        };

        let mut first = true;
        while let Some(node) = current {
            let node_ref = node.borrow();
            if node_ref.is_root {
                break;
            }
            if let Some(node_value) = &node_ref.value {
                if !first {
                    output.push_str(sep);
                }
                output.push_str(&format!("{}", node_value));
                first = false;
            }
            current = node_ref.next.as_ref().cloned();
        }

        println!("{}", output);
    }

    pub fn log(&self, separator: Option<&str>)
    where
        T: Debug,
    {
        let sep = separator.unwrap_or(", ");

        if self.length == 0 {
            println!("DoubleRinkedList []");
            return;
        }

        let mut output = String::from("DoubleRinkedList [");
        let mut current = {
            let root_ref = self.root.borrow();
            root_ref.next.as_ref().cloned()
        };

        let mut first = true;
        while let Some(node) = current {
            let node_ref = node.borrow();
            if node_ref.is_root {
                break;
            }
            if let Some(node_value) = &node_ref.value {
                if !first {
                    output.push_str(sep);
                }
                output.push_str(&format!("{:?}", node_value));
                first = false;
            }
            current = node_ref.next.as_ref().cloned();
        }

        output.push(']');
        println!("{}", output);
    }

    pub fn splice(&mut self, index: usize, delete_count: usize, items: Vec<T>) -> Result<Vec<T>> {
        if index > self.length {
            return Err(ListError::IndexOutOfBounds {
                index,
                length: self.length
            });
        }

        let mut removed = Vec::new();

        self.move_cursor_at_index(index)?;

        for _ in 0..delete_count.min(self.length - index) {
            if let Ok(value) = self.remove_after_cursor() {
                removed.push(value);
            } else {
                break;
            }
        }

        for item in items {
            self.insert_after_cursor(item)?;
        }

        Ok(removed)
    }

    pub fn clear(&mut self) {
        // Simple clear that doesn't use complex cursor operations to avoid infinite loops
        self.length = 0;
        let mut root_ref = self.root.borrow_mut();
        root_ref.next = Some(self.root.clone());
        root_ref.previous = Some(Rc::downgrade(&self.root));
        drop(root_ref);
        self.reset_cursor();
    }

    pub fn reverse(&mut self) {
        if self.length <= 1 {
            return;
        }

        let values = self.to_vec();
        self.clear();

        for value in values.into_iter().rev() {
            let _ = self.insert_at_end(value); // Safe because we're rebuilding
        }
    }

    pub fn validate_cursor(&self) -> bool {
        if self.length == 0 {
            let before_is_root = self.cursor.before.as_ref()
                .and_then(|w| w.upgrade())
                .map(|node| node.borrow().is_root)
                .unwrap_or(false);
            let after_is_root = self.cursor.after.as_ref()
                .and_then(|w| w.upgrade())
                .map(|node| node.borrow().is_root)
                .unwrap_or(false);
            return before_is_root && after_is_root;
        }

        if let (Some(before_weak), Some(after_weak)) = (&self.cursor.before, &self.cursor.after) {
            if let (Some(before_node), Some(after_node)) = (before_weak.upgrade(), after_weak.upgrade()) {
                if let Some(before_next) = &before_node.borrow().next {
                    return Rc::ptr_eq(before_next, &after_node);
                }
            }
        }
        false
    }

    pub fn reset_cursor(&mut self) {
        let root_weak = Rc::downgrade(&self.root);
        if self.length == 0 {
            self.cursor = Cursor {
                before: Some(root_weak.clone()),
                after: Some(root_weak),
            };
        } else {
            if let Some(first_node) = &self.root.borrow().next {
                if !first_node.borrow().is_root {
                    let first_weak = Rc::downgrade(first_node);
                    self.cursor = Cursor {
                        before: Some(root_weak),
                        after: Some(first_weak),
                    };
                    return;
                }
            }
            self.cursor = Cursor {
                before: Some(root_weak.clone()),
                after: Some(root_weak),
            };
        }
    }

    pub fn get_cursor(&self) -> Cursor<T> {
        self.cursor.clone()
    }

    pub fn set_cursor(&mut self, cursor: Cursor<T>) -> Result<()> {
        let temp_cursor = self.cursor.clone();
        self.cursor = cursor;

        if !self.validate_cursor() {
            self.cursor = temp_cursor; // Restore previous cursor
            return Err(ListError::InvalidCursor);
        }

        Ok(())
    }

    pub fn move_cursor_at_begin(&mut self) -> Result<()> {
        let root_weak = Rc::downgrade(&self.root);
        if self.length == 0 {
            self.cursor = Cursor {
                before: Some(root_weak.clone()),
                after: Some(root_weak),
            };
        } else {
            if let Some(first_node) = &self.root.borrow().next {
                if !first_node.borrow().is_root {
                    let first_weak = Rc::downgrade(first_node);
                    self.cursor = Cursor {
                        before: Some(root_weak),
                        after: Some(first_weak),
                    };
                } else {
                    self.cursor = Cursor {
                        before: Some(root_weak.clone()),
                        after: Some(root_weak),
                    };
                }
            }
        }
        Ok(())
    }

    pub fn move_cursor_at_end(&mut self) -> Result<()> {
        debug_print!(self, "move_cursor_at_end() called, length: {}", self.length);
        let root_weak = Rc::downgrade(&self.root);
        if self.length == 0 {
            debug_print!(self, "Empty list, setting cursor between root and root");
            self.cursor = Cursor {
                before: Some(root_weak.clone()),
                after: Some(root_weak),
            };
        } else {
            debug_print!(self, "Non-empty list, finding last node");
            if let Some(last_weak) = &self.root.borrow().previous {
                if let Some(last_node) = last_weak.upgrade() {
                    let is_root = last_node.borrow().is_root;
                    debug_print!(self, "Last node is_root: {}", is_root);
                    if !is_root {
                        let last_weak_new = Rc::downgrade(&last_node);
                        self.cursor = Cursor {
                            before: Some(last_weak_new),
                            after: Some(root_weak),
                        };
                        debug_print!(self, "Cursor set after last element");
                        return Ok(());
                    } else {
                        debug_print!(self, "Last node is root (empty list case)");
                        self.cursor = Cursor {
                            before: Some(root_weak.clone()),
                            after: Some(root_weak),
                        };
                        return Ok(());
                    }
                }
            }
            debug_print!(self, "Fallback to move_cursor_at_begin");
            self.move_cursor_at_begin()?;
        }
        debug_print!(self, "move_cursor_at_end completed");
        Ok(())
    }

    pub fn move_cursor_at_index(&mut self, index: usize) -> Result<()> {
        if index > self.length {
            return Err(ListError::IndexOutOfBounds {
                index,
                length: self.length
            });
        }

        if index == 0 {
            return self.move_cursor_at_begin();
        }

        if index == self.length {
            return self.move_cursor_at_end();
        }

        if self.length == 0 {
            return if index == 0 {
                Ok(())
            } else {
                Err(ListError::IndexOutOfBounds {
                    index,
                    length: self.length
                })
            };
        }

        let mut current = self.root.clone();
        for _ in 0..index {
            let next = if let Some(next_ref) = current.borrow().next.as_ref() {
                next_ref.clone()
            } else {
                return Err(ListError::IndexOutOfBounds {
                    index,
                    length: self.length
                });
            };
            if next.borrow().is_root {
                return Err(ListError::IndexOutOfBounds {
                    index,
                    length: self.length
                });
            }
            current = next;
        }

        let next_node = {
            let current_ref = current.borrow();
            current_ref.next.as_ref().map(|n| n.clone())
        };

        if let Some(next_node) = next_node {
            let current_weak = Rc::downgrade(&current);
            let next_weak = Rc::downgrade(&next_node);
            self.cursor = Cursor {
                before: Some(current_weak),
                after: Some(next_weak),
            };
            Ok(())
        } else {
            Err(ListError::IndexOutOfBounds {
                index,
                length: self.length
            })
        }
    }

    pub fn move_cursor_to_next(&mut self) -> Result<()> {
        if self.length == 0 {
            return Err(ListError::EmptyList);
        }

        if let Some(after_weak) = &self.cursor.after {
            if let Some(after_node) = after_weak.upgrade() {
                if after_node.borrow().is_root {
                    return Err(ListError::IndexOutOfBounds {
                        index: self.length,
                        length: self.length
                    });
                }

                if let Some(next_node) = &after_node.borrow().next {
                    let new_before = Rc::downgrade(&after_node);
                    let new_after = Rc::downgrade(next_node);
                    self.cursor = Cursor {
                        before: Some(new_before),
                        after: Some(new_after),
                    };
                    return Ok(());
                }
            }
        }
        Err(ListError::InvalidCursor)
    }

    pub fn move_cursor_to_previous(&mut self) -> Result<()> {
        if self.length == 0 {
            return Err(ListError::EmptyList);
        }

        if let Some(before_weak) = &self.cursor.before {
            if let Some(before_node) = before_weak.upgrade() {
                if before_node.borrow().is_root {
                    return Err(ListError::IndexOutOfBounds {
                        index: 0,
                        length: self.length
                    });
                }

                if let Some(prev_weak) = &before_node.borrow().previous {
                    if let Some(prev_node) = prev_weak.upgrade() {
                        let new_after = Rc::downgrade(&before_node);
                        let new_before = Rc::downgrade(&prev_node);
                        self.cursor = Cursor {
                            before: Some(new_before),
                            after: Some(new_after),
                        };
                        return Ok(());
                    }
                }
            }
        }
        Err(ListError::InvalidCursor)
    }

    pub fn insert_after_cursor(&mut self, value: T) -> Result<()> {
        debug_print!(self, "insert_after_cursor({:?}) called, length: {}", value, self.length);
        
        if !self.validate_cursor() {
            debug_print!(self, "Cursor invalid, resetting");
            self.reset_cursor();
        }

        let new_node = if let Some(ref mut pool) = self.pool {
            debug_print!(self, "Getting node from pool");
            pool.get_node(value)
        } else {
            debug_print!(self, "Creating new node (no pool)");
            Rc::new(RefCell::new(Node::new(value)))
        };

        if self.length == 0 {
            debug_print!(self, "Inserting first element in empty list");
            {
                let mut root_ref = self.root.borrow_mut();
                let mut new_node_ref = new_node.borrow_mut();

                root_ref.next = Some(new_node.clone());
                root_ref.previous = Some(Rc::downgrade(&new_node));
                
                new_node_ref.next = Some(self.root.clone());
                new_node_ref.previous = Some(Rc::downgrade(&self.root));
            }

            self.length = 1;
            let root_weak = Rc::downgrade(&self.root);
            self.cursor = Cursor {
                before: Some(Rc::downgrade(&new_node)),
                after: Some(root_weak),
            };
            debug_print!(self, "First element inserted successfully, length now: {}", self.length);
            return Ok(());
        }

        debug_print!(self, "Inserting into non-empty list");
        if let (Some(before_weak), Some(after_weak)) = (&self.cursor.before, &self.cursor.after) {
            if let (Some(before_node), Some(after_node)) = (before_weak.upgrade(), after_weak.upgrade()) {
                debug_print!(self, "Got cursor before and after nodes, inserting between them");
                {
                    let mut new_node_ref = new_node.borrow_mut();
                    let mut before_ref = before_node.borrow_mut();
                    let mut after_ref = after_node.borrow_mut();

                    new_node_ref.previous = Some(Rc::downgrade(&before_node));
                    new_node_ref.next = Some(after_node.clone());
                    
                    before_ref.next = Some(new_node.clone());
                    after_ref.previous = Some(Rc::downgrade(&new_node));
                }

                self.length += 1;
                let new_node_weak = Rc::downgrade(&new_node);
                self.cursor.before = Some(new_node_weak);
                debug_print!(self, "Element inserted successfully, new length: {}", self.length);
                return Ok(());
            }
        }

        if let Some(ref mut pool) = self.pool {
            pool.return_node(new_node);
        }
        Err(ListError::InvalidCursor)
    }

    pub fn remove_after_cursor(&mut self) -> Result<T> {
        if self.length == 0 {
            return Err(ListError::EmptyList);
        }

        if !self.validate_cursor() {
            self.reset_cursor();
            return Err(ListError::InvalidCursor);
        }

        if let Some(after_weak) = &self.cursor.after {
            if let Some(after_node) = after_weak.upgrade() {
                if after_node.borrow().is_root {
                    return Err(ListError::IndexOutOfBounds {
                        index: self.length,
                        length: self.length
                    });
                }

                let value = {
                    let after_ref = after_node.borrow();
                    after_ref.value.clone()
                        .ok_or(ListError::InvalidCursor)?
                };

                let next_node = {
                    let after_ref = after_node.borrow();
                    after_ref.next.as_ref().map(|n| n.clone()).unwrap_or_else(|| self.root.clone())
                };

                let next_weak = Rc::downgrade(&next_node);
                self.cursor.after = Some(next_weak);

                if let Some(before_weak) = &self.cursor.before {
                    if let Some(before_node) = before_weak.upgrade() {
                        let is_root = next_node.borrow().is_root;
                        {
                            let mut before_ref = before_node.borrow_mut();
                            before_ref.next = if is_root {
                                None
                            } else {
                                Some(next_node.clone())
                            };
                        }
                        {
                            let mut next_ref = next_node.borrow_mut();
                            next_ref.previous = Some(Rc::downgrade(&before_node));
                        }
                    }
                }

                self.length -= 1;

                if self.length == 0 {
                    self.reset_cursor();
                }

                if let Some(ref mut pool) = self.pool {
                    pool.return_node(after_node);
                }

                Ok(value)
            } else {
                Err(ListError::InvalidCursor)
            }
        } else {
            Err(ListError::InvalidCursor)
        }
    }

    pub fn remove_before_cursor(&mut self) -> Result<T> {
        if self.length == 0 {
            return Err(ListError::EmptyList);
        }

        if !self.validate_cursor() {
            self.reset_cursor();
            return Err(ListError::InvalidCursor);
        }

        if let Some(before_weak) = &self.cursor.before {
            if let Some(before_node) = before_weak.upgrade() {
                if before_node.borrow().is_root {
                    return Err(ListError::IndexOutOfBounds {
                        index: 0,
                        length: self.length
                    });
                }

                let value = {
                    let before_ref = before_node.borrow();
                    before_ref.value.clone()
                        .ok_or(ListError::InvalidCursor)?
                };

                let prev_node = if let Some(prev_weak) = &before_node.borrow().previous {
                    if let Some(prev) = prev_weak.upgrade() {
                        prev
                    } else {
                        self.root.clone()
                    }
                } else {
                    self.root.clone()
                };

                let prev_weak = Rc::downgrade(&prev_node);
                self.cursor.before = Some(prev_weak);

                if let Some(after_weak) = &self.cursor.after {
                    if let Some(after_node) = after_weak.upgrade() {
                        let is_root = after_node.borrow().is_root;
                        {
                            let mut prev_ref = prev_node.borrow_mut();
                            if is_root {
                                prev_ref.next = None;
                            } else {
                                prev_ref.next = Some(after_node.clone());
                            }
                        }
                        {
                            let mut after_ref = after_node.borrow_mut();
                            after_ref.previous = Some(Rc::downgrade(&prev_node));
                        }
                    }
                }

                self.length -= 1;

                if self.length == 0 {
                    self.reset_cursor();
                }

                if let Some(ref mut pool) = self.pool {
                    pool.return_node(before_node);
                }

                Ok(value)
            } else {
                Err(ListError::InvalidCursor)
            }
        } else {
            Err(ListError::InvalidCursor)
        }
    }

    pub fn insert_before_cursor(&mut self, value: T) -> Result<()> {
        let new_node = if let Some(ref mut pool) = self.pool {
            pool.get_node(value)
        } else {
            Rc::new(RefCell::new(Node::new(value)))
        };

        if self.length == 0 {
            {
                let mut root_ref = self.root.borrow_mut();
                let mut new_node_ref = new_node.borrow_mut();

                new_node_ref.next = None;
                new_node_ref.previous = Some(Rc::downgrade(&self.root));
                root_ref.next = Some(new_node.clone());
                root_ref.previous = Some(Rc::downgrade(&new_node));
            }

            self.length = 1;
            let root_weak = Rc::downgrade(&self.root);
            let new_node_weak = Rc::downgrade(&new_node);
            self.cursor = Cursor {
                before: Some(root_weak),
                after: Some(new_node_weak),
            };
            return Ok(());
        }

        if let (Some(before_weak), Some(after_weak)) = (&self.cursor.before, &self.cursor.after) {
            if let (Some(before_node), Some(after_node)) = (before_weak.upgrade(), after_weak.upgrade()) {
                let is_root = after_node.borrow().is_root;

                {
                    let mut new_node_ref = new_node.borrow_mut();
                    let mut before_ref = before_node.borrow_mut();

                    new_node_ref.previous = Some(Rc::downgrade(&before_node));
                    before_ref.next = Some(new_node.clone());

                    if is_root {
                        new_node_ref.next = None;
                    } else {
                        new_node_ref.next = Some(after_node.clone());
                    }
                }

                {
                    let mut after_ref = after_node.borrow_mut();
                    after_ref.previous = Some(Rc::downgrade(&new_node));
                }

                self.length += 1;
                let new_node_weak = Rc::downgrade(&new_node);
                self.cursor.after = Some(new_node_weak);
                return Ok(());
            }
        }

        if let Some(ref mut pool) = self.pool {
            pool.return_node(new_node);
        }
        Err(ListError::InvalidCursor)
    }
}

impl<T> Drop for DoubleRinkedList<T>
where
    T: Clone + Debug,
{
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T> Default for DoubleRinkedList<T>
where
    T: Clone + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for DoubleRinkedList<T>
where
    T: Clone + Debug,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for item in iter {
            let _ = list.insert_at_end(item); // Safe because we're building the list
        }
        list
    }
}

pub struct DoubleLinkedListIterator<T>
where
    T: Clone + Debug,
{
    current: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Iterator for DoubleLinkedListIterator<T>
where
    T: Clone + Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = &self.current {
            let (value, next) = {
                let node_ref = node.borrow();
                if node_ref.is_root {
                    return None;
                }

                let value = node_ref.value.clone();
                let next = node_ref.next.as_ref().cloned();
                (value, next)
            }; // node_ref borrow ends here

            if let Some(ref next_node) = next {
                if next_node.borrow().is_root {
                    self.current = None; // Stop iteration at root
                } else {
                    self.current = next;
                }
            } else {
                self.current = None;
            }
            
            value
        } else {
            None
        }
    }
}

impl<T> IntoIterator for DoubleRinkedList<T>
where
    T: Clone + Debug,
{
    type Item = T;
    type IntoIter = DoubleLinkedListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        let current = if self.length == 0 {
            None
        } else {
            let root_ref = self.root.borrow();
            if let Some(first_node) = root_ref.next.as_ref() {
                if !first_node.borrow().is_root {
                    Some(first_node.clone())
                } else {
                    None // Empty list where root points to itself
                }
            } else {
                None
            }
        };

        DoubleLinkedListIterator { current }
    }
}

impl<T> IntoIterator for &DoubleRinkedList<T>
where
    T: Clone + Debug,
{
    type Item = T;
    type IntoIter = DoubleLinkedListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        let current = if self.length == 0 {
            None
        } else {
            let root_ref = self.root.borrow();
            if let Some(first_node) = root_ref.next.as_ref() {
                if !first_node.borrow().is_root {
                    Option::Some(first_node.clone())
                } else {
                    Option::None // Empty list where root points to itself
                }
            } else {
                None
            }
        };

        DoubleLinkedListIterator { current }
    }
}

pub type List<T> = DoubleRinkedList<T>;
