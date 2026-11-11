//! Prueba nativa: convierte un DXF a DWG y lo relee para confirmar que quedó válido.
use acadrust::io::dwg::DwgReader;

fn main() -> Result<(), String> {
    let a: Vec<String> = std::env::args().collect();
    let dxf = std::fs::read(&a[1]).map_err(|e| e.to_string())?;
    let ver = a.get(3).map(|s| s.as_str()).unwrap_or("2018");
    let dwg = dwgconv::convertir(&dxf, ver)?;
    std::fs::write(&a[2], &dwg).map_err(|e| e.to_string())?;
    let mut rd = DwgReader::from_file(&a[2]).map_err(|e| e.to_string())?;
    let doc = rd.read().map_err(|e| e.to_string())?;
    println!(
        "escrito {} bytes · releído: version {:?}, {} figuras, {} capas",
        dwg.len(),
        doc.version,
        doc.entities().count(),
        doc.layers.len()
    );
    Ok(())
}
