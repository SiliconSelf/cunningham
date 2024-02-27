//! Helper tools for OCR functionality

use rten_imageio::read_image;
use rten_tensor::NdTensor;

/// A source that can provide an image to the OCR system
pub(crate) trait ImageSource {
    /// Load the image into a 3D tensor
    fn load(&self) -> NdTensor<f32, 3>;
}

/// An image stored as a file
pub(crate) struct LocalImage {
    /// The path the image is stored at
    pub(crate) path: &'static str,
}

impl ImageSource for LocalImage {
    fn load(&self) -> NdTensor<f32, 3> {
        read_image(self.path).expect("Failed to load provided image path")
    }
}
