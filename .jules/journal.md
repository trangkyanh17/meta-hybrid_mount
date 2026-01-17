## 2025-10-31 - Cargo Lock Sync & Safe Xattr
**洞察:** `Cargo.lock` 与 `Cargo.toml` 版本不一致 (`2.0.72` vs `2.1.2`) 导致构建时隐式更新。
**准则:** 提交前检查 `git diff Cargo.lock`，确认是预期的同步修复。

**洞察:** `src/utils.rs` 中混合使用了 `unsafe libc` 和安全的 `extattr` crate。
**准则:** 优先使用已引入的安全 Rust 包装库 (`extattr`) 替代手动 `unsafe` FFI 调用，特别是对于文件系统操作。
