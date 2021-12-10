
use crate::pb::Spec;
use image::ImageOutputFormat;

mod photon;
pub use photon::Photon;

// Engine trait：未来可以更多的engine，主流程只需要替换engine
pub trait Engine {
  // 对 engine 按照 specs进行一系列有序的处理
  fn apply(&mut self, specs: &[Spec]);
  // 从engine中生成目标图片，注意这里用的是self，而非self的引用
  fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

// SpecTransform: 未来如果添加更多的spec，只需要实现它即可
pub trait SpecTransform<T> {
  // 对图片使用op做transform
  fn transform(&mut self, op: T);
}

