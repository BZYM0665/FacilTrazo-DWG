//! Conversor DXF→DWG de FácilTrazo.
//! Recibe el texto DXF que FácilTrazo ya sabe generar y devuelve el archivo DWG binario
//! en la versión de AutoCAD elegida. Usa acadrust, la misma librería con que FácilTrazo
//! lee los DWG (y la misma que usa Open CAD Studio para guardarlos).
use std::io::Cursor;

use acadrust::io::dwg::{DwgReader, DwgWriter};
use acadrust::io::dxf::DxfWriter;
use acadrust::{DxfReader, DxfVersion};
use wasm_bindgen::prelude::*;

fn version_de(v: &str) -> DxfVersion {
    match v {
        "2000" => DxfVersion::AC1015,
        "2004" => DxfVersion::AC1018,
        "2007" => DxfVersion::AC1021,
        "2010" => DxfVersion::AC1024,
        "2013" => DxfVersion::AC1027,
        _ => DxfVersion::AC1032, // 2018, la que usan los AutoCAD actuales
    }
}

/// Conversión en sí (sirve igual en nativo y en WebAssembly).
pub fn convertir(dxf: &[u8], version: &str) -> Result<Vec<u8>, String> {
    let reader = DxfReader::from_reader(Cursor::new(dxf.to_vec())).map_err(|e| e.to_string())?;
    let mut doc = reader.read().map_err(|e| e.to_string())?;
    doc.version = version_de(version);
    DwgWriter::write_to_vec(&doc).map_err(|e| e.to_string())
}

/// Punto de entrada para el navegador.
#[wasm_bindgen]
pub fn dxf_to_dwg(dxf: &[u8], version: &str) -> Result<Vec<u8>, JsError> {
    convertir(dxf, version).map_err(|e| JsError::new(&e))
}

/// Lectura de un DWG: devuelve el DXF (texto) con todo lo que trae el archivo, incluido el color
/// verdadero (RGB) de capas y figuras, que el convertidor anterior (dwgdxf) perdía.
pub fn convertir_dwg_a_dxf(dwg: &[u8]) -> Result<Vec<u8>, String> {
    let mut lector = DwgReader::from_stream(Cursor::new(dwg.to_vec()));
    let mut doc = lector.read().map_err(|e| e.to_string())?;
    // Los DWG anteriores a 2007 guardan los textos en la página de códigos del equipo; al escribirlos
    // como DXF de esa época saldrían en ANSI. Se escribe siempre como DXF 2018 (UTF-8): los acentos,
    // la ñ y los símbolos (Ø, °, ²) llegan intactos a FácilTrazo.
    match doc.version {
        DxfVersion::AC1021 | DxfVersion::AC1024 | DxfVersion::AC1027 | DxfVersion::AC1032 => {}
        _ => doc.version = DxfVersion::AC1032,
    }
    DxfWriter::new(&doc).write_to_vec().map_err(|e| e.to_string())
}

/// Punto de entrada para el navegador: DWG (bytes) -> DXF (bytes de texto).
#[wasm_bindgen]
pub fn dwg_to_dxf(dwg: &[u8]) -> Result<Vec<u8>, JsError> {
    convertir_dwg_a_dxf(dwg).map_err(|e| JsError::new(&e))
}
