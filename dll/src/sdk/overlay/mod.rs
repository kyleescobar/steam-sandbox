use std::sync::RwLock;
use egui_backend::egui::{Context, Window};
use egui_backend::egui::mutex::Mutex;
use egui_backend::{GfxBackend, UserApp, WindowBackend};
use egui_render_wgpu::WgpuBackend;
use egui_window_glfw_passthrough::GlfwWindow;
use once_cell::sync::OnceCell;
use crate::sdk::{Global, SDK};

pub struct OverlayComponent {
    renderer: Box<dyn FnMut(&Context) + Send>
}
unsafe impl Sync for OverlayComponent {}

struct OverlayState {
    hidden: bool
}

pub struct Overlay {
    panels: RwLock<Vec<OverlayComponent>>,
    state: Mutex<OverlayState>,
}

impl Global for Overlay {
    fn cell() -> &'static OnceCell<Self> {
        static INSTANCE: OnceCell<Overlay> = OnceCell::new();
        &INSTANCE
    }

    fn create() -> Self {
        let overlay = Overlay {
            panels: RwLock::new(vec![]),
            state: Mutex::new(OverlayState {
                hidden: true
            })
        };
        overlay.start();
        overlay
    }
}

impl UserApp<GlfwWindow, WgpuBackend> for Overlay {
    fn run(&mut self, ctx: &Context, window_backend: &mut GlfwWindow, gl_backend: &mut WgpuBackend) {
        Window::new("Hello World").show(ctx, |ui| {

        });

        if ctx.wants_keyboard_input() || ctx.wants_pointer_input() {
            window_backend.window.set_mouse_passthrough(false);
        } else {
            window_backend.window.set_mouse_passthrough(true);
        }
    }
}

impl Overlay {
    fn start(&self) {
        let mut window_backend = GlfwWindow::new(Default::default(), Default::default());
        window_backend.window.set_decorated(false);

        let gl_backend = WgpuBackend::new(&mut window_backend, Default::default());
        window_backend.run_event_loop(gl_backend, self);
    }
}