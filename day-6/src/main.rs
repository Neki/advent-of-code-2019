use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let space_objects = load_tree();
    let mut distances: SpaceObjectsDistances = HashMap::new();
    set_distance_to_root(0, "COM", &space_objects, &mut distances);
    let result = compute_total_distance(&distances);
    println!("{}", result);
}

fn part_2() {
    let space_objects = load_tree();
    let mut distances: SpaceObjectsDistances = HashMap::new();
    set_distance_to_root(0, "COM", &space_objects, &mut distances);

    let santa_ancestors = compute_ancestors("SAN", &space_objects, "COM").unwrap();
    let you_ancestors = compute_ancestors("YOU", &space_objects, "COM").unwrap();

    // find lowest common ancestor
    let mut last_ancestor = santa_ancestors[0];
    for ancestors in santa_ancestors.iter().rev().zip(you_ancestors.iter().rev()) {
        if ancestors.0 != ancestors.1 {
            break;
        }
        last_ancestor = ancestors.0;
    }

    let result = (distances["SAN"] - distances[last_ancestor])
        + (distances["YOU"] - distances[last_ancestor])
        - 2;
    println!("{}", result);
}

fn compute_ancestors<'a>(
    final_node: &str,
    tree: &'a SpaceObjectsTree,
    current_node: &'a str,
) -> Option<Vec<&'a str>> {
    if final_node == current_node {
        let mut result: Vec<&str> = Vec::new();
        result.push(current_node);
        return Some(result);
    }
    let children = tree.get(current_node).expect(current_node);
    for child in children {
        if let Some(mut ancestors) = compute_ancestors(final_node, tree, child) {
            ancestors.push(&current_node);
            return Some(ancestors);
        }
    }
    None
}

fn set_distance_to_root(
    current_distance: usize,
    current_node: &str,
    tree: &SpaceObjectsTree,
    distances: &mut SpaceObjectsDistances,
) {
    distances.insert(current_node.to_string(), current_distance);
    let children = tree.get(current_node).expect(current_node);
    for child in children {
        set_distance_to_root(current_distance + 1, child, tree, distances);
    }
}

fn compute_total_distance(distances: &SpaceObjectsDistances) -> usize {
    distances.values().sum()
}

type SpaceObject = String;
// not a good type (a tree would also need an explicit root)
// but it works
type SpaceObjectsTree = HashMap<SpaceObject, Vec<SpaceObject>>;
type SpaceObjectsDistances = HashMap<SpaceObject, usize>;

fn load_tree() -> SpaceObjectsTree {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut space_objects: HashMap<SpaceObject, Vec<SpaceObject>> = HashMap::new();

    for line in reader.lines() {
        let line_unwrapped = line.unwrap();
        let mut splits = line_unwrapped.split(')');
        let center = splits.next().unwrap();
        let orbiter = splits.next().unwrap();
        assert!(splits.next().is_none());
        let orbiters = space_objects.get_mut(center);
        match orbiters {
            Some(existing_orbiters) => {
                existing_orbiters.push(orbiter.to_string());
            }
            None => {
                let mut orbiters: Vec<SpaceObject> = Vec::new();
                orbiters.push(orbiter.to_string());
                space_objects.insert(center.to_string(), orbiters);
            }
        }
        if space_objects.get(orbiter).is_none() {
            space_objects.insert(orbiter.to_string(), Vec::new());
        }
    }

    space_objects
}
