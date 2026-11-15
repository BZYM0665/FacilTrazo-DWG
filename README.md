# FácilTrazo · Dibujo 2D

Programa de dibujo técnico 2D compatible con DWG/DXF de AutoCAD, pensado para personas de habla hispana de América Latina. **Es gratuito.**

Corre completo en el navegador (Windows, Linux y otros sistemas) y se puede instalar como aplicación (Edge/Chrome → Instalar FácilTrazo). También hay **versiones de escritorio para Windows y Linux** (ver «Descargar para el computador»).

**Se le puede conectar una IA** (por ejemplo Claude) para que lea y dibuje en el dibujo abierto. Se activa en **⚙ Configuración › General › «Conectar con IA»**; viene apagado. Más abajo, en «Conexión con IA», están los detalles.

> **Licencia:** uso gratuito, **sin modificación**. Esto **no** es software de código abierto (open source): el código se publica para que pueda verse y descargarse, no para que se modifique. Más abajo, en «Licencia», está el resumen.

## Qué hay en este repositorio

- `index.html` — la aplicación completa (un solo archivo).
- `manifest.webmanifest`, `sw.js` y los archivos `icon-*.png` / `maskable-*.png` — instalación, ícono y funcionamiento sin internet.
- `converter/` y `dist/dwg/` — conversor DXF→DWG compilado a WebAssembly (va incrustado en `index.html`).
- `conector/` — el conector para conectar una IA (archivo `.mcpb` para Claude Desktop) y `AVISOS_DEL_CONECTOR.txt`, con los componentes de terceros que lleva.
- `LICENSE.txt` — la licencia de uso.
- Las versiones de escritorio no están entre los archivos del repositorio: se descargan desde **Releases**.
- `AVISOS_DE_TERCEROS.txt` — componentes de terceros y sus licencias.

## Descargar para el computador

Las versiones de escritorio son el mismo programa dentro de una ventana propia, y no necesitan internet. Se descargan desde **[Releases](https://github.com/BZYM0665/FacilTrazo-DWG/releases/latest)**, en la lista «Assets». Son para equipos de 64 bits (x86-64).

| Sistema | Archivo | Cómo se usa |
|---|---|---|
| Windows 10 y 11 | `FacilTrazo-1.0.3-Windows-instalador.exe` | Instalación de un clic, solo para su usuario (no pide permisos de administrador): queda en `%LOCALAPPDATA%\Programs\FacilTrazo` y crea accesos directos. No permite elegir la carpeta; para eso use el portable. |
| Windows 10 y 11 | `FacilTrazo-1.0.3-Windows-portable.exe` | Un solo archivo, sin instalar. Guarda sus ajustes en una carpeta `FacilTrazo-datos` junto al archivo. |
| Ubuntu, Linux Mint, Debian y derivados | `FacilTrazo-1.0.3-Linux.deb` | En una terminal: `sudo apt install ./FacilTrazo-1.0.3-Linux.deb`. Queda en el menú de aplicaciones, categoría Gráficos. |
| Casi cualquier Linux | `FacilTrazo-1.0.3-Linux.AppImage` | Archivo único sin instalar: dé permiso de ejecución (`chmod +x FacilTrazo-1.0.3-Linux.AppImage`) y ábralo. |
| Casi cualquier Linux | `FacilTrazo-1.0.3-Linux.tar.gz` | Descomprima (`tar -xzf FacilTrazo-1.0.3-Linux.tar.gz`) y ejecute `faciltrazo-escritorio` dentro de la carpeta. |

Notas:

- **Windows:** los archivos no están firmados digitalmente, así que Windows SmartScreen puede mostrar «Windows protegió su PC». Para continuar: «Más información» → «Ejecutar de todas formas».
- **AppImage en Ubuntu 22.04 o más nuevo y en Linux Mint:** puede pedir `libfuse2` (`sudo apt install libfuse2`; en Ubuntu 24.04 el paquete se llama `libfuse2t64`). Si no quiere instalarlo, ejecute el archivo con `--appimage-extract-and-run`.
- **Comprobar la descarga:** el archivo `SHA256SUMS.txt` del Release trae la huella SHA-256 de cada archivo (en Windows: `certutil -hashfile archivo SHA256`; en Linux: `sha256sum archivo`).
- **Desinstalar:** en Windows, desde «Aplicaciones instaladas»; en Ubuntu/Mint, `sudo apt remove faciltrazo-escritorio`. Los archivos portables se borran como cualquier otro archivo.
- No hay versión para macOS ni para equipos ARM.

## Conexión con IA (opcional)

FácilTrazo puede trabajar junto a una IA que usted ya use. Se activa en **⚙ Configuración › General › «Conectar con IA»** (viene apagado).

- **Cómo funciona:** la IA se conecta por un conector local («FácilTrazo», estándar MCP) que se instala aparte en Claude Desktop como extensión `.mcpb`. Está en la carpeta `conector/` de este repositorio; los pasos están en «Instalar el conector». Se pide en la conversación: «conecta FácilTrazo».
- **Qué puede hacer la IA:** leer el dibujo abierto y medirlo; dibujar líneas, polilíneas, rectángulos, círculos, arcos, textos, cotas y achurados; modificar figuras; mirar una captura para comprobar el resultado; y guardar en DWG o DXF.
- **Control suyo:** cada orden de la IA es un paso que se puede **deshacer**; respeta el Modo revisión y las capas bloqueadas; la conexión solo funciona en su equipo (no sale a internet) y se apaga desmarcando la casilla.
- **Probado con:** Claude Desktop y Microsoft Edge. Con otras IA o navegadores no se ha probado.

### Instalar el conector (Claude Desktop)

Necesita **Claude Desktop** y FácilTrazo **rev18 o más nueva** (la versión se ve arriba a la izquierda, por ejemplo «v1.9.1 · rev18»; la de esta página y la de escritorio 1.0.3 lo son). No hace falta instalar Node.js: el conector usa el que trae Claude Desktop.

1. Descargue **[FacilTrazo_conector_Claude_1.1.1.mcpb](https://github.com/BZYM0665/FacilTrazo-DWG/raw/main/conector/FacilTrazo_conector_Claude_1.1.1.mcpb)** (298.430 bytes). Para comprobar la descarga, su huella SHA-256 debe ser `fd1206d2b3a435bb89410bbf007a23e1b024a03f0fd620f925a55652a154e48d`.
2. En Claude Desktop abra **Ajustes → Extensiones**. En la sección «Desarrollador de extensiones» (según la versión, dentro de «Configuración avanzada») pulse **Instalar extensión** y elija el archivo. También puede arrastrar el archivo a esa pantalla. En Windows, el doble clic puede abrir el Bloc de notas si ese tipo de archivo no está asociado a ningún programa: use el botón. Los nombres exactos de las pantallas pueden cambiar con la versión de Claude Desktop.
3. **Cierre Claude Desktop por completo** (ícono de la bandeja del sistema → Salir) y ábralo de nuevo. Sin esto, la conversación puede seguir usando una versión anterior del conector.
4. Abra FácilTrazo y, en una conversación nueva de Claude, pida: «conecta FácilTrazo». Si no conecta sola, pegue el código de conexión que Claude le muestre en **⚙ Configuración › General › Conectar con IA**. Edge puede preguntar si permite «Acceder a otras aplicaciones y servicios en este dispositivo»: pulse **Permitir**.

Para actualizar el conector, quite la extensión anterior en Ajustes → Extensiones, instale la nueva y reinicie Claude Desktop. El archivo `.mcpb` lleva dentro la licencia de FácilTrazo (`LICENSE.txt`); los componentes de terceros que empaqueta están en `conector/AVISOS_DEL_CONECTOR.txt` (no van dentro del `.mcpb`).

## Privacidad

Los dibujos nunca salen del computador: la página no envía datos a ningún servidor.

Si usted activa la conexión con IA, la IA que use puede leer el dibujo abierto, y ese contenido lo procesa el servicio de esa IA según sus propias condiciones. Active la conexión solo con dibujos que pueda compartir con ese servicio.

## Licencia

Resumen (vale solo el texto completo de `LICENSE.txt`, en español):

- Puede descargar y usar FácilTrazo gratis, también en su trabajo profesional.
- Puede compartir copias idénticas, sin modificar y sin cobrar.
- No puede modificarlo, hacer versiones derivadas, descompilarlo, venderlo ni cobrar por él.
- Sus dibujos y archivos son suyos.

Los componentes de terceros que lleva el programa tienen sus propias licencias; están en `AVISOS_DE_TERCEROS.txt` (los del conector, en `conector/AVISOS_DEL_CONECTOR.txt`).

## Comentarios y mejoras

Para enviar comentarios, avisar de un error o proponer una mejora, abra un «Issue» en la pestaña **Issues** de este repositorio. No incluya información confidencial ni archivos de clientes: el repositorio es público y cualquiera puede verlo.
