# Azure Engine — Technical Documentation

**Version:** V0.1  
**Language:** Rust (2024 edition)  
**Platform:** Linux (Wayland)  
**Status:** Active development — persistent interactive window achieved

---

## Table of Contents

1. [What is Azure Engine?](#1-what-is-azure-engine)
2. [Where it fits in the Azure ecosystem](#2-where-it-fits-in-the-azure-ecosystem)
3. [Core concepts you need to understand first](#3-core-concepts-you-need-to-understand-first)
4. [Architecture overview](#4-architecture-overview)
5. [Project structure](#5-project-structure)
6. [Models — data structures](#6-models--data-structures)
7. [Managers — logic and operations](#7-managers--logic-and-operations)
8. [The full window creation sequence](#8-the-full-window-creation-sequence)
9. [Dependencies](#9-dependencies)
10. [Current capabilities and limitations](#10-current-capabilities-and-limitations)
11. [What comes next](#11-what-comes-next)

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

---

## 4. Architecture overview

The codebase follows a strict model/manager separation, consistent with the conventions established in Azure Core:

**Models** hold data. They are plain structs with constructors and getters, no business logic.

**Managers** hold logic. They are collections of free functions that operate on models and send/receive messages over the connection.

The `Window` struct is the central object exposed to the rest of the platform. It owns the `WaylandConnection`, all Wayland object ids, the pixel pointer, and the dimensions. It implements the `AzureWindowProvider` trait from Azure Core, making it usable by Foundation and applications without any Wayland-specific knowledge on their side.

---

## 5. Project structure

```
azure-engine/
├── Cargo.toml                          # Package definition and dependencies
└── src/
    ├── lib.rs                          # Crate root — declares modules, houses tests
    └── platform/
        ├── mod.rs                      # Declares the wayland submodule
        └── wayland/
            ├── mod.rs                  # Declares models and managers submodules
            ├── models/
            │   ├── mod.rs
            │   ├── connection.rs           # WaylandConnection — socket wrapper
            │   ├── registry.rs             # WaylandGlobal + WaylandRegistry
            │   ├── shared_memory.rs        # WaylandMemory — shared memory descriptor
            │   ├── window.rs               # Window — the main platform object
            │   └── object_id_allocator.rs  # ObjectIdAllocator — dynamic id assignment
            └── managers/
                ├── mod.rs
                ├── connection_manager.rs    # Opens the Wayland socket
                ├── registry_manager.rs      # Discovers compositor services
                ├── bind_manager.rs          # Binds (activates) any service
                ├── shared_memory_manager.rs # Creates, maps, and unmaps shared memory
                ├── shm_manager.rs           # Creates pixel pool and buffer
                ├── compositor_manager.rs    # Creates a drawing surface
                ├── xdg_manager.rs           # Promotes surface to a desktop window
                ├── surface_manager.rs       # Commits, event loop, configure handling
                └── window_manager.rs        # window_create() — full creation sequence
```

---

## 6. Models — data structures

### `models/connection.rs` — `WaylandConnection`

The low-level socket wrapper. Wraps a `UnixStream` and exposes four communication methods.

**Fields:**
- `stream: UnixStream` — the open socket connected to the Wayland compositor.

**Methods:**

`new(stream: UnixStream) -> WaylandConnection`
Constructs a connection from an already-open stream. Called exclusively by `connection_manager::connect()`.

`send(&mut self, data: &[u8]) -> Result<(), String>`
Sends a slice of bytes over the socket. Used for all standard Wayland messages.

`send_with_fd(&mut self, data: &[u8], fd: RawFd) -> Result<(), String>`
Sends bytes alongside a file descriptor using `sendmsg` with `SCM_RIGHTS`. Required for transmitting shared memory to the compositor. Internally constructs `libc::msghdr` and `libc::cmsghdr` in `unsafe` blocks.

`receive(&mut self, buf: &mut [u8]) -> Result<(), String>`
Reads exactly `buf.len()` bytes from the socket using `read_exact`.

`set_nonblocking(&self, nonblocking: bool) -> Result<(), String>`
Switches the socket between blocking and non-blocking mode. Used by `poll_event` to attempt a read without waiting.

---

### `models/registry.rs` — `WaylandGlobal` and `WaylandRegistry`

**`WaylandGlobal`** — a single service announced by the compositor.

Fields: `name: String`, `version: u32`, `id: u32`.

**`WaylandRegistry`** — the complete list of services the compositor offers.

Fields: `globals: Vec<WaylandGlobal>`.

---

### `models/shared_memory.rs` — `WaylandMemory`

A descriptor for a region of shared memory.

**Fields:**
- `fd: i32` — the kernel file descriptor referencing the memory region.
- `size: usize` — the total size in bytes.

---

### `models/window.rs` — `Window`

The central platform object. Owns the connection, all Wayland object ids, the pixel pointer, and the dimensions. Implements `AzureWindowProvider` from Azure Core.

**Fields:**
- `surface_id: u32` — the `wl_surface` object id.
- `buffer_id: u32` — the `wl_buffer` object id.
- `pool_id: u32` — the `wl_shm_pool` object id.
- `xdg_toplevel_id: u32` — the `xdg_toplevel` object id.
- `xdg_wm_id: u32` — the `xdg_wm_base` object id.
- `xdg_surface_id: u32` — the `xdg_surface` object id.
- `width: i32`, `height: i32` — window dimensions in pixels.
- `ptr: *mut u8` — raw pointer to the shared memory pixel buffer.
- `connection: WaylandConnection` — the owned Wayland socket connection.

**`AzureWindowProvider` implementation:**

`width() -> i32` / `height() -> i32` — return current dimensions.

`render(pixels: &[u8]) -> Result<(), String>` — copies a pixel buffer into the shared memory via `from_raw_parts_mut` and `copy_from_slice`. The buffer must be exactly `width * height * 4` bytes in ARGB8888 format.

`poll_event() -> Option<WindowEvent>` — attempts a non-blocking read from the socket. If no data is available, returns `None` immediately. Otherwise parses the incoming message and returns the matching `WindowEvent` variant (`WindowClose`, `WindowResize`, etc.).

**Additional method:**

`connection_mut(&mut self) -> &mut WaylandConnection` — gives mutable access to the connection for use by managers that operate on a `&mut Window`.

---

### `models/object_id_allocator.rs` — `ObjectIdAllocator`

A simple monotonic counter for assigning Wayland object ids.

**Fields:**
- `next: u32` — the next available id. Starts at `4` (ids 1-3 are reserved by `get_registry`).

**Methods:**

`new() -> ObjectIdAllocator` — initializes the allocator starting at id `4`.

`next_id(&mut self) -> u32` — returns the current id and increments the counter.

**Note:** Currently `window_manager.rs` still uses hardcoded ids for the initial window creation sequence. The allocator is available and ready; it will be integrated when multi-window support is needed.

---

## 7. Managers — logic and operations

### `managers/connection_manager.rs`

**Function:** `connect() -> Result<WaylandConnection, String>`

Reads `WAYLAND_DISPLAY` and `XDG_RUNTIME_DIR`, constructs the socket path, and opens a `UnixStream` to it.

---

### `managers/registry_manager.rs`

**Functions:**
- `get_registry(connection) -> Result<WaylandRegistry, String>`
- `find_global(registry, interface) -> Option<u32>`

`get_registry` sends `wl_display.get_registry` and `wl_display.sync`, then reads messages in a loop until the sync callback fires, parsing each `wl_registry.global` event into a `WaylandGlobal`.

`find_global` looks up a service by interface name in the registry and returns its numeric id. Must be called with the result of `get_registry` from the same session — ids vary between compositor sessions.

---

### `managers/bind_manager.rs`

**Function:** `bind_global(connection, name, interface, version, new_id) -> Result<u32, String>`

Sends a `wl_registry.bind` message to activate a compositor service. Encodes the interface name as a Wayland string (null-terminated, 4-byte padded). Used for `wl_shm`, `wl_compositor`, and `xdg_wm_base`.

---

### `managers/shared_memory_manager.rs`

**Functions:**
- `create_shared_memory(size) -> Result<WaylandMemory, String>` — calls `memfd_create` + `ftruncate`.
- `map_memory(memory) -> Result<*mut u8, String>` — calls `mmap`, returns a raw pointer to the pixel buffer.
- `unmap_memory(ptr, size) -> Result<(), String>` — calls `munmap` to release the mapping after the window closes.

---

### `managers/shm_manager.rs`

**Functions:**
- `create_shm_pool(connection, fd, size) -> Result<u32, String>` — sends the fd to the compositor via `send_with_fd`.
- `create_buffer(connection, pool_id, width, height) -> Result<u32, String>` — declares a rectangular ARGB8888 image from the pool.

---

### `managers/compositor_manager.rs`

**Function:** `create_surface(connection, compositor_id) -> Result<u32, String>`

Requests a blank `wl_surface` from the compositor.

---

### `managers/xdg_manager.rs`

**Functions:**
- `get_xdg_surface(connection, xdg_wm_base_id, surface_id) -> Result<u32, String>` — promotes a surface to shell management.
- `get_toplevel(connection, xdg_surface_id) -> Result<u32, String>` — declares it a top-level desktop window.
- `ack_configure(connection, xdg_surface_id, serial) -> Result<(), String>` — confirms a configure event from the compositor.
- `attach(connection, surface_id, buffer_id) -> Result<(), String>` — links a pixel buffer to a surface.

---

### `managers/surface_manager.rs`

**Functions:**
- `commit(connection, surface_id) -> Result<(), String>` — tells the compositor to apply pending changes and display the surface.
- `wait_for_configure(connection, xdg_surface_id) -> Result<u32, String>` — blocking loop that waits for the first configure event and returns its serial.
- `run_event_loop(window, xdg_surface_id, xdg_toplevel_id, surface_id, xdg_wm_base_id) -> Result<(), String>` — the main application loop. Runs until the user closes the window. Handles: compositor ping/pong (keeps the window responsive), `xdg_surface.configure` (acks and commits), `xdg_toplevel.close` (exits cleanly), and `wl_display.error` (returns a descriptive error).

---

### `managers/window_manager.rs`

**Function:** `window_create(width, height) -> Result<Window, String>`

Orchestrates the entire window creation sequence in a single call:

1. Opens the Wayland connection
2. Discovers compositor services via the registry
3. Binds `wl_shm`, `wl_compositor`, `xdg_wm_base`
4. Allocates and maps shared memory, fills it with white pixels
5. Creates the shm pool and buffer
6. Creates the surface, xdg_surface, and toplevel
7. Performs the configure/ack handshake
8. Attaches the buffer and commits
9. Returns a fully initialized `Window` ready for use

---

## 8. The full window creation sequence

The complete sequence, as executed by `window_create`:

```
connect()                              → opens Unix socket to compositor
get_registry()                         → discovers all available services
find_global("wl_shm/compositor/xdg")  → gets dynamic numeric ids
bind_global("wl_shm", 4)              → activates shm service
create_shared_memory(w*h*4)           → allocates RAM via memfd_create + ftruncate
map_memory(fd)                         → maps RAM into process address space
pixels.fill(0xFF)                      → writes white pixels (ARGB8888)
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
attach(surface, buffer)               → links pixel data to the surface
commit(surface)                        → displays the window on screen

→ caller invokes run_event_loop() to keep window alive
→ caller invokes unmap_memory() after the loop exits
```

After `window_create`, the caller has a `Window` object. It calls `run_event_loop(&mut window, ...)` to keep it alive and responsive, and `unmap_memory` to clean up when done.

---

## 9. Dependencies

```toml
[dependencies]
libc = "0.2"
azure-core = { path = "../azure-core" }
```

| Dependency | Purpose |
|---|---|
| `libc` | System calls not in Rust std: `memfd_create`, `ftruncate`, `mmap`, `munmap`, `sendmsg`, `CMSG_*`, `SCM_RIGHTS` |
| `azure-core` | `AzureWindowProvider` trait and `WindowEvent` enum |

---

## 10. Current capabilities

**What works:**
- Opening a persistent, interactive window on any Linux Wayland desktop (GNOME, KDE, Sway...)
- Dynamic compositor service discovery — no hardcoded session-specific ids
- Shared memory allocation and pixel-level access via `mmap`
- File descriptor transmission between processes via `SCM_RIGHTS`
- Full event loop: ping/pong keepalive, configure/ack, clean close on user request
- Protocol error detection with descriptive messages
- `AzureWindowProvider` trait implemented — Foundation-ready API
- `render(pixels)` — write arbitrary pixel content to the window
- `poll_event()` — non-blocking event polling returning `WindowEvent`
- `ObjectIdAllocator` — dynamic id assignment ready for multi-window use
- Proper memory cleanup via `unmap_memory` on window close
- Single entry point: `window_create(width, height)` encapsulates the entire sequence

**Current limitations:**
- Object ids in `window_manager.rs` are still partially hardcoded — `ObjectIdAllocator` exists but is not yet wired in
- No keyboard/mouse input routing — `poll_event` detects close and resize, but not key presses or mouse moves yet
- No dynamic resize — `WindowResize` events are detected but the buffer is not recreated
- `rendering/` module does not exist yet — only solid color fill is possible
- Linux/Wayland only — `win32/` and `cocoa/` implementations not started

---

## 11. What comes next

**Dynamic resize** — when `poll_event` returns `WindowResize(w, h)`, recreate the shared memory, pool, buffer, and remap pixels at the new size.

**Input routing** — bind `wl_seat`, parse keyboard and mouse events, return them as `WindowKeyPress` and `WindowMouseMove` variants.

**`rendering/` module** — drawing primitives from scratch:
- Rectangles, lines, circles — pixel-level rasterization
- Alpha blending and transparency
- Font loading and text rendering (Bézier glyph rasterization) — the largest remaining piece of work in Engine

**`win32/` and `cocoa/`** — future platform implementations of `AzureWindowProvider` for Windows and macOS.

---

*Azure Engine is built entirely from scratch, without any GUI framework or Wayland binding library. Every byte of every Wayland message is constructed manually. This is intentional: the goal is complete understanding and control of the entire display stack, from the Unix socket to the pixels on screen.*
