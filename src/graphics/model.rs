// #![allow(
//     dead_code,
//     unused_variables,
//     clippy::manual_slice_size_calculation,
//     clippy::too_many_arguments,
//     clippy::unnecessary_wraps
// )]

// use anyhow::{Ok, Result};
// use cgmath::{vec2, vec3};
// use std::collections::HashMap;
// use std::fs::File;
// use std::io::BufReader;

// use super::Vertex;

// pub struct Model {
//     data: ModelData,
// }

// impl Model {
//     pub unsafe fn default() -> Result<Self> {
//         Ok(Self {
//             data: ModelData::default()
//         })
//     }

//     /// Creates our the app.
//     pub unsafe fn load(path: &str) -> Result<Model> {
//         // create data
//         let mut data = ModelData::default();

//         // create the reader to use
//         let mut reader = BufReader::new(File::open(path)?);

//         // load the object model
//         let (models, _) = tobj::load_obj_buf(
//             &mut reader,
//             &tobj::LoadOptions {
//                 triangulate: true,
//                 ..Default::default()
//             },
//             |_| Ok(Default::default()),
//         )?;

//         // get vertices
//         let mut unique_vertices = HashMap::new();

//         // loop over models and each indices
//         for model in &models {
//             for index in &model.mesh.indices {
//                 // calulate offsets
//                 let position_offset = (3 * index) as usize;
//                 let texel_offset = (2 * index) as usize;

//                 // create vertex
//                 let vertex = Vertex {
//                     position: vec3(
//                         model.mesh.positions[position_offset],
//                         model.mesh.positions[position_offset + 1],
//                         model.mesh.positions[position_offset + 2],
//                     ),
//                     texel: vec2(
//                         model.mesh.texcoords[texel_offset],
//                         1.0 - model.mesh.texcoords[texel_offset + 1],
//                     ),
//                     color: vec3(1.0, 1.0, 1.0),
//                 };

//                 // push the indieces
//                 if let Some(index) = unique_vertices.get(&vertex) {
//                     data.indices.push(*index as u32);
//                 } else {
//                     let index = data.vertices.len();
//                     unique_vertices.insert(vertex, index);
//                     data.vertices.push(vertex);
//                     data.indices.push(index as u32);
//                 }
//             }
//         }
//         Ok(Model::default()?)
//     }
// }

// /// The Vulkan handles and associated properties used by the app.
// #[derive(Clone, Debug, Default)]
// struct ModelData {
//     // model
//     vertices: Vec<Vertex>,
//     indices: Vec<u32>,
// }
