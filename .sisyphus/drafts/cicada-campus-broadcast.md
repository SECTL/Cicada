# Draft: Cicada (知了) - 校园公告投屏系统

## 项目概览
- **项目名称**: Cicada（知了）
- **语言**: Rust
- **平台**: Windows 7+, x86/x64, 手机
- **核心功能**: 后台一键发布公告，教室大屏实时显示

## 用户需求（已确认）
- [x] 使用 Rust 开发
- [x] 支持 Win7 以上，x86 和 x64
- [x] 支持手机端
- [x] 系统托盘图标
- [x] 单独的设置窗口
- [x] 置底/置顶浮窗（可自定义）
- [x] WebSocket 连接服务器
- [x] 支持自定义服务器，默认提供官方服务器
- [x] 发布消息需要登录账号
- [x] 可在手机发布，也可在本地修改
- [x] 需要完整的项目仓库文件规划

## 技术决策
- **GUI 框架**: Tauri v1（Web 技术栈 + Rust 后端，支持 Win7）
- **程序架构**: 单程序 + 模式切换（显示模式 / 管理模式，切换需登录）
- **服务端范围**: 先做客户端，服务端后补
- **手机端**: Android + Web（PWA 或响应式 Web）

- **Tauri 前端**: React + TypeScript
- **浮窗行为**: 常驻但可手动显隐，可切换置顶/正常层级（不是贴桌面）
- **设置窗口**: 全套（连接 + 显示 + 行为 + 系统）
- **多屏幕**: 支持，一台电脑多浮窗分别显示在不同屏幕
- **WebSocket**: 双向通信（推送公告 + 接收确认 + 心跳保活）
- **消息格式**: HTML 富文本 + 图片支持
- **消息类型**: 普通公告 / 紧急通知 / 一般通知
- **开机启动**: 自启 → 托盘 → 浮窗自动显示
- **自动更新**: 静默下载 + 重启安装
- **发布者姓名**: 可从 SECTL user_name 自动获取，也可手动覆盖
- **平台支持**: Windows 7+ / Linux / macOS, x86 + x64

## SECTL-auth 认证集成（已确认）
- **协议**: OAuth 2.0 + PKCE（强制），无需 client_secret
- **Base URL**: `https://appwrite.sectl.cn`
- **认证流程**: authorize → token → userinfo
- **设备标识**: device_uuid (标准 UUID 格式)
- **Token**: Access Token 可设永不过期，支持 Refresh Token
- **用户信息**: user_id, user_name, email, permission
- **远程退登**: Token 失效检测 + 本地清理

## 待确认问题
- WebSocket 实时推送协议
- 公告 API 端点定义
- 浮窗显示内容细节（文本格式、紧急通知样式）
- 置底层级实现方式（Windows 技术限制）
- 更新机制
- 其他系统行为

## 范围边界
- INCLUDE: （待定）
- EXCLUDE: （待定）
