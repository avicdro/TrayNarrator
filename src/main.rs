//! TrayNarrator - Lector TTS ultraligero para Windows usando Piper TTS
//!
//! Este programa escucha atajos de teclado globales para leer texto seleccionado.
//! - F8: Copia el texto seleccionado y lo lee en voz alta
//! - F9: Pausa/Reanuda la reproducción
//! - Ctrl+[: Reducir velocidad (más rápido)
//! - Ctrl+]: Aumentar velocidad (más lento)
//!
//! COMPILACIÓN CRUZADA (desde WSL2):
//! cargo build --release --target x86_64-pc-windows-gnu
//!
//! El binario estará en: target/x86_64-pc-windows-gnu/release/tray_narrator.exe

// Oculta la ventana de consola en Windows (aplicación GUI sin consola visible)
#![windows_subsystem = "windows"]

use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use arboard::Clipboard;
use enigo::{Enigo, Key, Keyboard, Settings};
use inputbot::KeybdKey;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use rodio::{Decoder, OutputStream, Sink};

// Para ocultar la ventana de Piper en Windows
#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

// ═══════════════════════════════════════════════════════════════════════════════
// CONFIGURACIÓN - Modifica estas constantes según tu instalación
// ═══════════════════════════════════════════════════════════════════════════════

/// Ruta al ejecutable de Piper TTS (formato Windows)
const RUTA_PIPER: &str = r"C:\TrayNarrator\piper\piper.exe";

/// Ruta al modelo de voz .onnx de Piper (formato Windows)  
const RUTA_MODELO: &str = r"C:\TrayNarrator\piper\es_ES-sharvard-medium.onnx";

/// Ruta del archivo temporal WAV
const RUTA_TEMP_WAV: &str = r"C:\TrayNarrator\temp.wav";

/// Ruta del archivo de log para debugging
const RUTA_LOG: &str = r"C:\TrayNarrator\log.txt";

/// Tiempo de espera después de simular Ctrl+C (milisegundos)
const DELAY_COPIAR_MS: u64 = 150;

/// Velocidad inicial (length_scale * 100, ej: 80 = 0.8)
const VELOCIDAD_INICIAL: u32 = 80;

/// Velocidad mínima (50 = 0.5, muy rápido)
const VELOCIDAD_MIN: u32 = 50;

/// Velocidad máxima (150 = 1.5, muy lento)
const VELOCIDAD_MAX: u32 = 150;

/// Incremento/decremento de velocidad
const VELOCIDAD_PASO: u32 = 10;

// ═══════════════════════════════════════════════════════════════════════════════
// COMANDOS Y ESTADO GLOBAL
// ═══════════════════════════════════════════════════════════════════════════════

/// Comandos que se envían al hilo de audio
#[derive(Debug)]
enum ComandoAudio {
    /// Reproducir el archivo WAV temporal
    Reproducir,
    /// Detener la reproducción actual
    Detener,
    /// Pausar/Reanudar la reproducción
    TogglePausa,
}

// Estados de reproducción
const ESTADO_IDLE: u8 = 0;
const ESTADO_REPRODUCIENDO: u8 = 1;
const ESTADO_PAUSADO: u8 = 2;

/// Estado atómico de reproducción
static ESTADO_AUDIO: AtomicU8 = AtomicU8::new(ESTADO_IDLE);

/// Velocidad actual (length_scale * 100)
static VELOCIDAD_ACTUAL: AtomicU32 = AtomicU32::new(VELOCIDAD_INICIAL);

/// Flag para indicar que la aplicación debe terminar
static DEBE_SALIR: AtomicBool = AtomicBool::new(false);

lazy_static! {
    /// Canal para enviar comandos al hilo de audio
    static ref CANAL_AUDIO: Mutex<Option<Sender<ComandoAudio>>> = Mutex::new(None);
}

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCIÓN DE LOGGING
// ═══════════════════════════════════════════════════════════════════════════════

/// Escribe un mensaje en el archivo de log para debugging
fn log(mensaje: &str) {
    if let Ok(mut archivo) = OpenOptions::new().create(true).append(true).open(RUTA_LOG) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let _ = writeln!(archivo, "[{}] {}", timestamp, mensaje);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SYSTEM TRAY (Windows)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(windows)]
mod tray {
    use super::*;
    use std::mem::zeroed;
    use std::ptr::null_mut;
    use windows_sys::Win32::Foundation::*;
    use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows_sys::Win32::UI::Shell::*;
    use windows_sys::Win32::UI::WindowsAndMessaging::*;

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
}

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCIONES AUXILIARES
// ═══════════════════════════════════════════════════════════════════════════════

/// Simula la pulsación de Ctrl+C para copiar el texto seleccionado
fn simular_copiar() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Error inicializando Enigo: {}", e))?;

    // Simular Ctrl+C
    enigo
        .key(Key::Control, enigo::Direction::Press)
        .map_err(|e| format!("Error presionando Ctrl: {}", e))?;
    enigo
        .key(Key::Unicode('c'), enigo::Direction::Click)
        .map_err(|e| format!("Error presionando C: {}", e))?;
    enigo
        .key(Key::Control, enigo::Direction::Release)
        .map_err(|e| format!("Error soltando Ctrl: {}", e))?;

    thread::sleep(Duration::from_millis(DELAY_COPIAR_MS));
    Ok(())
}

/// Lee el texto del portapapeles
fn leer_portapapeles() -> Result<String, String> {
    let mut clipboard =
        Clipboard::new().map_err(|e| format!("Error accediendo al portapapeles: {}", e))?;

    let texto = clipboard
        .get_text()
        .map_err(|e| format!("Error leyendo texto del portapapeles: {}", e))?;

    if texto.trim().is_empty() {
        return Err("El portapapeles está vacío o no contiene texto".to_string());
    }

    Ok(texto)
}

/// Obtiene la velocidad actual como float
fn obtener_velocidad() -> f32 {
    VELOCIDAD_ACTUAL.load(Ordering::SeqCst) as f32 / 100.0
}

/// Genera audio WAV usando Piper TTS
fn generar_audio_piper(texto: &str) -> Result<(), String> {
    let velocidad = obtener_velocidad();
    log(&format!(
        "Generando audio (velocidad: {}) para: '{}'",
        velocidad,
        &texto[..texto.len().min(50)]
    ));

    let texto_limpio = texto
        .replace('\r', " ")
        .replace('\n', " ")
        .trim()
        .to_string();

    if texto_limpio.is_empty() {
        return Err("Texto vacío después de limpiar".to_string());
    }

    let mut comando = Command::new(RUTA_PIPER);
    comando
        .arg("--model")
        .arg(RUTA_MODELO)
        .arg("--output_file")
        .arg(RUTA_TEMP_WAV)
        .arg("--length_scale")
        .arg(velocidad.to_string())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    #[cfg(windows)]
    comando.creation_flags(CREATE_NO_WINDOW);

    let mut proceso = comando.spawn().map_err(|e| {
        let msg = format!("Error iniciando Piper: {}", e);
        log(&msg);
        msg
    })?;

    if let Some(ref mut stdin) = proceso.stdin {
        stdin
            .write_all(texto_limpio.as_bytes())
            .map_err(|e| format!("Error escribiendo a Piper: {}", e))?;
    }
    drop(proceso.stdin.take());

    let output = proceso
        .wait_with_output()
        .map_err(|e| format!("Error esperando a Piper: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let msg = format!("Piper error: {:?}. {}", output.status.code(), stderr);
        log(&msg);
        return Err(msg);
    }

    log("Piper terminó correctamente");
    Ok(())
}

/// Envía un comando al hilo de audio
fn enviar_comando(comando: ComandoAudio) {
    let guard = CANAL_AUDIO.lock();
    if let Some(ref sender) = *guard {
        let _ = sender.send(comando);
    }
}

/// Ajusta la velocidad (positivo = más lento, negativo = más rápido)
fn ajustar_velocidad(incremento: i32) {
    let actual = VELOCIDAD_ACTUAL.load(Ordering::SeqCst);
    let nueva = if incremento > 0 {
        (actual + VELOCIDAD_PASO).min(VELOCIDAD_MAX)
    } else {
        actual.saturating_sub(VELOCIDAD_PASO).max(VELOCIDAD_MIN)
    };

    if nueva != actual {
        VELOCIDAD_ACTUAL.store(nueva, Ordering::SeqCst);
        log(&format!(
            "Velocidad ajustada: {}% (length_scale: {:.2})",
            nueva,
            nueva as f32 / 100.0
        ));
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HILO DE AUDIO
// ═══════════════════════════════════════════════════════════════════════════════

fn hilo_audio(receiver: Receiver<ComandoAudio>) {
    log("Hilo de audio iniciado");

    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok(s) => {
            log("Stream de audio inicializado");
            s
        }
        Err(e) => {
            log(&format!("Error audio: {}", e));
            return;
        }
    };

    let mut sink_actual: Option<Sink> = None;

    loop {
        if DEBE_SALIR.load(Ordering::SeqCst) {
            break;
        }

        match receiver.recv_timeout(Duration::from_millis(100)) {
            Ok(ComandoAudio::Reproducir) => {
                log("Comando Reproducir");

                if let Some(ref sink) = sink_actual {
                    sink.stop();
                }

                match Sink::try_new(&stream_handle) {
                    Ok(sink) => match File::open(RUTA_TEMP_WAV) {
                        Ok(archivo) => match Decoder::new(BufReader::new(archivo)) {
                            Ok(fuente) => {
                                log("Reproduciendo...");
                                sink.append(fuente);
                                ESTADO_AUDIO.store(ESTADO_REPRODUCIENDO, Ordering::SeqCst);
                                sink_actual = Some(sink);
                            }
                            Err(e) => log(&format!("Error decoder: {}", e)),
                        },
                        Err(e) => log(&format!("Error abriendo WAV: {}", e)),
                    },
                    Err(e) => log(&format!("Error sink: {}", e)),
                }
            }

            Ok(ComandoAudio::Detener) => {
                if let Some(ref sink) = sink_actual {
                    sink.stop();
                }
                sink_actual = None;
                ESTADO_AUDIO.store(ESTADO_IDLE, Ordering::SeqCst);
            }

            Ok(ComandoAudio::TogglePausa) => {
                if let Some(ref sink) = sink_actual {
                    let estado = ESTADO_AUDIO.load(Ordering::SeqCst);
                    if estado == ESTADO_REPRODUCIENDO {
                        sink.pause();
                        ESTADO_AUDIO.store(ESTADO_PAUSADO, Ordering::SeqCst);
                    } else if estado == ESTADO_PAUSADO {
                        sink.play();
                        ESTADO_AUDIO.store(ESTADO_REPRODUCIENDO, Ordering::SeqCst);
                    }
                }
            }

            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // Timeout normal, verificar estado
            }
            Err(_) => break,
        }

        // Verificar si el audio terminó
        if let Some(ref sink) = sink_actual {
            if sink.empty() && ESTADO_AUDIO.load(Ordering::SeqCst) == ESTADO_REPRODUCIENDO {
                ESTADO_AUDIO.store(ESTADO_IDLE, Ordering::SeqCst);
            }
        }
    }

    log("Hilo de audio terminado");
}

// ═══════════════════════════════════════════════════════════════════════════════
// MANEJADORES DE TECLAS
// ═══════════════════════════════════════════════════════════════════════════════

fn manejar_f8() {
    log("=== F8 presionado ===");
    enviar_comando(ComandoAudio::Detener);

    log("Simulando Ctrl+C...");
    if let Err(e) = simular_copiar() {
        log(&format!("Error Ctrl+C: {}", e));
        return;
    }

    log("Leyendo portapapeles...");
    let texto = match leer_portapapeles() {
        Ok(t) => {
            log(&format!("Texto: {} chars", t.len()));
            t
        }
        Err(e) => {
            log(&format!("Error portapapeles: {}", e));
            return;
        }
    };

    if let Err(e) = generar_audio_piper(&texto) {
        log(&format!("Error Piper: {}", e));
        return;
    }

    log("Enviando Reproducir...");
    enviar_comando(ComandoAudio::Reproducir);
}

fn manejar_f9() {
    let estado = ESTADO_AUDIO.load(Ordering::SeqCst);
    if estado != ESTADO_IDLE {
        enviar_comando(ComandoAudio::TogglePausa);
    }
}

fn manejar_mas_lento() {
    log("Ctrl+] - Más lento");
    ajustar_velocidad(1); // Aumentar length_scale = más lento
}

fn manejar_mas_rapido() {
    log("Ctrl+[ - Más rápido");
    ajustar_velocidad(-1); // Reducir length_scale = más rápido
}

// ═══════════════════════════════════════════════════════════════════════════════
// HILO DE INPUTBOT
// ═══════════════════════════════════════════════════════════════════════════════

fn hilo_inputbot() {
    log("Hilo inputbot iniciado");

    // F8: Leer
    KeybdKey::F8Key.bind(|| {
        thread::spawn(manejar_f8);
    });

    // F9: Pausar/Reanudar
    KeybdKey::F9Key.bind(|| {
        manejar_f9();
    });

    // Ctrl+[ : Más rápido
    KeybdKey::LBracketKey.bind(|| {
        if KeybdKey::LControlKey.is_pressed() || KeybdKey::RControlKey.is_pressed() {
            manejar_mas_rapido();
        }
    });

    // Ctrl+] : Más lento
    KeybdKey::RBracketKey.bind(|| {
        if KeybdKey::LControlKey.is_pressed() || KeybdKey::RControlKey.is_pressed() {
            manejar_mas_lento();
        }
    });

    // Bucle de eventos
    inputbot::handle_input_events();

    log("Hilo inputbot terminado");
}

// ═══════════════════════════════════════════════════════════════════════════════
// PUNTO DE ENTRADA
// ═══════════════════════════════════════════════════════════════════════════════

fn main() {
    log("=== TrayNarrator iniciado ===");
    log(&format!("Velocidad inicial: {}%", VELOCIDAD_INICIAL));

    // Crear canal de audio
    let (sender, receiver) = mpsc::channel::<ComandoAudio>();
    {
        let mut guard = CANAL_AUDIO.lock();
        *guard = Some(sender);
    }

    // Iniciar hilo de audio
    thread::spawn(move || {
        hilo_audio(receiver);
    });

    // Iniciar hilo de inputbot
    thread::spawn(|| {
        hilo_inputbot();
    });

    // En Windows, ejecutar el system tray en el hilo principal
    #[cfg(windows)]
    {
        tray::run_tray();
    }

    // En otros sistemas, simplemente esperar
    #[cfg(not(windows))]
    {
        loop {
            if DEBE_SALIR.load(Ordering::SeqCst) {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
    }

    log("=== TrayNarrator terminado ===");
}
