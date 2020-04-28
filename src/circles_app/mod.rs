mod circle;
use crate::app::status::Status;
use crate::app::App;
use crate::circles_app::circle::Circle;
use crate::col_mesh_renderer::ColMeshRenderer;
use crate::vulkan::Vulkan;
use glam::Vec2;
use raw_window_handle::HasRawWindowHandle;
use slog::Logger;
use std::rc::Rc;
use std::time::{Duration, Instant};
use winit::dpi::{PhysicalSize, Size};
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder, WindowId};

pub struct CirclesApp {
    circles: Vec<Circle>,
    field_size: Vec2,
    previous_update: Instant,
    first_update: bool,
    logger: Logger,
    vk: Rc<Vulkan>,
    col_mesh_renderer: ColMeshRenderer,
    mesh_window: Window,
    // sprite_window: Window,
}

impl CirclesApp {
    pub fn new(logger: Logger, field_size: Vec2, event_loop: &EventLoop<()>) -> Self {
        let mesh_window = Self::create_mesh_window(event_loop);
        let vk = Rc::new(Vulkan::new("Circles", logger.clone()));
        let width = mesh_window.inner_size().width;
        let height = mesh_window.inner_size().height;
        let window_size = ash::vk::Extent2D { width, height };
        let col_mesh_renderer = ColMeshRenderer::new(
            mesh_window.raw_window_handle(),
            vk.clone(),
            window_size,
            logger.clone(),
        );
        Self {
            circles: Vec::new(),
            field_size,
            previous_update: Instant::now(),
            first_update: true,
            logger,
            mesh_window,
            vk,
            col_mesh_renderer,
        }
    }

    fn create_mesh_window(event_loop: &EventLoop<()>) -> Window {
        WindowBuilder::new()
            .with_inner_size(Size::Physical(PhysicalSize::new(800, 600)))
            .build(event_loop)
            .expect("Can't create mesh window")
    }

    fn elapsed_time(&self) -> Duration {
        if self.first_update {
            trace!(self.logger, "First update");
            Duration::from_secs(0)
        } else {
            let elapse = Instant::now() - self.previous_update;
            trace!(self.logger, "Elapse time: {:?}.", elapse);
            elapse
        }
    }
}

impl App for CirclesApp {
    type Event = ();

    fn process_event(&mut self, _event: &Self::Event, _wt: &EventLoopWindowTarget<()>) -> Status {
        Status::Run
    }

    fn update(&mut self, _wt: &EventLoopWindowTarget<()>) -> Status {
        trace!(self.logger, "App update called");
        let elapsed_time = self.elapsed_time();
        self.first_update = false;
        self.previous_update = Instant::now();
        for circle in &mut self.circles {
            if circle.left() < 0f32 || circle.right() > self.field_size.x() {
                circle.reflect_x()
            }

            if circle.top() < 0f32 || circle.bot() > self.field_size.y() {
                circle.reflect_y()
            }

            circle.update(elapsed_time);
        }
        std::thread::sleep(Duration::from_millis(15));

        Status::Finish
    }

    fn draw(&mut self, _window_id: WindowId) {}
}
