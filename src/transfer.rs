use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use futures::StreamExt;
use image::GenericImageView;
use serde_json::Value;
use tch::{nn, vision, Device, Kind, Tensor};
use tch::nn::ModuleT;
use tch::vision::{
    alexnet, convmixer, densenet, efficientnet, imagenet, inception, mobilenet, resnet, squeezenet,
    vgg,
};
use tokenizers::tokenizer::{Result, Tokenizer};

fn normalize_image(image: Tensor, mean: &[f32], std: &[f32]) -> Tensor {
    let mean = Tensor::of_slice(mean).view([3, 1, 1]);
    let std = Tensor::of_slice(std).view([3, 1, 1]);
    ((image - &mean) / &std).to_kind(Kind::Float)
}

pub(crate) fn label_transfer(image_path: String) -> Result<String> {

    let model_dir = PathBuf::from_str("./transfer_learning")?;
    let device = Device::cuda_if_available();

    // let model_path = Path::join(&model_dir, "resnet18.ot");
    let model_path = Path::join(&model_dir, "best_model_scripted.pt");
    let mut vs = nn::VarStore::new(Device::Cpu);

    let image = imagenet::load_image_and_resize224(image_path)?;

    let model = tch::CModule::load(model_path)?;
    // let output = model.forward_ts(&[image.unsqueeze(0)])?.softmax(-1, Kind::Float);
    let output = image.unsqueeze(0).apply(&model).softmax(-1, Kind::Float);
    println!("{}", output);
    let best_index = output.argmax(-1, false).int64_value(&[]) as usize;
    println!("best_index: {}", best_index);


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
        println!("{}",class_name.clone());
        Ok(class_name.clone())
    } else {
        Ok(format!("Class name not found for index: {}", best_index).into())
    }
}
