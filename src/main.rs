//Fetching Top Products
//link list corresponding to top products in different countries
//combine these linked list to consolidated linked lists

//Tools -> Linkedlist, Iterators

#[derive(Debug)]
struct SinglyLinkedList<T: std::fmt::Debug + std::marker::Copy> {
    head: Pointer<T>,
}

#[derive(Debug)]
struct Node<T: std::fmt::Debug + std::marker::Copy> {
    element: T,
    next: Pointer<T>,
}

type Pointer<T> = Option<Box<Node<T>>>;

impl<T: std::fmt::Debug + std::marker::Copy> SinglyLinkedList<T> {
    fn create_empty_list() -> SinglyLinkedList<T> {
        SinglyLinkedList { head: None }
    }

    fn add(&mut self, element: T) {
        let previous_head = self.head.take();
        let new_head = Box::new(Node {
            element: element,
            next: previous_head,
        });
        self.head = Some(new_head);
    }

    fn remove(&mut self) -> Option<T> {
        let previous_head = self.head.take();
        match previous_head {
            Some(old_head) => {
                self.head = old_head.next;
                Some(old_head.element)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<T> {
        match &self.head {
            Some(node) => Some(node.element),
            None => None,
        }
    }

    fn print(&self) {
        let mut traversal = &self.head;
        while true {
            match traversal {
                Some(node) => {
                    print!("{:?} ", node.element);
                    traversal = &node.next;
                }
                None => {
                    break;
                }
            }
        }
    }

    fn reverse(&mut self) {
        if self.head.is_none() || self.head.as_ref().unwrap().next.is_none() {
            return;
        }
        let mut prev: Pointer<T> = None;
        let mut current = self.head.take();

        while current.is_some() {
            let next = current.as_mut().unwrap().next.take();
            current.as_mut().unwrap().next = prev.take();
            prev = current.take();
            current = next;
        }

        self.head = prev.take();
    }
}

fn sort_lists(vec_list: &mut Vec<SinglyLinkedList<i32>>) -> SinglyLinkedList<i32> {
    let mut sorted_list = SinglyLinkedList::create_empty_list();
    let mut values: Vec<i32> = Vec::new();
    while true {
        let values = vec_list
            .into_iter()
            .map(|x| x.head.as_ref().unwrap().element)
            .collect::<Vec<i32>>();

        let min_val = *values.iter().min().unwrap();
        let min_index = values
            .iter()
            .position(|&x| x == min_val)
            .unwrap();

        sorted_list.add(min_val);
        vec_list[min_index].remove();

        if vec_list[min_index].head.is_none() {
            vec_list.remove(min_index);
        }

        if vec_list.len() == 0 {
            break;
        }
    }
    sorted_list
}
fn main() {
    let mut list1 = SinglyLinkedList::create_empty_list();
    list1.add(45);
    list1.add(40);
    list1.add(35);
    list1.add(30);
    list1.add(25);
    list1.add(10);

    let mut list2 = SinglyLinkedList::create_empty_list();
    list2.add(50);
    list2.add(42);
    list2.add(33);
    list2.add(28);
    list2.add(22);
    list2.add(15);

    let mut list3 = SinglyLinkedList::create_empty_list();
    list3.add(55);
    list3.add(48);
    list3.add(38);
    list3.add(29);

    let mut result = sort_lists(&mut vec![list1, list2, list3]);
    result.print();

    result.reverse();
    println!("\nReversed List:");
    result.print();
}
