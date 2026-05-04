# 参与贡献

想加功能或者修 bug 的话，先开个 issue 说一声，避免重复劳动。

## 开发环境

Rust stable 就行，没别的特殊要求。前端部分需要 Node 20+。

```bash
rustup default stable
cargo --version  # >= 1.75
node --version   # >= 20
```

Linux 的话记得装系统依赖（README 里写了）。

## 代码风格

不用纠结，提交前跑一下：

```bash
cargo fmt --all --check
cargo clippy --workspace --exclude cicada-client -- -D warnings
```

前端这边：

```bash
cd crates/cicada-client/ui && npx tsc --noEmit
```

CI 也会跑这些，不过 push 之前自己查一遍省得被打回来。

## 测试

写新功能之前先写测试。Rust 这边 `cargo test` 跑得很快，没什么理由跳过。

```bash
# 全部测试（除了客户端，那个要 GUI 环境）
cargo test --workspace --exclude cicada-client

# 只跑某个模块
cargo test -p cicada-core -- auth
cargo test -p cicada-api -- rest
```

Mock 服务器用来测 API 交互，不用连真实后端：

```bash
cargo run -p cicada-mock-server
# 另开终端
curl http://localhost:3001/api/v1/announcements
```

## 提 PR 之前

1. 确保改动只涉及你要修的东西，别顺手重构无关代码
2. `cargo test --workspace --exclude cicada-client` 全绿
3. `cargo fmt` 和 `cargo clippy` 没告警
4. 如果是新功能，补上测试
5. commit message 别太随意，至少说清楚改了什么、为什么

## 项目结构指南

- **cicada-core** — 如果改动涉及类型定义、OAuth、WebSocket、配置，改这里。这个 crate 被其他所有 crate 依赖，改之前想清楚影响范围。
- **cicada-api** — REST 客户端封装。和 API 规范（`api-spec/openapi.yaml`）保持一致。
- **cicada-client** — 桌面应用。Rust 部分管托盘、窗口、系统集成；React 部分管 UI。前后端通过 Tauri `invoke` / `event` 通信。
- **cicada-mock-server** — 纯开发工具，不要往里面加生产级逻辑。用内存存储，重启数据就没了。

## 架构原则

- Rust 后端不碰 DOM，前端不碰文件系统。通过 Tauri IPC 拆清楚。
- 配置全部走 `cicada-core/src/config/`，不要在各个模块里自己读文件。
- Token 存系统钥匙串，不要明文写磁盘，不要在 commit 里留任何密钥。
- WebSocket 重连逻辑在 `ws/client.rs` 里统一处理，上层只收 `WsEvent`。

## 遇到问题

- Tauri 部分编译不过通常是系统缺 webkit/gtk 库，README 里有装法
- OAuth 流程调试可以先对着 Mock 服务器测，别反复打真实 SECTL
- 不知道某个改动放哪个 crate 合适？开 issue 问

## License

和项目一样，GPL-3.0。你提的代码默认按这个协议。
