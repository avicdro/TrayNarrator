//! Generación de audio con Piper TTS.

use std::io::Write;
use std::process::{Command, Stdio};

use crate::config::{RUTA_MODELO, RUTA_PIPER, RUTA_TEMP_WAV};
use crate::logging::log;
use crate::state::obtener_velocidad;

// Para ocultar la ventana de Piper en Windows
#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
use crate::config::CREATE_NO_WINDOW;

/// Genera audio WAV usando Piper TTS
pub fn generar_audio_piper(texto: &str) -> Result<(), String> {
    let velocidad = obtener_velocidad();
    log(&format!(
        "Generando audio (velocidad: {}) para: '{}'",
        velocidad,
        &texto[..texto.len().min(50)]
    ));

    let texto_limpio = texto.replace(['\r', '\n'], " ").trim().to_string();

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
