//! Hilo de reproducción de audio con rodio.

use std::fs::File;
use std::io::BufReader;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use rodio::{Decoder, OutputStream, Sink};

use crate::config::RUTA_TEMP_WAV;
use crate::logging::log;
use crate::state::{
    ComandoAudio, DEBE_SALIR, ESTADO_AUDIO, ESTADO_IDLE, ESTADO_PAUSADO, ESTADO_REPRODUCIENDO,
};

/// Hilo principal de reproducción de audio.
///
/// Recibe comandos por el canal y controla la reproducción con rodio.
pub fn hilo_audio(receiver: Receiver<ComandoAudio>) {
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
