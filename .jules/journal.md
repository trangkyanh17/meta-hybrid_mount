## 2026-01-16 - Global Mutex Safety
**洞察:** `src/try_umount.rs` uses global `static` `Mutex` (`HISTORY`, `LIST`). Accessing them with `unwrap()` risks panic propagation if a thread gets poisoned, potentially crashing the service.
**准则:** Always use `anyhow` (`.map_err(...)`) to handle `Mutex` locking errors for global state to prevent service crashes and provide better context.
