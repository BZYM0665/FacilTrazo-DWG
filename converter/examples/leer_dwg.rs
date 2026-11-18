//! Prueba nativa: lee un DWG y lo escribe como DXF con el nuevo lector.
fn main() -> Result<(), String> {
    let a: Vec<String> = std::env::args().collect();
    let dwg = std::fs::read(&a[1]).map_err(|e| e.to_string())?;
    let dxf = dwgconv::convertir_dwg_a_dxf(&dwg)?;
    std::fs::write(&a[2], &dxf).map_err(|e| e.to_string())?;
    println!("DWG {} bytes -> DXF {} bytes", dwg.len(), dxf.len());
    Ok(())
}
