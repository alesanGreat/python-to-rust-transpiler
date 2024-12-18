# Py2Rust: A Source-to-Source Translation Framework for SDL2-based Graphics Code

An experimental static analysis and source transformation tool that performs AST-based translation from Python/SDL2 graphics code to equivalent Rust implementations. Implements lexical analysis, syntactic parsing, and source code generation with emphasis on graphics primitive operations and event handling constructs.


## Overview

Py2Rust is an experimental compiler that attempts to convert simple Python programs using SDL2 into Rust code. The project is currently in early development and serves as a proof of concept for automated Python-to-Rust translation of graphics code.

## Current Capabilities

* **Basic SDL2 Graphics Translation:**
  - Rectangle drawing and filling
  - Simple circle drawing
  - Basic window management
  - RGB color support

* **Event Handling:**
  - Window closing events
  - Basic keyboard input detection

* **Project Structure:**
  - Generates Cargo.toml with required dependencies
  - Creates basic Rust project structure
  - Handles SDL2 initialization and cleanup

## Technical Implementation

1. **Tokenization:** Regex-based Python code parsing
2. **AST Analysis:** Basic Abstract Syntax Tree generation
3. **State Management:** Variable and draw call tracking
4. **Code Generation:** Rust SDL2 code output

## Usage

1. **Clone and Build:**
    ```bash
    git clone https://github.com/alesanGreat/python-to-rust-transpiler
    cd YOUR_REPO_NAME
    cargo build --release
    ```

2. **Run:**
    ```bash
    ./target/release/py2rust <your_python_file.py>
    ```

## Example Input

```python
# Simple SDL2 Python code example
import sdl2
import sdl2.ext

ANCHO = 800
ALTO = 600
BLANCO = (255, 255, 255)
NEGRO = (0, 0, 0)
ROJO = (255, 0, 0)

sdl2.ext.init()
window = sdl2.ext.Window("Example", size=(ANCHO, ALTO))
window.show()
renderer = sdl2.ext.Renderer(window)

running = True
while running:
    for event in sdl2.ext.get_events():
        if event.type == sdl2.SDL_QUIT:
            running = False
    
    renderer.clear(NEGRO)
    renderer.fill(sdl2.SDL_Rect(100, 100, 50, 50), ROJO)
    renderer.present()

sdl2.ext.quit()
```

## Known Limitations

* Prototype stage - expect bugs and incomplete features
* Limited to very basic Python SDL2 code
* No support for complex Python features
* Basic error handling
* Generated code may need manual adjustments
* No texture or sprite support yet

## Development Goals

* Improve code generation reliability
* Expand SDL2 feature support
* Better error messages
* Documentation improvements

## Contributing

This is an experimental project and contributions are welcome. Please check the issues page for current development needs.

## License

MIT License - See [LICENSE](/LICENSE)

---

*Keywords: python compiler rust, sdl2 graphics, game development tools, python to rust translation, sdl2 rust, python rust converter, game programming, cross compilation, 2d graphics, pygame rust*