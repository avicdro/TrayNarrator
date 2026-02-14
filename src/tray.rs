//! System tray icon con menú contextual.
//!
//! Usa `tray-icon` + `muda` para crear un icono en la bandeja del sistema
//! con menú para controlar la velocidad de TTS y salir de la aplicación.
//! El icono se incrusta en el binario con `include_bytes!`.

use std::sync::atomic::Ordering;

use image::ImageReader;
use muda::{CheckMenuItem, Menu, MenuEvent, PredefinedMenuItem, Submenu};
use tray_icon::menu::MenuId;
use tray_icon::{Icon, TrayIconBuilder};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;

use crate::config::{VELOCIDADES_PRESET, VERSION};
use crate::logging::log;
use crate::state::{
    establecer_velocidad, etiqueta_velocidad_actual, indice_preset_actual, DEBE_SALIR,
    VELOCIDAD_ACTUAL,
};

// ═══════════════════════════════════════════════════════════════════════════════
// ICONO
// ═══════════════════════════════════════════════════════════════════════════════

/// Bytes del icono PNG incrustados en el binario en tiempo de compilación.
const ICON_BYTES: &[u8] = include_bytes!("../assets/traynarrator-icon.png");

/// Carga y decodifica el icono PNG incrustado a formato RGBA nativo.
fn load_tray_icon() -> Icon {
    let img = ImageReader::new(std::io::Cursor::new(ICON_BYTES))
        .with_guessed_format()
        .expect("Error detectando formato del icono")
        .decode()
        .expect("Error decodificando icono PNG");

    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let raw = rgba.into_raw();

    Icon::from_rgba(raw, width, height).expect("Error creando Icon desde RGBA")
}

// ═══════════════════════════════════════════════════════════════════════════════
// MENÚ
// ═══════════════════════════════════════════════════════════════════════════════

/// Prefijo para IDs de presets de velocidad.
const ID_SPEED_PREFIX: &str = "speed_";
const ID_EXIT: &str = "exit";

/// Construye el menú contextual del tray con submenú de velocidad.
fn build_tray_menu() -> (Menu, Submenu, Vec<CheckMenuItem>) {
    let vel_actual = VELOCIDAD_ACTUAL.load(Ordering::SeqCst);
    let etiqueta = etiqueta_velocidad_actual();

    // Submenú de velocidad con check items
    let submenu_velocidad = Submenu::new(format!("Velocidad: {}", etiqueta), true);

    let mut check_items: Vec<CheckMenuItem> = Vec::with_capacity(VELOCIDADES_PRESET.len());

    for (i, (label, ls_x100)) in VELOCIDADES_PRESET.iter().enumerate() {
        let id = format!("{}{}", ID_SPEED_PREFIX, i);
        let checked = *ls_x100 == vel_actual;
        let item = CheckMenuItem::with_id(id, *label, true, checked, None);
        submenu_velocidad.append(&item).unwrap();
        check_items.push(item);
    }

    // Menú principal
    let version_item = muda::MenuItem::with_id(
        "version_info",
        format!("TrayNarrator v{}", VERSION),
        false,
        None,
    );
    let exit = muda::MenuItem::with_id(ID_EXIT, "Salir", true, None);

    let menu = Menu::new();
    menu.append(&version_item).unwrap();
    menu.append(&PredefinedMenuItem::separator()).unwrap();
    menu.append(&submenu_velocidad).unwrap();
    menu.append(&PredefinedMenuItem::separator()).unwrap();
    menu.append(&exit).unwrap();

    (menu, submenu_velocidad, check_items)
}

// ═══════════════════════════════════════════════════════════════════════════════
// EVENT LOOP
// ═══════════════════════════════════════════════════════════════════════════════

/// Estado de la aplicación para el event loop de winit.
struct TrayApp {
    /// Submenú de velocidad para actualizar su título.
    submenu_velocidad: Submenu,
    /// Check items de velocidad para actualizar las marcas.
    check_items: Vec<CheckMenuItem>,
    /// Se guarda para que no se destruya mientras corre el loop.
    _tray_icon: Option<tray_icon::TrayIcon>,
    /// Menú del tray (debe mantenerse vivo).
    _menu: Option<Menu>,
    /// Último índice de velocidad reflejado en el menú.
    ultimo_indice_velocidad: usize,
}

impl ApplicationHandler for TrayApp {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        // No creamos ventanas — solo necesitamos el event loop para el tray
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
        // Sin ventanas, no hay eventos de ventana que manejar
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        // Procesar eventos de menú del tray
        while let Ok(event) = MenuEvent::receiver().try_recv() {
            self.handle_menu_event(&event.id, event_loop);
        }

        // Sincronizar UI del tray con cambios de velocidad hechos fuera del menú
        // (por ejemplo, mediante hotkeys Ctrl+[ / Ctrl+]).
        self.sincronizar_velocidad_desde_estado();

        // Comprobar si debe salir (por ejemplo, desde un hotkey)
        if DEBE_SALIR.load(Ordering::SeqCst) {
            event_loop.exit();
        }
    }
}

impl TrayApp {
    /// Sincroniza el submenú de velocidad con el estado global actual.
    fn sincronizar_velocidad_desde_estado(&mut self) {
        let idx_actual = indice_preset_actual();
        if idx_actual != self.ultimo_indice_velocidad {
            let (etiqueta, _) = VELOCIDADES_PRESET[idx_actual];
            self.actualizar_checks_velocidad(idx_actual);
            self.submenu_velocidad
                .set_text(format!("Velocidad: {}", etiqueta));
            self.ultimo_indice_velocidad = idx_actual;
        }
    }

    /// Maneja los eventos del menú contextual del tray.
    fn handle_menu_event(&mut self, id: &MenuId, event_loop: &ActiveEventLoop) {
        let id_str = id.0.as_str();

        if id_str == ID_EXIT {
            log("Salir seleccionado desde el tray");
            DEBE_SALIR.store(true, Ordering::SeqCst);
            event_loop.exit();
            return;
        }

        // Comprobar si es un preset de velocidad
        if let Some(idx_str) = id_str.strip_prefix(ID_SPEED_PREFIX) {
            if let Ok(idx) = idx_str.parse::<usize>() {
                if idx < VELOCIDADES_PRESET.len() {
                    let (etiqueta, ls_x100) = VELOCIDADES_PRESET[idx];
                    establecer_velocidad(ls_x100);
                    self.actualizar_checks_velocidad(idx);
                    self.submenu_velocidad
                        .set_text(format!("Velocidad: {}", etiqueta));
                    self.ultimo_indice_velocidad = idx;
                    log(&format!("Tray: Velocidad → {}", etiqueta));
                }
            }
        }
    }

    /// Actualiza las marcas de los check items: solo el seleccionado queda marcado.
    fn actualizar_checks_velocidad(&self, seleccionado: usize) {
        for (i, item) in self.check_items.iter().enumerate() {
            item.set_checked(i == seleccionado);
        }
    }
}

/// Crea el system tray y ejecuta el event loop principal.
///
/// Esta función bloquea el hilo actual (debe ejecutarse en el hilo principal).
/// Los hotkeys (F8/F9) siguen funcionando en su propio hilo.
pub fn run_tray() {
    let icon = load_tray_icon();
    let (menu, submenu_velocidad, check_items) = build_tray_menu();

    let event_loop = EventLoop::new().expect("Error creando event loop");
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

    // Construir el tray icon
    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu.clone()))
        .with_tooltip("TrayNarrator - F8:Leer F9:Pausar")
        .with_icon(icon)
        .build()
        .expect("Error creando tray icon");

    log("System tray iniciado");

    let mut app = TrayApp {
        submenu_velocidad,
        check_items,
        _tray_icon: Some(tray_icon),
        _menu: Some(menu),
        ultimo_indice_velocidad: indice_preset_actual(),
    };

    event_loop
        .run_app(&mut app)
        .expect("Error en el event loop del tray");
}
