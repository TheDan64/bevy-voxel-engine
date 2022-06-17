use bevy::prelude::*;
use crossbeam_channel::Receiver;

use super::{pos::PosComponent, static_mesh::vertex::Vertex, ChunkComponent};

#[derive(Component)]
pub struct ComputeChunkGeneration(pub Receiver<(PosComponent, Box<ChunkComponent>, Vec<Vertex>)>);