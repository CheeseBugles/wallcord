use super::{image::WCImage, status::ImageStatus};
use crate::egui::ColorImage;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

pub type WCImages = Vec<Arc<Mutex<WCImage>>>;

///  A container for managing multiple image download and decode tasks. \
///  Each image is stored as an `Arc<Mutex<WCImage>>` and is processed in a separate thread.
pub struct WCImageTasks {
    pub images: WCImages,
    n_threads: usize,
    timeout: u64,
}
impl Default for WCImageTasks {
    fn default() -> Self {
        Self {
            images: Default::default(),
            n_threads: 10,
            timeout: 3,
        }
    }
}

impl WCImageTasks {
    /// Loads the given images and starts fetching them in the background. \
    /// **Note:** This should be called **only once** during initialization to avoid spawning duplicate download threads.
    pub fn load_images(&mut self, images: WCImages) {
        self.images = images;
        self.fetch_images();
    }

    /// Spawns a background thread for each image task to download and decode the image. \
    /// **Note:** This should be called **only once** during initialization to avoid spawning duplicate download threads.
    fn fetch_images(&self) {
        let timeout = self.timeout;
        let n_threads = self.n_threads;
        let images_clone: Vec<_> = self.images.iter().cloned().collect();

        std::thread::spawn(move || {
            let pool = ThreadPool::new(n_threads); // 💡 create inside the thread
            for image in images_clone {
                pool.execute(move || {
                    let url = {
                        let image_guard = image.lock().unwrap();
                        image_guard.src.clone()
                    };
                    let client = reqwest::blocking::Client::builder()
                        .timeout(std::time::Duration::from_secs(timeout))
                        .build()
                        .unwrap();

                    let resp = match client.get(&url).send() {
                        Ok(c) => c,
                        Err(_) => {
                            image.lock().unwrap().status = ImageStatus::Faild;
                            return;
                        }
                    };

                    let bytes = match resp.bytes() {
                        Ok(f) => f,
                        Err(_) => {
                            image.lock().unwrap().status = ImageStatus::Faild;
                            return;
                        }
                    };
                    let format = match image::guess_format(&bytes) {
                        Ok(f) => f,
                        Err(_) => {
                            image.lock().unwrap().status = ImageStatus::Faild;
                            return;
                        }
                    };
                    let image_buf = match image::load_from_memory_with_format(&bytes, format) {
                        Ok(img) => img.to_rgba8(),
                        Err(_) => {
                            image.lock().unwrap().status = ImageStatus::Faild;
                            return;
                        }
                    };

                    let size = [image_buf.width() as usize, image_buf.height() as usize];
                    let pixels = image_buf.into_vec();
                    let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);

                    let mut task_guard = image.lock().unwrap();
                    task_guard.raw_image = Some(color_image);
                    task_guard.status = ImageStatus::Done;
                });
            }

            pool.join();
        });
    }
}
