use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use futures::StreamExt;
use image::GenericImageView;
use serde_json::Value;
use tch::{nn, vision, Device, Kind, Tensor};

fn normalize_image(image: Tensor, mean: &[f32], std: &[f32]) -> Tensor {
    let mean = Tensor::of_slice(mean).view([3, 1, 1]);
    let std = Tensor::of_slice(std).view([3, 1, 1]);
    ((image - &mean) / &std).to_kind(Kind::Float)
}

pub(crate) fn label_transfer(image_path: String) -> tch::Result<(String)> {
    let model_dir = PathBuf::from_str("./transfer_learning")?;
    let model_path = Path::join(&model_dir, "best_model.pt");

    // let model_path = Path::new("./model/best_model.pt");

    let mut vs = nn::VarStore::new(Device::Cpu);
    // You should replace the number 1000 with the number of classes in your dataset
    let resnet18 = vision::resnet::resnet18(&vs.root(), 3);
    vs.load(model_path)?;

    let mean = [0.485, 0.456, 0.406];
    let std = [0.229, 0.224, 0.225];

    let img = image::open(image_path)?.to_rgb8();


    let resized = image::imageops::resize(&img, 224, 224, image::imageops::FilterType::Triangle);
    let image: Tensor = tch::vision::image::image_to_tensor(&resized).unsqueeze(0);

    let normalized_image = normalize_image(image, &mean, &std);

    let output = resnet18
        .forward(&normalized_image)
        .softmax(-1, Kind::Float)?;

    let best_index = output.argmax(-1, false).int64_value(&[]) as usize;

    let file = File::open(Path::join(&model_dir, "transfer_learning.json")).unwrap();
    let reader = BufReader::new(file);
    let labels: Value = serde_json::from_reader(reader)?;
    let index_to_name: HashMap<usize, String> = labels
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, v)| (i, v.as_str().unwrap().to_owned()))
        .collect();

    if let Some(class_name) = index_to_name.get(&best_index) {
        Ok(class_name.clone())
    } else {
        Ok(format!("Class name not found for index: {}", best_index).into())
    }
}
