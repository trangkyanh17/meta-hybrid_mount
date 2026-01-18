## 2024-05-23 - Secure Temporary File Creation
**洞察:** `src/utils.rs` 中的 `atomic_write` 之前使用基于时间戳和 PID 的伪随机文件名，这在安全性要求高的后端环境中（尤其是 Android root 环境）是不够安全的，可能导致竞争条件或预测攻击。
**准则:** 必须使用 `/dev/urandom` 获取加密安全的随机字节来生成临时文件名，并配合 `OpenOptions::new().create_new(true)` 确保原子性创建。避免仅仅为了随机数生成而引入重量级的 `rand` crate。
