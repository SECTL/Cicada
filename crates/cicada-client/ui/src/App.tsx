import { useState, useMemo } from "react";
import {
  createTheme, ThemeProvider, CssBaseline, Box, Drawer,
  List, ListItemButton, ListItemIcon, ListItemText,
  Typography, IconButton, Divider, Fade, Slide, Tooltip
} from "@mui/material";
import {
  DesktopWindows, Edit, Settings, LightMode, DarkMode,
  ChevronLeft, ChevronRight
} from "@mui/icons-material";
import Login from "./pages/Login";
import AnnouncementEditor from "./pages/AnnouncementEditor";
import SettingsPage from "./pages/Settings";

const DRAWER_WIDTH = 220;
const DRAWER_COLLAPSED_WIDTH = 72;

type Page = "display" | "admin" | "settings";

const pages = [
  { key: "display" as const, icon: <DesktopWindows />, label: "显示模式" },
  { key: "admin" as const, icon: <Edit />, label: "管理模式" },
  { key: "settings" as const, icon: <Settings />, label: "设置" },
];

function App() {
  const [page, setPage] = useState<Page>("admin");
  const [loggedIn, setLoggedIn] = useState(false);
  const [mode, setMode] = useState<"light" | "dark">("dark");
  const [drawerOpen, setDrawerOpen] = useState(true);

  const theme = useMemo(() => createTheme({
    palette: {
      mode,
      primary: { main: mode === "dark" ? "#90caf9" : "#1976d2" },
      background: {
        default: mode === "dark" ? "#0a0e17" : "#f5f7fa",
        paper: mode === "dark" ? "#111827" : "#ffffff",
      },
    },
    typography: {
      fontFamily: '"PingFang SC","Microsoft YaHei","Noto Sans SC",sans-serif',
    },
    components: {
      MuiDrawer: {
        styleOverrides: {
          paper: {
            borderRight: `1px solid ${mode === "dark" ? "rgba(255,255,255,0.08)" : "rgba(0,0,0,0.08)"}`,
            backgroundImage: "none",
          },
        },
      },
      MuiCard: {
        styleOverrides: {
          root: {
            borderRadius: 16,
            boxShadow: mode === "dark"
              ? "0 4px 20px rgba(0,0,0,0.4)"
              : "0 4px 20px rgba(0,0,0,0.06)",
            border: `1px solid ${mode === "dark" ? "rgba(255,255,255,0.06)" : "rgba(0,0,0,0.06)"}`,
          },
        },
      },
    },
  }), [mode]);

  const currentPageData = pages.find((p) => p.key === page);

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Box sx={{ display: "flex", minHeight: "100vh", bgcolor: "background.default" }}>
        {/* 侧边导航栏 */}
        <Drawer
          variant="permanent"
          sx={{
            width: drawerOpen ? DRAWER_WIDTH : DRAWER_COLLAPSED_WIDTH,
            flexShrink: 0,
            transition: "width 0.3s cubic-bezier(0.4, 0, 0.2, 1)",
            "& .MuiDrawer-paper": {
              width: drawerOpen ? DRAWER_WIDTH : DRAWER_COLLAPSED_WIDTH,
              transition: "width 0.3s cubic-bezier(0.4, 0, 0.2, 1)",
              overflowX: "hidden",
              bgcolor: "background.paper",
            },
          }}
        >
          {/* Logo 区域 */}
          <Box sx={{
            p: 2,
            display: "flex",
            alignItems: "center",
            justifyContent: drawerOpen ? "flex-start" : "center",
            minHeight: 64,
          }}>
            <Box sx={{
              width: 36,
              height: 36,
              borderRadius: "10px",
              bgcolor: "primary.main",
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              mr: drawerOpen ? 1.5 : 0,
              flexShrink: 0,
            }}>
              <Typography variant="h6" sx={{ color: "#fff", fontWeight: 700, fontSize: 18 }}>
                知
              </Typography>
            </Box>
            {drawerOpen && (
              <Fade in={drawerOpen} timeout={300}>
                <Typography variant="h6" noWrap sx={{ fontWeight: 600, fontSize: 18 }}>
                  知了 Cicada
                </Typography>
              </Fade>
            )}
          </Box>

          <Divider sx={{ opacity: 0.5 }} />

          {/* 导航菜单 */}
          <List sx={{ flex: 1, px: 1, py: 1.5 }}>
            {pages.map((p) => (
              <Tooltip
                key={p.key}
                title={!drawerOpen ? p.label : ""}
                placement="right"
                arrow
              >
                <ListItemButton
                  selected={page === p.key}
                  onClick={() => setPage(p.key)}
                  sx={{
                    borderRadius: "12px",
                    mb: 0.5,
                    minHeight: 48,
                    justifyContent: drawerOpen ? "initial" : "center",
                    px: drawerOpen ? 2 : 1.5,
                    transition: "all 0.2s ease",
                    position: "relative",
                    overflow: "hidden",
                    "&.Mui-selected": {
                      bgcolor: "primary.main",
                      color: "#fff",
                      "& .MuiListItemIcon-root": { color: "#fff" },
                      "&:hover": { bgcolor: "primary.dark" },
                    },
                    "&:hover": {
                      bgcolor: mode === "dark" ? "rgba(255,255,255,0.05)" : "rgba(0,0,0,0.04)",
                    },
                  }}
                >
                  <ListItemIcon
                    sx={{
                      minWidth: drawerOpen ? 40 : "auto",
                      mr: drawerOpen ? 1 : "auto",
                      justifyContent: "center",
                      color: page === p.key ? "inherit" : "text.secondary",
                      transition: "color 0.2s ease",
                    }}
                  >
                    {p.icon}
                  </ListItemIcon>
                  {drawerOpen && (
                    <Fade in={drawerOpen} timeout={300}>
                      <ListItemText
                        primary={p.label}
                        primaryTypographyProps={{
                          fontSize: 14,
                          fontWeight: page === p.key ? 600 : 400,
                        }}
                      />
                    </Fade>
                  )}
                </ListItemButton>
              </Tooltip>
            ))}
          </List>

          <Divider sx={{ opacity: 0.5 }} />

          {/* 底部操作区 */}
          <Box sx={{
            p: 1.5,
            display: "flex",
            alignItems: "center",
            justifyContent: drawerOpen ? "space-between" : "center",
            flexDirection: drawerOpen ? "row" : "column",
            gap: 1,
          }}>
            <Tooltip title={mode === "dark" ? "切换亮色模式" : "切换暗色模式"} arrow>
              <IconButton
                onClick={() => setMode(mode === "dark" ? "light" : "dark")}
                sx={{
                  bgcolor: mode === "dark" ? "rgba(255,255,255,0.05)" : "rgba(0,0,0,0.04)",
                  transition: "all 0.2s ease",
                  "&:hover": {
                    bgcolor: mode === "dark" ? "rgba(255,255,255,0.1)" : "rgba(0,0,0,0.08)",
                  },
                }}
              >
                {mode === "dark" ? <LightMode fontSize="small" /> : <DarkMode fontSize="small" />}
              </IconButton>
            </Tooltip>

            <Tooltip title={drawerOpen ? "收起侧边栏" : "展开侧边栏"} arrow>
              <IconButton
                onClick={() => setDrawerOpen(!drawerOpen)}
                size="small"
                sx={{
                  bgcolor: mode === "dark" ? "rgba(255,255,255,0.05)" : "rgba(0,0,0,0.04)",
                }}
              >
                {drawerOpen ? <ChevronLeft fontSize="small" /> : <ChevronRight fontSize="small" />}
              </IconButton>
            </Tooltip>
          </Box>
        </Drawer>

        {/* 主内容区 */}
        <Box
          component="main"
          sx={{
            flex: 1,
            p: { xs: 2, sm: 3, md: 4 },
            ml: 0,
            transition: "margin 0.3s ease",
            overflow: "auto",
          }}
        >
          {/* 页面标题 */}
          <Box sx={{ mb: 3, display: "flex", alignItems: "center", gap: 1.5 }}>
            {currentPageData?.icon}
            <Typography variant="h5" sx={{ fontWeight: 600 }}>
              {currentPageData?.label}
            </Typography>
          </Box>

          {/* 页面内容 */}
          <Slide direction="up" in={page === "display"} mountOnEnter unmountOnExit timeout={300}>
            <Box>
              {page === "display" && (
                <Box sx={{
                  textAlign: "center",
                  pt: 15,
                  display: "flex",
                  flexDirection: "column",
                  alignItems: "center",
                  gap: 2,
                }}>
                  <Box sx={{
                    width: 120,
                    height: 120,
                    borderRadius: "24px",
                    bgcolor: mode === "dark" ? "rgba(144,202,249,0.1)" : "rgba(25,118,210,0.08)",
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                  }}>
                    <DesktopWindows sx={{ fontSize: 56, color: "primary.main", opacity: 0.8 }} />
                  </Box>
                  <Typography variant="h6" color="text.secondary">
                    显示模式 — 浮窗已在屏幕上显示
                  </Typography>
                  <Typography variant="body2" color="text.disabled">
                    您可以在设置中调整浮窗的样式和位置
                  </Typography>
                </Box>
              )}
            </Box>
          </Slide>

          <Slide direction="up" in={page === "admin"} mountOnEnter unmountOnExit timeout={300}>
            <Box>
              {page === "admin" && (
                <Box sx={{ maxWidth: 800, mx: "auto" }}>
                  <Login onLogin={setLoggedIn} />
                  {loggedIn && <AnnouncementEditor />}
                </Box>
              )}
            </Box>
          </Slide>

          <Slide direction="up" in={page === "settings"} mountOnEnter unmountOnExit timeout={300}>
            <Box>
              {page === "settings" && (
                <Box sx={{ maxWidth: 720, mx: "auto" }}>
                  <SettingsPage />
                </Box>
              )}
            </Box>
          </Slide>
        </Box>
      </Box>
    </ThemeProvider>
  );
}

export default App;
