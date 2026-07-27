# Azure Engine — Technical Documentation

**Version:** V0.2  
**Language:** Rust (2024 edition)  
**Platform:** Linux (Wayland)  
**Status:** Active development — rendering module operational

---

## Table of Contents

1. [What is Azure Engine?](#1-what-is-azure-engine)
2. [Where it fits in the Azure ecosystem](#2-where-it-fits-in-the-azure-ecosystem)
3. [Core concepts you need to understand first](#3-core-concepts-you-need-to-understand-first)
4. [Architecture overview](#4-architecture-overview)
5. [Project structure](#5-project-structure)
6. [Models — data structures](#6-models--data-structures)
7. [Managers — logic and operations](#7-managers--logic-and-operations)
8. [Rendering module](#8-rendering-module)
9. [The full window creation sequence](#9-the-full-window-creation-sequence)
10. [Dependencies](#10-dependencies)
11. [Current capabilities and limitations](#11-current-capabilities-and-limitations)
12. [What comes next](#12-what-comes-next)

---

## 1. What is Azure Engine?

Azure Engine is the lowest-level runtime component of the Azure platform. Its single responsibility is to bridge the gap between Azure applications and the operating system's display infrastructure — in practical terms, it answers the question: *how does a Rust program get a visible, persistent, interactive window on screen, without using any UI framework?*

Every graphical application needs to go through a series of system-level steps before it can render anything: it must establish a communication channel with the display server, negotiate a memory region that both the app and the display server can access, declare the existence of a surface, confirm a configuration handshake, and then continuously process events to stay alive and responsive. Azure Engine handles all of these steps from scratch, speaking the raw binary Wayland protocol directly over a Unix socket.

This is intentionally the hardest possible approach. No bindings library, no abstraction layer, no framework. Every byte sent to the compositor is constructed by hand, in the exact format defined by the Wayland protocol specification.

---

## 2. Where it fits in the Azure ecosystem

Azure is organized in five layers, from the most abstract to the most concrete:

```
┌─────────────────────────────────────┐
│           Applications              │
│   AzureWork · AzureMail · AzureDev  │
├─────────────────────────────────────┤
│         Azure Foundation            │
│  Shared UI · Navigation · Manifests │
├─────────────────────────────────────┤
│           Azure Services            │
│  Identity · Permissions · Search    │
├─────────────────────────────────────┤
│             Azure Core              │
│  Contracts · Events · Registry      │
├─────────────────────────────────────┤
│             Azure Engine            │
│  Wayland · Window · Input · Render  │
└─────────────────────────────────────┘
```

Azure Engine sits at the very bottom of the stack. Azure Core defines the logical rules of the platform. Azure Engine makes it visible. Foundation and all applications depend only on the `AzureWindowProvider` trait defined in Core — never on Wayland-specific code directly.

---

## 3. Core concepts you need to understand first

Before reading the code, three concepts are essential.

**The compositor** is the program responsible for managing all windows on screen. On a Linux desktop running GNOME, this is Mutter. It is an ordinary user-space process, but it holds a privilege that other programs do not: direct access to the screen hardware. Every application that wants to display something must go through the compositor. The compositor receives each app's content, composites everything together into one final image, and sends that image to the physical screen.

**Wayland** is the protocol that applications use to communicate with the compositor. It is a binary protocol exchanged over a Unix socket — a special file on disk (`/run/user/1000/wayland-0`) that acts as a bidirectional pipe between two processes. Every message has a fixed structure: a 4-byte object ID (who is this message addressed to), a 4-byte combined field (message size in the upper 16 bits, opcode in the lower 16 bits), and any number of 4-byte arguments. All integers are encoded in little-endian byte order.

**Shared memory** is the mechanism that lets an application's pixel data reach the compositor without being copied. The application allocates a region of memory using the `memfd_create` system call, fills it with pixel data, and then transmits the file descriptor (a number referencing that memory region) to the compositor using a special kernel mechanism called `SCM_RIGHTS`. Once the compositor receives this file descriptor, both processes point to the exact same physical RAM — there is no network transfer, no data copy, just two processes reading and writing the same bytes.

**The pixel buffer** is a flat 1D array of bytes in shared memory. Every pixel occupies exactly 4 bytes in BGRA order (Blue, Green, Red, Alpha). The position of pixel `(x, y)` in this flat array is computed as `(y * width + x) * 4`. Wayland requires the `damage_buffer` call before each `commit` to signal which region of the buffer has changed.

---

## 4. Architecture overview

The codebase follows a strict model/manager/service separation:

**Models** hold data. They are plain structs with constructors and getters, no business logic.

**Managers** hold logic. They are collections of free functions that operate on models and send/receive messages over the connection.

**Services** (in the rendering module) are isolated operations on the canvas — each shape, effect, or utility is a separate file.

The `Window` struct is the central object exposed to the rest of the platform. It owns the `WaylandConnection`, all Wayland object ids, the pixel pointer, and the dimensions. It implements the `AzureWindowProvider` trait from Azure Core, making it usable by Foundation and applications without any Wayland-specific knowledge on their side.

---

## 5. Project structure

```
azure-engine/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── Sora-VariableFont_wght.ttf
    ├── platform/
    │   ├── mod.rs
    │   └── wayland/
    │       ├── mod.rs
    │       ├── models/
    │       │   ├── mod.rs
    │       │   ├── connection.rs
    │       │   ├── registry.rs
    │       │   ├── shared_memory.rs
    │       │   ├── window.rs
    │       │   └── object_id_allocator.rs
    │       └── managers/
    │           ├── mod.rs
    │           ├── connection_manager.rs
    │           ├── registry_manager.rs
    │           ├── bind_manager.rs
    │           ├── shared_memory_manager.rs
    │           ├── shm_manager.rs
    │           ├── compositor_manager.rs
    │           ├── xdg_manager.rs
    │           ├── surface_manager.rs
    │           └── window_manager.rs
    └── rendering/
        ├── mod.rs
        ├── models/
        │   ├── mod.rs
        │   ├── color.rs
        │   ├── pixel.rs
        │   ├── canvas.rs
        │   └── glyph.rs
        ├── services/
        │   ├── mod.rs
        │   ├── buffer.rs
        │   ├── effects.rs
        │   ├── shapes/
        │   │   ├── mod.rs
        │   │   ├── rect.rs
        │   │   ├── line.rs
        │   │   └── circle.rs
        │   └── text/
        │       ├── mod.rs
        │       ├── loader.rs
        │       ├── glyph.rs
        │       ├── renderer.rs
        │       └── kerning.rs
        └── managers/
            ├── mod.rs
            └── renderer.rs
```

---

## 6. Models — data structures

### `models/connection.rs` — `WaylandConnection`

The low-level socket wrapper. Wraps a `UnixStream` and exposes four communication methods.

**Fields:**
- `stream: UnixStream` — the open socket connected to the Wayland compositor.

**Methods:**

`new(stream: UnixStream) -> WaylandConnection`
Constructs a connection from an already-open stream.

`send(&mut self, data: &[u8]) -> Result<(), String>`
Sends a slice of bytes over the socket.

`send_with_fd(&mut self, data: &[u8], fd: RawFd) -> Result<(), String>`
Sends bytes alongside a file descriptor using `sendmsg` with `SCM_RIGHTS`.

`receive(&mut self, buf: &mut [u8]) -> Result<(), String>`
Reads exactly `buf.len()` bytes from the socket.

`set_nonblocking(&self, nonblocking: bool) -> Result<(), String>`
Switches the socket between blocking and non-blocking mode.

---

### `models/registry.rs` — `WaylandGlobal` and `WaylandRegistry`

**`WaylandGlobal`** — a single service announced by the compositor.
Fields: `name: String`, `version: u32`, `id: u32`.

**`WaylandRegistry`** — the complete list of services the compositor offers.
Fields: `globals: Vec<WaylandGlobal>`.

---

### `models/shared_memory.rs` — `WaylandMemory`

Fields: `fd: i32`, `size: usize`.

---

### `models/window.rs` — `Window`

The central platform object. Implements `AzureWindowProvider` from Azure Core.

**Fields:** `surface_id`, `buffer_id`, `pool_id`, `xdg_toplevel_id`, `xdg_wm_id`, `xdg_surface_id`, `width: i32`, `height: i32`, `ptr: *mut u8`, `connection: WaylandConnection`.

**`AzureWindowProvider` implementation:**
- `render(pixels: &[u8])` — copies pixel buffer into shared memory.
- `poll_event() -> Option<WindowEvent>` — non-blocking event polling.

---

### `models/object_id_allocator.rs` — `ObjectIdAllocator`

Monotonic counter for Wayland object ids starting at `4`.

---

### `rendering/models/color.rs` — `Color`

```rust
pub struct Color { pub r: u8, pub g: u8, pub b: u8, pub a: u8 }
```
`Color::new(r, g, b, a) -> Color`

---

### `rendering/models/pixel.rs` — `Pixel`

```rust
pub struct Pixel { pub x: u32, pub y: u32, pub color: Color }
```

---

### `rendering/models/canvas.rs` — `Canvas`

```rust
pub struct Canvas { pub width: u32, pub height: u32, pub buffer: Vec<u8> }
```
`Canvas::new(width, height) -> Canvas` — allocates `width * height * 4` bytes initialized to zero.

---

### `rendering/models/glyph.rs` — `Glyph`

Holds a rasterized glyph's geometry after extraction from a TTF font.

```rust
pub struct Glyph {
    pub width: f32,
    pub height: f32,
    pub advance_width: f32,
    pub contours: Vec<Vec<(f32, f32)>>,
}
```

- `width` / `height` — bounding box in scaled pixels.
- `advance_width` — horizontal cursor advance after drawing this glyph.
- `contours` — the outline as a list of closed polygons, each a list of `(x, y)` points in `[0, width] × [0, height]` space (origin at bottom-left, matching font coordinate conventions).

---

## 7. Managers — logic and operations

### `managers/connection_manager.rs`
`connect() -> Result<WaylandConnection, String>` — opens the Wayland socket.

### `managers/registry_manager.rs`
- `get_registry(connection) -> Result<WaylandRegistry, String>`
- `find_global(registry, interface) -> Option<u32>`

### `managers/bind_manager.rs`
`bind_global(connection, name, interface, version, new_id) -> Result<u32, String>`

### `managers/shared_memory_manager.rs`
- `create_shared_memory(size) -> Result<WaylandMemory, String>`
- `map_memory(memory) -> Result<*mut u8, String>`
- `unmap_memory(ptr, size) -> Result<(), String>`

### `managers/shm_manager.rs`
- `create_shm_pool(connection, fd, size) -> Result<u32, String>`
- `create_buffer(connection, pool_id, width, height) -> Result<u32, String>`

### `managers/compositor_manager.rs`
`create_surface(connection, compositor_id) -> Result<u32, String>`

### `managers/xdg_manager.rs`
- `get_xdg_surface(connection, xdg_wm_base_id, surface_id) -> Result<u32, String>`
- `get_toplevel(connection, xdg_surface_id) -> Result<u32, String>`
- `ack_configure(connection, xdg_surface_id, serial) -> Result<(), String>`
- `attach(connection, surface_id, buffer_id) -> Result<(), String>`

### `managers/surface_manager.rs`
- `commit(connection, surface_id) -> Result<(), String>`
- `damage_buffer(connection, surface_id, x, y, width, height) -> Result<(), String>` — signals to Wayland which region has changed. Must be called before every `commit` after drawing.
- `wait_for_configure(connection, xdg_surface_id) -> Result<u32, String>`
- `run_event_loop(window, xdg_surface_id, xdg_toplevel_id, surface_id, xdg_wm_base_id) -> Result<(), String>`

### `managers/window_manager.rs`
`window_create(width, height) -> Result<Window, String>` — full creation sequence.

---

## 8. Rendering module

The rendering module is a CPU-based software renderer operating on a flat pixel buffer. It is entirely independent of Wayland — it writes to a `Canvas` in memory, and the caller copies the canvas to the window's shared memory before committing.

### The rendering pipeline

```
draw_*(canvas)           → writes pixels into canvas.buffer
copy_nonoverlapping()    → copies canvas.buffer into window.ptr()
damage_buffer()          → marks the region as changed
attach() + commit()      → Wayland displays the result
```

### Pixel addressing

Every pixel is addressed by the formula `(y * width + x) * 4`. Wayland expects pixels in BGRA order. The rendering module stores pixels in RGBA order internally — the BGRA conversion happens at the `set_pixel` level.

---

### `rendering/services/buffer.rs`

**`get_pixel_index(x, y, width) -> usize`**
Returns the byte offset of pixel `(x, y)` in the flat buffer.

**`set_pixel(buffer, x, y, width, height, color)`**
Writes RGBA values at the correct offset. Bounds-checked — silently ignores out-of-bounds writes.

---

### `rendering/services/shapes/`

**`rect.rs`**

`draw_rect(x, y, width, height, color, canvas)`
Fills a solid rectangle by iterating over all pixels in the bounding box.

`draw_rect_rounded(x, y, width, height, radius, color, canvas)`
Rectangle with rounded corners. Composed of three filled rects and four `draw_circle_filled` calls at the corners.

---

**`line.rs`**

`draw_line_horizontal(x, x_end, y, color, canvas)`
Horizontal line from `x` to `x_end` at fixed `y`.

`draw_line_vertical(y, y_end, x, color, canvas)`
Vertical line from `y` to `y_end` at fixed `x`.

`draw_line(x, y, x_end, y_end, color, canvas)`
General line via Bresenham's algorithm. Handles all angles and directions. Uses `i32` coordinates to support all four directional combinations.

---

**`circle.rs`**

`draw_circle(cx, cy, radius, color, canvas)`
Circle outline via the Midpoint Circle algorithm. Exploits 8-fold symmetry.

`draw_circle_filled(cx, cy, radius, color, canvas)`
Filled circle by drawing horizontal spans between symmetric boundary points.

---

### `rendering/services/effects.rs`

**`blend_pixel(buffer, x, y, width, height, color)`**
Composites a semi-transparent color over the existing pixel using the standard Porter-Duff `over` formula in sRGB space.

**`draw_gradient_horizontal(x, y, width, height, color_start, color_end, canvas)`**
Fills a rectangle with a left-to-right linear gradient between two colors.

**`draw_vertical_gradient(x, y, width, height, color_start, color_end, canvas)`**
Fills a rectangle with a top-to-bottom linear gradient between two colors.

**`draw_angular_gradiant(x, y, width, height, angle, color_start, color_end, canvas)`**
Fills a rectangle with a gradient along an arbitrary angle in degrees.

**`apply_aa(distance) -> u8`**
Returns an alpha value in `[0, 255]` for a signed distance field input. Used for smooth circle outlines.

---

### `rendering/services/text/`

The text subsystem loads TTF fonts, extracts glyph outlines, and rasterizes them with high-quality anti-aliasing. It is entirely CPU-based and depends only on `ttf-parser` for outline data.

---

**`text/loader.rs`**

`load_font(path: &str) -> Result<Vec<u8>, String>`
Reads a TTF/OTF font file from disk into a byte buffer. The buffer is passed by reference to all subsequent glyph operations.

---

**`text/glyph.rs`**

`exctract_glyph(font_data: &[u8], character: char, size: f32) -> Result<Glyph, String>`

Extracts a single character's outline from the font and returns a `Glyph` ready for rasterization.

**Pipeline:**
1. Parse the font face with `ttf_parser::Face::parse`.
2. Look up the glyph id for `character`.
3. Walk the TTF outline commands (`move_to`, `line_to`, `quad_to`, `curve_to`, `close`) using a custom `OutlineBuilder`.
4. Bezier curves are flattened via **recursive De Casteljau subdivision** with a flatness threshold of `0.5` font units — curves split until the maximum deviation of any control point from the chord is below this threshold (capped at 8 levels of recursion). This produces smooth outlines with the minimum number of line segments.
5. All contour points are scaled by `size / units_per_em` and normalized so the origin is at the glyph's bounding box minimum. The result is in `[0, width] × [0, height]` pixel space.

---

**`text/renderer.rs`**

`draw_glyph(glyph: &Glyph, x: u32, y: u32, color: &Color, canvas: &mut Canvas)`

Rasterizes a `Glyph` onto the canvas at pixel position `(x, y)` using a high-quality scanline algorithm.

**Algorithm:**

1. For each pixel row `py` (0 to `height`):
   - Cast **8 vertical sub-scanlines** at positions `py + (s + 0.5) / 8` for `s` in `0..8`.
   - For each sub-scanline, find all x-intersections with the glyph contour edges using the standard half-open interval rule (avoids double-counting shared vertices).
   - Sort intersections and fill in pairs (even-odd rule).
   - For each pixel column within a filled span, accumulate the **exact fractional horizontal coverage** of `[px, px+1]` that falls inside the span.
2. Normalize accumulated coverage across 8 sub-scanlines to get a value in `[0, 1]`.
3. Multiply by the color's own alpha channel.
4. **Blend in linear light:** convert the foreground color and background pixel from sRGB to linear, apply the Porter-Duff `over` operator, convert back to sRGB.

The gamma-correct blend step is the most visually significant: blending in perceptually-encoded sRGB space produces edges that are too dark at mid-coverage. Blending in linear space gives edges the correct physical weight — identical to what FreeType and all modern text engines produce.

---

**`text/kerning.rs`**

`get_advance(glyph: &Glyph) -> f32`
Returns the horizontal advance width of a glyph (cursor movement after drawing).

---

### `rendering/managers/renderer.rs`

The public API of the rendering module.

`draw_rect(x, y, width, height, color, canvas)`
Delegates to `shapes::rect::draw_rect`.

`draw_text(text, font_path, x, y, size, color, canvas) -> Result<(), String>`
Draws a string of characters at position `(x, y)` in the given font and size. Iterates over each character, extracts its glyph, draws it, then advances the cursor by `advance_width`.

---

### Correct rendering sequence (from tests)

```rust
let mut canvas = Canvas::new(800, 800);
let red = Color::new(255, 0, 0, 255);

draw_rect(20, 50, 160, 120, &red, &mut canvas);
draw_circle(520, 110, 60, &red, &mut canvas);
draw_gradient_horizontal(220, 430, 180, 100, &red, &blue, &mut canvas);
draw_text("Hello", "src/Sora-VariableFont_wght.ttf", 100, 700, 32.0, &red, &mut canvas)?;

unsafe {
    std::ptr::copy_nonoverlapping(
        canvas.buffer.as_ptr(),
        window.ptr(),
        canvas.buffer.len(),
    );
}

attach(window.connection_mut(), surface_id, buffer_id)?;
damage_buffer(window.connection_mut(), surface_id, 0, 0, width, height)?;
commit(window.connection_mut(), surface_id)?;
run_event_loop(&mut window, ...)?;
```

---

## 9. The full window creation sequence

```
connect()                              → opens Unix socket to compositor
get_registry()                         → discovers all available services
find_global("wl_shm/compositor/xdg")  → gets dynamic numeric ids
bind_global("wl_shm", 4)              → activates shm service
create_shared_memory(w*h*4)           → allocates RAM via memfd_create + ftruncate
map_memory(fd)                         → maps RAM into process address space
create_shm_pool(fd, size)             → tells compositor to use our RAM
create_buffer(pool, width, height)    → declares a displayable image
bind_global("wl_compositor", 7)       → activates compositor service
create_surface(compositor)            → creates blank drawing surface
bind_global("xdg_wm_base", 9)        → activates shell service
get_xdg_surface(xdg_wm_base, surf)   → promotes surface to shell management
get_toplevel(xdg_surface)             → declares it a top-level window
commit(surface)                        → triggers configure event from compositor
wait_for_configure(xdg_surface)       → waits for compositor approval
ack_configure(xdg_surface, serial)    → confirms the configuration
damage_buffer(surface, 0, 0, w, h)   → marks full buffer as changed
attach(surface, buffer)               → links pixel data to the surface
commit(surface)                        → displays the window on screen

→ caller draws into a Canvas, copies to window.ptr(), then attach+damage+commit
→ caller invokes run_event_loop() to keep window alive
→ caller invokes unmap_memory() after the loop exits
```

---

## 10. Dependencies

```toml
[dependencies]
libc = "0.2"
ttf-parser = "0.21"
azure-core = { path = "../azure-core" }
```

| Dependency | Purpose |
|---|---|
| `libc` | `memfd_create`, `ftruncate`, `mmap`, `munmap`, `sendmsg`, `CMSG_*`, `SCM_RIGHTS` |
| `ttf-parser` | TTF/OTF font parsing — glyph outline extraction, bounding boxes, advance widths |
| `azure-core` | `AzureWindowProvider` trait and `WindowEvent` enum |

---

## 11. Current capabilities and limitations

**What works:**
- Persistent, interactive window on any Linux Wayland desktop
- Dynamic compositor service discovery
- Shared memory allocation and pixel-level access
- File descriptor transmission via `SCM_RIGHTS`
- Full event loop: ping/pong, configure/ack, clean close
- `AzureWindowProvider` trait implemented — `render(pixels)` and `poll_event()`
- **CPU software renderer — fully operational:**
  - `Canvas` — drawing surface with flat pixel buffer
  - `Color` — RGBA color model
  - `set_pixel` / `get_pixel_index` — pixel-level access with bounds checking
  - `draw_rect` — filled rectangle
  - `draw_rect_rounded` — rectangle with rounded corners
  - `draw_line_horizontal` / `draw_line_vertical` — axis-aligned lines
  - `draw_line` — general line via Bresenham's algorithm
  - `draw_circle` — circle outline via Midpoint Circle algorithm
  - `draw_circle_filled` — filled circle
  - `blend_pixel` — Porter-Duff alpha compositing
  - `draw_gradient_horizontal` / `draw_vertical_gradient` — linear gradients
  - `draw_angular_gradiant` — gradient along an arbitrary angle
  - `draw_text` — high-quality TTF text rendering with full AA

**Current limitations:**
- Wayland pixel order is BGRA — conversion handled at `set_pixel` level
- Object ids in `window_manager.rs` partially hardcoded
- No keyboard/mouse input routing yet
- No dynamic resize

---

## 12. What comes next

**Input routing:**
- Bind `wl_seat`, parse keyboard and pointer events
- Return as `WindowKeyPress`, `WindowMouseMove` variants from `poll_event()`

**Dynamic resize:**
- Recreate shared memory and buffer on `WindowResize` event

**Future platforms:**
- `win32/` — `AzureWindowProvider` implementation using the Win32 API (CreateWindow, GDI shared memory)
- `cocoa/` — `AzureWindowProvider` implementation using Cocoa / Core Graphics on macOS

All platform-specific code will live under `src/platform/<target>/` behind the same `AzureWindowProvider` trait, keeping Foundation and all applications fully portable.

---

*Azure Engine is built entirely from scratch, without any GUI framework or Wayland binding library. Every byte of every Wayland message is constructed manually. The rendering module follows the same philosophy: no graphics library, pure CPU rasterization from first principles.*
