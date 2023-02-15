use std::ptr::null_mut;
use std::sync::{Mutex, RwLock};
use active_win_pos_rs::get_active_window;
use egui_backend::egui::{Align2, Color32, Context, FontId, Pos2, Window};
use egui_backend::{GfxBackend, UserApp, WindowBackend};
use egui_render_wgpu::WgpuBackend;
use egui_window_glfw_passthrough::GlfwWindow;
use faithe::internal::get_current_process_id;
use once_cell::sync::OnceCell;
use winapi::shared::basetsd::LONG_PTR;
use winapi::shared::windef::{HWND, RECT};
use winapi::um::winuser::{GetWindowThreadProcessId, GWL_EXSTYLE, SetWindowLongPtrA, WS_EX_TOOLWINDOW};
use windows_win::raw::window::get_thread_process_id;
use crate::sdk::Global;

static mut WINDOW: HWND = null_mut();

pub struct OverlayComponent {
    renderer: Box<dyn FnMut(&Context) + Send>
}
unsafe impl Sync for OverlayComponent {}

struct OverlayState {
    components_hidden: bool,
    fully_hidden: bool,
}

pub struct Overlay {
    panels: RwLock<Vec<OverlayComponent>>,
    state: Mutex<OverlayState>
}

impl Global for Overlay {
    fn cell() -> &'static OnceCell<Self> {
        static INSTANCE: OnceCell<Overlay> = OnceCell::new();
        &INSTANCE
    }

    fn create() -> Self {
        Overlay {
            panels: RwLock::new(vec![]),
            state: Mutex::new(OverlayState {
                components_hidden: true,
                fully_hidden: true,
            })
        }
    }
}

struct OverlayApp {}
impl UserApp<GlfwWindow, WgpuBackend> for OverlayApp {
    fn run(&mut self, egui_context: &Context, window_backend: &mut GlfwWindow, _gl_backend: &mut WgpuBackend) {
        let pid = get_current_process_id();
        let active_window = get_active_window();
        if active_window.is_err() {
            return;
        }

        let act_window = active_window.map_err(|_| {} ).unwrap();
        let overlay_pid = get_thread_process_id(window_backend.window.get_win32_window() as _);
        if act_window.process_id != overlay_pid.0 as u64 && act_window.process_id != pid as u64 {
            return;
        }

        Window::new("Hello World").show(egui_context, |ui| {
            ui.label("Darwin is a jew.");
        });

        let painter = egui_context.debug_painter();
        painter.text(Pos2::new(500_f32, 500_f32), Align2::CENTER_TOP, "Conan sucks.", FontId::monospace(25_f32), Color32::RED);

        if egui_context.wants_keyboard_input() || egui_context.wants_pointer_input() {
            window_backend.window.set_mouse_passthrough(false);
        } else {
            window_backend.window.set_mouse_passthrough(true);
        }

        let pos = act_window.position;
        window_backend.window.set_pos(pos.x as i32, pos.y as i32);
        window_backend.window.set_size(pos.width as i32, pos.height as i32);
    }
}

impl Overlay {
    pub fn start(&self) {
        let app = OverlayApp {};

        let mut window_backend = GlfwWindow::new(Default::default(), Default::default());
        window_backend.window.set_decorated(false);
        window_backend.window.set_size_polling(true);
        window_backend.window.set_pos_polling(true);
        unsafe { SetWindowLongPtrA(window_backend.window.get_win32_window() as _, GWL_EXSTYLE, WS_EX_TOOLWINDOW as LONG_PTR); }
        unsafe { WINDOW = window_backend.window.get_win32_window() as _ };

        let gl_backend = WgpuBackend::new(&mut window_backend, Default::default());
        window_backend.run_event_loop(gl_backend, app);
    }
}