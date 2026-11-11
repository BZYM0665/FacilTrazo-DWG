//! Conversor DXF→DWG de FácilTrazo.
//! Recibe el texto DXF que FácilTrazo ya sabe generar y devuelve el archivo DWG binario
//! en la versión de AutoCAD elegida. Usa acadrust, la misma librería con que FácilTrazo
//! lee los DWG (y la misma que usa Open CAD Studio para guardarlos).
use std::io::Cursor;

use acadrust::io::dwg::DwgWriter;
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
