use std::cell::RefCell;
use std::fmt::Debug;
use std::ptr::slice_from_raw_parts_mut;
use std::rc::Rc;
use std::simd::f32x4;
use glam::Vec4;
pub use ozz_animation_rs::*;
use crate::bytebuffer::ByteBuffer;

#[repr(C)]
pub struct Float3Key {
    pub ratio: f32,
    pub track: u16,
    pub value: [u16; 3],
}
#[repr(C)]
pub struct QuaternionKey {
    pub ratio: f32,
    // track: 13 => The track this key frame belongs to.
    // largest: 2 => The largest component of the quaternion.
    // sign: 1 => The sign of the largest component. 1 for negative.
    bit_field: u16,
    value: [i16; 3], // The quantized value of the 3 smallest components.
}

struct Vec3f {
    x: f32,
    y: f32,
    z: f32
}


struct Quatf {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

#[repr(C)]
pub struct RustUnsafeArrayFloat3Key {
    pub data: *mut ozz_animation_rs::animation::Float3Key,
    pub count: i32,
    pub allocator: i32
}

#[repr(C)]
pub struct RustUnsafeArrayQuaternionKey {
    pub data: *mut ozz_animation_rs::animation::QuaternionKey,
    pub count: i32,
    pub allocator: i32
}

#[repr(C)]
pub struct PrimroseAnimationInput {
    pub channel_count: i32,
    pub duration: f32,
    pub translations: RustUnsafeArrayFloat3Key,
    pub rotations: RustUnsafeArrayQuaternionKey,
    pub scales: RustUnsafeArrayFloat3Key,
}

#[repr(C)]
pub struct SoaVec3 {
    pub x: std::simd::f32x4,
    pub y: std::simd::f32x4,
    pub z: std::simd::f32x4,
}

#[repr(C)]
pub struct SoaQuat {
    pub x: std::simd::f32x4,
    pub y: std::simd::f32x4,
    pub z: std::simd::f32x4,
    pub w: std::simd::f32x4,
}

#[repr(C)]
pub struct SoaTransform {
    pub translation: SoaVec3,
    pub rotation: SoaQuat,
    pub scale: SoaVec3,
}


// Skeleton
#[repr(C)]
pub struct PrimroseSkeletonInput {
    pub joint_count: i32,
    pub joint_names: ByteBuffer,
    pub joint_parents: *mut i16,
    pub bind_poses: *mut ozz_animation_rs::SoaTransform,
}

#[no_mangle]
pub extern "C" fn ozz_skeleton_load_primrose(input: *mut PrimroseSkeletonInput) -> Skeleton
{
    // empty list
    let joint_names: JointHashMap = JointHashMap::default();

    let skeleton = ozz_animation_rs::Skeleton {
        joint_names,
        joint_rest_poses: unsafe { Vec::from_raw_parts((*input).bind_poses, (*input).joint_count as usize, (*input).joint_count as usize) },
        joint_parents: unsafe { Vec::from_raw_parts((*input).joint_parents, (*input).joint_count as usize, (*input).joint_count as usize) }
    };

    Skeleton { ptr: Box::into_raw(Box::new(skeleton)) }
}

#[no_mangle]
pub extern "C" fn ozz_skeleton_free(skeleton: Skeleton) {
    unsafe {
        let _ = Box::from_raw(skeleton.ptr);
    }
}

#[no_mangle]
pub extern "C" fn ozz_skeleton_num_joints(skeleton: Skeleton) -> i32 {
    unsafe {
        (*skeleton.ptr).num_joints() as i32
    }
}

#[no_mangle]
pub extern "C" fn ozz_skeleton_num_soa_joints(skeleton: Skeleton) -> i32 {
    unsafe {
        (*skeleton.ptr).num_soa_joints() as i32
    }
}


// Animation

#[no_mangle]
pub unsafe extern "C" fn ozz_animation_load_primrose(
    input: *const PrimroseAnimationInput,
) -> Animation
{
    let translations = Vec::from_raw_parts((*input).translations.data, (*input).translations.count as usize, (*input).translations.count as usize);
    let rotations = Vec::from_raw_parts((*input).rotations.data, (*input).rotations.count as usize, (*input).rotations.count as usize);
    let scales = Vec::from_raw_parts((*input).scales.data, (*input).scales.count as usize, (*input).scales.count as usize);

    let animation = ozz_animation_rs::Animation {
        duration: (*input).duration,
        num_tracks: (*input).channel_count as usize,
        translations,
        rotations,
        scales,
        name: "primrose".to_string()
    };

    Animation {
        ptr: Box::into_raw(Box::new(animation))
    }
}

#[no_mangle]
pub unsafe extern "C" fn ozz_animation_free(animation: Animation) {
    let _ = Box::from_raw(animation.ptr);
}

#[no_mangle]
pub extern "C" fn ozz_animation_num_tracks(animation: Animation) -> i32 {
    unsafe {
        (*animation.ptr).num_tracks() as i32
    }
}

#[no_mangle]
pub extern "C" fn ozz_animation_num_soa_tracks(animation: Animation) -> i32 {
    unsafe {
        (*animation.ptr).num_soa_tracks() as i32
    }
}

#[no_mangle]
pub extern "C" fn ozz_animation_duration(animation: Animation) -> f32 {
    unsafe {
        (*animation.ptr).duration() as f32
    }
}


// Sampling Job
#[no_mangle]
pub extern "C" fn ozz_samplingcontext_new(max_tracks: i32) -> SamplingContext {
    unsafe {
        SamplingContext { ptr: Box::into_raw(Box::new(ozz_animation_rs::SamplingContext::new(max_tracks as usize))) }
    }
}

#[no_mangle]
pub extern "C" fn ozz_samplingcontext_free(context: SamplingContext) {
    unsafe {
        let _ = Box::from_raw(context.ptr);
    }
}

#[no_mangle]
pub extern "C" fn ozz_samplingjob_set_context(sampling_job: SamplingJob, context: SamplingContext) {
    unsafe {
        (*sampling_job.ptr).set_context((*context.ptr).clone());
    }
}

#[no_mangle]
pub extern "C" fn ozz_samplingjob_set_animation(sampling_job: SamplingJob, animation: Animation) {
    unsafe {
        (*sampling_job.ptr).set_animation(animation.ptr);
    }
}

#[no_mangle]
pub extern "C" fn ozz_samplingjob_set_output(sampling_job: SamplingJob, output: SoaTransformBuffer) {
    unsafe {
        (*sampling_job.ptr).set_output(output);
    }
}

#[no_mangle]
pub extern "C" fn ozz_samplingjob_set_ratio(sampling_job: SamplingJob, ratio: f32) {
    unsafe {
        (*sampling_job.ptr).set_ratio(ratio);
    }
}

#[no_mangle]
pub extern "C" fn ozz_samplingjob_run(sampling_job: SamplingJob) -> bool {
    unsafe { (*sampling_job.ptr).run().is_ok() }
}

#[no_mangle]
pub extern "C" fn ozz_samplingcontext_max_tracks(context: SamplingContext) -> i32 {
    unsafe {
        (*context.ptr).max_tracks() as i32
    }
}


#[repr(C)]
#[derive(Clone)]
pub struct SoaTransformBuffer {
    pub data: *mut ozz_animation_rs::SoaTransform,
    pub count: i32,
    pub allocator: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct Vec4fBuffer {
    pub data: *mut f32x4,
    pub count: i32,
    pub allocator: i32,
}


impl OzzBuf<ozz_animation_rs::SoaTransform> for SoaTransformBuffer {
    type Buf<'t> where Self: 't = &'t [ozz_animation_rs::SoaTransform];

    fn buf(&self) -> Result<Self::Buf<'_>, OzzError> {
        if self.data.is_null() {
            return Err(OzzError::InvalidIndex);
        }

        let slice = unsafe {
            std::slice::from_raw_parts(self.data, self.count as usize)
        };

        Ok(slice)
    }
}

impl OzzMutBuf<ozz_animation_rs::SoaTransform> for SoaTransformBuffer {
    type MutBuf<'t> where Self: 't = &'t mut [ozz_animation_rs::SoaTransform];

    fn mut_buf(&mut self) -> Result<Self::MutBuf<'_>, OzzError> {
        if self.data.is_null() {
            return Err(OzzError::InvalidIndex);
        }

        let slice = unsafe {
            std::slice::from_raw_parts_mut(self.data, self.count as usize)
        };

        Ok(slice)
    }
}

// Blending Job
/*
#[no_mangle]
extern "C" fn ozz_blendingjob_new(
    context: BlendingContext,
    skeleton: Skeleton,
    threshold: f32,
    output: SoaTransformBuffer
) -> BlendingJob {

    unsafe {
    let mut blending_job: ozz_animation_rs::BlendingJob<*const ozz_animation_rs::Skeleton, SoaTransformBuffer, SoaTransformBuffer> = ozz_animation_rs::BlendingJob::default();

    blending_job.set_skeleton(skeleton.ptr);
    blending_job.set_context((*context.ptr).clone());
    blending_job.set_threshold(threshold);
    blending_job.set_output(output);

        BlendingJob { ptr: Box::into_raw(Box::new(blending_job)) }
    }
}
*/
#[no_mangle]
extern "C" fn ozz_blendingjob_new() -> BlendingJob {

    unsafe {
        let mut blending_job: ozz_animation_rs::BlendingJob<*const ozz_animation_rs::Skeleton, SoaTransformBuffer, SoaTransformBuffer> = ozz_animation_rs::BlendingJob::default();
        BlendingJob { ptr: Box::into_raw(Box::new(blending_job)) }
    }
}

#[no_mangle]
pub extern "C" fn ozz_blendingjob_free(blending_job: BlendingJob) {
    unsafe {
        let _ = Box::from_raw(blending_job.ptr);
    }
}

#[no_mangle]
pub extern "C" fn ozz_blendingjob_run(blending_job: BlendingJob) {
    unsafe { (*blending_job.ptr).run().unwrap(); }
}

#[no_mangle]
pub extern "C" fn ozz_blendingjob_validate(blending_job: BlendingJob) -> bool {
    unsafe { (*blending_job.ptr).validate() }
}

#[no_mangle]
pub extern "C" fn ozz_blendingjob_set_skeleton(blending_job: BlendingJob, skeleton: Skeleton) {
    unsafe { (*blending_job.ptr).set_skeleton(skeleton.ptr) }
}

#[no_mangle]
pub extern "C" fn ozz_blendingjob_set_layer_count(blending_job: BlendingJob, layer_count: i32) {
    unsafe {
        (*blending_job.ptr).layers_mut().set_len(layer_count as usize);
    }
}

#[no_mangle]
pub extern "C" fn ozz_blendingjob_set_layer(
    blending_job: BlendingJob,
    layer_index: i32,
    layer: BlendingLayer
) {
    unsafe {
        (*blending_job.ptr).layers_mut()[layer_index as usize] = (*layer.ptr).clone();
    }
}

#[no_mangle]
pub extern "C" fn ozz_blendingjob_set_additive_layer_count(blending_job: BlendingJob, layer_count: i32) {
    unsafe {
        (*blending_job.ptr).layers_mut().set_len(layer_count as usize);
    }
}

#[no_mangle]
pub extern "C" fn ozz_blendingjob_set_additive_layer(blending_job: BlendingJob, layer_index: i32, layer: BlendingLayer) {
    unsafe {
        (*blending_job.ptr).layers_mut()[layer_index as usize] = (*layer.ptr).clone();
    }
}

#[no_mangle]
pub extern "C" fn ozz_blendingjob_set_threshold(blending_job: BlendingJob, threshold: f32) {
    unsafe {
        (*blending_job.ptr).set_threshold(threshold);
    }
}

#[no_mangle]
pub extern "C" fn ozz_blendingjob_set_output(blending_job: BlendingJob, output: SoaTransformBuffer) {
    unsafe {
        (*blending_job.ptr).set_output(output);
    }
}

#[repr(C)]
pub struct Animation {
    pub ptr: *mut ozz_animation_rs::Animation
}

#[repr(C)]
pub struct Skeleton {
    pub ptr: *mut ozz_animation_rs::Skeleton
}

#[repr(C)]
pub struct BlendingContext {
    pub ptr: *mut ozz_animation_rs::BlendingContext
}

#[repr(C)]
pub struct SamplingContext {
    pub ptr: *mut ozz_animation_rs::SamplingContext
}

#[repr(C)]
pub struct BlendingJob {
    pub ptr: *mut ozz_animation_rs::BlendingJob<*const ozz_animation_rs::Skeleton, SoaTransformBuffer, SoaTransformBuffer>
}

#[repr(C)]
pub struct SamplingJob {
    pub ptr: *mut ozz_animation_rs::SamplingJob<*const ozz_animation_rs::Animation, SoaTransformBuffer>
}

#[repr(C)]
pub struct BlendingLayer {
    pub ptr: *mut ozz_animation_rs::BlendingLayer<SoaTransformBuffer>
}

#[no_mangle]
extern "C" fn ozz_blendinglayer_new_with_weight(transforms: SoaTransformBuffer, weight: f32) -> BlendingLayer {

    unsafe {
        let mut blending_job: ozz_animation_rs::BlendingLayer<SoaTransformBuffer> = ozz_animation_rs::BlendingLayer::with_weight(transforms, weight);
        BlendingLayer { ptr: Box::into_raw(Box::new(blending_job)) }
    }
}

#[no_mangle]
extern "C" fn ozz_blendinglayer_new_with_joint_weights(transforms: SoaTransformBuffer, joint_weights: Vec4fBuffer) -> BlendingLayer {

    unsafe {
        let joint_weights = Vec::from_raw_parts(joint_weights.data as *mut glam::Vec4, joint_weights.count as usize, joint_weights.count as usize);
        let mut blending_job: ozz_animation_rs::BlendingLayer<SoaTransformBuffer> = ozz_animation_rs::BlendingLayer::with_joint_weights(transforms, joint_weights);
        BlendingLayer { ptr: Box::into_raw(Box::new(blending_job)) }
    }
}

#[no_mangle]
pub extern "C" fn ozz_blendinglayer_free(blending_layer: BlendingLayer) {
    unsafe {
        let _ = Box::from_raw(blending_layer.ptr);
    }
}

#[no_mangle]
pub extern "C" fn ozz_blendinglayer_set_additive(blending_layer: BlendingLayer, buffer: SoaTransformBuffer) {
    unsafe {
        (*blending_layer.ptr).transform = buffer;
    }
}

