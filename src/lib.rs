use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb888,
    geometry::{OriginDimensions, Size},
    draw_target::DrawTarget,
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    platform::run_return::EventLoopExtRunReturn,
    window::{Window, WindowBuilder},
    dpi::LogicalSize,
};

use pixels::{
    Pixels,
    SurfaceTexture,
    wgpu::Color,
};

use pyo3::prelude::*;

use std::sync::Arc;

const BYTES_PER_PIXEL: usize = 4;

#[derive(Debug, PartialEq, Copy, Clone)]
#[pyclass]
pub enum Orientation {
    PORTRAIT,
    LANDSCAPE,
}

#[pyclass]
pub struct PixelsDisplay {
    // window: Window,
    // el: Arc<EventLoop<()>>,
    // underlying Framebuffer struct
    pixels: Pixels,
    // pixel width and height of screen
    width: u32,
    height: u32,
    // orientation of device
    orientation: Orientation,
}

#[pymethods]
impl PixelsDisplay {
    #[new]
    pub fn new(width: u32, height: u32) -> PixelsDisplay {

        let el = EventLoop::new();
        println!("created event loop");

        let window: Window = WindowBuilder::new()
            .with_title("PixelsDisplay")
            .with_inner_size(LogicalSize::new(width, height))
            .build(&el)
            .unwrap();
        println!("fb width: {:?}", width);
        println!("fb height: {:?}", height);

        let st = SurfaceTexture::new(width, height, &window);
        println!("created surface texture");

        el.run(move |event, _, control_flow| {
            control_flow.set_poll();
            control_flow.set_wait();

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => control_flow.set_exit(),
                Event::WindowEvent {
                    event: WindowEvent::MouseInput {..},
                    window_id,
                } => {
                    //log::debug!("{:?}", event);
                    println!("{:?}", event);
                },
                Event::MainEventsCleared => {
                    window.request_redraw();
                },
                Event::RedrawRequested(_) => {
                    //canvas.
                },
                _ => (),
            }
        }); // event loop

        Self {
            // window: window,
            // el: Arc::new(el),
            pixels: Pixels::new(width, height, st).unwrap(),
            width: width,
            height: height,
            // start in PORTRAIT
            orientation: Orientation::PORTRAIT,
        }
    }

    // pub fn run_event_loop(&mut self) {
    //     self.el.take().unwrap().run_return(move |event, _, control_flow| {
    //         control_flow.set_poll();
    //         control_flow.set_wait();
    //     });
    // }

    pub fn get_orientation(&self) -> Orientation {
        return self.orientation;
    }

    pub fn set_orientation(&mut self, o: Orientation) {
        if o != self.orientation {
            let (w, h) = (self.width, self.height);
            self.width = h;
            self.height = w;
            self.orientation = o;
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        return (self.width, self.height);
    }

    /*** TODO ***/
    //pub fn clear(&mut self) {
    //    let (_prefix, pixels, _suffix) = unsafe { self.fb.frame.align_to_mut::<u32>() };
    //    for i in 0..pixels.len() {
    //        pixels[i] = 0u32;
    //    }
    //}

    //pub fn fill(&mut self, color: u32) {
    //    let (_prefix, pixels, _suffix) = unsafe { self.fb.frame.align_to_mut::<u32>() };
    //    for idx in 0..pixels.len() {
    //        pixels[idx] = color;
    //    }
    //}

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        // Get both of the pixel buffers as 4 byte slices
        let (_fb_prefix, fb_pixels, _fb_suffix) = unsafe { self.pixels.get_frame_mut().align_to_mut::<u32>() };
        let (_circpy_prefix, circpy_pixels, _circpy_suffic) = unsafe {
            bytes.align_to::<u32>()
        };
        // For now we only support drawing to the whole screen
        assert!(circpy_pixels.len() == (self.width*self.height) as usize, "TODO: support sub rectangles");
        // Loop through the screen coordinates, and draw the pixels
        for x in 0..self.width-1 {
            for y in 0..self.height-1 {
                let circpy_idx = (x + y*self.width) as usize;
                // Get the index into the framebuffer (accounting for hidden indexes)
                let fb_idx: usize = match self.orientation {
                    Orientation::PORTRAIT => {
                        (x + y*(BYTES_PER_PIXEL as u32)) as usize
                    },
                    Orientation::LANDSCAPE => {
                        ((self.height - 1 - y) + x*(BYTES_PER_PIXEL as u32)) as usize
                    },
                };
                fb_pixels[fb_idx] = circpy_pixels[circpy_idx];
            }            
        }
    }
    //
    //pub fn set_pixel(&mut self, idx: usize, color: u32) {
    //    let (_prefix, pixels, _suffix) = unsafe { self.fb.frame.align_to_mut::<u32>() };
    //    pixels[idx] = color;
    //}

}


//impl PixelsDisplay {
//    
//    pub fn set_idx(&mut self, idx: usize, color: Bgr888) {
//        let (_prefix, pixels, _suffix) = unsafe { self.fb.frame.align_to_mut::<u32>() };
//        pixels[idx] = (color.into_storage()) as u32;
//    }
//
//}


#[pymodule]
fn displayio_pixels(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PixelsDisplay>()?;
    m.add_class::<Orientation>()?;
    Ok(())
}

impl DrawTarget for PixelsDisplay {
    type Color = Rgb888;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let mut pxs = self.pixels.get_frame_mut();
        for Pixel(coord, color) in pixels.into_iter() {
            let (x, y) = (coord.x as u32, coord.y as u32);
            // constrain pixels to screen area
            if x > self.width-1 || y > self.height-1 {
                continue;
            }
            let idx: usize = ((x as usize)*BYTES_PER_PIXEL) + (y as usize)*((self.width as usize)*BYTES_PER_PIXEL); 
            let px: [u8; 4] = [color.r(), color.g(), color.b(), 255];
            pxs[idx..(idx+4)].copy_from_slice(&px);
        }

        return Ok(());
    }

}

impl OriginDimensions for PixelsDisplay {
    fn size(&self) -> Size {
        return Size::new(self.width, self.height);
    }
}
