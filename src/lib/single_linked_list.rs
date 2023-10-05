use std::rc::Rc;

pub struct SingleLinkedList<T> {
    head: Option<Rc<Node<T>>>,
    tail: Option<Rc<Node<T>>>,
    size: u16,
}

impl<T> SingleLinkedList<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn add(&mut self, value: T) {
        let node = Rc::new(Node::new(value, self.head.clone()));
        if let None = &self.head {
            self.tail = Some(node.clone());
        }
        self.head = Some(node);

        self.size += 1;
    }

    pub fn remove(&mut self) {
        if let Some(node) = &self.head {
            self.head = node.next.clone();

            if let None = self.head {
                self.tail = None;
            }

            self.size -= 1;
        }
    }

    pub fn get(&self, index: usize) -> Option<T> {
        let mut iteration = 0;
        let mut node = self.head.clone();

        while iteration < index {
            if let Some(n) = node {
                node = n.next.clone();
            }
            iteration += 1;
        }

        match node {
            Some(n) => Some(n.value),
            None => None,
        }
    }

    pub fn reverse(&mut self) {
        let mut head = &self.head;
        let mut temp_node: Option<Rc<Node<T>>> = None;

        while let Some(node) = head {
            let new_node = Rc::new(Node::new(node.value, temp_node));
            if new_node.next.is_none() {
                self.tail = Some(new_node.clone());
            }
            temp_node = Some(new_node);

            head = &node.next;
        }

        self.head = temp_node;
    }

    pub fn head(&self) -> Option<T> {
        match &self.head {
            Some(n) => Some(n.value),
            None => None,
        }
    }

    pub fn tail(&self) -> Option<T> {
        match &self.tail {
            Some(n) => Some(n.value),
            None => None,
        }
    }

    pub fn size(&self) -> u16 {
        self.size
    }
}

#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Rc<Node<T>>>) -> Self {
        Self { value, next }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let list: SingleLinkedList<i32> = SingleLinkedList::new();
        assert_eq!(list.size(), 0);
        assert_eq!(list.head(), None);
        assert_eq!(list.tail(), None);
    }

    #[test]
    fn test_add_and_remove() {
        let mut list: SingleLinkedList<i32> = SingleLinkedList::new();

        list.add(1);
        assert_eq!(list.size(), 1);
        assert_eq!(list.head(), Some(1));
        assert_eq!(list.tail(), Some(1));

        list.add(2);
        assert_eq!(list.size(), 2);
        assert_eq!(list.head(), Some(2));
        assert_eq!(list.tail(), Some(1));

        list.remove();
        assert_eq!(list.size(), 1);
        assert_eq!(list.head(), Some(1));
        assert_eq!(list.tail(), Some(1));

        list.remove();
        assert_eq!(list.size(), 0);
        assert_eq!(list.head(), None);
        assert_eq!(list.tail(), None);
    }

    #[test]
    fn test_get() {
        let mut list: SingleLinkedList<i32> = SingleLinkedList::new();
        list.add(1);
        list.add(2);
        list.add(3);

        assert_eq!(list.get(0), Some(3));
        assert_eq!(list.get(1), Some(2));
        assert_eq!(list.get(2), Some(1));
        assert_eq!(list.get(3), None);
    }

    #[test]
    fn test_reverse() {
        let mut list: SingleLinkedList<i32> = SingleLinkedList::new();
        list.add(1);
        list.add(2);
        list.add(3);

        list.reverse();

        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(2));
        assert_eq!(list.get(2), Some(3));
    }
}
