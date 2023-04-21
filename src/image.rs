
use tract_ndarray::Array;
use tract_onnx::prelude::*;
use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    str::FromStr,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct ImageNetLabels(Vec<String>);

pub(crate) fn label(image_path:String) -> TractResult<(String)> {
    let model_dir = PathBuf::from_str("./onnx")?;

    let model = tract_onnx::onnx()
        .model_for_path(Path::join(&model_dir, "resnet.onnx"))?
        .with_input_fact(0, f32::fact([1, 3, 224, 224]).into())?
        .into_optimized()?
        .into_runnable()?;

    let mean = Array::from_shape_vec((1, 3, 1, 1), vec![0.485, 0.456, 0.406])?;
    let std = Array::from_shape_vec((1, 3, 1, 1), vec![0.229, 0.224, 0.225])?;

    let img = image::open(image_path).unwrap().to_rgb8();
    let resized = image::imageops::resize(&img, 224, 224, ::image::imageops::FilterType::Triangle);
    let image: Tensor =
        ((tract_ndarray::Array4::from_shape_fn((1, 3, 224, 224), |(_, c, y, x)| {
            resized[(x as _, y as _)][c] as f32 / 255.0
        }) - mean)
            / std)
            .into();

    let result = model.run(tvec!(image.into()))?;

    let best = result[0]
        .to_array_view::<f32>()?
        .iter()
        .cloned()
        .zip(1..)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let file = File::open(Path::join(&model_dir, "imagenet-simple-labels.json")).unwrap();
    let reader = BufReader::new(file);
    let labels: ImageNetLabels = serde_json::from_reader(reader).unwrap();
    let index_to_name: HashMap<usize, String> = labels.0.into_iter().enumerate().collect();
    if let Some((value, index)) = best {
        if let Some(class_name) = index_to_name.get(&(index - 1)) {
            // println!("result: Some(({:.6}, {} -> {}))", value, index, class_name);
            Ok(class_name.clone())
        } else {
            // println!("result: Some(({:.6}, {}))", value, index);
            Ok(format!("Class name not found for index: {}", index).into())
        }
    } else {
        Ok("No result".to_string())

        // println!("result: None");
    }
}
