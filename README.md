# Cicada (知了)

一个给学校用的公告投屏工具。后台写好公告点发布，教室大屏上实时显示。不用浏览器，一个无边框窗口贴在桌面上，普通、紧急、一般通知三种样式。

电脑右下角托盘常驻，有个设置面板调字体颜色透明度什么的。手机浏览器打开也能发公告。

## 能跑的平台

- Windows 7 及以上 (x86 / x64)
- Linux (x86_64)
- macOS (x86_64 / arm64)

手机端是个 PWA，浏览器打开就行，不用装 App。

## 安装

每个平台提供这几种包，挑顺手的下：

| 平台 | 格式 |
|------|------|
| Windows | `.exe` 安装包、`.zip` 便携版 |
| Linux | `.deb`、`.AppImage`、`.zip` |
| macOS | `.dmg` |

下载后正常装就行。Windows 7 用户用 zip 版解压直接跑。

## 开发/自行编译

先装 Rust: https://rustup.rs

```bash
# 核心库和 Mock 服务器
cargo build -p cicada-core -p cicada-api -p cicada-mock-server
cargo test --workspace --exclude cicada-client

# Mock 服务器 (开发用，模拟后端)
cargo run -p cicada-mock-server
# 然后 http://localhost:3001/api/v1/announcements

# 桌面客户端 (需要系统 webkit/gtk 依赖)
cd crates/cicada-client/ui && npm install
cargo tauri dev
```

**Linux 用户额外装这些：**
```bash
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev
```

**手机 PWA：**
```bash
cd mobile && npm install && npm run dev
```

## 认证

用 SECTL 统一认证 (OAuth 2.0 + PKCE)。不存密码，只存 token 到系统钥匙串。发布公告才需要登录，大屏显示端不需要。

Mock 模式下不验证 token，填什么都行。

## 公告类型

- **普通** — 灰底白字
- **紧急** — 红底闪烁边框，需要确认/被覆盖才消失
- **通知** — 蓝底，右上角可关闭

内容支持 HTML，可以插图片。

## 配置

第一次启动自动生成默认配置，存在：
- Windows: `%APPDATA%/Cicada/config.toml`
- Linux: `~/.config/cicada/config.toml`
- macOS: `~/Library/Application Support/Cicada/config.toml`

可配的项：服务器地址、Client ID、字号颜色透明度、窗口尺寸位置、是否自启、是否置顶。

## 开发

```bash
# 跑全部测试
cargo test -p cicada-core -p cicada-api -p cicada-mock-server

# Mock 服务器跑起来后可以 curl 测
curl http://localhost:3001/api/v1/announcements
curl -X POST http://localhost:3001/api/v1/announcements \
  -H "Content-Type: application/json" \
  -d '{"title":"test","content_html":"<p>hello</p>","announcement_type":"normal","publisher_name":"dev"}'
```

没什么特别要注意的。Tauri 那部分因为要 webkit，没法在纯命令行环境编译。

## License

GPL-3.0
