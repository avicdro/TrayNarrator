//! System tray icon para Windows.
//!
//! Crea un icono en la bandeja del sistema con menú contextual.

use std::mem::zeroed;
use std::ptr::null_mut;
use std::sync::atomic::Ordering;

use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::Shell::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

use crate::logging::log;
use crate::state::{DEBE_SALIR, VELOCIDAD_ACTUAL};

const WM_TRAYICON: u32 = WM_USER + 1;
const ID_TRAY_EXIT: u16 = 1001;
const ID_TRAY_VELOCIDAD: u16 = 1002;

/// Convierte un string Rust a wide string (UTF-16) para Windows API
fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Crea y ejecuta el system tray
pub fn run_tray() {
    unsafe {
        let class_name = to_wide("TrayNarratorClass");
        let window_name = to_wide("TrayNarrator");

        let hinstance = GetModuleHandleW(null_mut());

        // Registrar clase de ventana
        let wc = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: LoadIconW(null_mut(), IDI_APPLICATION),
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
            lpszClassName: class_name.as_ptr(),
        };

        RegisterClassW(&wc);

        // Crear ventana oculta
        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            window_name.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            hinstance,
            null_mut(),
        );

        if hwnd.is_null() {
            log("Error creando ventana para tray");
            return;
        }

        // Crear icono en la bandeja
        let tip = to_wide("TrayNarrator - F8:Leer F9:Pausar");
        let mut nid: NOTIFYICONDATAW = zeroed();
        nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
        nid.hWnd = hwnd;
        nid.uID = 1;
        nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
        nid.uCallbackMessage = WM_TRAYICON;
        nid.hIcon = LoadIconW(null_mut(), IDI_APPLICATION);

        // Copiar tooltip
        let tip_bytes = tip.len().min(128);
        nid.szTip[..tip_bytes].copy_from_slice(&tip[..tip_bytes]);

        Shell_NotifyIconW(NIM_ADD, &nid);

        log("System tray iniciado");

        // Bucle de mensajes
        let mut msg: MSG = zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            if DEBE_SALIR.load(Ordering::SeqCst) {
                break;
            }
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        // Limpiar
        Shell_NotifyIconW(NIM_DELETE, &nid);
        DestroyWindow(hwnd);
    }
}

/// Procedimiento de ventana para manejar eventos del tray
unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_TRAYICON => {
            let event = (lparam & 0xFFFF) as u32;
            if event == WM_RBUTTONUP {
                // Click derecho - mostrar menú
                let mut pt: POINT = zeroed();
                GetCursorPos(&mut pt);

                let hmenu = CreatePopupMenu();

                // Mostrar velocidad actual
                let vel = VELOCIDAD_ACTUAL.load(Ordering::SeqCst);
                let vel_text = to_wide(&format!("Velocidad: {}%", vel));
                AppendMenuW(
                    hmenu,
                    MF_STRING | MF_GRAYED,
                    ID_TRAY_VELOCIDAD as usize,
                    vel_text.as_ptr(),
                );

                AppendMenuW(hmenu, MF_SEPARATOR, 0, null_mut());

                let salir_text = to_wide("Salir");
                AppendMenuW(hmenu, MF_STRING, ID_TRAY_EXIT as usize, salir_text.as_ptr());

                SetForegroundWindow(hwnd);
                TrackPopupMenu(
                    hmenu,
                    TPM_BOTTOMALIGN | TPM_LEFTALIGN,
                    pt.x,
                    pt.y,
                    0,
                    hwnd,
                    null_mut(),
                );
                DestroyMenu(hmenu);
            }
            0
        }
        WM_COMMAND => {
            let id = (wparam & 0xFFFF) as u16;
            if id == ID_TRAY_EXIT {
                log("Salir seleccionado desde el tray");
                DEBE_SALIR.store(true, Ordering::SeqCst);
                PostQuitMessage(0);
            }
            0
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
