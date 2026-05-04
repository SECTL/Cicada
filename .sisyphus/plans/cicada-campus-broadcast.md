# Cicada（知了）校园公告投屏系统 — 工作规划

## TL;DR

> **快速总结**：构建跨平台（Win7+/Linux/macOS）校园公告投屏桌面客户端，教室大屏实时接收公告，管理端一键发布。集成 SECTL 统一认证（OAuth 2.0 + PKCE），WebSocket 实时双向通信，支持手机 Web 端发布。
>
> **交付物**：
> - 桌面客户端（Tauri v1 + React/TS + Rust）：显示模式 + 管理模式
> - 系统托盘、设置窗口、多屏幕浮窗
> - 手机 Web PWA（发布公告）
> - API 规范文档（OpenAPI）+ 本地 Mock 服务器
> - 核心库（可复用的 Rust crate）
>
> **预估工作量**：Large（25 个任务，7 个并行波次）
> **并行执行**：YES — 7 个波次
> **关键路径**：Task 1 → 5 → 7 → 13 → 17 → 23 → F1-F4 → 用户确认

---

## 背景

### 原始需求
构建「知了」校园公告投屏系统。后台一键发布公告，教室大屏实时显示。支持系统托盘、设置窗口、置顶/正常层级浮窗。集成已有 SECTL 认证平台。支持自定义服务器，默认提供官方服务。

### 访谈总结

**关键决策**：
- **GUI 框架**：Tauri v1 — Web 前端灵活 + Rust 后端高性能，支持 Win7
- **前端**：React + TypeScript
- **程序架构**：单程序 + 模式切换（显示模式 / 管理模式，切换需 OAuth 登录）
- **服务端范围**：先做客户端 + API 规范 + Mock 服务器，服务端后补
- **手机端**：Web PWA（响应式，发布公告用）
- **平台**：Windows 7+ / Linux / macOS，x86 + x64
- **浮窗行为**：常驻可显隐，置顶/正常层级切换（非贴桌面式置底）
- **消息格式**：HTML 富文本 + 图片，支持普通/紧急/通知三种类型
- **WebSocket**：双向通信（推送 + ACK + 心跳）
- **认证**：OAuth 2.0 + PKCE（强制），对接 SECTL（`https://appwrite.sectl.cn`）
- **开机**：自启 → 最小化托盘 → 浮窗自动显示
- **更新**：静默下载 + 重启安装
- **发布者**：自动获取 SECTL user_name，可手动覆盖
- **测试**：TDD（`cargo test`）

**SECTL-auth 集成发现**：
- Base URL: `https://appwrite.sectl.cn`
- OAuth 端点：`/oauth/authorize` → `/api/oauth/token` → `/api/oauth/userinfo`
- 强制 PKCE，无需 client_secret（公开客户端）
- 需要 device_uuid（标准 UUID 格式）
- Access Token 可设永不过期（`expires_in: -1`）
- 支持 Refresh Token 轮换
- 远程退登：Token 失效检测 + 本地清理
- Userinfo 返回：user_id, user_name, email, permission

### Metis 自审（缺口分析）

**已识别并处理**：
- 离线/断网行为：WS 断连自动重连（指数退避），Token 过期自动刷新
- 多公告并发：客户端队列管理，按时间/优先级排序
- 长内容/图片加载失败：滚动容器 + 降级占位
- HiDPI/多分辨率：Tauri 自动处理，CSS 响应式
- 浮窗交互：非交互区域点击穿透（CSS `pointer-events`）

**范围锁定（明确排除）**：
- ❌ 服务端实现（仅规范 + Mock）
- ❌ iOS 原生 App
- ❌ 消息定时发布/模板/统计面板
- ❌ 真正的「贴桌面式」置底（跨平台不可靠）
- ❌ 多教室分组定向发布（v2 功能）

**默认假设**（可覆盖）：
- 每台教室电脑有唯一 device_uuid 用于标识
- 浮窗默认右下角，尺寸 400×300px，半透明背景
- 紧急通知：红色边框 + 闪烁标题（CSS animation）
- Mock 服务器用 Python（快速开发）

---

## 工作目标

### 核心目标
构建「知了」桌面客户端，实现教室大屏实时公告显示 + 管理端公告发布，集成 SECTL 认证，支持手机 Web 发布。

### 具体交付物
- `crates/cicada-core/` — 共享核心库（类型、配置、认证、WS 客户端）
- `crates/cicada-client/` — Tauri 桌面应用（显示 + 管理模式）
- `crates/cicada-api/` — API 客户端 + 类型定义
- `crates/cicada-mock-server/` — 本地开发 Mock 服务器
- `mobile/` — 手机 Web PWA
- `api-spec/openapi.yaml` — API 规范文档
- `.github/workflows/ci.yml` — CI 配置（跨平台构建）

### 完成定义
- [ ] `cargo build --workspace` 三平台通过
- [ ] `cargo test --workspace` 全部通过
- [ ] 浮窗可显示 HTML 公告，置顶/正常层级可切换
- [ ] OAuth 登录流程可走通（Mock 模式 + 真实 SECTL）
- [ ] 手机 Web 可发布公告，桌面端实时接收

### 必须包含
- 系统托盘（常驻，右键菜单：显示浮窗/隐藏浮窗/设置/退出）
- 设置窗口（服务器地址、OAuth 配置、显示样式、浮窗行为、开机自启）
- 浮窗（HTML 渲染、置顶/正常层级、拖拽移动、调整大小、多屏幕）
- WebSocket 双向通信（推送公告、ACK 确认、心跳保活）
- Token 自动刷新 + 安全存储

### 必须排除（护栏）
- ❌ 不实现服务端业务逻辑（仅 Mock）
- ❌ 不使用已废弃的 API（Tauri v2 only 的功能）
- ❌ 不在浮窗中使用 `alert()` / `prompt()` 等阻塞 UI
- ❌ 不硬编码 SECTL client_id / 服务器地址（必须可配置）
- ❌ 不在前端存储敏感 Token（由 Rust 后端安全管理）

---

## 验证策略

> **零人工干预** — 所有验证由 Agent 执行。不允许 "用户手动测试/确认" 的验收标准。

### 测试决策
- **基础设施**：不存在（全新项目），将在 Wave 1 搭建
- **自动化测试**：TDD（测试驱动开发）
- **框架**：`cargo test`（Rust 端）+ `vitest`（前端 React 组件）+ `playwright`（E2E）
- **TDD 流程**：每个任务遵循 RED（写失败测试）→ GREEN（最小实现）→ REFACTOR

### QA 策略
每个任务必须包含 Agent 可执行的 QA 场景（详见 TODO 模板）。
证据保存至 `.sisyphus/evidence/task-{N}-{场景名}.{扩展名}`。

- **Rust 后端**：`cargo test` + `cargo clippy`
- **React 前端**：Playwright 浏览器自动化
- **API**：curl 发送请求，验证响应
- **桌面集成**：Playwright + Tauri（通过 WebView 交互）
- **Mock 服务器**：curl 测试端点

---

## 执行策略

### 并行执行波次

> 最大化并行吞吐。每波独立任务可同时执行。
> 目标：每波 5-8 个任务。最终集成波除外。

```
Wave 1（即刻开始 — 基础设施 + 类型定义）：
├── Task 1: Rust Workspace 搭建 [quick]
├── Task 2: 核心类型定义 [quick]
├── Task 3: 配置管理模块 [quick]
├── Task 4: API 规范文档 (OpenAPI) [quick]
├── Task 5: OAuth PKCE 认证客户端 [deep]
├── Task 6: CI/CD 配置 [quick]

Wave 2（Wave 1 完成后 — 核心通信层，最大并行）：
├── Task 7: WebSocket 客户端 [deep]
├── Task 8: REST API 客户端 [quick]
├── Task 9: Mock 服务器 [deep]
├── Task 10: Tauri 项目脚手架 + React 集成 [visual-engineering]

Wave 3（Wave 2 完成后 — 桌面基础设施，最大并行）：
├── Task 11: 系统托盘 [deep]
├── Task 12: 窗口管理器 [deep]
├── Task 13: 开机自启 + 静默启动 [quick]
├── Task 14: Token 安全存储 [quick]
├── Task 15: 自动更新机制 [deep]

Wave 4（Wave 3 完成后 — 显示模式 UI，最大并行）：
├── Task 16: 浮窗组件（HTML 渲染器）[visual-engineering]
├── Task 17: 多屏幕浮窗管理器 [visual-engineering]
├── Task 18: 公告显示样式（普通/紧急/通知）[visual-engineering]

Wave 5（Wave 3 完成后 — 管理模式 UI，与 Wave 4 并行）：
├── Task 19: 登录流程 UI [visual-engineering]
├── Task 20: 公告编辑器（富文本 + 发布）[visual-engineering]
├── Task 21: 设置窗口 [visual-engineering]
├── Task 22: 模式切换逻辑 [deep]

Wave 6（Wave 4 + Wave 5 完成后 — 移动端 + 集成）：
├── Task 23: 手机 Web PWA [visual-engineering]
├── Task 24: 端到端集成测试 [deep]
├── Task 25: 跨平台构建打包 [deep]

Wave FINAL（所有任务完成后 — 4 个并行审查，然后等用户确认）：
├── Task F1: 计划合规审计 (oracle)
├── Task F2: 代码质量审查
├── Task F3: 实际手工 QA
├── Task F4: 范围保真度检查
→ 汇总结果 → 获得用户明确确认
```

**关键路径**：Task 1 → Task 5 → Task 7 → Task 16 → Task 18 → Task 24 → F1-F4 → 用户确认
**并行加速**：约 65% 比完全串行快
**最大并发**：7 个任务（Wave 2）

### 依赖矩阵

- **1**: — — 2, 3, 4, 5, 6 (Wave 1)
- **2-4**: 1 — 5, 7, 8, 9, 10 (Wave 2)
- **5**: 1, 2 — 7, 8, 19 (Wave 2 + 5)
- **6**: 1 — — (独立)
- **7**: 2, 5 — 16, 18, 24 (Wave 3+)
- **8**: 2, 5 — 19, 20, 21, 24 (Wave 3+)
- **9**: 4 — 24 (独立)
- **10**: 1 — 11, 12, 13, 14, 15 (Wave 3)
- **11-15**: 10 — 16-22 (Wave 4+5)
- **16**: 7, 10, 12 — 17, 18, 24 (Wave 4+)
- **17**: 12, 16 — 18, 24 (Wave 4+)
- **18**: 7, 16, 17 — 24 (Wave 4+)
- **19**: 5, 8, 10, 14 — 22, 24 (Wave 5)
- **20**: 8, 10, 19 — 22, 24 (Wave 5)
- **21**: 3, 10, 13, 14 — 24 (Wave 5)
- **22**: 10, 19, 20 — 24 (Wave 5)
- **23**: 4, 8 — 24 (Wave 6)
- **24**: 7, 8, 16-23 — 25 (Wave 6)
- **25**: 24 — F1-F4 (Wave 6)

### Agent 调度摘要

- **Wave 1**: 6 个任务 — T1-T4 → `quick`, T5 → `deep`, T6 → `quick`
- **Wave 2**: 4 个 — T7 → `deep`, T8 → `quick`, T9 → `deep`, T10 → `visual-engineering`
- **Wave 3**: 5 个 — T11 → `deep`, T12 → `deep`, T13 → `quick`, T14 → `quick`, T15 → `deep`
- **Wave 4**: 3 个 — T16-T18 → `visual-engineering`
- **Wave 5**: 4 个 — T19-T21 → `visual-engineering`, T22 → `deep`
- **Wave 6**: 3 个 — T23 → `visual-engineering`, T24 → `deep`, T25 → `deep`
- **FINAL**: 4 个 — F1 → `oracle`, F2-F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

- [ ] 1. **Rust Workspace + 项目结构搭建**

  **要做什么**：
  - 创建 workspace 级别 `Cargo.toml`，定义 `[workspace]` members：`cicada-core`, `cicada-api`, `cicada-client`, `cicada-mock-server`
  - 创建各 crate 目录骨架：`crates/{core,api,client,mock-server}/`，各自含 `Cargo.toml` 和 `src/lib.rs`（或 `src/main.rs`）
  - `cicada-core`：依赖 `serde`, `serde_json`, `chrono`, `uuid`, `thiserror`, `tokio`
  - `cicada-api`：依赖 `cicada-core`, `reqwest`, `serde`
  - `cicada-client`：Tauri v1 项目，依赖 `cicada-core`, `cicada-api`, `tauri` v1.x, `tokio-tungstenite`, `tray-icon`
  - `cicada-mock-server`：依赖 `cicada-core`, `tokio`, `warp`（或 `axum`）, `serde_json`
  - 创建 `mobile/` 目录（后续填充 React PWA）
  - 创建 `api-spec/` 目录（OpenAPI YAML）
  - 创建 `scripts/` 目录（Mock 服务器辅助脚本）
  - 配置 `.gitignore`：排除 `target/`, `node_modules/`, `dist/`

  **不得做**：
  - ❌ 不要添加未讨论的依赖（如数据库驱动）
  - ❌ 不要使用 Tauri v2 API

  **推荐 Agent Profile**：
  - **Category**: `quick`
    - 原因：纯项目结构搭建，不涉及复杂逻辑
  - **Skills**: [`git-master`]
    - `git-master`：需要创建初始提交

  **并行化**：
  - **可并行运行**：NO（Workspace 是其他所有任务的基础）
  - **并行组**：Wave 1（序列启动，后续任务依赖此结构）
  - **阻塞**：Task 2-6, 10
  - **被阻塞**：无（可立即开始）

  **参考**：
  - Tauri v1 项目结构：`https://v1.tauri.app/v1/guides/getting-started/setup/`
  - Cargo workspace 模式：标准的 `[workspace]` + `members` 配置

  **验收标准**：
  - [ ] `cargo build --workspace` 成功（即使各 crate 几乎为空）
  - [ ] 目录结构符合上述规划
  - [ ] `.gitignore` 覆盖 Rust + Node 构建产物

  **QA 场景**：
  ```
  Scenario: 工作区构建成功
    Tool: Bash
    Steps:
      1. cargo build --workspace
      2. 验证退出码为 0
      3. 检查 target/ 目录生成
    Expected Result: 全部 crate 编译通过，无错误
    Evidence: .sisyphus/evidence/task-1-build.txt

  Scenario: 目录结构完整
    Tool: Bash
    Steps:
      1. ls crates/cicada-core/src/lib.rs
      2. ls crates/cicada-api/src/lib.rs
      3. ls crates/cicada-client/src/main.rs
      4. ls crates/cicada-mock-server/src/main.rs
      5. ls mobile/
      6. ls api-spec/
    Expected Result: 所有路径存在
    Evidence: .sisyphus/evidence/task-1-structure.txt
  ```

  **提交**：YES
  - Message: `chore: initialize Cargo workspace with crate skeleton`
  - Files: `Cargo.toml`, `crates/*/Cargo.toml`, `crates/*/src/`, `.gitignore`

- [ ] 2. **核心类型定义**

  **要做什么**：
  - 在 `crates/cicada-core/src/types/` 下定义所有共享数据类型：
    - `Announcement` 结构体：`id` (Uuid), `title` (String), `content_html` (String), `announcement_type` (enum: Normal/Emergency/Notice), `publisher_name` (String), `publisher_user_id` (String), `created_at` (DateTime<Utc>), `updated_at` (Option<DateTime<Utc>>)
    - `AnnouncementType` 枚举：`Normal`, `Emergency`, `Notice`
    - `UserInfo` 结构体：映射 SECTL `/api/oauth/userinfo` 响应字段
    - `AuthToken` 结构体：`access_token`, `refresh_token`, `expires_in`, `user_id`
    - `DeviceInfo` 结构体：`device_uuid`, `device_name`, `platform` (enum)
    - `Config` 结构体：服务器地址、OAuth client_id、显示设置、行为设置（使用 `serde` 序列化）
    - `WsMessage` 枚举：ServerPush(Announcement) / ClientAck(uuid) / Heartbeat / Error
  - 所有类型派生 `Serialize`, `Deserialize`, `Clone`, `Debug`
  - 为 `AnnouncementType` 实现 `Display` trait
  - 编写单元测试验证序列化/反序列化往返

  **不得做**：
  - ❌ 不添加业务逻辑方法（仅数据结构）
  - ❌ 不引入数据库 ORM 相关 derive

  **推荐 Agent Profile**：
  - **Category**: `quick`
    - 原因：纯类型定义，无复杂逻辑
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 3, 4 并行）
  - **并行组**：Wave 1
  - **阻塞**：Task 5, 7, 8, 9, 21
  - **被阻塞**：Task 1（需要目录结构）

  **参考**：
  - serde 文档：`https://serde.rs/derive.html`
  - SECTL userinfo 响应格式（见草案中记录的字段）

  **验收标准**：
  - [ ] `cargo build -p cicada-core` 成功
  - [ ] `cargo test -p cicada-core` 全部通过（每个类型的序列化往返测试）
  - [ ] 所有类型有完整的 `#[derive]` 宏

  **QA 场景**：
  ```
  Scenario: Announcement 序列化往返
    Tool: Bash
    Steps:
      1. cargo test -p cicada-core -- announcement
      2. 验证测试通过
    Expected Result: 测试通过，JSON 序列化后反序列化一致
    Evidence: .sisyphus/evidence/task-2-types-test.txt

  Scenario: 紧急类型枚举转换
    Tool: Bash
    Steps:
      1. cargo test -p cicada-core -- announcement_type
    Expected Result: Emergency.to_string() == "emergency"，from_str 往返正确
    Evidence: .sisyphus/evidence/task-2-enum-test.txt
  ```

  **提交**：YES
  - Message: `feat(core): add shared types (Announcement, UserInfo, AuthToken, Config, WsMessage)`
  - Files: `crates/cicada-core/src/types/*.rs`, `crates/cicada-core/src/lib.rs`

- [ ] 3. **配置管理模块**

  **要做什么**：
  - 在 `crates/cicada-core/src/config/` 实现配置加载/保存：
    - `AppConfig` 结构体：包含所有可配置项（server_url, ws_url, client_id, display_settings, behavior_settings）
    - `DisplaySettings`：font_size, font_color, bg_color, bg_opacity, window_width, window_height, window_position
    - `BehaviorSettings`：auto_start, start_minimized, show_floating_on_start, floating_topmost, default_mode
    - 配置文件格式：TOML（`config.toml`），存放在用户数据目录
    - 平台特定配置路径：Windows `%APPDATA%/Cicada/`，Linux `~/.config/cicada/`，macOS `~/Library/Application Support/Cicada/`
    - `ConfigManager` 结构体：`load()`, `save()`, `get()`, `update()` 方法
    - 默认配置：硬编码默认值，首次运行时自动生成
    - 配置热加载：监听文件变化（可选，使用 `notify` crate）
    - 保留未知键（向前兼容）
  - 单元测试：测试加载、保存、默认值、路径正确性

  **不得做**：
  - ❌ 不要在配置中存储敏感信息（Token 由 Task 14 独立管理）
  - ❌ 不要使用系统注册表（仅文件存储）

  **推荐 Agent Profile**：
  - **Category**: `quick`
    - 原因：标准配置管理模式，逻辑简单
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 2, 4 并行）
  - **并行组**：Wave 1
  - **阻塞**：Task 13, 14, 21
  - **被阻塞**：Task 1（需要目录结构）

  **参考**：
  - `dirs` crate 文档：跨平台用户数据目录
  - `toml` + `serde` 序列化
  - `notify` crate（可选热加载）

  **验收标准**：
  - [ ] `cargo test -p cicada-core -- config` 全部通过
  - [ ] 测试覆盖：默认值加载、TOML 解析、保存+重新加载一致性、路径生成
  - [ ] 三平台配置路径正确

  **QA 场景**：
  ```
  Scenario: 配置保存和加载往返
    Tool: Bash
    Steps:
      1. cargo test -p cicada-core -- config::test_save_and_load
      2. 验证测试断言 config 保存后再加载内容一致
    Expected Result: 测试通过，所有字段值一致
    Evidence: .sisyphus/evidence/task-3-config-test.txt

  Scenario: 默认配置生成
    Tool: Bash
    Steps:
      1. cargo test -p cicada-core -- config::test_default_config
    Expected Result: 默认配置包含所有必需字段且值合理
    Evidence: .sisyphus/evidence/task-3-default-test.txt
  ```

  **提交**：YES
  - Message: `feat(core): add configuration management with TOML persistence`
  - Files: `crates/cicada-core/src/config/*.rs`

- [ ] 4. **API 规范文档 (OpenAPI)**

  **要做什么**：
  - 在 `api-spec/openapi.yaml` 编写完整的 REST API 规范（OpenAPI 3.0）：
    - `GET /api/v1/announcements` — 获取公告列表（支持分页、类型筛选）
    - `GET /api/v1/announcements/{id}` — 获取单条公告详情
    - `POST /api/v1/announcements` — 发布公告（需 Bearer Token）
    - `PUT /api/v1/announcements/{id}` — 编辑公告（需 Bearer Token）
    - `DELETE /api/v1/announcements/{id}` — 删除公告（需 Bearer Token）
    - WebSocket `ws://{host}/ws/v1/announcements` — 实时推送（协议说明）
  - 定义请求/响应 schema：`AnnouncementCreate`, `AnnouncementUpdate`, `AnnouncementListResponse`
  - WebSocket 消息格式规范：JSON 消息结构、心跳间隔（30s）、ACK 机制
  - 包含认证说明（Bearer Token from SECTL OAuth）
  - 包含错误码定义

  **不得做**：
  - ❌ 不定义服务端实现细节（数据库、缓存等）
  - ❌ 不定义 SECTL OAuth 端点（已在 SECTL 文档中，仅引用）

  **推荐 Agent Profile**：
  - **Category**: `writing`
    - 原因：纯文档/规范编写
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 2, 3 并行）
  - **并行组**：Wave 1
  - **阻塞**：Task 8, 9, 23, 24
  - **被阻塞**：Task 1（需要目录结构）

  **参考**：
  - OpenAPI 3.0 规范：`https://swagger.io/specification/`
  - SECTL OAuth 参考（草案中已记录）

  **验收标准**：
  - [ ] `openapi.yaml` 通过在线校验器（如 `https://editor.swagger.io/`）
  - [ ] 包含所有 5 个 REST 端点 + WebSocket 协议说明
  - [ ] 所有 schema 有 description 和 example

  **QA 场景**：
  ```
  Scenario: OpenAPI 规范语法校验
    Tool: Bash
    Steps:
      1. 使用 yamllint 或在线工具校验 YAML 语法
      2. 检查所有必填字段存在
    Expected Result: 无 YAML 语法错误，OpenAPI 结构完整
    Evidence: .sisyphus/evidence/task-4-openapi-valid.txt

  Scenario: 规范可读性检查
    Tool: Bash
    Steps:
      1. wc -l api-spec/openapi.yaml
    Expected Result: 文件 > 100 行，包含详细的端点描述
    Evidence: .sisyphus/evidence/task-4-linecount.txt
  ```

  **提交**：YES
  - Message: `docs(api): add OpenAPI 3.0 specification for announcement API`
  - Files: `api-spec/openapi.yaml`

- [ ] 5. **OAuth PKCE 认证客户端**

  **要做什么**：
  - 在 `crates/cicada-core/src/auth/` 实现 OAuth 2.0 + PKCE 客户端：
    - `OAuthClient` 结构体：`client_id`, `base_url`, `redirect_uri`
    - PKCE 工具：`generate_code_verifier()` → 随机 32 字节 → base64url；`generate_code_challenge(verifier)` → SHA256 → base64url
    - `build_authorize_url()` → 完整授权 URL（含 code_challenge、state）
    - `exchange_code_for_token(code, code_verifier, device_uuid, ip)` → POST `/api/oauth/token`
    - `refresh_access_token(refresh_token, device_uuid, ip)` → POST `/api/oauth/refresh`
    - `get_userinfo(access_token)` → GET `/api/oauth/userinfo`
    - `introspect_token(access_token)` → POST `/api/oauth/introspect`
    - `logout(access_token)` → POST `/api/oauth/logout`
  - 使用 `reqwest` 进行 HTTP 请求（`cicada-core` 需要 `reqwest` 依赖）
  - Token 过期自动检测（检查 `expires_in` 或 API 返回 401）
  - 错误处理：映射 SECTL 错误码到自定义错误类型
  - 单元测试：Mock HTTP 服务器测试授权流程各步骤

  **不得做**：
  - ❌ 不在此模块启动本地 HTTP 服务器（回调处理在 Task 19 的桌面端实现）
  - ❌ 不实现 OAuth 之外的其他 SECTL API

  **推荐 Agent Profile**：
  - **Category**: `deep`
    - 原因：涉及 OAuth 2.0 + PKCE 密码学，需要仔细实现
  - **Skills**: []
  - **评估但省略**：无

  **并行化**：
  - **可并行运行**：YES（与 Task 6 并行，在 Task 2 完成后）
  - **并行组**：Wave 1
  - **阻塞**：Task 7, 8, 19
  - **被阻塞**：Task 1, 2

  **参考**：
  - SECTL OAuth Core API：`D:/GitHub/SECTL-auth/public/docs/api-reference/oauth-core.md`（已在草案中完整记录）
  - PKCE RFC 7636：`https://datatracker.ietf.org/doc/html/rfc7636`
  - `reqwest` 文档：异步 HTTP 客户端

  **验收标准**：
  - [ ] `cargo test -p cicada-core -- auth` 全部通过
  - [ ] PKCE 流程：code_verifier → code_challenge 往返正确
  - [ ] 测试覆盖：成功换 Token、刷新 Token、Token 失效、网络错误

  **QA 场景**：
  ```
  Scenario: PKCE verifier/challenge 生成和验证
    Tool: Bash
    Steps:
      1. cargo test -p cicada-core -- auth::test_pkce_generation
      2. 验证 code_challenge 由 code_verifier 正确派生
    Expected Result: SHA256(base64url(verifier)) == challenge，长度 43 字符
    Evidence: .sisyphus/evidence/task-5-pkce-test.txt

  Scenario: 完整授权流程（Mock 服务器）
    Tool: Bash
    Steps:
      1. cargo test -p cicada-core -- auth::test_full_oauth_flow
      2. 验证 Mock 服务器返回正确 token
    Expected Result: exchange_code_for_token 返回 AuthToken 结构体
    Evidence: .sisyphus/evidence/task-5-oauth-flow-test.txt

  Scenario: Token 刷新流程
    Tool: Bash
    Steps:
      1. cargo test -p cicada-core -- auth::test_token_refresh
    Expected Result: 新 access_token 与旧的不同
    Evidence: .sisyphus/evidence/task-5-refresh-test.txt
  ```

  **提交**：YES
  - Message: `feat(core): implement OAuth 2.0 PKCE authentication client for SECTL`
  - Files: `crates/cicada-core/src/auth/*.rs`

- [ ] 6. **CI/CD 配置**

  **要做什么**：
  - 创建 `.github/workflows/ci.yml`：
    - 触发条件：`push` 到 `main`，`pull_request`
    - 矩阵构建：`ubuntu-latest`, `windows-latest`, `macos-latest`
    - 步骤：checkout → 安装 Rust → `cargo build --workspace` → `cargo test --workspace` → `cargo clippy --workspace -- -D warnings` → `cargo fmt --check`
    - 安装 Tauri 系统依赖（`libwebkit2gtk-4.0-dev` 等 for Linux）
  - 创建 `.github/workflows/release.yml`（骨架）：
    - 触发条件：`v*` tag
    - 构建 Tauri 桌面应用（三平台）
  - 确保 `Cargo.lock` 被提交（workspace 项目）

  **不得做**：
  - ❌ 不配置自动发布到 GitHub Releases（骨架即可）
  - ❌ 不配置代码签名

  **推荐 Agent Profile**：
  - **Category**: `quick`
    - 原因：标准 CI 模板配置
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 5 并行）
  - **并行组**：Wave 1
  - **阻塞**：无（独立任务）
  - **被阻塞**：Task 1（需要目录结构）

  **参考**：
  - Tauri v1 CI 指南：`https://v1.tauri.app/v1/guides/building/cross-platform`
  - GitHub Actions 语法

  **验收标准**：
  - [ ] `.github/workflows/ci.yml` 语法正确
  - [ ] 包含三平台矩阵构建
  - [ ] `Cargo.lock` 已提交

  **QA 场景**：
  ```
  Scenario: CI 配置语法校验
    Tool: Bash
    Steps:
      1. 检查 YAML 语法（可以用 yamllint 或目视检查）
      2. 验证所有必需步骤存在
    Expected Result: 无语法错误，包含 build/test/clippy/fmt 步骤
    Evidence: .sisyphus/evidence/task-6-ci-config.txt

  Scenario: Cargo.lock 已提交
    Tool: Bash
    Steps:
      1. git ls-files Cargo.lock
    Expected Result: 输出 Cargo.lock（表示已跟踪）
    Evidence: .sisyphus/evidence/task-6-lockfile.txt
  ```

  **提交**：YES
  - Message: `ci: add GitHub Actions CI workflow (cross-platform build + test)`
  - Files: `.github/workflows/ci.yml`, `.github/workflows/release.yml`

- [ ] 7. **WebSocket 客户端**

  **要做什么**：
  - 在 `crates/cicada-core/src/ws/` 实现 WebSocket 客户端：
    - `WsClient` 结构体：`connect(url)`, `disconnect()`, `send(msg)`, 接收通道
    - 使用 `tokio-tungstenite` + `futures-util`（异步流式处理）
    - 自动重连：指数退避（1s, 2s, 4s, 8s, max 60s），无限重试
    - 心跳保活：每 30 秒发送 Ping 帧，超时 10 秒无 Pong 视为断连
    - 消息序列化：`WsMessage` 枚举 ↔ JSON（使用 `serde_json`）
    - 接收队列：使用 `tokio::sync::mpsc` 通道，避免阻塞 WS 读取循环
    - `WsEvent` 枚举暴露给上层：`Connected`, `Disconnected`, `Message(WsMessage)`, `Error`
    - 线程安全：`Arc<Mutex<>>` 或 actor 模式管理内部状态
  - 重连时自动重新订阅/同步状态
  - 单元测试：Mock WebSocket 服务器测试连接、断连、重连、消息收发

  **不得做**：
  - ❌ 不在 WS 客户端中处理业务逻辑（仅负责传输）
  - ❌ 不使用同步阻塞 I/O

  **推荐 Agent Profile**：
  - **Category**: `deep`
    - 原因：异步网络编程 + 重连策略 + 心跳机制，需要仔细处理并发
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 8, 9, 10 并行）
  - **并行组**：Wave 2
  - **阻塞**：Task 16, 18, 24
  - **被阻塞**：Task 2, 5（需要类型定义和 Auth 状态）

  **参考**：
  - `tokio-tungstenite` 文档：`https://docs.rs/tokio-tungstenite/latest/tokio_tungstenite/`
  - `futures-util::StreamExt` 用于异步流处理

  **验收标准**：
  - [ ] `cargo test -p cicada-core -- ws` 全部通过
  - [ ] 测试：连接成功 → 接收消息 → 发送 ACK → 断连 → 自动重连 → 恢复
  - [ ] 心跳机制：Ping/Pong 正常工作

  **QA 场景**：
  ```
  Scenario: WebSocket 连接和消息收发
    Tool: Bash
    Steps:
      1. cargo test -p cicada-core -- ws::test_connect_and_receive
      2. 验证 Mock 服务器发送 WsMessage::ServerPush，客户端正确接收
    Expected Result: 客户端通过 mpsc 通道收到消息，JSON 解析正确
    Evidence: .sisyphus/evidence/task-7-ws-connect.txt

  Scenario: 断连自动重连
    Tool: Bash
    Steps:
      1. cargo test -p cicada-core -- ws::test_reconnect
      2. Mock 服务器先拒绝连接，然后接受重连
      3. 验证重连成功且消息恢复
    Expected Result: 客户端在退避后成功重连，状态恢复
    Evidence: .sisyphus/evidence/task-7-ws-reconnect.txt

  Scenario: 心跳 Ping/Pong
    Tool: Bash
    Steps:
      1. cargo test -p cicada-core -- ws::test_heartbeat
      2. 等待 30s，验证发送了 Ping
      3. 模拟无 Pong，验证断连检测
    Expected Result: 心跳正常发送，超时触发断连
    Evidence: .sisyphus/evidence/task-7-heartbeat.txt
  ```

  **提交**：YES
  - Message: `feat(core): implement WebSocket client with auto-reconnect and heartbeat`
  - Files: `crates/cicada-core/src/ws/*.rs`

- [ ] 8. **REST API 客户端**

  **要做什么**：
  - 在 `crates/cicada-api/src/rest/` 实现 REST API 客户端：
    - `ApiClient` 结构体：`new(base_url)`, 使用 `reqwest::Client`（连接池复用）
    - `get_announcements(page, per_page, type_filter)` → `GET /api/v1/announcements`
    - `get_announcement(id)` → `GET /api/v1/announcements/{id}`
    - `create_announcement(payload, token)` → `POST /api/v1/announcements`（带 Bearer auth）
    - `update_announcement(id, payload, token)` → `PUT /api/v1/announcements/{id}`
    - `delete_announcement(id, token)` → `DELETE /api/v1/announcements/{id}`
  - 请求/响应类型在 `crates/cicada-api/src/models/` 定义（使用 Task 4 的 OpenAPI 规范）
  - 错误处理：HTTP 4xx/5xx → 自定义 `ApiError` 枚举
  - Token 自动注入：从 `AuthToken` 提取 Bearer header
  - 响应反序列化：`AnnouncementListResponse`，`Announcement` 等
  - 单元测试：Mock HTTP 服务器（`wiremock` 或 `httpmock`）

  **不得做**：
  - ❌ 不在 API 客户端中管理 Token（由上层传入）
  - ❌ 不实现 GraphQL 或 gRPC

  **推荐 Agent Profile**：
  - **Category**: `quick`
    - 原因：标准 REST 客户端封装，模式固定
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 7, 9, 10 并行）
  - **并行组**：Wave 2
  - **阻塞**：Task 19, 20, 21, 23, 24
  - **被阻塞**：Task 2, 5（需要类型定义），Task 4（API 规范参考）

  **参考**：
  - `reqwest` 文档：`https://docs.rs/reqwest/latest/reqwest/`
  - Task 4 的 OpenAPI 规范（端点路径和参数）
  - `wiremock` crate：`https://docs.rs/wiremock/latest/wiremock/`

  **验收标准**：
  - [ ] `cargo test -p cicada-api -- rest` 全部通过
  - [ ] 测试覆盖全部 5 个端点（GET list, GET one, POST, PUT, DELETE）
  - [ ] 错误处理：404、401、500 正确映射

  **QA 场景**：
  ```
  Scenario: 获取公告列表
    Tool: Bash
    Steps:
      1. cargo test -p cicada-api -- rest::test_get_announcements
      2. Mock 返回 2 条公告，验证解析正确
    Expected Result: AnnouncementListResponse.announcements.len() == 2
    Evidence: .sisyphus/evidence/task-8-rest-list.txt

  Scenario: 发布公告（带认证）
    Tool: Bash
    Steps:
      1. cargo test -p cicada-api -- rest::test_create_announcement
      2. 请求头包含 Authorization: Bearer {token}
      3. Mock 返回 201 + Announcement
    Expected Result: 响应解析成功，created_at 不为空
    Evidence: .sisyphus/evidence/task-8-rest-create.txt

  Scenario: API 错误处理
    Tool: Bash
    Steps:
      1. cargo test -p cicada-api -- rest::test_error_handling
      2. Mock 返回 401 Unauthorized
    Expected Result: 返回 ApiError::Unauthorized
    Evidence: .sisyphus/evidence/task-8-rest-error.txt
  ```

  **提交**：YES
  - Message: `feat(api): implement REST API client for announcement endpoints`
  - Files: `crates/cicada-api/src/rest/*.rs`, `crates/cicada-api/src/models/*.rs`

- [ ] 9. **Mock 服务器**

  **要做什么**：
  - 在 `crates/cicada-mock-server/src/main.rs` 实现本地开发 Mock 服务器：
    - 使用 `axum` 框架（轻量、async、适合 Mock 场景）
    - 实现 Task 4 OpenAPI 规范中定义的全部 5 个 REST 端点
    - 内存存储：`Arc<RwLock<Vec<Announcement>>>` — 预置 3 条示例公告
    - WebSocket 端点：`/ws/v1/announcements` — 新客户端连接时推送全部现有公告
    - 模拟实时推送：POST 创建公告后，通过 WebSocket 广播给所有连接客户端
    - 心跳：每 30 秒 Ping
    - Token 验证模拟：检查 Authorization header 是否以 "Bearer " 开头（不真正验证）
    - CORS 配置：允许所有来源（开发用）
    - 监听端口：`3001`（可配置）
    - 启动日志：打印所有端点 URL
  - 无需身份验证绕过（Mock 模式接受任何 Token）

  **不得做**：
  - ❌ 不实现真实的 Token 验证
  - ❌ 不使用数据库（纯内存）
  - ❌ 不暴露到外网（仅 localhost）

  **推荐 Agent Profile**：
  - **Category**: `deep`
    - 原因：需要同时处理 REST + WebSocket + 广播逻辑
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 7, 8, 10 并行）
  - **并行组**：Wave 2
  - **阻塞**：Task 24（集成测试需要 Mock 服务器）
  - **被阻塞**：Task 2, 4（需要类型定义和 API 规范）

  **参考**：
  - `axum` 文档：`https://docs.rs/axum/latest/axum/`
  - Task 4 的 OpenAPI 规范

  **验收标准**：
  - [ ] `cargo run -p cicada-mock-server` 启动成功，监听端口 3001
  - [ ] `curl http://localhost:3001/api/v1/announcements` 返回预置公告
  - [ ] `curl -X POST http://localhost:3001/api/v1/announcements -H "Authorization: Bearer test" -H "Content-Type: application/json" -d '{...}'` 创建成功
  - [ ] WebSocket 连接 `ws://localhost:3001/ws/v1/announcements` 收到推送

  **QA 场景**：
  ```
  Scenario: Mock 服务器启动和 REST 端点
    Tool: Bash
    Preconditions: 编译成功
    Steps:
      1. cargo run -p cicada-mock-server &
      2. sleep 2
      3. curl -s http://localhost:3001/api/v1/announcements | python3 -m json.tool
      4. 验证返回 JSON 数组，至少 3 条
      5. kill %1
    Expected Result: HTTP 200，JSON 数组 3+ 条公告
    Failure Indicators: 连接拒绝、空响应、非 JSON
    Evidence: .sisyphus/evidence/task-9-mock-rest.txt

  Scenario: WebSocket 实时推送
    Tool: Bash
    Preconditions: Mock 服务器运行中
    Steps:
      1. 使用 websocat 或类似工具连接 ws://localhost:3001/ws/v1/announcements
      2. curl -X POST 创建一条新公告
      3. 验证 WebSocket 客户端收到推送
    Expected Result: WS 客户端收到新公告的 JSON 消息
    Evidence: .sisyphus/evidence/task-9-mock-ws.txt
  ```

  **提交**：YES
  - Message: `feat(mock): implement local development mock server with REST + WebSocket`
  - Files: `crates/cicada-mock-server/src/main.rs`, `crates/cicada-mock-server/Cargo.toml`

- [ ] 10. **Tauri 项目脚手架 + React 集成**

  **要做什么**：
  - 初始化 Tauri v1 项目在 `crates/cicada-client/`：
    - 使用 `create-tauri-app` 或手动配置 `tauri.conf.json`
    - 前端目录：`crates/cicada-client/ui/`
    - Rust 入口：`crates/cicada-client/src/main.rs`（`tauri::Builder` + 注册 cicada-core/cicada-api 依赖）
  - React + TypeScript 项目初始化：
    - `ui/package.json`：依赖 `react`, `react-dom`, `typescript`, `vite`（构建工具）
    - 创建基础 `App.tsx`：空壳，准备接收模式切换逻辑
    - 创建 `index.html`：挂载点 + Tauri 上下文注入
  - Tauri 配置：
    - `tauri.conf.json`：窗口标题「知了」、默认尺寸、无边框窗口设置
    - 启用 Tauri v1 的 `window`、`shell`、`process` 插件
    - 配置 `devPath` 和 `distDir`
  - Rust 端配置：
    - `cicada-client/Cargo.toml`：`tauri` v1.x 依赖 + `cicada-core` + `cicada-api` 路径依赖
  - 验证：`cargo tauri dev` 可启动带 React 界面的窗口

  **不得做**：
  - ❌ 不要使用 Tauri v2（与 Win7 不兼容）
  - ❌ 不要添加状态管理库（Redux/Zustand 等 — 后续任务按需添加）

  **推荐 Agent Profile**：
  - **Category**: `visual-engineering`
    - 原因：涉及前端脚手架 + Tauri 配置 + UI 构建工具链
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 7, 8, 9 并行）
  - **并行组**：Wave 2
  - **阻塞**：Task 11-22（所有 UI 任务依赖此脚手架）
  - **被阻塞**：Task 1（需要 workspace 结构）

  **参考**：
  - Tauri v1 项目创建：`https://v1.tauri.app/v1/guides/getting-started/setup/`
  - Vite + React 配置：`https://vitejs.dev/guide/`
  - Tauri v1 `tauri.conf.json` 参考

  **验收标准**：
  - [ ] `cargo tauri dev` 启动成功，显示 React 界面
  - [ ] `cargo build -p cicada-client` 编译通过
  - [ ] React 热重载在开发模式下工作

  **QA 场景**：
  ```
  Scenario: Tauri 开发模式启动
    Tool: Bash
    Steps:
      1. cd crates/cicada-client
      2. cargo tauri dev &
      3. sleep 5
      4. 检查进程是否存活
      5. kill %1
    Expected Result: 无崩溃，Tauri 窗口出现
    Evidence: .sisyphus/evidence/task-10-tauri-dev.txt

  Scenario: 前端构建
    Tool: Bash
    Steps:
      1. cd crates/cicada-client/ui && npm run build
      2. 验证 dist/ 目录生成
    Expected Result: 构建成功，dist/index.html 存在
    Evidence: .sisyphus/evidence/task-10-ui-build.txt
  ```

  **提交**：YES
  - Message: `feat(client): initialize Tauri v1 project with React + TypeScript scaffold`
  - Files: `crates/cicada-client/Cargo.toml`, `crates/cicada-client/tauri.conf.json`, `crates/cicada-client/src/main.rs`, `crates/cicada-client/ui/*`

- [ ] 11. **系统托盘**

  **要做什么**：
  - 在 `crates/cicada-client/src/tray/` 实现系统托盘：
    - 使用 `tray-icon` crate（或 `tauri` v1 内置 tray API）
    - 托盘图标：创建简单 SVG/PNG 图标（知了logo），嵌入为 `include_bytes!`
    - 右键菜单：
      - 「显示浮窗」— 恢复/显示所有浮窗
      - 「隐藏浮窗」— 隐藏所有浮窗（不关闭）
      - 「设置...」— 打开设置窗口
      - 「关于知了」— 显示版本信息
      - 分隔线
      - 「退出」— 完全退出程序（含确认对话框）
    - 左键单击托盘图标：切换浮窗显示/隐藏
    - 程序关闭到托盘：点击窗口 X 按钮 → 隐藏窗口，托盘保持运行
  - 托盘 Tooltip：显示「知了 - 校园公告投屏」
  - Mac 兼容：macOS 菜单栏图标
  - 单元测试：菜单结构验证

  **不得做**：
  - ❌ 不要使用仅 Windows 特定的托盘 API（需跨平台）
  - ❌ 不要在退出时强制关闭而不保存状态

  **推荐 Agent Profile**：
  - **Category**: `deep`
    - 原因：跨平台系统托盘 API 差异较大，需要平台适配
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 12, 13, 14, 15 并行）
  - **并行组**：Wave 3
  - **阻塞**：Task 19, 22
  - **被阻塞**：Task 10（需要 Tauri 项目）

  **参考**：
  - `tray-icon` crate：`https://docs.rs/tray-icon/latest/tray_icon/`
  - Tauri v1 tray API：`https://v1.tauri.app/v1/guides/features/system-tray/`

  **验收标准**：
  - [ ] `cargo build -p cicada-client` 托盘模块编译通过
  - [ ] 托盘菜单包含全部 5 项
  - [ ] 三平台托盘图标正确显示

  **QA 场景**：
  ```
  Scenario: 托盘菜单结构
    Tool: Bash (Playwright 无法直接测试原生托盘)
    Steps:
      1. cargo test -p cicada-client -- tray
      2. 验证菜单项数量 == 5
    Expected Result: 测试通过，菜单包含预期项
    Evidence: .sisyphus/evidence/task-11-tray-test.txt

  Scenario: 托盘图标资源存在
    Tool: Bash
    Steps:
      1. ls crates/cicada-client/src/tray/icon.png 或类似路径
    Expected Result: 图标文件存在
    Evidence: .sisyphus/evidence/task-11-icon.txt
  ```

  **提交**：YES
  - Message: `feat(client): implement cross-platform system tray with context menu`
  - Files: `crates/cicada-client/src/tray/*.rs`

- [ ] 12. **窗口管理器**

  **要做什么**：
  - 在 `crates/cicada-client/src/windows/` 实现窗口管理器：
    - `WindowManager` 结构体：管理主窗口、浮窗、设置窗口的生命周期
    - 浮窗创建：`create_floating_window(screen_index, config)` → 无边框、无任务栏图标、置顶可选、半透明背景
    - 浮窗销毁：`destroy_floating_window(window_id)`
    - 浮窗定位：根据配置（屏幕、位置、尺寸）创建窗口
    - 多屏幕检测：使用 `tauri::window` API 枚举屏幕，为每个屏幕创建独立浮窗
    - 窗口层级切换：`set_topmost(window_id, bool)` — Tauri `set_always_on_top`
    - 窗口显隐：`show_floating_windows()` / `hide_floating_windows()`
    - DPI 感知：自动处理 HiDPI 缩放
  - 主窗口（管理模式）：标准窗口，可最小化到托盘
  - 设置窗口：模态对话框，阻止主窗口交互
  - 屏幕变化响应：监听显示器插拔事件，自动调整浮窗
  - 单元测试：窗口创建/销毁逻辑（Mock Tauri 窗口 handle）

  **不得做**：
  - ❌ 不要在浮窗上显示窗口控制按钮（最小化/最大化/关闭）
  - ❌ 不要让浮窗出现在 Alt+Tab 列表中

  **推荐 Agent Profile**：
  - **Category**: `deep`
    - 原因：跨平台窗口管理 + 多屏幕 + DPI 处理，较复杂
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 11, 13, 14, 15 并行）
  - **并行组**：Wave 3
  - **阻塞**：Task 16, 17, 18
  - **被阻塞**：Task 10（需要 Tauri 项目）

  **参考**：
  - Tauri v1 窗口 API：`https://v1.tauri.app/v1/api/js/window`
  - `tauri::WindowBuilder` 无边框窗口配置

  **验收标准**：
  - [ ] `cargo build -p cicada-client` 窗口模块编译通过
  - [ ] 窗口管理器可以创建/销毁/显示/隐藏浮窗
  - [ ] 多屏幕枚举正常工作
  - [ ] 置顶切换生效

  **QA 场景**：
  ```
  Scenario: 浮窗创建配置
    Tool: Bash
    Steps:
      1. cargo test -p cicada-client -- windows::test_create_floating
      2. 验证窗口属性：decorations = false, skip_taskbar = true
    Expected Result: 测试通过，窗口配置正确
    Evidence: .sisyphus/evidence/task-12-window-config.txt

  Scenario: 屏幕枚举
    Tool: Bash
    Steps:
      1. cargo test -p cicada-client -- windows::test_enumerate_screens
    Expected Result: 至少返回 1 个屏幕
    Evidence: .sisyphus/evidence/task-12-screens.txt
  ```

  **提交**：YES
  - Message: `feat(client): implement window manager with multi-screen floating window support`
  - Files: `crates/cicada-client/src/windows/*.rs`

- [ ] 13. **开机自启 + 静默启动**

  **要做什么**：
  - 在 `crates/cicada-client/src/autostart/` 实现开机自启：
    - Windows：创建注册表项 `HKCU\Software\Microsoft\Windows\CurrentVersion\Run\Cicada`
    - Linux：创建 `~/.config/autostart/cicada.desktop` 文件（符合 XDG 规范）
    - macOS：创建 LaunchAgent `~/Library/LaunchAgents/com.cicada.app.plist`
    - `enable_autostart()` / `disable_autostart()` / `is_autostart_enabled()` 方法
    - 静默启动参数：`--silent` — 启动后直接最小化到托盘，浮窗自动显示
  - 读取配置中的 `auto_start` 和 `start_minimized` 设置
  - 主函数 `main.rs` 初始化流程：
    1. 解析启动参数
    2. 加载配置
    3. 初始化 Auth（检查已存储 Token）
    4. 初始化 WS（如有 Token）
    5. 创建托盘
    6. 根据模式创建窗口
  - 单元测试：自启注册/取消/检测逻辑

  **不得做**：
  - ❌ 不要使用管理员权限注册自启（用户级即可）
  - ❌ 不要在卸载时残留注册表/文件

  **推荐 Agent Profile**：
  - **Category**: `quick`
    - 原因：平台特定的文件/注册表操作，逻辑固定
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 11, 12, 14, 15 并行）
  - **并行组**：Wave 3
  - **阻塞**：Task 21（设置窗口需要此功能）
  - **被阻塞**：Task 3, 10（需要配置管理和项目结构）

  **参考**：
  - `auto-launch` crate：`https://docs.rs/auto-launch/latest/auto_launch/`
  - 或手动实现各平台注册逻辑

  **验收标准**：
  - [ ] `cargo test -p cicada-client -- autostart` 通过
  - [ ] 各平台自启注册/取消逻辑正确
  - [ ] `--silent` 参数解析正确

  **QA 场景**：
  ```
  Scenario: 自启注册（当前平台）
    Tool: Bash
    Steps:
      1. cargo test -p cicada-client -- autostart::test_enable_disable
      2. 验证 enable → is_enabled 返回 true
      3. 验证 disable → is_enabled 返回 false
    Expected Result: 注册/取消往返一致
    Evidence: .sisyphus/evidence/task-13-autostart-test.txt

  Scenario: 静默启动参数
    Tool: Bash
    Steps:
      1. cargo test -p cicada-client -- test_silent_arg
      2. 验证 --silent 参数正确解析
    Expected Result: silent 标志 == true
    Evidence: .sisyphus/evidence/task-13-silent-test.txt
  ```

  **提交**：YES
  - Message: `feat(client): implement cross-platform auto-start and silent launch`
  - Files: `crates/cicada-client/src/autostart/*.rs`, `crates/cicada-client/src/main.rs`

- [ ] 14. **Token 安全存储**

  **要做什么**：
  - 在 `crates/cicada-core/src/auth/storage.rs` 实现 Token 安全存储：
    - 加密存储 Access Token 和 Refresh Token
    - 平台后端选择：
      - Windows：使用 DPAPI（`winapi` + CryptProtectData）
      - Linux：使用 `keyring` crate（Secret Service API / gnome-keyring）
      - macOS：使用 Keychain（`security-framework` crate）
    - 降级方案：如果 keyring 不可用 → 加密写入本地文件（使用 `ring` crate AEAD 加密，密钥派生自 machine-id）
    - `TokenStorage` trait：`save_tokens(AuthToken)`, `load_tokens() -> Option<AuthToken>`, `clear_tokens()`
    - Token 过期检测：`is_token_expired() -> bool`
    - Token 变化时自动保存（观察者模式或主动调用）
  - 单元测试：Token 存储/加载/清除（使用 Mock 或内存后端）

  **不得做**：
  - ❌ 不将 Token 明文存储在配置文件或日志中
  - ❌ 不在内存中长时间保留明文 Token（使用后尽快 drop）

  **推荐 Agent Profile**：
  - **Category**: `quick`
    - 原因：标准安全存储模式，使用成熟 crate
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 11, 12, 13, 15 并行）
  - **并行组**：Wave 3
  - **阻塞**：Task 19, 21（设置/登录需要 Token 管理）
  - **被阻塞**：Task 3, 10（需要配置管理）

  **参考**：
  - `keyring` crate：`https://docs.rs/keyring/latest/keyring/`
  - `ring` crate AEAD 加密：`https://docs.rs/ring/latest/ring/aead/`

  **验收标准**：
  - [ ] `cargo test -p cicada-core -- auth::storage` 通过
  - [ ] 测试：存储 → 加载一致性；清除后加载返回 None
  - [ ] Token 未以明文写入文件

  **QA 场景**：
  ```
  Scenario: Token 存储和加载往返
    Tool: Bash
    Steps:
      1. cargo test -p cicada-core -- auth::storage::test_save_and_load
      2. 存储 AuthToken → 加载 → 比较字段
    Expected Result: 加载的 Token 与存储的一致
    Evidence: .sisyphus/evidence/task-14-token-test.txt

  Scenario: Token 清除
    Tool: Bash
    Steps:
      1. cargo test -p cicada-core -- auth::storage::test_clear
      2. 存储 → 清除 → 加载
    Expected Result: 清除后加载返回 None
    Evidence: .sisyphus/evidence/task-14-clear-test.txt
  ```

  **提交**：YES
  - Message: `feat(core): implement secure token storage with platform keychain integration`
  - Files: `crates/cicada-core/src/auth/storage.rs`

- [ ] 15. **自动更新机制**

  **要做什么**：
  - 在 `crates/cicada-client/src/updater/` 实现自动更新：
    - 使用 Tauri v1 内置 `updater` 插件（`tauri-plugin-updater`）
    - 更新检查：启动时异步检查（不阻塞 UI）
    - 更新源：GitHub Releases API 或自定义更新服务器
    - 静默下载：后台下载安装包，显示进度（托盘通知）
    - 安装：下载完成后提示「重启以安装更新」
    - 更新配置：`tauri.conf.json` 中配置更新端点 URL 和公钥签名验证
    - 降级保护：验证下载文件签名后才安装
    - 手动检查：设置窗口中「检查更新」按钮
  - 单元测试：更新检查逻辑（Mock 更新服务器）

  **不得做**：
  - ❌ 不强制更新（用户可选择稍后重启）
  - ❌ 不在更新时删除用户配置

  **推荐 Agent Profile**：
  - **Category**: `deep`
    - 原因：涉及平台特定安装逻辑 + 签名验证
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 11, 12, 13, 14 并行）
  - **并行组**：Wave 3
  - **阻塞**：无直接阻塞
  - **被阻塞**：Task 10（需要 Tauri 项目）

  **参考**：
  - Tauri v1 updater 插件：`https://v1.tauri.app/v1/guides/distribution/updater`
  - 公钥签名验证配置

  **验收标准**：
  - [ ] `cargo build -p cicada-client` 包含 updater 编译
  - [ ] 更新检查逻辑正确（Mock 场景）
  - [ ] 签名验证逻辑正确

  **QA 场景**：
  ```
  Scenario: 更新检查逻辑
    Tool: Bash
    Steps:
      1. cargo test -p cicada-client -- updater::test_check_update
      2. Mock 返回新版本信息
    Expected Result: 检测到新版本，返回更新 URL 和版本号
    Evidence: .sisyphus/evidence/task-15-update-check.txt

  Scenario: 最新版本无需更新
    Tool: Bash
    Steps:
      1. cargo test -p cicada-client -- updater::test_already_latest
    Expected Result: 返回 AlreadyLatest
    Evidence: .sisyphus/evidence/task-15-no-update.txt
  ```

  **提交**：YES
  - Message: `feat(client): implement auto-update with silent download and signature verification`
  - Files: `crates/cicada-client/src/updater/*.rs`, `crates/cicada-client/tauri.conf.json`

- [ ] 16. **浮窗组件（HTML 渲染器）**

  **要做什么**：
  - 在 `crates/cicada-client/ui/src/components/FloatingWindow.tsx` 实现浮窗 React 组件：
    - 渲染公告 HTML 内容：使用 `dangerouslySetInnerHTML`（内容来自可信服务器）
    - 支持 Markdown 降级：如果 HTML 解析失败，显示纯文本
    - 自定义样式：字体大小、颜色、背景色、透明度从配置读取
    - 布局：
      - 顶部：公告标题（滚动时固定）
      - 中部：正文内容（可滚动，`overflow-y: auto`）
      - 底部：发布者 + 时间（固定）
    - 紧急通知特殊样式：
      - 红色闪烁边框（CSS `@keyframes blink`）
      - 背景色：浅红（`rgba(255, 0, 0, 0.1)`）
      - 标题加粗、大号字体
    - 一般通知样式：蓝色边框，正常字体
    - 图片处理：`max-width: 100%`，加载失败显示占位符
    - 窗口拖拽：通过 Tauri `data-tauri-drag-region` 属性实现无边框窗口拖拽
    - 右键菜单：「置顶/取消置顶」「隐藏」「设置...」
    - 鼠标悬停时显示关闭按钮（X），平时透明
  - Tauri 端：将 WS 收到的公告通过 Event 发送到前端（`app_handle.emit_all("announcement", payload)`）
  - 前端监听 Tauri Event：`listen('announcement', callback)`

  **不得做**：
  - ❌ 不执行公告 HTML 中的 JavaScript（仅渲染静态 HTML）
  - ❌ 不加载外部资源（`<img src="http://...">` 需要白名单或代理）

  **推荐 Agent Profile**：
  - **Category**: `visual-engineering`
    - 原因：前端 UI 组件 + 动画 + Tauri 事件集成
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 17, 18 并行）
  - **并行组**：Wave 4
  - **阻塞**：Task 24
  - **被阻塞**：Task 7, 10, 12（需要 WS 客户端、Tauri 项目、窗口管理器）

  **参考**：
  - Tauri v1 Event 系统：`https://v1.tauri.app/v1/guides/features/events/`
  - `data-tauri-drag-region`：Tauri 无边框窗口拖拽属性
  - React `dangerouslySetInnerHTML`

  **验收标准**：
  - [ ] 浮窗可渲染 HTML 内容
  - [ ] 紧急/普通/通知三种样式各有视觉差异
  - [ ] 窗口可拖拽移动
  - [ ] 右键菜单正常

  **QA 场景**：
  ```
  Scenario: 浮窗渲染普通公告
    Tool: Playwright
    Preconditions: Tauri dev 模式运行，Mock 服务器发送一条普通公告
    Steps:
      1. 等待浮窗出现（选择器: .floating-window）
      2. 验证公告标题可见（文本: "测试公告"）
      3. 验证内容区域包含 HTML 元素
      4. 验证底部显示发布者和时间
      5. 截图: .sisyphus/evidence/task-16-normal.png
    Expected Result: 浮窗正确渲染，内容完整
    Failure Indicators: 浮窗不出现、内容为空、HTML 未渲染
    Evidence: .sisyphus/evidence/task-16-normal.png

  Scenario: 浮窗渲染紧急公告
    Tool: Playwright
    Preconditions: 发送 type=emergency 的公告
    Steps:
      1. 验证浮窗边框颜色为红色
      2. 验证边框闪烁动画（CSS animation 存在）
      3. 验证背景色为浅红
      4. 截图: .sisyphus/evidence/task-16-emergency.png
    Expected Result: 紧急样式明显区别于普通公告
    Evidence: .sisyphus/evidence/task-16-emergency.png

  Scenario: 浮窗右键菜单
    Tool: Playwright
    Steps:
      1. 右键点击浮窗区域
      2. 验证菜单出现（选择器: .context-menu）
      3. 验证包含「置顶」「隐藏」「设置」选项
      4. 截图: .sisyphus/evidence/task-16-context-menu.png
    Expected Result: 菜单正常显示
    Evidence: .sisyphus/evidence/task-16-context-menu.png
  ```

  **提交**：YES
  - Message: `feat(ui): implement floating window with HTML announcement rendering`
  - Files: `crates/cicada-client/ui/src/components/FloatingWindow.tsx`

- [ ] 17. **多屏幕浮窗管理器**

  **要做什么**：
  - 在 `crates/cicada-client/ui/src/components/FloatingWindowManager.tsx` 实现多屏幕管理：
    - 检测可用屏幕列表（通过 Tauri 前端 API `window.screens()`）
    - 为每个屏幕创建独立浮窗实例（每个有独立的 window label）
    - 屏幕变化响应：显示器插拔 → 自动增删浮窗
    - 浮窗位置持久化：每个浮窗记住自己最后的位置（屏幕索引 + x, y）
    - 配置同步：所有浮窗共享同一显示配置（字体、颜色等）
    - Rust 端管理多个 Tauri 窗口实例（`WindowManager` 的多窗口支持）
  - Tauri 命令（Rust → 前端）：
    - `get_screens()` → 返回屏幕列表
    - `create_floating_window(screen_index)` → 创建浮窗
    - `destroy_floating_window(label)` → 销毁浮窗
    - `update_all_floating_windows(config)` → 批量更新样式
  - 前端入口 `App.tsx`：初始化时调用 `get_screens`，自动创建浮窗

  **不得做**：
  - ❌ 不在每个浮窗中独立运行完整的 React 应用（共享状态）
  - ❌ 不限制最大浮窗数量（理论上支持任意多屏幕）

  **推荐 Agent Profile**：
  - **Category**: `visual-engineering`
    - 原因：前端多窗口协调 + Tauri 多窗口 API
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 16, 18 并行）
  - **并行组**：Wave 4
  - **阻塞**：Task 24
  - **被阻塞**：Task 12, 16（需要窗口管理器和浮窗组件）

  **参考**：
  - Tauri v1 多窗口：`https://v1.tauri.app/v1/guides/features/multiwindow`
  - Tauri v1 前端 `window` API：`@tauri-apps/api/window`

  **验收标准**：
  - [ ] 多屏幕检测正确（至少主屏幕）
  - [ ] 浮窗可独立创建/销毁
  - [ ] 显示器插拔自适应

  **QA 场景**：
  ```
  Scenario: 多浮窗创建
    Tool: Playwright
    Steps:
      1. 启动应用，验证至少 1 个浮窗出现
      2. 检查浮窗 label（通过 Tauri window API）
    Expected Result: 浮窗数量 == 屏幕数量
    Evidence: .sisyphus/evidence/task-17-multi-window.txt

  Scenario: 浮窗位置持久化
    Tool: Playwright
    Steps:
      1. 拖拽浮窗到新位置
      2. 关闭并重启应用
      3. 验证浮窗出现在上次位置
    Expected Result: 位置坐标一致（±5px 容差）
    Evidence: .sisyphus/evidence/task-17-position-persist.txt
  ```

  **提交**：YES
  - Message: `feat(ui): implement multi-screen floating window manager with position persistence`
  - Files: `crates/cicada-client/ui/src/components/FloatingWindowManager.tsx`, `crates/cicada-client/src/commands/*.rs`

- [ ] 18. **公告显示样式（普通/紧急/通知）**

  **要做什么**：
  - 在 `crates/cicada-client/ui/src/styles/announcements.css` 实现三种公告类型的视觉样式：
    - **普通公告**：白色/透明背景，黑色文字，蓝色细边框，常规字号
    - **紧急通知**：
      - 红色边框 3px + 闪烁动画（`@keyframes emergency-blink { 0%, 100% { border-color: red; } 50% { border-color: transparent; } }`）
      - 背景 `rgba(255, 0, 0, 0.08)`
      - 标题：大号粗体 + 红色文字 + 闪烁
      - 顶部固定横幅：「⚠️ 紧急通知」
      - 持续显示直到被新公告替换或手动关闭
    - **一般通知**：蓝色边框 2px，浅蓝背景，正常字号，右上角可关闭
  - 公告切换动画：淡入淡出（`transition: opacity 0.5s`）
  - 多条公告队列：如有未读公告，底部显示「下一条 →」导航按钮
  - 空状态：无公告时显示「暂无公告」（灰色文字，居中）
  - 长内容自动滚动：内容高度 > 可视区域 → `overflow-y: auto`
  - 字体自定义：从配置读取 `font_size`, `font_color`, `font_family`
  - 使用 CSS 变量统一样式管理（`--announcement-bg`, `--announcement-border`, `--announcement-font` 等）

  **不得做**：
  - ❌ 不要让紧急通知可被忽略（需显式确认/新公告覆盖）
  - ❌ 不使用 JavaScript 动画（纯 CSS 性能更好）

  **推荐 Agent Profile**：
  - **Category**: `visual-engineering`
    - 原因：纯前端样式 + 动画，视觉效果密集
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 16, 17 并行）
  - **并行组**：Wave 4
  - **阻塞**：Task 24
  - **被阻塞**：Task 7, 16（需要 WS 推送和浮窗组件）

  **参考**：
  - CSS `@keyframes` 动画：`https://developer.mozilla.org/en-US/docs/Web/CSS/@keyframes`
  - CSS 变量：`https://developer.mozilla.org/en-US/docs/Web/CSS/Using_CSS_custom_properties`

  **验收标准**：
  - [ ] 三种类型视觉差异明显
  - [ ] 紧急通知闪烁动画正常
  - [ ] 公告切换有过渡效果
  - [ ] 空状态正确显示

  **QA 场景**：
  ```
  Scenario: 三种公告类型视觉对比
    Tool: Playwright
    Steps:
      1. 依次发送普通、紧急、通知三种公告
      2. 每切换一次截图
      3. 对比三张截图，验证边框颜色和样式差异
    Expected Result: 紧急=红色闪烁，通知=蓝色，普通=灰色
    Evidence: .sisyphus/evidence/task-18-all-types.png

  Scenario: 空状态
    Tool: Playwright
    Preconditions: Mock 服务器无公告
    Steps:
      1. 启动应用
      2. 验证浮窗显示「暂无公告」
      3. 截图
    Expected Result: 文字居中，灰色，表示空状态
    Evidence: .sisyphus/evidence/task-18-empty-state.png

  Scenario: 公告队列导航
    Tool: Playwright
    Preconditions: Mock 服务器有 3 条公告
    Steps:
      1. 验证显示第一条
      2. 点击「下一条 →」
      3. 验证显示第二条
    Expected Result: 按钮可点击，公告正确切换
    Evidence: .sisyphus/evidence/task-18-queue-nav.png
  ```

  **提交**：YES
  - Message: `feat(ui): implement three announcement display styles with animations`
  - Files: `crates/cicada-client/ui/src/styles/announcements.css`

- [ ] 19. **登录流程 UI**

  **要做什么**：
  - 在 `crates/cicada-client/ui/src/pages/Login.tsx` 实现登录流程：
    - OAuth 浏览器授权流程：
      1. 用户点击「登录」→ Rust 端生成 PKCE challenge + 授权 URL
      2. 打开系统默认浏览器（`tauri::api::shell::open`）
      3. 用户在浏览器中登录 SECTL 并授权
      4. SECTL 回调到本地 HTTP 服务器（Rust 端启动临时 `localhost` 服务器接收回调）
      5. 提取 authorization code
      6. 用 code + code_verifier 换取 tokens（Task 5）
      7. 存储 Token（Task 14）
      8. UI 更新为已登录状态
    - Rust 端实现：
      - 临时本地 HTTP 服务器（`tiny_http` 或 `axum`）：监听 `localhost:PORT/callback`
      - 接收回调 → 提取 `code` 参数 → 关闭服务器
      - Tauri 命令：`start_login()`, `handle_callback(code)`
    - 前端状态：
      - 未登录：显示「登录 SECTL」按钮
      - 登录中：显示「等待浏览器授权...」+ spinner
      - 已登录：显示用户名 + 「退出登录」按钮
    - 错误处理：授权失败、网络错误、用户取消
    - 自动登录：启动时检查已存储 Token → 自动验证有效性（introspect）→ 有效则跳过登录

  **不得做**：
  - ❌ 不在 Tauri WebView 内嵌登录页面（使用系统浏览器，安全性更好）
  - ❌ 不在 UI 中显示 Token 内容

  **推荐 Agent Profile**：
  - **Category**: `visual-engineering`
    - 原因：前端 UI + Rust 本地 HTTP 服务器 + OAuth 回调处理
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 20, 21 并行）
  - **并行组**：Wave 5
  - **阻塞**：Task 22
  - **被阻塞**：Task 5, 8, 10, 14（需要 OAuth 客户端、API 客户端、Tauri 项目、Token 存储）

  **参考**：
  - SECTL OAuth 流程（草案中已记录完整端点）
  - `tiny_http` crate：轻量本地 HTTP 服务器用于 OAuth 回调
  - Tauri `shell::open`：`https://v1.tauri.app/v1/api/js/shell#open`

  **验收标准**：
  - [ ] 登录按钮可点击，打开浏览器
  - [ ] 回调接收正常（Mock 模式测试）
  - [ ] Token 存储后 UI 状态更新
  - [ ] 退出登录清除 Token

  **QA 场景**：
  ```
  Scenario: 登录流程（Mock 模式）
    Tool: Playwright
    Steps:
      1. 点击「登录 SECTL」按钮
      2. 验证浏览器被打开（或 Mock 中直接模拟回调）
      3. 模拟回调 code → UI 显示已登录状态
      4. 点击「退出登录」
      5. 验证回到未登录状态
    Expected Result: 完整登录-退出循环正常
    Evidence: .sisyphus/evidence/task-19-login-flow.png

  Scenario: 自动登录（已存储 Token）
    Tool: Playwright
    Preconditions: Token 已存储且有效
    Steps:
      1. 重启应用
      2. 验证 UI 直接显示已登录（用户名可见）
    Expected Result: 无需手动登录
    Evidence: .sisyphus/evidence/task-19-auto-login.txt

  Scenario: 登录失败处理
    Tool: Playwright
    Steps:
      1. Mock 服务器返回 401
      2. 验证 UI 显示错误提示「登录失败，请重试」
    Expected Result: 错误提示清晰，可重试
    Evidence: .sisyphus/evidence/task-19-login-error.png
  ```

  **提交**：YES
  - Message: `feat(ui): implement OAuth login flow with browser-based SECTL authorization`
  - Files: `crates/cicada-client/ui/src/pages/Login.tsx`, `crates/cicada-client/src/commands/auth.rs`

- [ ] 20. **公告编辑器（富文本 + 发布）**

  **要做什么**：
  - 在 `crates/cicada-client/ui/src/pages/AnnouncementEditor.tsx` 实现公告编辑器：
    - 表单字段：
      - 标题（必填，文本输入框）
      - 公告类型（下拉：普通/紧急/通知）
      - 发布者姓名（文本输入框，默认填充 SECTL user_name，可修改）
      - 内容编辑器：富文本编辑
        - 使用轻量富文本编辑器（如 `@tiptap/react` 或 `quill` 或简单 `contenteditable`）
        - 支持：加粗、斜体、字号、颜色、图片插入（Base64 或 URL）
        - 实时预览（分屏：编辑 | 预览）
    - 操作按钮：
      - 「预览」— 在浮窗上实时预览（本地渲染，不发布）
      - 「发布」— 调用 API `POST /api/v1/announcements`（需登录）
      - 「存草稿」— 保存到本地 localStorage（可选）
    - 发布后：清空表单，显示成功提示
    - 错误处理：标题为空验证、发布失败提示、网络错误重试
    - 已发布列表（可选，底部展示最近发布的 5 条）

  **不得做**：
  - ❌ 不实现完整的所见即所得编辑器（轻量即可，内容可纯 HTML）
  - ❌ 不在发布前做服务端内容审核（客户端只做基本验证）

  **推荐 Agent Profile**：
  - **Category**: `visual-engineering`
    - 原因：前端富文本编辑 + API 集成
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 19, 21 并行）
  - **并行组**：Wave 5
  - **阻塞**：Task 22, 24
  - **被阻塞**：Task 8, 10, 19（需要 API 客户端、Tauri 项目、登录状态）

  **参考**：
  - `@tiptap/react`：`https://tiptap.dev/docs/editor/getting-started/install/react`
  - 或纯 `contenteditable` + `document.execCommand`

  **验收标准**：
  - [ ] 表单验证：标题为空时阻止发布
  - [ ] 富文本编辑基本功能（加粗、颜色）
  - [ ] 发布成功后 Mock 服务器收到请求
  - [ ] 浮窗实时预览正常

  **QA 场景**：
  ```
  Scenario: 发布普通公告
    Tool: Playwright
    Preconditions: 已登录
    Steps:
      1. 输入标题: "测试公告"
      2. 选择类型: 普通
      3. 输入内容: "<b>粗体文字</b>"
      4. 点击「预览」→ 验证浮窗显示公告
      5. 点击「发布」
      6. 验证成功提示出现
    Expected Result: 公告成功创建，API 返回 201
    Evidence: .sisyphus/evidence/task-20-publish.png

  Scenario: 空标题验证
    Tool: Playwright
    Steps:
      1. 标题留空
      2. 点击「发布」
      3. 验证「标题不能为空」提示
    Expected Result: 表单验证阻止提交
    Evidence: .sisyphus/evidence/task-20-validation.png

  Scenario: 发布者姓名覆盖
    Tool: Playwright
    Steps:
      1. 修改发布者输入框为 "李老师"
      2. 发布公告
      3. 验证 Mock 服务器收到的 payload.publisher_name == "李老师"
    Expected Result: 自定义姓名生效
    Evidence: .sisyphus/evidence/task-20-publisher-override.txt
  ```

  **提交**：YES
  - Message: `feat(ui): implement announcement editor with rich text and live preview`
  - Files: `crates/cicada-client/ui/src/pages/AnnouncementEditor.tsx`

- [ ] 21. **设置窗口**

  **要做什么**：
  - 在 `crates/cicada-client/ui/src/pages/Settings.tsx` 实现设置窗口：
    - 标签页式布局（Tabs）：
      1. **连接设置**：
         - 服务器地址（REST API）：文本输入 + 「测试连接」按钮
         - WebSocket 地址：文本输入
         - SECTL Client ID：文本输入
         - 心跳间隔（秒）：数字输入（默认 30）
      2. **显示设置**：
         - 字体：下拉选择 + 大小滑块（12-72px）
         - 文字颜色：颜色选择器
         - 背景颜色：颜色选择器 + 透明度滑块（0-100%）
         - 浮窗尺寸：宽 x 高（px）
         - 默认屏幕：下拉选择
      3. **行为设置**：
         - 开机自启：开关（调用 Task 13）
         - 启动时最小化到托盘：开关
         - 启动时自动显示浮窗：开关
         - 浮窗默认置顶：开关
         - 默认启动模式：显示 / 管理
      4. **关于**：
         - 版本号：从 `tauri.conf.json` 读取
         - 「检查更新」按钮（调用 Task 15）
         - GitHub 链接
    - 保存按钮：「应用」— 即时生效 + 「保存」— 持久化到配置文件
    - 取消/重置：「恢复默认」
    - 从 Rust 端加载当前配置：`invoke('get_config')` → `AppConfig`
    - 保存配置到 Rust 端：`invoke('update_config', { config })`

  **不得做**：
  - ❌ 不在设置窗口中暴露 Token（安全考虑）
  - ❌ 不修改 SECTL OAuth 端点（固定在代码中）

  **推荐 Agent Profile**：
  - **Category**: `visual-engineering`
    - 原因：前端表单密集型 UI + 多个交互功能
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 19, 20 并行）
  - **并行组**：Wave 5
  - **阻塞**：Task 24
  - **被阻塞**：Task 3, 10, 13, 14（需要配置管理、Tauri 项目、自启、Token 存储）

  **参考**：
  - Tauri `invoke` 前后端通信：`https://v1.tauri.app/v1/guides/features/command`
  - React 表单组件库（可选 `@mui/material` 或纯 CSS）

  **验收标准**：
  - [ ] 4 个标签页全部可切换
  - [ ] 配置修改 → 保存 → 重启 → 配置持久化
  - [ ] 「测试连接」可调用 API 并显示结果
  - [ ] 开机自启开关功能正常

  **QA 场景**：
  ```
  Scenario: 配置保存和加载
    Tool: Playwright
    Steps:
      1. 打开设置窗口
      2. 修改字体大小为 24px
      3. 点击「保存」
      4. 关闭设置窗口
      5. 重新打开设置窗口
      6. 验证字体大小仍为 24px
    Expected Result: 配置持久化成功
    Evidence: .sisyphus/evidence/task-21-config-persist.png

  Scenario: 测试连接
    Tool: Playwright
    Preconditions: Mock 服务器运行
    Steps:
      1. 输入 Mock 服务器地址: http://localhost:3001
      2. 点击「测试连接」
      3. 验证显示「连接成功」
    Expected Result: 连接测试通过
    Evidence: .sisyphus/evidence/task-21-test-connection.png

  Scenario: 恢复默认
    Tool: Playwright
    Steps:
      1. 修改多项设置
      2. 点击「恢复默认」
      3. 验证所有设置恢复默认值
    Expected Result: 设置重置为默认
    Evidence: .sisyphus/evidence/task-21-reset-default.png
  ```

  **提交**：YES
  - Message: `feat(ui): implement settings window with connection, display, behavior tabs`
  - Files: `crates/cicada-client/ui/src/pages/Settings.tsx`

- [ ] 22. **模式切换逻辑**

  **要做什么**：
  - 在 `crates/cicada-client/ui/src/App.tsx` 实现模式切换：
    - 两种模式：
      1. **显示模式**（Display Mode）：教室大屏使用，仅浮窗 + 托盘，无主窗口
      2. **管理模式**（Admin Mode）：管理员使用，主窗口显示公告编辑器和管理功能
    - 模式切换入口：
      - 托盘菜单：「切换到管理模式」（需登录验证）
      - 管理模式窗口：「切换到显示模式」
    - 切换逻辑：
      - 显示 → 管理：打开 OAuth 登录（如未登录）→ 显示主窗口 → 浮窗保持显示
      - 管理 → 显示：关闭主窗口 → 隐藏编辑器 → 仅保留浮窗 + 托盘
    - 默认模式：从配置 `default_mode` 读取
    - Tauri 状态管理：使用 React Context 管理全局模式状态
    - 模式相关 UI 变化：
      - 显示模式：托盘 Tooltip 显示「知了 - 显示模式」
      - 管理模式：托盘 Tooltip 显示「知了 - 管理模式」
  - Rust 端：`set_mode(mode: AppMode)` Tauri 命令
  - 单元测试：模式切换状态机

  **不得做**：
  - ❌ 不让显示模式意外进入管理模式（必须验证登录）
  - ❌ 不丢失模式切换过程中的浮窗状态

  **推荐 Agent Profile**：
  - **Category**: `deep`
    - 原因：涉及状态机 + 多窗口协调 + Tauri IPC
  - **Skills**: []

  **并行化**：
  - **可并行运行**：NO（依赖 Task 19, 20 完成）
  - **并行组**：Wave 5
  - **阻塞**：Task 24
  - **被阻塞**：Task 10, 11, 19, 20（需要 Tauri 项目、托盘、登录、编辑器）

  **参考**：
  - React Context API：`https://react.dev/reference/react/createContext`
  - Tauri 状态管理：`tauri::State`

  **验收标准**：
  - [ ] 显示 → 管理模式切换需要登录
  - [ ] 管理 → 显示模式切换平滑无闪烁
  - [ ] 浮窗在模式切换中不丢失

  **QA 场景**：
  ```
  Scenario: 显示模式切换到管理模式（需登录）
    Tool: Playwright
    Preconditions: 显示模式运行中
    Steps:
      1. 右键托盘 → 点击「切换到管理模式」
      2. 验证出现登录界面（未登录时）
      3. 登录成功后验证主窗口显示
    Expected Result: 模式切换成功，主窗口显示编辑器
    Evidence: .sisyphus/evidence/task-22-display-to-admin.png

  Scenario: 管理模式切换到显示模式
    Tool: Playwright
    Preconditions: 管理模式运行中
    Steps:
      1. 点击「切换到显示模式」
      2. 验证主窗口关闭
      3. 验证浮窗仍在显示
    Expected Result: 平滑切换，浮窗不中断
    Evidence: .sisyphus/evidence/task-22-admin-to-display.png
  ```

  **提交**：YES
  - Message: `feat(client): implement display/admin mode switching with login gate`
  - Files: `crates/cicada-client/ui/src/App.tsx`, `crates/cicada-client/src/commands/mode.rs`

- [ ] 23. **手机 Web PWA**

  **要做什么**：
  - 在 `mobile/` 目录创建独立的 React PWA 项目（Vite + React + TypeScript）：
    - **登录页面**（`Login.tsx`）：
      - 网页版 OAuth 授权流程（PKCE 在浏览器中实现）
      - 登录按钮 → 跳转 SECTL 授权页面（`window.location.href`）
      - 回调页面（`Callback.tsx`）：提取 URL 中的 `code` → 换取 Token → 存储到 localStorage
      - 已登录状态检测
    - **发布页面**（`Publish.tsx`）：
      - 公告编辑器（简化版）：
        - 标题输入框
        - 类型选择（普通/紧急/通知）
        - 内容输入（纯文本 + 基本 HTML 标签支持）
        - 发布者姓名输入（默认从 SECTL user_name 填充）
        - 「发布」按钮 → 调用 REST API
        - 「清空」按钮
      - 发布历史列表（最近 10 条，可滑动刷新）
    - **公告列表页面**（`AnnouncementList.tsx`）：
      - 拉取并显示已发布公告列表
      - 下拉刷新
      - 点击查看详情
  - PWA 特性：
    - `manifest.json`：应用名称「知了」、图标、主题色
    - Service Worker：离线缓存基础页面
    - 添加到主屏幕提示
  - 响应式设计：适配 320px-768px 移动屏幕
  - API 调用复用 `cicada-api` 的类型定义（拷贝或共享 JSON schema）
  - 配置文件：`.env` 中设置 `VITE_API_BASE_URL`

  **不得做**：
  - ❌ 不实现浮窗功能（手机端仅发布）
  - ❌ 不引入 React Native（纯 Web PWA）
  - ❌ 不在此实现富文本编辑器（简单即可）

  **推荐 Agent Profile**：
  - **Category**: `visual-engineering`
    - 原因：前端 PWA + 响应式 + OAuth Web 流程
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 24, 25 并行，但 Task 24 依赖此任务完成）
  - **并行组**：Wave 6
  - **阻塞**：Task 24
  - **被阻塞**：Task 4, 8（需要 API 规范和 REST 客户端模式）

  **参考**：
  - SECTL OAuth Web 流程（草案中记录）
  - Vite PWA 插件：`https://vite-pwa-org.netlify.app/`
  - 响应式设计最佳实践

  **验收标准**：
  - [ ] `npm run build` 在 `mobile/` 下成功
  - [ ] 登录流程在手机浏览器中可走通（Mock 模式）
  - [ ] 发布公告后 Mock 服务器收到请求
  - [ ] PWA manifest 有效

  **QA 场景**：
  ```
  Scenario: 手机 Web 登录流程
    Tool: Playwright (Mobile viewport: iPhone 12)
    Steps:
      1. 打开 mobile/index.html
      2. 点击「登录」
      3. 模拟 OAuth 回调
      4. 验证显示「已登录」
      Screenshot: .sisyphus/evidence/task-23-mobile-login.png
    Expected Result: 登录流程完整
    Evidence: .sisyphus/evidence/task-23-mobile-login.png

  Scenario: 手机发布公告
    Tool: Playwright (Mobile viewport)
    Preconditions: 已登录
    Steps:
      1. 输入标题: "手机端测试公告"
      2. 选择类型: 通知
      3. 输入内容: "从手机发布的测试"
      4. 点击「发布」
      5. 验证成功提示
      Screenshot: .sisyphus/evidence/task-23-mobile-publish.png
    Expected Result: API 返回 201
    Evidence: .sisyphus/evidence/task-23-mobile-publish.png

  Scenario: 响应式布局
    Tool: Playwright (3 viewports: 375px, 414px, 768px)
    Steps:
      1. 每个 viewport 截图
      2. 验证布局不错位、不溢出
    Expected Result: 所有尺寸布局正确
    Evidence: .sisyphus/evidence/task-23-responsive.png
  ```

  **提交**：YES
  - Message: `feat(mobile): implement PWA for mobile announcement publishing`
  - Files: `mobile/src/*`, `mobile/package.json`, `mobile/vite.config.ts`

- [ ] 24. **端到端集成测试**

  **要做什么**：
  - 编写端到端集成测试（跨越多个模块的完整流程）：
    - Rust 端集成测试（`tests/integration/` 在各 crate 中）：
      1. OAuth 完整流程：PKCE 生成 → 授权 URL 构建 → Token 交换（Mock）→ Token 刷新
      2. WebSocket 生命周期：连接 → 订阅 → 接收推送 → ACK → 断连 → 重连
      3. API + WS 联合：REST 创建公告 → WS 推送验证
      4. 配置持久化：修改 → 保存 → 重新加载 → 验证一致性
    - 前端 E2E 测试（Playwright）：
      1. 完整用户旅程：启动 → 登录 → 发布公告 → 浮窗显示 → 退出
      2. 多屏幕模拟：创建多浮窗 → 验证内容同步
      3. 设置修改 → 验证浮窗样式变化
      4. 紧急公告 → 验证闪烁动画 + 轮询队列
      5. Mock 服务器断连 → 验证 WebSocket 重连 + 消息恢复
    - 跨组件集成：
      - 桌面端发布 → 手机 Web 可见
      - 手机 Web 发布 → 桌面浮窗实时显示
  - 测试基础设施：
    - 配置 Playwright + Tauri 测试环境
    - 启动 Mock 服务器作为测试 fixture
    - 测试数据工厂函数

  **不得做**：
  - ❌ 不在 CI 中运行 Tauri GUI 测试（仅 `cargo test`）
  - ❌ 不让 E2E 测试依赖真实 SECTL 服务器

  **推荐 Agent Profile**：
  - **Category**: `deep`
    - 原因：跨模块集成测试 + 多工具（Playwright + Rust test + Mock）
  - **Skills**: [`playwright`]
    - `playwright`：浏览器 E2E 自动化

  **并行化**：
  - **可并行运行**：YES（与 Task 23, 25 并行）
  - **并行组**：Wave 6
  - **阻塞**：Task 25
  - **被阻塞**：Task 7, 8, 16-23（需要所有功能模块就绪）

  **参考**：
  - Rust 集成测试：`tests/` 目录约定
  - Playwright + Tauri 测试

  **验收标准**：
  - [ ] `cargo test --workspace` 全部集成测试通过
  - [ ] Playwright E2E 测试至少 5 个场景通过
  - [ ] 跨组件（桌面+手机）数据流验证

  **QA 场景**：
  ```
  Scenario: 完整发布到显示流程
    Tool: Playwright + Bash
    Steps:
      1. 启动 Mock 服务器
      2. 启动 Tauri 应用
      3. Playwright: 登录 → 发布公告
      4. Playwright: 验证浮窗显示新公告
      5. Bash: curl 验证 Mock 服务器收到 POST
    Expected Result: 全链路数据一致
    Evidence: .sisyphus/evidence/task-24-full-flow.png

  Scenario: WebSocket 断连恢复
    Tool: Bash
    Steps:
      1. cargo test -p cicada-core -- ws::integration::test_disconnect_recovery
      2. 模拟服务器断连 10 秒
      3. 验证客户端自动重连成功
      4. 验证重连后仍能收到新消息
    Expected Result: 重连成功，消息不丢失
    Evidence: .sisyphus/evidence/task-24-ws-recovery.txt
  ```

  **提交**：YES
  - Message: `test: add end-to-end integration tests across all modules`
  - Files: `crates/*/tests/**/*.rs`, `e2e/*.spec.ts`

- [ ] 25. **跨平台构建打包**

  **要做什么**：
  - 配置 Tauri 打包配置（`tauri.conf.json` → `tauri.bundle`）：
    - Windows：`.msi` 安装包（x86 + x64 架构）
      - 安装路径：`%ProgramFiles%/Cicada/`
      - 开始菜单快捷方式
      - 桌面快捷方式（可选）
      - 卸载程序
    - Linux：`.deb`（Debian/Ubuntu） + `.AppImage`（通用）
      - `.desktop` 文件
      - 图标嵌入
    - macOS：`.dmg` 安装镜像
      - 应用程序拖放到 `/Applications`
  - 构建脚本：
    - `scripts/build.sh`：一键构建所有平台包
    - 使用 GitHub Actions（Task 6 的 release workflow）自动构建
  - x86 + x64 双架构支持：
    - Windows：`cargo tauri build --target i686-pc-windows-msvc` + `x86_64-pc-windows-msvc`
    - Linux：`i686-unknown-linux-gnu` + `x86_64-unknown-linux-gnu`
  - 图标资源：
    - 创建应用图标（`app-icon.png`，1024x1024）
    - 使用 `tauri icon` 命令生成各平台所需尺寸
  - 签名（可选骨架）：
    - Windows：代码签名证书配置占位
    - macOS：开发者 ID 签名配置占位
  - Windows 7 兼容性：
    - 确保链接到兼容的 Windows SDK 版本
    - 测试 Windows 7 x86 运行

  **不得做**：
  - ❌ 不在此任务中修复功能 bug（仅打包配置）
  - ❌ 不签署发布包（骨架即可）

  **推荐 Agent Profile**：
  - **Category**: `deep`
    - 原因：跨平台打包 + 多架构 + CI 集成
  - **Skills**: []

  **并行化**：
  - **可并行运行**：YES（与 Task 23, 24 并行）
  - **并行组**：Wave 6
  - **阻塞**：F1-F4
  - **被阻塞**：Task 24（需要集成测试先通过）

  **参考**：
  - Tauri v1 打包指南：`https://v1.tauri.app/v1/guides/building/`
  - `tauri-bundler` 配置：`https://v1.tauri.app/v1/api/config#bundleconfig`

  **验收标准**：
  - [ ] `cargo tauri build` 在当前平台成功生成安装包
  - [ ] 安装包可安装/卸载
  - [ ] 应用图标在所有平台显示正确
  - [ ] CI release workflow 可触发构建

  **QA 场景**：
  ```
  Scenario: Windows MSI 构建
    Tool: Bash
    Steps:
      1. cargo tauri build --target x86_64-pc-windows-msvc
      2. ls src-tauri/target/release/bundle/msi/
      3. 验证 .msi 文件存在且 > 5MB
    Expected Result: MSI 安装包生成成功
    Evidence: .sisyphus/evidence/task-25-windows-msi.txt

  Scenario: Linux AppImage 构建
    Tool: Bash
    Steps:
      1. cargo tauri build --target x86_64-unknown-linux-gnu
      2. ls src-tauri/target/release/bundle/appimage/
      3. 验证 .AppImage 文件存在
    Expected Result: AppImage 生成成功，可执行
    Evidence: .sisyphus/evidence/task-25-linux-appimage.txt

  Scenario: 图标资源完整
    Tool: Bash
    Steps:
      1. ls crates/cicada-client/icons/
      2. 验证包含多尺寸图标
    Expected Result: 所有必需图标尺寸存在
    Evidence: .sisyphus/evidence/task-25-icons.txt
  ```

  **提交**：YES
  - Message: `build: configure cross-platform packaging for Windows/Linux/macOS x86+x64`
  - Files: `crates/cicada-client/tauri.conf.json`, `scripts/build.sh`, `crates/cicada-client/icons/*`

---

## Final Verification Wave（最终验证波次 — 所有实现任务后）

> 4 个审查 Agent 并行运行。必须全部 APPROVE。汇总结果呈现给用户，获得明确确认后方可标记完成。
> **不要自动进行。等待用户明确批准后再标记工作完成。**

- [ ] F1. **计划合规审计** — `oracle`
  从头到尾阅读计划。检查每个"必须包含"：验证实现存在（读文件、curl 端点、运行命令）。检查每个"必须排除"：搜索代码库中的禁止模式——如果发现则拒绝并标注 `文件:行号`。检查 `.sisyphus/evidence/` 中的证据文件是否存在。将交付物与计划进行比对。
  输出：`必须包含 [N/N] | 必须排除 [N/N] | 任务 [N/N] | 结论：APPROVE/REJECT`

- [ ] F2. **代码质量审查** — `unspecified-high`
  运行 `cargo clippy --workspace -- -D warnings` + `cargo fmt --check` + `cargo test --workspace`。审查所有变更文件：`unwrap()` 使用、空 `catch`/错误吞没、`println!` 调试残留、注释掉的代码。检查 AI slop：过度注释、过度抽象、泛型命名（data/result/item/temp）。
  输出：`构建 [PASS/FAIL] | Clippy [PASS/FAIL] | 测试 [N pass/N fail] | 文件 [N clean/N issues] | 结论`

- [ ] F3. **实际手工 QA** — `unspecified-high`（+ `playwright` skill）
  从干净状态开始。执行每个任务中的**每一个** QA 场景——严格按照步骤，捕获证据。测试跨任务集成（功能组合使用，不是孤立测试）。测试边缘情况：空状态、无效输入、快速操作。保存到 `.sisyphus/evidence/final-qa/`。
  输出：`场景 [N/N pass] | 集成 [N/N] | 边缘情况 [N tested] | 结论`

- [ ] F4. **范围保真度检查** — `deep`
  对每个任务：阅读"要做什么"，阅读实际 diff（git log/diff）。验证 1:1 —— 规范中的每项都已构建（无遗漏），规范外的任何内容都未构建（无蔓延）。检查"不得做"合规性。检测跨任务污染：Task N 触碰 Task M 的文件。标记未记录的变更。
  输出：`任务 [N/N compliant] | 污染 [CLEAN/N issues] | 未记录 [CLEAN/N files] | 结论`

---

## 提交策略

每个任务独立提交，格式：`type(scope): 描述`

示例：
- `feat(core): add announcement types and serialization`
- `feat(auth): implement OAuth PKCE client`
- `feat(ui): add floating window HTML renderer`
- `test(ws): add websocket reconnection tests`

---

## 成功标准

### 验证命令
```bash
# 全部构建通过
cargo build --workspace

# 全部测试通过
cargo test --workspace

# 零 Clippy 警告
cargo clippy --workspace -- -D warnings

# 桌面应用启动（开发模式）
cargo tauri dev

# Mock 服务器启动
cargo run -p cicada-mock-server

# API 端点验证
curl http://localhost:3001/api/v1/announcements
```

### 最终检查清单
- [ ] 所有"必须包含"项已实现
- [ ] 所有"必须排除"模式不存在
- [ ] 所有自动化测试通过（Rust + 前端）
- [ ] 三平台构建成功（Windows/Linux/macOS）
- [ ] 浮窗 HTML 渲染正确
- [ ] OAuth 登录流程可走通
- [ ] WebSocket 实时推送正常
- [ ] 手机 Web 可发布公告
