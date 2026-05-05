import { useEffect, useState } from "react";
import { invoke } from "../utils/tauri";
import {
  Card, CardContent, CardHeader, Tabs, Tab, TextField, MenuItem,
  Button, Typography, Switch, Slider, Stack, Box, FormControlLabel,
  FormGroup, Chip, Divider, IconButton, Alert, CircularProgress,
  Paper, Fade, Tooltip, Skeleton
} from "@mui/material";
import {
  Save, Restore, GitHub, ContentCopy, Wifi, Link,
  Palette, Tune, Info, CheckCircle, ErrorOutline
} from "@mui/icons-material";

interface AppConfig {
  connection: {
    server_url: string;
    ws_url: string;
    client_id: string;
    heartbeat_interval_secs: number;
  };
  display: {
    font_size: number;
    font_color: string;
    font_family: string;
    bg_color: string;
    bg_opacity: number;
    window_width: number;
    window_height: number;
  };
  behavior: {
    auto_start: boolean;
    start_minimized: boolean;
    show_floating_on_start: boolean;
    floating_topmost: boolean;
    default_mode: string;
  };
}

interface BuildInfo {
  version: string;
  git_hash: string;
  build_date: string;
  rustc_version: string;
  target: string;
}

const Settings: React.FC = () => {
  const [config, setConfig] = useState<AppConfig | null>(null);
  const [buildInfo, setBuildInfo] = useState<BuildInfo | null>(null);
  const [tab, setTab] = useState(0);
  const [toast, setToast] = useState("");
  const [testing, setTesting] = useState(false);
  const [testResult, setTestResult] = useState<"ok" | "fail" | null>(null);
  const [copied, setCopied] = useState(false);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  useEffect(() => {
    const loadConfig = async () => {
      try {
        setLoading(true);
        setError("");
        const cfg = await invoke<AppConfig>("get_config");
        setConfig(cfg);
      } catch (e) {
        setError("加载配置失败: " + String(e));
        // 使用默认配置作为后备
        setConfig({
          connection: {
            server_url: "",
            ws_url: "",
            client_id: "",
            heartbeat_interval_secs: 30,
          },
          display: {
            font_size: 24,
            font_color: "#ffffff",
            font_family: "",
            bg_color: "#000000",
            bg_opacity: 0.8,
            window_width: 400,
            window_height: 300,
          },
          behavior: {
            auto_start: false,
            start_minimized: false,
            show_floating_on_start: true,
            floating_topmost: true,
            default_mode: "display",
          },
        });
      } finally {
        setLoading(false);
      }
    };

    const loadBuildInfo = async () => {
      try {
        const info = await invoke<BuildInfo>("get_build_info");
        setBuildInfo(info);
      } catch {
        // 构建信息可选，失败不处理
      }
    };

    loadConfig();
    loadBuildInfo();
  }, []);

  const set = (section: string, field: string, value: unknown) => {
    if (!config) return;
    setConfig({ ...config, [section]: { ...(config as any)[section], [field]: value } });
  };

  const save = async () => {
    if (!config) return;
    try {
      await invoke("update_config", { config });
      if (config.behavior.auto_start) {
        await invoke("enable_autostart").catch(() => {});
      } else {
        await invoke("disable_autostart").catch(() => {});
      }
      setToast("已保存");
      setTimeout(() => setToast(""), 2000);
    } catch (e) {
      setError("保存失败: " + String(e));
    }
  };

  const reset = async () => {
    try {
      const c = await invoke<AppConfig>("reset_config");
      setConfig(c);
      setToast("已恢复默认");
      setTimeout(() => setToast(""), 2000);
    } catch (e) {
      setError("恢复默认失败: " + String(e));
    }
  };

  const testConnection = async () => {
    if (!config) return;
    setTesting(true);
    setTestResult(null);
    try {
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), 5000);
      const res = await fetch(`${config.connection.server_url}/api/v1/announcements`, {
        signal: controller.signal,
        method: "HEAD",
      });
      clearTimeout(timeoutId);
      if (res.ok) {
        setTestResult("ok");
      } else {
        setTestResult("fail");
      }
    } catch (err: any) {
      if (err.name === "AbortError") {
        setTestResult("fail");
      } else {
        setTestResult("fail");
      }
    }
    setTesting(false);
  };

  const copyHash = async () => {
    if (!buildInfo) return;
    await navigator.clipboard.writeText(buildInfo.git_hash);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const tabLabels = [
    { label: "连接", icon: <Link fontSize="small" /> },
    { label: "显示", icon: <Palette fontSize="small" /> },
    { label: "行为", icon: <Tune fontSize="small" /> },
    { label: "关于", icon: <Info fontSize="small" /> },
  ];

  // 加载中状态
  if (loading) {
    return (
      <Card>
        <CardHeader title="设置" />
        <CardContent>
          <Stack spacing={3}>
            <Skeleton variant="rectangular" height={60} sx={{ borderRadius: "12px" }} />
            <Skeleton variant="rectangular" height={120} sx={{ borderRadius: "12px" }} />
            <Skeleton variant="rectangular" height={80} sx={{ borderRadius: "12px" }} />
          </Stack>
        </CardContent>
      </Card>
    );
  }

  // 错误状态（但有默认配置）
  if (error && !config) {
    return (
      <Card>
        <CardContent sx={{ p: 5, textAlign: "center" }}>
          <ErrorOutline sx={{ fontSize: 48, color: "error.main", mb: 2 }} />
          <Typography variant="h6" color="error" gutterBottom>
            加载失败
          </Typography>
          <Typography variant="body2" color="text.secondary" sx={{ mb: 3 }}>
            {error}
          </Typography>
          <Button
            variant="outlined"
            onClick={() => window.location.reload()}
            sx={{ borderRadius: "10px" }}
          >
            重新加载
          </Button>
        </CardContent>
      </Card>
    );
  }

  // 确保 config 不为 null
  const currentConfig = config!;

  return (
    <Fade in={true} timeout={500}>
      <Card>
        <CardHeader
          title="设置"
          action={
            <Stack direction="row" spacing={1.5}>
              <Button
                variant="outlined"
                startIcon={<Restore />}
                onClick={reset}
                sx={{ borderRadius: "10px" }}
              >
                恢复默认
              </Button>
              <Button
                variant="contained"
                startIcon={<Save />}
                onClick={save}
                sx={{
                  borderRadius: "10px",
                  boxShadow: "0 4px 12px rgba(25,118,210,0.3)",
                }}
              >
                保存
              </Button>
            </Stack>
          }
          sx={{ pb: 0 }}
        />

        <Tabs
          value={tab}
          onChange={(_, v) => setTab(v)}
          sx={{
            borderBottom: 1,
            borderColor: "divider",
            px: 2,
            mt: 1,
          }}
        >
          {tabLabels.map((t, i) => (
            <Tab
              key={i}
              icon={t.icon}
              iconPosition="start"
              label={t.label}
              sx={{
                textTransform: "none",
                minHeight: 44,
                borderRadius: "8px 8px 0 0",
              }}
            />
          ))}
        </Tabs>

        <CardContent sx={{ pt: 3 }}>
          {/* 错误提示 */}
          {error && (
            <Alert severity="warning" sx={{ mb: 3, borderRadius: "10px" }}>
              {error}
            </Alert>
          )}

          {/* 连接设置 */}
          {tab === 0 && (
            <Fade in={true} timeout={400}>
              <Stack spacing={3}>
                <Paper
                  variant="outlined"
                  sx={{ p: 3, borderRadius: "12px", bgcolor: "background.default" }}
                >
                  <Typography variant="subtitle2" sx={{ mb: 2, fontWeight: 600, color: "primary.main" }}>
                    服务器连接
                  </Typography>
                  <Stack spacing={2.5}>
                    <TextField
                      label="REST API 地址"
                      value={currentConfig.connection.server_url}
                      onChange={(e) => set("connection", "server_url", e.target.value)}
                      fullWidth
                      variant="outlined"
                      placeholder="https://api.example.com"
                    />
                    <TextField
                      label="WebSocket 地址"
                      value={currentConfig.connection.ws_url}
                      onChange={(e) => set("connection", "ws_url", e.target.value)}
                      fullWidth
                      variant="outlined"
                      placeholder="wss://ws.example.com"
                    />
                    <TextField
                      label="SECTL Client ID"
                      value={currentConfig.connection.client_id}
                      onChange={(e) => set("connection", "client_id", e.target.value)}
                      fullWidth
                      variant="outlined"
                    />
                  </Stack>
                </Paper>

                <Paper
                  variant="outlined"
                  sx={{ p: 3, borderRadius: "12px", bgcolor: "background.default" }}
                >
                  <Typography variant="subtitle2" sx={{ mb: 2, fontWeight: 600, color: "primary.main" }}>
                    连接测试
                  </Typography>
                  <Stack direction="row" spacing={2} alignItems="center" sx={{ mb: 2 }}>
                    <Button
                      variant="outlined"
                      startIcon={testing ? <CircularProgress size={16} /> : <Wifi />}
                      onClick={testConnection}
                      disabled={testing}
                      sx={{ borderRadius: "10px" }}
                    >
                      {testing ? "测试中..." : "测试连接"}
                    </Button>
                    {testResult === "ok" && (
                      <Chip
                        icon={<CheckCircle fontSize="small" />}
                        label="连接成功"
                        color="success"
                        size="small"
                      />
                    )}
                    {testResult === "fail" && (
                      <Alert severity="error" sx={{ py: 0, borderRadius: "8px" }}>
                        连接失败
                      </Alert>
                    )}
                  </Stack>
                </Paper>

                <Paper
                  variant="outlined"
                  sx={{ p: 3, borderRadius: "12px", bgcolor: "background.default" }}
                >
                  <Typography variant="subtitle2" sx={{ mb: 2, fontWeight: 600, color: "primary.main" }}>
                    心跳设置
                  </Typography>
                  <Typography variant="body2" sx={{ mb: 2 }}>
                    心跳间隔: <strong>{currentConfig.connection.heartbeat_interval_secs}s</strong>
                  </Typography>
                  <Slider
                    min={10}
                    max={120}
                    value={currentConfig.connection.heartbeat_interval_secs}
                    onChange={(_, v) => set("connection", "heartbeat_interval_secs", v as number)}
                    valueLabelDisplay="auto"
                    valueLabelFormat={(v) => `${v}s`}
                    marks={[
                      { value: 10, label: "10s" },
                      { value: 60, label: "60s" },
                      { value: 120, label: "120s" },
                    ]}
                  />
                </Paper>
              </Stack>
            </Fade>
          )}

          {/* 显示设置 */}
          {tab === 1 && (
            <Fade in={true} timeout={400}>
              <Stack spacing={3}>
                <Paper
                  variant="outlined"
                  sx={{ p: 3, borderRadius: "12px", bgcolor: "background.default" }}
                >
                  <Typography variant="subtitle2" sx={{ mb: 2, fontWeight: 600, color: "primary.main" }}>
                    字体设置
                  </Typography>
                  <Typography variant="body2" sx={{ mb: 2 }}>
                    字号: <strong>{currentConfig.display.font_size}px</strong>
                  </Typography>
                  <Slider
                    min={12}
                    max={72}
                    value={currentConfig.display.font_size}
                    onChange={(_, v) => set("display", "font_size", v as number)}
                    valueLabelDisplay="auto"
                    valueLabelFormat={(v) => `${v}px`}
                    marks={[
                      { value: 12, label: "12px" },
                      { value: 24, label: "24px" },
                      { value: 48, label: "48px" },
                      { value: 72, label: "72px" },
                    ]}
                    sx={{ mb: 3 }}
                  />

                  <Typography variant="body2" sx={{ mb: 1.5, fontWeight: 500 }}>
                    字体
                  </Typography>
                  <TextField
                    select
                    value={currentConfig.display.font_family}
                    onChange={(e) => set("display", "font_family", e.target.value)}
                    sx={{ width: 280 }}
                  >
                    <MenuItem value="sans-serif">sans-serif</MenuItem>
                    <MenuItem value="PingFang SC, sans-serif">PingFang SC</MenuItem>
                    <MenuItem value="Microsoft YaHei, sans-serif">Microsoft YaHei</MenuItem>
                    <MenuItem value="Noto Sans SC, sans-serif">Noto Sans SC</MenuItem>
                    <MenuItem value="serif">serif</MenuItem>
                    <MenuItem value="monospace">monospace</MenuItem>
                  </TextField>
                </Paper>

                <Paper
                  variant="outlined"
                  sx={{ p: 3, borderRadius: "12px", bgcolor: "background.default" }}
                >
                  <Typography variant="subtitle2" sx={{ mb: 2, fontWeight: 600, color: "primary.main" }}>
                    颜色设置
                  </Typography>
                  <Stack direction="row" spacing={4} sx={{ mb: 3 }}>
                    <Box>
                      <Typography variant="body2" gutterBottom sx={{ fontWeight: 500 }}>
                        文字颜色
                      </Typography>
                      <Box sx={{ display: "flex", alignItems: "center", gap: 1.5 }}>
                        <input
                          type="color"
                          value={currentConfig.display.font_color}
                          onChange={(e) => set("display", "font_color", e.target.value)}
                          style={{
                            width: 48,
                            height: 48,
                            border: "none",
                            borderRadius: "10px",
                            cursor: "pointer",
                            padding: 0,
                          }}
                        />
                        <Chip label={currentConfig.display.font_color} size="small" variant="outlined" />
                      </Box>
                    </Box>
                    <Box>
                      <Typography variant="body2" gutterBottom sx={{ fontWeight: 500 }}>
                        背景颜色
                      </Typography>
                      <Box sx={{ display: "flex", alignItems: "center", gap: 1.5 }}>
                        <input
                          type="color"
                          value={currentConfig.display.bg_color}
                          onChange={(e) => set("display", "bg_color", e.target.value)}
                          style={{
                            width: 48,
                            height: 48,
                            border: "none",
                            borderRadius: "10px",
                            cursor: "pointer",
                            padding: 0,
                          }}
                        />
                        <Chip label={currentConfig.display.bg_color} size="small" variant="outlined" />
                      </Box>
                    </Box>
                  </Stack>

                  <Typography variant="body2" sx={{ mb: 2 }}>
                    背景透明度: <strong>{Math.round(currentConfig.display.bg_opacity * 100)}%</strong>
                  </Typography>
                  <Slider
                    min={0}
                    max={100}
                    value={Math.round(currentConfig.display.bg_opacity * 100)}
                    onChange={(_, v) => set("display", "bg_opacity", (v as number) / 100)}
                    valueLabelDisplay="auto"
                    valueLabelFormat={(v) => `${v}%`}
                  />
                </Paper>

                <Paper
                  variant="outlined"
                  sx={{ p: 3, borderRadius: "12px", bgcolor: "background.default" }}
                >
                  <Typography variant="subtitle2" sx={{ mb: 2, fontWeight: 600, color: "primary.main" }}>
                    窗口尺寸
                  </Typography>
                  <Stack direction="row" spacing={3}>
                    <TextField
                      label="窗口宽度"
                      type="number"
                      value={currentConfig.display.window_width}
                      onChange={(e) => set("display", "window_width", Number(e.target.value))}
                      sx={{ width: 160 }}
                      InputProps={{ endAdornment: <Typography variant="caption" color="text.secondary">px</Typography> }}
                    />
                    <TextField
                      label="窗口高度"
                      type="number"
                      value={currentConfig.display.window_height}
                      onChange={(e) => set("display", "window_height", Number(e.target.value))}
                      sx={{ width: 160 }}
                      InputProps={{ endAdornment: <Typography variant="caption" color="text.secondary">px</Typography> }}
                    />
                  </Stack>
                </Paper>
              </Stack>
            </Fade>
          )}

          {/* 行为设置 */}
          {tab === 2 && (
            <Fade in={true} timeout={400}>
              <Stack spacing={3}>
                <Paper
                  variant="outlined"
                  sx={{ p: 3, borderRadius: "12px", bgcolor: "background.default" }}
                >
                  <Typography variant="subtitle2" sx={{ mb: 2, fontWeight: 600, color: "primary.main" }}>
                    启动行为
                  </Typography>
                  <FormGroup>
                    <FormControlLabel
                      control={
                        <Switch
                          checked={currentConfig.behavior.auto_start}
                          onChange={(e) => set("behavior", "auto_start", e.target.checked)}
                        />
                      }
                      label="开机自启"
                    />
                    <FormControlLabel
                      control={
                        <Switch
                          checked={currentConfig.behavior.start_minimized}
                          onChange={(e) => set("behavior", "start_minimized", e.target.checked)}
                        />
                      }
                      label="启动时最小化到托盘"
                    />
                    <FormControlLabel
                      control={
                        <Switch
                          checked={currentConfig.behavior.show_floating_on_start}
                          onChange={(e) => set("behavior", "show_floating_on_start", e.target.checked)}
                        />
                      }
                      label="启动时自动显示浮窗"
                    />
                    <FormControlLabel
                      control={
                        <Switch
                          checked={currentConfig.behavior.floating_topmost}
                          onChange={(e) => set("behavior", "floating_topmost", e.target.checked)}
                        />
                      }
                      label="浮窗默认置顶"
                    />
                  </FormGroup>
                </Paper>

                <Paper
                  variant="outlined"
                  sx={{ p: 3, borderRadius: "12px", bgcolor: "background.default" }}
                >
                  <Typography variant="subtitle2" sx={{ mb: 2, fontWeight: 600, color: "primary.main" }}>
                    默认模式
                  </Typography>
                  <TextField
                    select
                    label="默认启动模式"
                    value={currentConfig.behavior.default_mode}
                    onChange={(e) => set("behavior", "default_mode", e.target.value)}
                    sx={{ width: 220 }}
                  >
                    <MenuItem value="display">显示模式</MenuItem>
                    <MenuItem value="admin">管理模式</MenuItem>
                  </TextField>
                </Paper>
              </Stack>
            </Fade>
          )}

          {/* 关于页面 */}
          {tab === 3 && (
            <Fade in={true} timeout={400}>
              <Stack spacing={3}>
                <Paper
                  variant="outlined"
                  sx={{
                    p: 4,
                    borderRadius: "16px",
                    textAlign: "center",
                    bgcolor: "background.default",
                  }}
                >
                  {/* Logo */}
                  <Box
                    sx={{
                      width: 80,
                      height: 80,
                      borderRadius: "20px",
                      bgcolor: "primary.main",
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "center",
                      mx: "auto",
                      mb: 2,
                      boxShadow: "0 8px 24px rgba(25,118,210,0.25)",
                    }}
                  >
                    <Typography variant="h3" sx={{ color: "#fff", fontWeight: 700 }}>
                      知
                    </Typography>
                  </Box>

                  <Typography variant="h4" sx={{ fontWeight: 700, mb: 0.5 }}>
                    知了 Cicada
                  </Typography>
                  <Typography variant="body1" color="text.secondary" gutterBottom>
                    校园公告投屏系统
                  </Typography>

                  <Stack
                    direction="row"
                    spacing={1}
                    justifyContent="center"
                    sx={{ my: 2 }}
                  >
                    <Chip
                      label={buildInfo ? `v${buildInfo.version}` : "v0.1.0"}
                      color="primary"
                      size="small"
                      sx={{ fontWeight: 600 }}
                    />
                    <Chip label={buildInfo?.target || "unknown"} size="small" variant="outlined" />
                  </Stack>
                </Paper>

                {buildInfo && (
                  <Paper
                    variant="outlined"
                    sx={{ p: 3, borderRadius: "12px", bgcolor: "background.default" }}
                  >
                    <Typography variant="subtitle2" sx={{ mb: 2, fontWeight: 600, color: "primary.main" }}>
                      构建信息
                    </Typography>
                    <Stack spacing={1.5}>
                      <Stack direction="row" alignItems="center" spacing={1}>
                        <Typography variant="body2" color="text.secondary" sx={{ width: 80 }}>
                          提交哈希
                        </Typography>
                        <Typography variant="body2" sx={{ fontFamily: "monospace" }}>
                          {buildInfo.git_hash.slice(0, 8)}
                        </Typography>
                        <Tooltip title={copied ? "已复制" : "复制"}>
                          <IconButton size="small" onClick={copyHash}>
                            {copied ? <CheckCircle fontSize="small" color="success" /> : <ContentCopy fontSize="small" />}
                          </IconButton>
                        </Tooltip>
                      </Stack>
                      <Stack direction="row" alignItems="center" spacing={1}>
                        <Typography variant="body2" color="text.secondary" sx={{ width: 80 }}>
                          构建时间
                        </Typography>
                        <Typography variant="body2">{buildInfo.build_date}</Typography>
                      </Stack>
                      <Stack direction="row" alignItems="center" spacing={1}>
                        <Typography variant="body2" color="text.secondary" sx={{ width: 80 }}>
                          Rust 版本
                        </Typography>
                        <Typography variant="body2">{buildInfo.rustc_version}</Typography>
                      </Stack>
                    </Stack>
                  </Paper>
                )}

                <Paper
                  variant="outlined"
                  sx={{ p: 3, borderRadius: "12px", bgcolor: "background.default" }}
                >
                  <Typography variant="subtitle2" sx={{ mb: 2, fontWeight: 600, color: "primary.main" }}>
                    技术栈
                  </Typography>
                  <Stack direction="row" spacing={1} flexWrap="wrap" useFlexGap>
                    {["Tauri v1", "React 18", "MUI v6", "Rust", "OAuth 2.0 + PKCE"].map((tech) => (
                      <Chip key={tech} label={tech} size="small" variant="outlined" sx={{ borderRadius: "6px" }} />
                    ))}
                  </Stack>
                  <Divider sx={{ my: 2 }} />
                  <Typography variant="body2" color="text.secondary">
                    License: GPL-3.0
                  </Typography>
                  <Button
                    startIcon={<GitHub />}
                    sx={{ mt: 2, borderRadius: "10px" }}
                    variant="outlined"
                    onClick={() => window.open("https://github.com", "_blank")}
                  >
                    访问 GitHub
                  </Button>
                </Paper>
              </Stack>
            </Fade>
          )}
        </CardContent>

        {/* Toast 提示 */}
        <Fade in={!!toast} timeout={300}>
          <Box>
            {toast && (
              <Alert
                severity="success"
                sx={{
                  mx: 3,
                  mb: 3,
                  borderRadius: "10px",
                }}
                icon={<CheckCircle />}
              >
                {toast}
              </Alert>
            )}
          </Box>
        </Fade>
      </Card>
    </Fade>
  );
};

export default Settings;
