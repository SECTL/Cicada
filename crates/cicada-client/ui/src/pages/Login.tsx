import { useState } from "react";
import { invoke } from "../utils/tauri";
import {
  Box, Button, Card, CardContent, Typography, Alert,
  CircularProgress, Avatar, Chip, Stack, Fade, Paper, LinearProgress
} from "@mui/material";
import { Login as LoginIcon, Logout, Person, Shield, CheckCircle } from "@mui/icons-material";

interface UserInfo {
  user_id: string;
  user_name: string;
  email: string;
  permission: number;
}

interface Props {
  onLogin: (v: boolean) => void;
}

const Login: React.FC<Props> = ({ onLogin }) => {
  const [loading, setLoading] = useState(false);
  const [userInfo, setUserInfo] = useState<UserInfo | null>(null);
  const [error, setError] = useState("");
  const [progress, setProgress] = useState(0);

  const handleLogin = async () => {
    setLoading(true);
    setError("");
    setProgress(10);

    try {
      const result: any = await invoke("start_login");
      setProgress(30);
      window.open(result.url, "_blank");

      // 模拟进度条动画
      const progressInterval = setInterval(() => {
        setProgress((prev) => {
          if (prev >= 80) {
            clearInterval(progressInterval);
            return 80;
          }
          return prev + 5;
        });
      }, 1500);

      setTimeout(async () => {
        clearInterval(progressInterval);
        const code = prompt("请输入 SECTL 返回的授权码:");
        if (code) {
          try {
            setProgress(90);
            const info: UserInfo = await invoke("complete_login", {
              code,
              codeVerifier: result.code_verifier,
            });
            setProgress(100);
            setTimeout(() => {
              setUserInfo(info);
              onLogin(true);
              setLoading(false);
              setProgress(0);
            }, 300);
          } catch (e) {
            setError("登录失败: " + String(e));
            setLoading(false);
            setProgress(0);
          }
        } else {
          setLoading(false);
          setProgress(0);
        }
      }, 30000);
    } catch (e) {
      setError(String(e));
      setLoading(false);
      setProgress(0);
    }
  };

  const handleLogout = async () => {
    try {
      await invoke("logout");
      setUserInfo(null);
      onLogin(false);
    } catch (e) {
      setError(String(e));
    }
  };

  // 已登录状态展示
  if (userInfo) {
    return (
      <Fade in={true} timeout={500}>
        <Card sx={{ mb: 3, overflow: "visible" }}>
          <CardContent sx={{ p: 3 }}>
            <Stack direction="row" spacing={2.5} alignItems="center">
              <Avatar
                sx={{
                  width: 56,
                  height: 56,
                  bgcolor: "primary.main",
                  boxShadow: "0 4px 12px rgba(25,118,210,0.3)",
                }}
              >
                <Person sx={{ fontSize: 28 }} />
              </Avatar>
              <Box sx={{ flex: 1 }}>
                <Typography variant="h6" sx={{ fontWeight: 600, mb: 0.5 }}>
                  {userInfo.user_name}
                </Typography>
                <Typography variant="body2" color="text.secondary">
                  {userInfo.email}
                </Typography>
              </Box>
              <Stack direction="row" spacing={1} alignItems="center">
                <Chip
                  icon={<CheckCircle fontSize="small" />}
                  label="已登录"
                  color="success"
                  size="small"
                  sx={{ fontWeight: 500 }}
                />
                <Button
                  variant="outlined"
                  size="small"
                  color="error"
                  startIcon={<Logout />}
                  onClick={handleLogout}
                  sx={{ borderRadius: "8px" }}
                >
                  退出
                </Button>
              </Stack>
            </Stack>
          </CardContent>
        </Card>
      </Fade>
    );
  }

  return (
    <Fade in={true} timeout={500}>
      <Card sx={{ mb: 3, overflow: "visible" }}>
        <CardContent sx={{ p: 0 }}>
          {loading ? (
            <Box sx={{ p: 5, textAlign: "center" }}>
              <Box sx={{ mb: 3 }}>
                <CircularProgress size={48} thickness={3} />
              </Box>
              <Typography variant="h6" sx={{ mb: 1, fontWeight: 500 }}>
                等待浏览器授权...
              </Typography>
              <Typography variant="body2" color="text.secondary" sx={{ mb: 3 }}>
                请在浏览器中完成 SECTL 认证，然后输入授权码
              </Typography>
              <Box sx={{ width: "100%", maxWidth: 360, mx: "auto" }}>
                <LinearProgress
                  variant="determinate"
                  value={progress}
                  sx={{
                    height: 6,
                    borderRadius: 3,
                    bgcolor: "rgba(25,118,210,0.1)",
                    "& .MuiLinearProgress-bar": {
                      borderRadius: 3,
                      transition: "transform 0.5s ease",
                    },
                  }}
                />
                <Typography variant="caption" color="text.secondary" sx={{ mt: 1, display: "block" }}>
                  {progress < 100 ? `进度: ${progress}%` : "即将完成..."}
                </Typography>
              </Box>
            </Box>
          ) : (
            <Box sx={{ p: { xs: 3, sm: 5 }, textAlign: "center" }}>
              {/* 安全图标 */}
              <Paper
                elevation={0}
                sx={{
                  width: 80,
                  height: 80,
                  borderRadius: "20px",
                  bgcolor: "primary.main",
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                  mx: "auto",
                  mb: 3,
                  boxShadow: "0 8px 24px rgba(25,118,210,0.25)",
                }}
              >
                <Shield sx={{ fontSize: 40, color: "#fff" }} />
              </Paper>

              <Typography variant="h4" sx={{ mb: 1, fontWeight: 700 }}>
                登录知了
              </Typography>
              <Typography variant="body1" color="text.secondary" sx={{ mb: 4, maxWidth: 360, mx: "auto" }}>
                使用 SECTL 统一认证账号登录，以发布和管理校园公告
              </Typography>

              <Button
                variant="contained"
                size="large"
                startIcon={<LoginIcon />}
                onClick={handleLogin}
                sx={{
                  py: 1.5,
                  px: 5,
                  borderRadius: "12px",
                  fontSize: 16,
                  fontWeight: 600,
                  boxShadow: "0 4px 16px rgba(25,118,210,0.3)",
                  "&:hover": {
                    boxShadow: "0 6px 20px rgba(25,118,210,0.4)",
                  },
                }}
              >
                登录 SECTL
              </Button>

              <Typography variant="caption" color="text.disabled" sx={{ mt: 3, display: "block" }}>
                登录即表示您同意使用 SECTL OAuth 2.0 + PKCE 安全认证
              </Typography>
            </Box>
          )}

          {/* 错误提示 */}
          <Fade in={!!error} timeout={300}>
            <Box>
              {error && (
                <Alert
                  severity="error"
                  sx={{
                    mx: 3,
                    mb: 3,
                    borderRadius: "10px",
                  }}
                  onClose={() => setError("")}
                >
                  {error}
                </Alert>
              )}
            </Box>
          </Fade>
        </CardContent>
      </Card>
    </Fade>
  );
};

export default Login;
