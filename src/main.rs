use std::mem;

#[derive(PartialEq, Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

fn mirror_tree<T>(root: &mut Node<T>) {
    let mut _nodes = vec![root];
    
    while let Some(node) = _nodes.pop() {
        mem::swap(&mut node.left, &mut node.right);

        match node.left {
            Some(ref mut boxed) => _nodes.push(boxed),
            _ => ()
        }

        match node.right {
            Some(ref mut boxed) => _nodes.push(boxed),
            _ => ()
        }
    }
}

//    10
//   /  \
// <>    11
#[test]
fn mirror_two_deep_tree() {
    let mut root = Node::<usize> {
        value: 10,
        left: Option::None,
        right: Option::Some(
            Box::new(
                Node::<usize> {
                    value: 11,
                    left: Option::None,
                    right: Option::None
                }
            )   
        )
    };

    mirror_tree::<usize>(&mut root);

    assert!(root == 
        Node::<usize> {
            value: 10,
            right: Option::None,
            left: Option::Some(
                Box::new(
                    Node::<usize> {
                        value: 11,
                        left: Option::None,
                        right: Option::None
                    }
                )   
            )
        }
    )
}

#[test]
fn mirror_three_deep_tree() {
    let mut root = Node::<usize> {
        value: 1,
        right: Option::Some(
            Box::new(
                Node::<usize> {
                    value: 5,
                    left: Option::None,
                    right: Option::None
                }
            )
        ),
        left: Option::Some(
            Box::new(
                Node::<usize> {
                    value: 2,
                    left: Option::Some(
                        Box::new(
                            Node::<usize> {
                                value: 11,
                                left: Option::None,
                                right: Option::None
                            }
                        )
                    ),
                    right: Option::Some(
                        Box::new(
                            Node::<usize> {
                                value: 12,
                                left: Option::None,
                                right: Option::None
                            }
                        )
                    )
                }
            )   
        )
    };

    mirror_tree::<usize>(&mut root);

    assert!(root == 
        Node::<usize> {
            value: 1,
            right: Option::Some(
                Box::new(
                    Node::<usize> {
                        value: 2,
                        right: Option::Some(
                            Box::new(
                                Node::<usize> {
                                    value: 11,
                                    left: Option::None,
                                    right: Option::None
                                }
                            )
                        ),
                        left: Option::Some(
                            Box::new(
                                Node::<usize> {
                                    value: 12,
                                    left: Option::None,
                                    right: Option::None
                                }
                            )
                        )
                    }
                )   
            ),
            left: Option::Some(
                Box::new(
                    Node::<usize> {
                        value: 5,
                        left: Option::None,
                        right: Option::None
                    }
                )
            ),
        }
    );
}