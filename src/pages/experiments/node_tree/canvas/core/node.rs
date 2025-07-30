use bevy::prelude::Resource;
use rand::{prelude::*, rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Resource, Serialize, Deserialize, PartialEq, Default)]
pub struct Node<T> {
    pub value: T,
    pub children: Vec<Node<T>>,
}
pub fn generate_random_tree(
    depth: usize,
    max_first_children: usize,
    max_subgroups: usize,
) -> Node<String> {
    let mut rng = rng();

    fn build_node(
        label: String,
        current_depth: usize,
        max_first_children: usize,
        max_subgroups: usize,
        rng: &mut ThreadRng,
    ) -> Node<String> {
        let mut node = Node {
            value: label.clone(),
            children: vec![],
        };

        if current_depth > 0 {
            let num_children = rng.random_range(3..=max_first_children);

            for i in 0..num_children {
                let child_label = format!("{}-{}", label, i);
                let mut child = Node {
                    value: child_label.clone(),
                    children: vec![],
                };

                // 70% chance of having subgroups, only if more depth is available
                if current_depth > 1 && rng.random_bool(0.7) {
                    let num_subgroups = rng.random_range(1..=max_subgroups);
                    for j in 0..num_subgroups {
                        let subgroup_label = format!("{}-{}", child_label, j);
                        let subgroup = build_node(
                            subgroup_label,
                            current_depth - 2,
                            max_first_children,
                            max_subgroups,
                            rng,
                        );
                        child.children.push(subgroup);
                    }
                }

                node.children.push(child);
            }
        }

        node
    }

    build_node(
        "Root".to_string(),
        depth,
        max_first_children,
        max_subgroups,
        &mut rng,
    )
}
// pub fn generate_random_tree(
//     _depth: usize,
//     _max_first_children: usize,
//     _max_subgroups: usize,
// ) -> Node<String> {
//     Node {
//         value: String::from("root"),
//         children: vec![
//             Node {
//                 value: String::from("child 1"),
//                 children: vec![
//                     Node {
//                         value: String::from("child 1 1"),
//                         children: Vec::new(),
//                     },
//                     Node {
//                         value: String::from("child 1 2"),
//                         children: Vec::new(),
//                     },
//                 ],
//             },
//             Node {
//                 value: String::from("child 2"),
//                 children: vec![
//                     Node {
//                         value: String::from("child 2 1"),
//                         children: Vec::new(),
//                     },
//                     Node {
//                         value: String::from("child 2 1"),
//                         children: Vec::new(),
//                     },
//                 ],
//             },
//             Node {
//                 value: String::from("child 3"),
//                 children: Vec::new(),
//             },
//             Node {
//                 value: String::from("child 4"),
//                 children: Vec::new(),
//             },
//             Node {
//                 value: String::from("child 5"),
//                 children: Vec::new(),
//             },
//         ],
//     }
// }
