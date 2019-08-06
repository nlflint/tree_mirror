use std::mem;

#[derive(PartialEq, Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

fn mirror_tree<T>(mut node: Node<T>) -> Node<T> {
    mem::swap(&mut node.left, &mut node.right);
    return node;
}

//    10
//   /  \
// <>    11
#[test]
fn mirror_two_deep_tree() {
    let root = Node::<usize> {
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

    let mirrored = mirror_tree::<usize>(root);

    assert!(mirrored == 
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