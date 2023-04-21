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
    let model_path = Path::join(&model_dir, "resnet18.ot");
    let mut vs = tch::nn::VarStore::new(tch::Device::Cpu);

    // Then the model is built on this variable store, and the weights are loaded.
    let resnet18 = tch::vision::resnet::resnet18(&vs.root(), imagenet::CLASS_COUNT);
    vs.load(model_path)?;
    let image = imagenet::load_image_and_resize224(image_path)?;

    // Apply the forward pass of the model to get the logits and convert them
    // to probabilities via a softmax.
    let output = resnet18
        .forward_t(&image.unsqueeze(0), /*train=*/ false)
        .softmax(-1, Kind::Float);

    // Finally print the top 5 categories and their associated probabilities.
    for (probability, class) in imagenet::top(&output, 5).iter() {
        println!("{:50} {:5.2}%", class, 100.0 * probability)
    }

    Ok("test".to_string())
    // let model_dir = PathBuf::from_str("./transfer_learning")?;
    // let model_path = Path::join(&model_dir, "best_model.pt");
    //
    // // let model_path = Path::new("./model/best_model.pt");
    //
    // let mut vs = nn::VarStore::new(Device::Cpu);
    // // You should replace the number 1000 with the number of classes in your dataset
    // let resnet18 = vision::resnet::resnet18(&vs.root(), 3);
    // vs.load(model_path)?;
    //
    // let mean = [0.485, 0.456, 0.406];
    // let std = [0.229, 0.224, 0.225];
    //
    // // let img = image::open(image_path)?.to_rgb8();
    //
    //
    // // let resized = image::imageops::resize(&img, 224, 224, image::imageops::FilterType::Triangle);
    // let resized = imagenet::load_image_and_resize224(image_path)?;
    //
    // let image: Tensor = resized.unsqueeze(0);
    //
    // let normalized_image = normalize_image(image, &mean, &std);
    //
    // let output = resnet18
    //     .forward_t(&normalized_image,false)
    //     .softmax(-1, Kind::Float);
    //
    // // let output =
    // //     resnet18.forward_t(&image.unsqueeze(0), /* train= */ false).softmax(-1, tch::Kind::Float); // Convert to probability.
    // for (probability, class) in imagenet::top(&output, 2).iter() {
    //     println!("{:50} {:5.2}%", class, 100.0 * probability)
    // }
    // Ok("test".to_string())

    // let best_index = output.argmax(-1, false).int64_value(&[]) as usize;
    // // println!("best_index: {}", best_index);
    //
    // let file = File::open(Path::join(&model_dir, "transfer_learning.json")).unwrap();
    // let reader = BufReader::new(file);
    // let labels: Value = serde_json::from_reader(reader)?;
    // let index_to_name: HashMap<usize, String> = labels
    //     .as_array()
    //     .unwrap()
    //     .iter()
    //     .enumerate()
    //     .map(|(i, v)| (i, v.as_str().unwrap().to_owned()))
    //     .collect();
    //
    // if let Some(class_name) = index_to_name.get(&best_index) {
    //     Ok(class_name.clone())
    // } else {
    //     Ok(format!("Class name not found for index: {}", best_index).into())
    // }
}
