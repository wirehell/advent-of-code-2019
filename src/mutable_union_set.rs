mod mutable_union_set {
    use std::rc::Rc;
    use std::borrow::{Borrow, BorrowMut};
    use std::cell::{RefCell, Ref};
    use std::ops::Deref;
    use std::cmp::Ordering;

    #[derive(Debug)]
    pub struct UnionSet {
        items: Vec<UnionItem>
    }

    #[derive(PartialEq, Eq,Debug)]
    pub struct UnionItem {
        value : i32,
        rank : RefCell<i32>,
        parent : Rc<RefCell<Option<i32>>>,
    }

    impl UnionSet {
        pub fn of(size: i32) -> UnionSet {
            let items = (0..size).map(|i|
                UnionItem { value: i, rank: RefCell::new(0), parent: Rc::new(RefCell::new(None)) }
            ).collect();
            UnionSet { items }
        }

        pub fn find(&self, i :i32) -> &UnionItem {
            let item  = &self.items[i as usize];
            let maybe_parent = RefCell::borrow(&item.parent);
            return match *maybe_parent {
                None => item,
                Some(parent) => self.find(parent),
            }
        }

        pub fn find_first(&self, i :i32) -> &UnionItem {
            return &self.items[i as usize]
        }

        pub fn union_sets(&self, item1 :i32, item2 :i32) -> &UnionItem {
            let r1 = self.find(item1);
            let r2 = self.find(item2);

            if r1 == r2 {
                return r1
            }

            let comparison = r1.rank.borrow().cmp(&*r2.rank.borrow());
            match comparison {
                Ordering::Less => {
                    println!("Less");
                    r1.parent.replace(Some(r2.value));
                    return r2
                },
                Ordering::Greater => {
                    println!("Greater");
                    r2.parent.replace(Some(r1.value));
                    return r1
                },
                Ordering::Equal => {
                    println!("Eq");
                    r2.parent.replace(Some(r1.value));
                    let prev_rank = *r1.rank.borrow();
                    r1.rank.replace(prev_rank + 1);
                    return r1
                }

            }

        }

        pub fn same_component(&self, item1 :i32, item2 :i32) -> bool {
            return self.find(item1) == self.find(item2);

        }
    }

}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::borrow::Borrow;
    use crate::mutable_union_set::mutable_union_set::UnionSet;

    #[test]
    fn test_simple() {
        let s = UnionSet::of(32);

        let b = s.find(3);
        println!("{:?}", b);

        s.union_sets(3,4);
        assert_eq!(s.same_component(3,2), false);

        assert_eq!(s.same_component(3,4), true);
        s.union_sets(2,3);
        assert_eq!(s.same_component(2,4), true);
    }

    #[test]
    fn it_doesnt_find_anything_in_empty() {
    }
}
