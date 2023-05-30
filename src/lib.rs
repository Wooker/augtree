use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug, PartialEq)]
pub enum AugTreeNode<T> {
    Node(Rc<RefCell<AugTree<T>>>),
    None,
}

impl<T> AugTreeNode<T> {
    fn from(value: AugTree<T>) -> Self {
        Self::Node(Rc::new(RefCell::new(value)))
    }
}

type Interval<T> = (T, T);

#[derive(Debug, PartialEq)]
pub struct AugTree<T> {
    interval: Interval<T>,
    highest: T,
    pub left: AugTreeNode<T>,
    right: AugTreeNode<T>,
}

impl<T> AugTree<T>
where
    T: Clone + PartialOrd,
{
    pub(crate) fn new(start: T, end: T) -> Self {
        let highest = end.clone();
        Self {
            highest,
            interval: (start, end),
            left: AugTreeNode::None,
            right: AugTreeNode::None,
        }
    }

    pub fn add(&mut self, node: AugTree<T>) {
        if node.interval.0 < self.interval.0 {
            if let AugTreeNode::Node(l) = &self.left {
                l.borrow_mut().add(node);
            } else {
                self.left = AugTreeNode::Node(Rc::new(RefCell::new(node)));
            }
        } else {
            if self.highest < node.highest {
                self.highest = node.highest.clone();
            }
            if let AugTreeNode::Node(l) = &self.right {
                l.borrow_mut().add(node);
            } else {
                self.right = AugTreeNode::Node(Rc::new(RefCell::new(node)));
            }
        }
    }

    pub fn range(&self) -> (T, T) {
        (self.interval.0.clone(), self.interval.1.clone())
    }
}

pub fn height<T>(node: &AugTreeNode<T>) -> usize
where
    T: Clone + PartialOrd + Debug,
{
    match node {
        AugTreeNode::None => 0,
        AugTreeNode::Node(n) => {
            let left_height = height(&n.clone().borrow().left);
            let right_height = height(&n.clone().borrow().right);
            left_height.max(right_height) + 1
        }
    }
}

pub fn search<T>(node: &AugTreeNode<T>, point: &T, result: &mut Vec<Interval<T>>)
where
    T: Clone + PartialOrd + Debug,
{
    match node {
        AugTreeNode::None => {
            println!("Node is none");
            return;
        }
        AugTreeNode::Node(n) => {
            let tree = n.borrow();
            if tree.highest < *point {
                println!("Highest({:?}) is smaller", tree.highest);
                return;
            }
            search(&tree.left, point, result);

            let (start, end) = tree.range();
            if &start <= point && &end > point {
                result.push(tree.interval.clone());
            }

            if point < &start {
                return;
            }

            search(&tree.right, point, result);
        }
    }
}

#[cfg(test)]
mod test_chrono_naivetime {
    use super::*;
    use chrono::NaiveTime;

    impl Default for AugTree<NaiveTime> {
        fn default() -> Self {
            let start = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
            let end = NaiveTime::from_hms_opt(23, 59, 59).unwrap();

            Self::new(start, end)
        }
    }

    #[test]
    fn initialization() {
        let node: AugTree<NaiveTime> = AugTree::default();
        assert_eq!(node.left, AugTreeNode::<NaiveTime>::None);
        assert_eq!(node.right, AugTreeNode::<NaiveTime>::None);

        let start = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let end = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
        assert_eq!(node.range().0, start);
        assert_eq!(node.range().1, end);
        assert_eq!(node.highest, end);
        assert_eq!(height(&AugTreeNode::Node(Rc::new(RefCell::new(node)))), 1);
    }

    #[test]
    fn add_node() {
        let mut node1: AugTree<NaiveTime> = AugTree::new(
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(15, 0, 0).unwrap(),
        );
        let node2: AugTree<NaiveTime> = AugTree::new(
            NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(11, 0, 0).unwrap(),
        );
        node1.add(node2);

        assert_ne!(node1.left, AugTreeNode::None);
        assert_eq!(node1.right, AugTreeNode::None);
        assert_eq!(node1.highest, NaiveTime::from_hms_opt(15, 0, 0).unwrap());

        let node3: AugTree<NaiveTime> = AugTree::new(
            NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
        );
        node1.add(node3);

        assert_ne!(node1.left, AugTreeNode::None);
        assert_ne!(node1.right, AugTreeNode::None);
        assert_eq!(node1.highest, NaiveTime::from_hms_opt(15, 0, 0).unwrap());
        assert_eq!(height(&AugTreeNode::Node(Rc::new(RefCell::new(node1)))), 2);
    }

    #[test]
    fn search_node_right() {
        let mut node1: AugTree<NaiveTime> = AugTree::new(
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(15, 0, 0).unwrap(),
        );
        let node2: AugTree<NaiveTime> = AugTree::new(
            NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(11, 0, 0).unwrap(),
        );
        let node3: AugTree<NaiveTime> = AugTree::new(
            NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
        );

        node1.add(node2);
        node1.add(node3);

        let mut result: Vec<Interval<NaiveTime>> = vec![];
        let root = Rc::new(RefCell::new(node1));
        search(
            &AugTreeNode::Node(root.clone()),
            &NaiveTime::from_hms_opt(13, 30, 0).unwrap(),
            &mut result,
        );

        dbg!(&result);
        assert_eq!(result.is_empty(), false);
        assert_eq!(height(&AugTreeNode::Node(root)), 2);
    }

    #[test]
    fn search_node_left() {
        let mut node1: AugTree<NaiveTime> = AugTree::new(
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(15, 0, 0).unwrap(),
        );
        let node2: AugTree<NaiveTime> = AugTree::new(
            NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(11, 0, 0).unwrap(),
        );
        let node3: AugTree<NaiveTime> = AugTree::new(
            NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
        );

        node1.add(node2);
        node1.add(node3);

        let mut result: Vec<Interval<NaiveTime>> = vec![];
        let root = Rc::new(RefCell::new(node1));
        search(
            &AugTreeNode::Node(root.clone()),
            &NaiveTime::from_hms_opt(10, 30, 0).unwrap(),
            &mut result,
        );

        dbg!(&result);
        assert_eq!(result.is_empty(), false);
        assert_eq!(height(&AugTreeNode::Node(root)), 2);
    }

    #[test]
    fn height_test() {
        let mut node1: AugTree<NaiveTime> = AugTree::new(
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
        );
        let node2: AugTree<NaiveTime> = AugTree::new(
            NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
        );
        let node3: AugTree<NaiveTime> = AugTree::new(
            NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(15, 0, 0).unwrap(),
        );

        node1.add(node2);
        node1.add(node3);

        dbg!(&node1);

        assert_eq!(node1.highest, NaiveTime::from_hms_opt(15, 0, 0).unwrap());
        // assert_eq!(node2.highest, NaiveTime::from_hms_opt(15, 0, 0).unwrap());
        // assert_eq!(node3.highest, NaiveTime::from_hms_opt(15, 0, 0).unwrap());
        assert_eq!(height(&AugTreeNode::Node(Rc::new(RefCell::new(node1)))), 3);
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{height, search, AugTree, AugTreeNode};

    #[test]
    fn init() {
        let mut node0 = AugTree::new(5, 15);
        let node1 = AugTree::new(10, 20);
        let node2 = AugTree::new(2, 3);
        let node3 = AugTree::new(1, 3);

        node0.add(node1);
        node0.add(node2);
        node0.add(node3);

        let root = Rc::new(RefCell::new(node0));
        assert_eq!(root.borrow().highest, 20);
        assert_eq!(height(&AugTreeNode::Node(root.clone())), 3);

        let mut intervals = vec![];
        search(&AugTreeNode::Node(root.clone()), &2, &mut intervals);
        assert_eq!(intervals, vec![(1, 3), (2, 3)])
    }
}
