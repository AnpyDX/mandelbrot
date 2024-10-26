use std::sync::{ Arc, Mutex };

use image::{ self, ImageError, ExtendedColorType::Rgb8 as R8G8A8 };
use num::Complex;

use super::task::Manager as TaskManager;


// Rnnderer's Target
pub struct RenderTarget {
    buffer: Vec<u8>,
    size: (u32, u32) // size: (width, height)
}

impl RenderTarget {
    pub fn new(size: (u32, u32)) -> RenderTarget {
        RenderTarget {
            buffer: vec![0u8; size.0 as usize * size.1 as usize * 3],
            size: size
        }
    }

    // store target into a png file
    pub fn store(&self, name: &str) -> Result<(), ImageError> {
        image::save_buffer(name, &self.buffer, self.size.0, self.size.1, R8G8A8)
    }
}

// Mandelbrot Renderer
pub struct Renderer;

impl Renderer {
    // TODO add thread number argument
    pub fn render(thread_num: usize, target: &mut RenderTarget, bounds: &(Complex<f64>, Complex<f64>)) {
        let mut task_mgr = TaskManager::new(thread_num);
        let size = target.size;
        let target_ref = Arc::new(Mutex::new(target));

        for y in 0..size.1 {

            let target = Arc::clone(&target_ref);

            task_mgr.submit(move || {
                for x in 0..size.0 {
                    let index = (y * size.0 + x) as usize * 3;
                    let clx = Self::pixel_to_complex(&(x, y), &size, bounds);
                    let mut lightness = 0u8;
                    match Self::escape_time(clx, 255) {
                        Some(time) => lightness = (255 - time).try_into().unwrap(),
                        None => ()
                    }

                    let mut target = target.lock().unwrap();
                    target.buffer[index] = lightness;
                    target.buffer[index + 1] = lightness;
                    target.buffer[index + 2] = lightness;
                }
            });
        }

        task_mgr.launch();
    }

    /* Internal Algorithm */

    /// caculate whether clx is inside mandelbrot set
    ///
    /// if clx is inside, return None, else return Some(loop_times)
    fn escape_time(clx: Complex<f64>, limit: u32) -> Option<u32>{
        let mut z = Complex { re: 0.0, im: 0.0 };
        for i in 0..limit {
            z = z * z + clx;

            if z.norm_sqr() > 4.0 {
                return Some(i);
            }
        }

        return None;
    }

    /// map a pixel to complex
    fn pixel_to_complex(
        pixel: &(u32, u32), 
        size: &(u32, u32), 
        bounds: &(Complex<f64>, Complex<f64>)) -> Complex<f64> {
        Complex {
            re: (bounds.1.re - bounds.0.re) * pixel.0 as f64 / (size.0 - 1) as f64,
            im: (bounds.0.im - bounds.1.im) * pixel.1 as f64 / (size.1 - 1) as f64
        }
    }
}