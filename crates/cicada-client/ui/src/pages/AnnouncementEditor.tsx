import { useState, useCallback } from "react";
import { invoke } from "../utils/tauri";
import {
  Card, CardContent, CardHeader, TextField, MenuItem, Button,
  Typography, Alert, Stack, Box, Chip, Fade, Tabs, Tab, Paper,
  IconButton, Tooltip, Divider
} from "@mui/material";
import {
  Send, Preview, Code, FormatBold, FormatItalic,
  FormatUnderlined, Title, Warning, Info, Campaign
} from "@mui/icons-material";

interface Announcement {
  id: string;
  title: string;
  created_at: string;
}

const TYPES = [
  { value: "normal", label: "普通公告", color: "primary", icon: <Campaign fontSize="small" /> },
  { value: "emergency", label: "紧急通知", color: "error", icon: <Warning fontSize="small" /> },
  { value: "notice", label: "一般通知", color: "info", icon: <Info fontSize="small" /> },
];

const QUICK_TAGS = ["考试安排", "活动通知", "课程调整", "放假通知", "重要提醒", "其他"];

const AnnouncementEditor: React.FC = () => {
  const [title, setTitle] = useState("");
  const [contentHtml, setContentHtml] = useState("");
  const [annType, setAnnType] = useState("normal");
  const [publisher, setPublisher] = useState("");
  const [msg, setMsg] = useState<{ type: "success" | "error"; text: string } | null>(null);
  const [previewTab, setPreviewTab] = useState(0);
  const [isPublishing, setIsPublishing] = useState(false);

  const insertTag = useCallback((tag: string) => {
    setTitle((prev) => (prev ? `[${tag}] ${prev}` : `[${tag}] `));
  }, []);

  const insertHtml = useCallback((html: string) => {
    setContentHtml((prev) => prev + html);
  }, []);

  const publish = async () => {
    if (!title.trim()) {
      setMsg({ type: "error", text: "标题不能为空" });
      return;
    }
    setIsPublishing(true);
    try {
      const announcement = await invoke<Announcement>("publish_announcement", {
        title: title.trim(),
        contentHtml,
        announcementType: annType,
        publisherName: publisher.trim() || "匿名",
      });
      setMsg({ type: "success", text: `公告发布成功：${announcement.title}` });
      setTitle("");
      setContentHtml("");
      setAnnType("normal");
    } catch (e) {
      setMsg({ type: "error", text: String(e) });
    } finally {
      setIsPublishing(false);
    }
  };

  const selectedType = TYPES.find((t) => t.value === annType);

  return (
    <Fade in={true} timeout={600}>
      <Card>
        <CardHeader
          title={
            <Stack direction="row" alignItems="center" spacing={1.5}>
              <Box
                sx={{
                  width: 36,
                  height: 36,
                  borderRadius: "10px",
                  bgcolor: `${selectedType?.color}.main`,
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                }}
              >
                {selectedType?.icon}
              </Box>
              <Typography variant="h6" sx={{ fontWeight: 600 }}>
                发布公告
              </Typography>
            </Stack>
          }
          sx={{ pb: 1 }}
        />

        <CardContent>
          <Stack spacing={3}>
            {/* 公告标题 */}
            <Box>
              <TextField
                label="公告标题"
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                fullWidth
                variant="outlined"
                placeholder="请输入公告标题..."
                InputProps={{
                  startAdornment: (
                    <Title sx={{ mr: 1, color: "text.secondary", fontSize: 20 }} />
                  ),
                }}
                sx={{
                  "& .MuiOutlinedInput-root": {
                    borderRadius: "12px",
                  },
                }}
              />
              {/* 快捷标签 */}
              <Stack direction="row" spacing={1} sx={{ mt: 1.5, flexWrap: "wrap", gap: 1 }}>
                <Typography variant="caption" color="text.secondary" sx={{ mr: 0.5, lineHeight: "24px" }}>
                  快捷标签:
                </Typography>
                {QUICK_TAGS.map((tag) => (
                  <Chip
                    key={tag}
                    label={tag}
                    size="small"
                    onClick={() => insertTag(tag)}
                    sx={{
                      borderRadius: "6px",
                      cursor: "pointer",
                      fontSize: 12,
                      "&:hover": {
                        bgcolor: "primary.main",
                        color: "#fff",
                      },
                    }}
                  />
                ))}
              </Stack>
            </Box>

            {/* 类型和发布者 */}
            <Stack direction={{ xs: "column", sm: "row" }} spacing={2}>
              <TextField
                select
                label="公告类型"
                value={annType}
                onChange={(e) => setAnnType(e.target.value)}
                sx={{ width: { xs: "100%", sm: 180 } }}
                SelectProps={{
                  renderValue: (value) => {
                    const type = TYPES.find((t) => t.value === value);
                    return (
                      <Stack direction="row" alignItems="center" spacing={1}>
                        {type?.icon}
                        <span>{type?.label}</span>
                      </Stack>
                    );
                  },
                }}
              >
                {TYPES.map((t) => (
                  <MenuItem key={t.value} value={t.value}>
                    <Stack direction="row" alignItems="center" spacing={1}>
                      {t.icon}
                      <span>{t.label}</span>
                    </Stack>
                  </MenuItem>
                ))}
              </TextField>
              <TextField
                label="发布者"
                value={publisher}
                onChange={(e) => setPublisher(e.target.value)}
                placeholder="留空使用登录名"
                sx={{ flex: 1 }}
              />
            </Stack>

            {/* HTML 编辑器工具栏 */}
            <Box>
              <Paper
                variant="outlined"
                sx={{
                  p: 1,
                  mb: 1,
                  borderRadius: "12px 12px 0 0",
                  borderBottom: "none",
                  display: "flex",
                  gap: 0.5,
                  flexWrap: "wrap",
                  alignItems: "center",
                }}
              >
                <Typography variant="caption" color="text.secondary" sx={{ mr: 1, ml: 0.5 }}>
                  快捷插入:
                </Typography>
                <Tooltip title="加粗">
                  <IconButton size="small" onClick={() => insertHtml("<b></b>")}>
                    <FormatBold fontSize="small" />
                  </IconButton>
                </Tooltip>
                <Tooltip title="斜体">
                  <IconButton size="small" onClick={() => insertHtml("<i></i>")}>
                    <FormatItalic fontSize="small" />
                  </IconButton>
                </Tooltip>
                <Tooltip title="下划线">
                  <IconButton size="small" onClick={() => insertHtml("<u></u>")}>
                    <FormatUnderlined fontSize="small" />
                  </IconButton>
                </Tooltip>
                <Divider orientation="vertical" flexItem sx={{ mx: 0.5 }} />
                <Tooltip title="段落">
                  <IconButton size="small" onClick={() => insertHtml("<p></p>")}>
                    <Title fontSize="small" />
                  </IconButton>
                </Tooltip>
                <Tooltip title="换行">
                  <IconButton size="small" onClick={() => insertHtml("<br />")}>
                    <Code fontSize="small" />
                  </IconButton>
                </Tooltip>
              </Paper>

              <TextField
                label="公告内容 (HTML)"
                value={contentHtml}
                onChange={(e) => setContentHtml(e.target.value)}
                multiline
                rows={10}
                fullWidth
                variant="outlined"
                placeholder="在此输入 HTML 格式的公告内容..."
                sx={{
                  "& .MuiOutlinedInput-root": {
                    borderRadius: "0 0 12px 12px",
                    fontFamily: '"SF Mono", "Cascadia Code", "Consolas", monospace',
                    fontSize: 13,
                  },
                }}
              />
            </Box>

            {/* 预览区域 */}
            {contentHtml && (
              <Fade in={true} timeout={400}>
                <Box>
                  <Tabs
                    value={previewTab}
                    onChange={(_, v) => setPreviewTab(v)}
                    sx={{ mb: 1 }}
                  >
                    <Tab
                      icon={<Preview fontSize="small" />}
                      iconPosition="start"
                      label="实时预览"
                    />
                    <Tab
                      icon={<Code fontSize="small" />}
                      iconPosition="start"
                      label="源代码"
                    />
                  </Tabs>

                  {previewTab === 0 ? (
                    <Paper
                      variant="outlined"
                      sx={{
                        p: 3,
                        borderRadius: "12px",
                        bgcolor: "action.hover",
                        minHeight: 100,
                        maxHeight: 300,
                        overflow: "auto",
                      }}
                    >
                      <div dangerouslySetInnerHTML={{ __html: contentHtml }} />
                    </Paper>
                  ) : (
                    <Paper
                      variant="outlined"
                      sx={{
                        p: 2,
                        borderRadius: "12px",
                        bgcolor: "background.default",
                        minHeight: 100,
                        maxHeight: 300,
                        overflow: "auto",
                      }}
                    >
                      <Typography
                        component="pre"
                        variant="body2"
                        sx={{
                          fontFamily: '"SF Mono", "Cascadia Code", "Consolas", monospace',
                          fontSize: 12,
                          whiteSpace: "pre-wrap",
                          wordBreak: "break-all",
                          m: 0,
                        }}
                      >
                        {contentHtml}
                      </Typography>
                    </Paper>
                  )}
                </Box>
              </Fade>
            )}

            {/* 发布按钮 */}
            <Button
              variant="contained"
              size="large"
              startIcon={<Send />}
              onClick={publish}
              disabled={isPublishing || !title.trim()}
              sx={{
                py: 1.5,
                borderRadius: "12px",
                fontSize: 16,
                fontWeight: 600,
                boxShadow: "0 4px 16px rgba(25,118,210,0.3)",
                "&:hover": {
                  boxShadow: "0 6px 20px rgba(25,118,210,0.4)",
                },
              }}
            >
              {isPublishing ? "发布中..." : "发布公告"}
            </Button>

            {/* 消息提示 */}
            <Fade in={!!msg} timeout={300}>
              <Box>
                {msg && (
                  <Alert
                    severity={msg.type}
                    onClose={() => setMsg(null)}
                    sx={{ borderRadius: "10px" }}
                  >
                    {msg.text}
                  </Alert>
                )}
              </Box>
            </Fade>
          </Stack>
        </CardContent>
      </Card>
    </Fade>
  );
};

export default AnnouncementEditor;
