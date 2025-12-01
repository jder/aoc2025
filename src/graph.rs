use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
    iter,
};

use itertools::Itertools;

pub fn find<V, EdgeIterator>(
    start: V,
    mut edges: impl FnMut(V) -> EdgeIterator,
    mut predicate: impl FnMut(&V) -> bool,
) -> impl Iterator<Item = V>
where
    EdgeIterator: Iterator<Item = V>,
    V: Eq + Hash + Clone,
{
    let mut visited = HashSet::new();
    let mut queue = vec![start];

    std::iter::from_fn(move || {
        while let Some(node) = queue.pop() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node.clone());
            queue.extend(edges(node.clone()));

            if predicate(&node) {
                return Some(node);
            }
        }
        None
    })
}

pub fn min_distances<V>(start: V, edges: impl Fn(&V) -> Vec<(V, u64)>) -> HashMap<V, u64>
where
    V: Eq + Ord + Hash + Clone,
{
    min_distances_inner(start, edges, |_, _| false)
}

fn min_distances_inner<V>(
    start: V,
    edges: impl Fn(&V) -> Vec<(V, u64)>,
    mut should_stop: impl FnMut(&V, u64) -> bool,
) -> HashMap<V, u64>
where
    V: Eq + Ord + Hash + Clone,
{
    let mut distances = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start)));
    while let Some(Reverse((distance, node))) = queue.pop() {
        if let Some(&previous_distance) = distances.get(&node) {
            if distance >= previous_distance {
                continue;
            }
        }
        distances.insert(node.clone(), distance);
        if should_stop(&node, distance) {
            break;
        }
        for (next_node, next_distance) in edges(&node) {
            queue.push(Reverse((distance + next_distance, next_node)));
        }
    }

    distances
}

pub fn min_distance_to<V>(
    start: V,
    goal: impl Fn(&V, u64) -> bool,
    edges: impl Fn(&V) -> Vec<(V, u64)>,
) -> Option<u64>
where
    V: Eq + Ord + Hash + Clone,
{
    let mut found_goal = None;
    min_distances_inner(start, edges, |node, distance| {
        if goal(node, distance) {
            found_goal = Some(distance);
            true
        } else {
            false
        }
    });
    found_goal
}

pub struct DistanceStorage<V> {
    indexes: HashMap<V, usize>,
    distances: Vec<Option<u64>>,
}

impl<V> DistanceStorage<V>
where
    V: Eq + Hash + Clone,
{
    fn new(values: impl Iterator<Item = V>) -> Self {
        let indexes: HashMap<V, usize> = values.enumerate().map(|(i, v)| (v, i)).collect();
        let distances = vec![None; indexes.len() * indexes.len()];
        Self { indexes, distances }
    }

    fn index(&self, start: &V, end: &V) -> usize {
        let start_index = self.indexes[start];
        let end_index = self.indexes[end];
        start_index * self.length() + end_index
    }

    pub fn get(&self, start: &V, end: &V) -> Option<u64> {
        self.distances[self.index(start, end)]
    }

    fn set(&mut self, start: &V, end: &V, distance: u64) -> Option<u64> {
        let index = self.index(start, end);
        let previous = self.distances[index];
        self.distances[index] = Some(distance);
        previous
    }

    fn vertex_indices(&self) -> impl Iterator<Item = usize> + use<V> {
        0..self.indexes.len()
    }

    fn length(&self) -> usize {
        self.indexes.len()
    }

    fn get_by_index(&self, start_index: usize, end_index: usize) -> Option<u64> {
        self.distances[start_index * self.length() + end_index]
    }

    fn set_by_index(&mut self, start_index: usize, end_index: usize, distance: u64) -> Option<u64> {
        let index = start_index * self.length() + end_index;
        let previous = self.distances[index];
        self.distances[index] = Some(distance);
        previous
    }

    pub fn iter(&self) -> impl Iterator<Item = (&V, &V, u64)> {
        self.indexes.iter().flat_map(move |(start, start_index)| {
            self.indexes.iter().map(move |(end, end_index)| {
                let distance = self.get_by_index(*start_index, *end_index).unwrap();
                (start, end, distance)
            })
        })
    }
}

pub fn all_pairs_min_distances<V>(
    verticies: Vec<V>,
    edges: impl Fn(&V) -> Vec<(V, u64)>,
) -> DistanceStorage<V>
where
    V: Eq + Ord + Hash + Clone,
{
    let mut storage = DistanceStorage::new(verticies.iter().cloned());
    for start in verticies.iter() {
        let nexts = edges(start);
        for (next, distance) in nexts {
            let previous = storage.set(start, &next, distance);
            assert!(previous.is_none());
        }
    }

    for middle in storage.vertex_indices() {
        for start in storage.vertex_indices() {
            for end in storage.vertex_indices() {
                let Some(start_to_middle) = storage.get_by_index(start, middle) else {
                    continue;
                };
                let Some(middle_to_end) = storage.get_by_index(middle, end) else {
                    continue;
                };
                let total_distance = start_to_middle + middle_to_end;
                let current_distance = storage.get_by_index(start, end).unwrap_or(u64::MAX);
                storage.set_by_index(start, end, total_distance.min(current_distance));
            }
        }
    }

    storage
}

pub fn all_paths<V>(start: V, end: V, edges: impl Fn(&V) -> Vec<V>) -> impl Iterator<Item = Vec<V>>
where
    V: Eq + Ord + Hash + Clone,
{
    let first_edges = edges(&start);
    let mut path_and_local_queues = vec![(start, first_edges)];
    return iter::from_fn(move || {
        while let Some((node, alternatives)) = path_and_local_queues.last_mut() {
            if *node == end {
                let node = node.clone();
                return Some(
                    path_and_local_queues
                        .iter()
                        .map(|(n, _)| n.clone())
                        .chain(iter::once(node))
                        .collect_vec(),
                );
            }

            if let Some(next_node) = alternatives.pop() {
                let next_edges = edges(&next_node);
                path_and_local_queues.push((next_node, next_edges));
            } else {
                path_and_local_queues.pop();
            }
        }

        None
    });
}

pub fn flood_fill_from<V, EI>(
    starts: impl Iterator<Item = V>,
    edges: impl Fn(&V) -> EI,
) -> Vec<Vec<V>>
where
    V: Eq + Hash + Clone,
    EI: Iterator<Item = V>,
{
    let mut regions = vec![];
    let mut visited: HashMap<V, usize> = HashMap::new();

    for start in starts {
        if visited.contains_key(&start) {
            continue;
        }
        let mut queue = vec![start];
        let mut region = vec![];
        while let Some(node) = queue.pop() {
            if visited.contains_key(&node) {
                continue;
            }
            visited.insert(node.clone(), regions.len());
            region.push(node.clone());
            for next_node in edges(&node) {
                queue.push(next_node);
            }
        }
        if !region.is_empty() {
            regions.push(region);
        }
    }

    regions
}
