/*
use fux_kdtree;
use fux_kdtree::{Kdtree, KdtreePointTrait};
use fux_kdtree::distance::*;
use crate::bytebuffer::ByteBuffer;
use crate::pitchtracker::{Tone, Tone_Tone};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Point3WithId {
    dims: [f64; 3],
    pub id: i32,
}

impl Point3WithId {
    pub fn new(id: i32, x: f64, y: f64, z: f64) -> Point3WithId {
        Point3WithId {
            dims: [x, y, z],
            id: id,
        }
    }
}
impl KdtreePointTrait for Point3WithId {
    #[inline] // the inline on this method is important! as without it there is ~25% speed loss on the tree when cross-crate usage.
    fn dims(&self) -> &[f64] {
        return &self.dims;
    }
}

#[no_mangle]
pub unsafe extern "C" fn KdTreeNew(points: *mut Point3WithId, point_count: i32) -> *mut Kdtree<Point3WithId>
{
    let points = std::slice::from_raw_parts_mut(points, point_count as usize);
    let tree = Kdtree::new(points).unwrap();
    Box::into_raw(Box::new(tree))
}

#[no_mangle]
pub unsafe extern "C" fn KdTreeDelete(tree: *mut Kdtree<Point3WithId>)
{
    let tree = Box::from_raw(tree);
    drop(tree);
}

#[no_mangle]
pub unsafe extern "C" fn KdTreeInsert(tree: *mut Kdtree<Point3WithId>, point: Point3WithId)
{
    let tree = &mut *tree;
    tree.insert_node(point);
}

#[no_mangle]
pub unsafe extern "C" fn KdTreeNearest(tree: *mut Kdtree<Point3WithId>, x: f64, y: f64, z: f64) -> i32
{
    let tree = &mut *tree;
    let nearest = tree.nearest_search(&Point3WithId::new(0, x, y, z));
    return nearest.id;
}

#[no_mangle]
pub unsafe extern "C" fn KdTreeWithin(tree: *mut Kdtree<Point3WithId>, x: f64, y: f64, z: f64, radius: f64, results: *mut i32, max_results: i32) -> i32
{
    let tree = &mut *tree;
    let nearest = tree.within(&Point3WithId::new(0, x, y, z), radius, &squared_euclidean);

    let result_count = if nearest.len() < max_results as usize { nearest.len() } else { max_results as usize };

    for i in 0..result_count as usize {
        *results.offset(i as isize) = nearest[i].id;
    }

    result_count as i32
}

#[no_mangle]
pub unsafe extern "C" fn KdTreeRebuildTree(tree: *mut Kdtree<Point3WithId>, points: *mut Point3WithId, point_count: i32)
{
    if (points.is_null()) {
        panic!("points is null");
    }
    if (tree.is_null()) {
        panic!("tree is null");
    }
    if (point_count < 0) {
        panic!("point_count is negative");
    }
    if (point_count == 0) {
        panic!("point_count is zero");
    }
    if (point_count > 1000000) {
        panic!("point_count is too large");
    }

    //let points = std::slice::from_raw_parts_mut(points, point_count as usize);
    let mut vec = Vec::from_raw_parts(points, point_count as usize, point_count as usize);
    let mut points = vec.as_mut_slice();
    let tree = &mut *tree;
    tree.rebuild_tree(points);
}

*/