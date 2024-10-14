
/*
let gltf = Gltf::open("examples/Box.gltf")?;
for scene in gltf.scenes() {
    for node in scene.nodes() {
        println!(
            "Node #{} has {} children",
            node.index(),
            node.children().count(),
        );
    }
}
 */

use crate::bytebuffer::ByteBuffer;
use crate::pitchtracker::{Tone, Tone_Tone};
