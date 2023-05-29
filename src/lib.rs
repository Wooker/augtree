#[derive(Debug, PartialEq)]
enum AugTreeNode<T> {
    Node(Box<AugTree<T>>),
    None,
}

#[derive(Debug, PartialEq)]
struct AugTree<T> {
    interval: (T, T),
    highest: T,
    left: AugTreeNode<T>,
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
            self.left = AugTreeNode::Node(Box::new(node));
        } else {
            self.highest = node.highest.clone();
            self.right = AugTreeNode::Node(Box::new(node));
        }
    }

    pub fn remove(&self, node: AugTreeNode<T>) {
        todo!()
    }

    pub fn range(&self) -> (&T, &T) {
        (&self.interval.0, &self.interval.1)
    }

    fn traverse() {
        todo!()
    }
}

#[cfg(test)]
mod test_chrono_naivetime {
    use super::*;
    use chrono::{DateTime, Duration, Local, NaiveTime};

    impl Default for AugTree<NaiveTime> {
        fn default() -> Self {
            let start = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
            let end = NaiveTime::from_hms_opt(23, 59, 59).unwrap();

            Self::new(start, end)
        }
    }

    #[test]
    fn initialization() {
        let flow: AugTree<NaiveTime> = AugTree::default();
        assert_eq!(flow.left, AugTreeNode::<NaiveTime>::None);
        assert_eq!(flow.right, AugTreeNode::<NaiveTime>::None);

        let start = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let end = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
        assert_eq!(flow.range().0, &start);
        assert_eq!(flow.range().1, &end);
        assert_eq!(flow.highest, end);
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
        assert_eq!(node1.highest, NaiveTime::from_hms_opt(14, 0, 0).unwrap());
    }
}
