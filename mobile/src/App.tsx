import React, { useEffect, useMemo, useState } from "react";

type AnnouncementType = "normal" | "emergency" | "notice";
type Page = "login" | "publish" | "list";

type Announcement = {
  id: string;
  title: string;
  content_html: string;
  announcement_type: AnnouncementType;
  publisher_name: string;
  publisher_user_id: string;
  created_at: string;
  updated_at: string | null;
};

type AnnouncementListResponse = {
  announcements: Announcement[];
  total: number;
  page: number;
  per_page: number;
};

const STORAGE_KEYS = {
  apiBase: "cicada_api_base",
  token: "cicada_token",
  user: "cicada_user",
};

const DEFAULT_API_BASE = "http://localhost:3001/api/v1";

const announcementTypeOptions: Array<{ value: AnnouncementType | "all"; label: string }> = [
  { value: "all", label: "全部类型" },
  { value: "normal", label: "普通公告" },
  { value: "emergency", label: "紧急通知" },
  { value: "notice", label: "一般通知" },
];

function App() {
  const [apiBase, setApiBase] = useState(localStorage.getItem(STORAGE_KEYS.apiBase) || DEFAULT_API_BASE);
  const [draftApiBase, setDraftApiBase] = useState(apiBase);
  const [token, setToken] = useState(localStorage.getItem(STORAGE_KEYS.token) || "mock-token");
  const [userName, setUserName] = useState(localStorage.getItem(STORAGE_KEYS.user) || "");
  const [page, setPage] = useState<Page>(userName ? "publish" : "login");
  const [sessionMsg, setSessionMsg] = useState("");

  const [title, setTitle] = useState("");
  const [content, setContent] = useState("");
  const [announcementType, setAnnouncementType] = useState<AnnouncementType>("normal");
  const [publisher, setPublisher] = useState(userName);
  const [publishMsg, setPublishMsg] = useState("");

  const [announcements, setAnnouncements] = useState<Announcement[]>([]);
  const [selectedType, setSelectedType] = useState<AnnouncementType | "all">("all");
  const [loadingList, setLoadingList] = useState(false);
  const [listMsg, setListMsg] = useState("");
  const [busyId, setBusyId] = useState<string | null>(null);

  const normalizedApiBase = useMemo(() => apiBase.trim().replace(/\/$/, ""), [apiBase]);

  useEffect(() => {
    setPublisher(userName);
  }, [userName]);

  const persistSession = (nextUserName: string, nextToken: string) => {
    localStorage.setItem(STORAGE_KEYS.user, nextUserName);
    localStorage.setItem(STORAGE_KEYS.token, nextToken);
    setUserName(nextUserName);
    setToken(nextToken);
    setPage("publish");
  };

  const saveApiBase = () => {
    const nextApiBase = draftApiBase.trim().replace(/\/$/, "");
    if (!nextApiBase) {
      setSessionMsg("服务器地址不能为空");
      return;
    }
    localStorage.setItem(STORAGE_KEYS.apiBase, nextApiBase);
    setApiBase(nextApiBase);
    setSessionMsg("已保存服务器地址");
  };

  const handleLogin = () => {
    const trimmedName = userName.trim();
    if (!trimmedName) {
      setSessionMsg("请先填写发布者名称");
      return;
    }
    persistSession(trimmedName, token.trim() || "mock-token");
    setSessionMsg("已进入 Mock 发布模式");
  };

  const handleLogout = () => {
    localStorage.removeItem(STORAGE_KEYS.user);
    localStorage.removeItem(STORAGE_KEYS.token);
    setUserName("");
    setToken("mock-token");
    setPublisher("");
    setPage("login");
  };

  const fetchAnnouncements = async (type: AnnouncementType | "all" = selectedType) => {
    setLoadingList(true);
    setListMsg("");

    try {
      const url = new URL(`${normalizedApiBase}/announcements`);
      if (type !== "all") {
        url.searchParams.set("type", type);
      }

      const res = await fetch(url.toString());
      if (!res.ok) {
        setListMsg(`拉取公告失败: ${res.status}`);
        return;
      }

      const data: AnnouncementListResponse = await res.json();
      setAnnouncements(data.announcements);
      if (data.announcements.length === 0) {
        setListMsg("当前没有公告");
      }
    } catch (error) {
      setListMsg(`网络错误: ${String(error)}`);
    } finally {
      setLoadingList(false);
    }
  };

  useEffect(() => {
    if (page === "list") {
      void fetchAnnouncements(selectedType);
    }
  }, [page, selectedType, normalizedApiBase]);

  const handlePublish = async () => {
    if (!title.trim()) {
      setPublishMsg("标题不能为空");
      return;
    }

    if (!publisher.trim()) {
      setPublishMsg("发布者不能为空");
      return;
    }

    setPublishMsg("");

    try {
      const res = await fetch(`${normalizedApiBase}/announcements`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token.trim() || "mock-token"}`,
        },
        body: JSON.stringify({
          title: title.trim(),
          content_html: content.trim() || `<p>${escapeHtml(title.trim())}</p>`,
          announcement_type: announcementType,
          publisher_name: publisher.trim(),
        }),
      });

      if (!res.ok) {
        setPublishMsg(`发布失败: ${res.status}`);
        return;
      }

      setPublishMsg("发布成功");
      setTitle("");
      setContent("");
      setPage("list");
      await fetchAnnouncements(selectedType);
    } catch (error) {
      setPublishMsg(`网络错误: ${String(error)}`);
    }
  };

  const handleDelete = async (id: string) => {
    setBusyId(id);
    setListMsg("");

    try {
      const res = await fetch(`${normalizedApiBase}/announcements/${id}`, {
        method: "DELETE",
        headers: {
          Authorization: `Bearer ${token.trim() || "mock-token"}`,
        },
      });

      if (!res.ok) {
        setListMsg(`删除失败: ${res.status}`);
        return;
      }

      setAnnouncements((current) => current.filter((item) => item.id !== id));
      setListMsg("公告已删除");
    } catch (error) {
      setListMsg(`网络错误: ${String(error)}`);
    } finally {
      setBusyId(null);
    }
  };

  return (
    <div style={pageShellStyle}>
      <div style={cardStyle}>
        <header style={headerStyle}>
          <div>
            <h1 style={{ margin: 0, fontSize: "1.5rem" }}>知了发布</h1>
            <p style={{ margin: "0.4rem 0 0", color: "#667085", fontSize: "0.95rem" }}>
              面向手机的校园公告发布端
            </p>
          </div>
          {userName ? <span style={tagStyle}>{userName}</span> : null}
        </header>

        <section style={panelStyle}>
          <label style={labelStyle}>服务器地址</label>
          <div style={rowStyle}>
            <input
              type="text"
              value={draftApiBase}
              onChange={(event) => setDraftApiBase(event.target.value)}
              placeholder={DEFAULT_API_BASE}
              style={{ ...inputStyle, marginBottom: 0 }}
            />
            <button onClick={saveApiBase} style={secondaryButtonStyle}>
              保存
            </button>
          </div>
          <p style={hintStyle}>Mock 服务器默认地址：http://localhost:3001/api/v1</p>
          {sessionMsg ? <p style={statusStyle(!sessionMsg.includes("不能为空") && !sessionMsg.includes("请先"))}>{sessionMsg}</p> : null}
        </section>

        <div style={{ display: "flex", gap: "0.75rem", marginBottom: "1rem" }}>
          <button onClick={() => setPage("publish")} style={navButtonStyle(page === "publish")}>
            发布公告
          </button>
          <button onClick={() => setPage("list")} style={navButtonStyle(page === "list")}>
            公告列表
          </button>
        </div>

        {!userName || page === "login" ? (
          <section style={panelStyle}>
            <h2 style={sectionTitleStyle}>进入发布模式</h2>
            <input
              type="text"
              value={userName}
              onChange={(event) => setUserName(event.target.value)}
              placeholder="发布者名称 *"
              style={inputStyle}
            />
            <input
              type="text"
              value={token}
              onChange={(event) => setToken(event.target.value)}
              placeholder="访问令牌（Mock 模式可留空）"
              style={inputStyle}
            />
            <button onClick={handleLogin} style={primaryButtonStyle}>
              进入 Mock 发布模式
            </button>
            <p style={hintStyle}>当前 Mock 服务不校验令牌，所以先把发布和排查流程做通。</p>
          </section>
        ) : null}

        {userName && page === "publish" ? (
          <section style={panelStyle}>
            <div style={sectionHeaderStyle}>
              <h2 style={sectionTitleStyle}>发布公告</h2>
              <button onClick={handleLogout} style={ghostButtonStyle}>
                退出
              </button>
            </div>

            <input
              type="text"
              value={title}
              onChange={(event) => setTitle(event.target.value)}
              placeholder="公告标题 *"
              style={inputStyle}
            />
            <select
              value={announcementType}
              onChange={(event) => setAnnouncementType(event.target.value as AnnouncementType)}
              style={inputStyle}
            >
              <option value="normal">普通公告</option>
              <option value="emergency">紧急通知</option>
              <option value="notice">一般通知</option>
            </select>
            <input
              type="text"
              value={publisher}
              onChange={(event) => setPublisher(event.target.value)}
              placeholder="发布者 *"
              style={inputStyle}
            />
            <textarea
              value={content}
              onChange={(event) => setContent(event.target.value)}
              placeholder="公告内容（支持 HTML，为空时自动使用标题生成）"
              style={{ ...inputStyle, minHeight: "10rem", resize: "vertical" }}
            />
            <button onClick={handlePublish} style={primaryButtonStyle}>
              立即发布
            </button>
            {publishMsg ? <p style={statusStyle(publishMsg.includes("成功"))}>{publishMsg}</p> : null}
          </section>
        ) : null}

        {page === "list" ? (
          <section style={panelStyle}>
            <div style={sectionHeaderStyle}>
              <h2 style={sectionTitleStyle}>公告列表</h2>
              <button onClick={() => void fetchAnnouncements(selectedType)} style={secondaryButtonStyle}>
                刷新
              </button>
            </div>

            <select
              value={selectedType}
              onChange={(event) => setSelectedType(event.target.value as AnnouncementType | "all")}
              style={inputStyle}
            >
              {announcementTypeOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>

            {loadingList ? <p style={hintStyle}>正在加载公告...</p> : null}
            {listMsg ? <p style={statusStyle(!listMsg.includes("失败") && !listMsg.includes("错误"))}>{listMsg}</p> : null}

            <div style={{ display: "grid", gap: "0.75rem" }}>
              {announcements.map((announcement) => (
                <article key={announcement.id} style={announcementCardStyle(announcement.announcement_type)}>
                  <div style={sectionHeaderStyle}>
                    <div>
                      <h3 style={{ margin: 0, fontSize: "1rem" }}>{announcement.title}</h3>
                      <p style={{ margin: "0.35rem 0 0", color: "#475467", fontSize: "0.88rem" }}>
                        {labelForType(announcement.announcement_type)} · {announcement.publisher_name}
                      </p>
                    </div>
                    {userName ? (
                      <button
                        onClick={() => void handleDelete(announcement.id)}
                        disabled={busyId === announcement.id}
                        style={dangerButtonStyle}
                      >
                        {busyId === announcement.id ? "删除中" : "删除"}
                      </button>
                    ) : null}
                  </div>

                  <div
                    style={{ color: "#101828", lineHeight: 1.6, fontSize: "0.95rem" }}
                    dangerouslySetInnerHTML={{ __html: announcement.content_html }}
                  />

                  <p style={{ margin: "0.75rem 0 0", color: "#667085", fontSize: "0.8rem" }}>
                    发布时间：{formatTime(announcement.created_at)}
                  </p>
                </article>
              ))}
            </div>
          </section>
        ) : null}
      </div>
    </div>
  );
}

function escapeHtml(value: string) {
  return value
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");
}

function formatTime(value: string) {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    return value;
  }

  return new Intl.DateTimeFormat("zh-CN", {
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  }).format(date);
}

function labelForType(type: AnnouncementType) {
  switch (type) {
    case "normal":
      return "普通公告";
    case "emergency":
      return "紧急通知";
    case "notice":
      return "一般通知";
  }
}

const pageShellStyle: React.CSSProperties = {
  minHeight: "100vh",
  padding: "1rem",
  background: "linear-gradient(180deg, #eff6ff 0%, #f8fafc 100%)",
  fontFamily: "system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif",
  color: "#101828",
};

const cardStyle: React.CSSProperties = {
  maxWidth: "42rem",
  margin: "0 auto",
  background: "#ffffff",
  borderRadius: "1rem",
  boxShadow: "0 16px 40px rgba(15, 23, 42, 0.08)",
  padding: "1rem",
};

const headerStyle: React.CSSProperties = {
  display: "flex",
  justifyContent: "space-between",
  alignItems: "flex-start",
  gap: "1rem",
  marginBottom: "1rem",
};

const panelStyle: React.CSSProperties = {
  padding: "1rem",
  border: "1px solid #e4e7ec",
  borderRadius: "0.9rem",
  background: "#fcfcfd",
  marginBottom: "1rem",
};

const labelStyle: React.CSSProperties = {
  display: "block",
  marginBottom: "0.45rem",
  color: "#344054",
  fontWeight: 600,
  fontSize: "0.92rem",
};

const rowStyle: React.CSSProperties = {
  display: "flex",
  gap: "0.75rem",
  alignItems: "center",
};

const inputStyle: React.CSSProperties = {
  width: "100%",
  padding: "0.8rem 0.9rem",
  fontSize: "1rem",
  marginBottom: "0.8rem",
  border: "1px solid #d0d5dd",
  borderRadius: "0.75rem",
  boxSizing: "border-box",
  background: "#ffffff",
};

const primaryButtonStyle: React.CSSProperties = {
  width: "100%",
  padding: "0.9rem 1rem",
  background: "#2563eb",
  color: "#ffffff",
  border: "none",
  borderRadius: "0.8rem",
  fontSize: "1rem",
  fontWeight: 600,
  cursor: "pointer",
};

const secondaryButtonStyle: React.CSSProperties = {
  padding: "0.8rem 1rem",
  background: "#e0f2fe",
  color: "#075985",
  border: "none",
  borderRadius: "0.75rem",
  fontSize: "0.95rem",
  fontWeight: 600,
  cursor: "pointer",
  whiteSpace: "nowrap",
};

const ghostButtonStyle: React.CSSProperties = {
  padding: "0.55rem 0.8rem",
  background: "transparent",
  color: "#475467",
  border: "1px solid #d0d5dd",
  borderRadius: "0.7rem",
  fontSize: "0.92rem",
  cursor: "pointer",
};

const dangerButtonStyle: React.CSSProperties = {
  padding: "0.55rem 0.8rem",
  background: "#fee4e2",
  color: "#b42318",
  border: "none",
  borderRadius: "0.7rem",
  fontSize: "0.9rem",
  cursor: "pointer",
};

const tagStyle: React.CSSProperties = {
  padding: "0.35rem 0.65rem",
  borderRadius: "999px",
  background: "#dbeafe",
  color: "#1d4ed8",
  fontSize: "0.85rem",
  fontWeight: 700,
};

const sectionHeaderStyle: React.CSSProperties = {
  display: "flex",
  justifyContent: "space-between",
  alignItems: "center",
  gap: "0.75rem",
  marginBottom: "0.8rem",
};

const sectionTitleStyle: React.CSSProperties = {
  margin: 0,
  fontSize: "1.08rem",
};

const navButtonStyle = (active: boolean): React.CSSProperties => ({
  flex: 1,
  padding: "0.85rem",
  background: active ? "#2563eb" : "#eef2ff",
  color: active ? "#ffffff" : "#344054",
  border: "none",
  borderRadius: "0.8rem",
  fontSize: "0.96rem",
  fontWeight: 700,
  cursor: "pointer",
});

const hintStyle: React.CSSProperties = {
  margin: "0.55rem 0 0",
  color: "#667085",
  fontSize: "0.9rem",
};

const statusStyle = (success: boolean): React.CSSProperties => ({
  margin: "0.75rem 0 0",
  color: success ? "#027a48" : "#b42318",
  fontSize: "0.95rem",
  fontWeight: 600,
});

const announcementCardStyle = (type: AnnouncementType): React.CSSProperties => ({
  padding: "0.95rem",
  borderRadius: "0.9rem",
  border: `1px solid ${type === "emergency" ? "#fda29b" : type === "notice" ? "#84caff" : "#d0d5dd"}`,
  background: type === "emergency" ? "#fef3f2" : type === "notice" ? "#eff8ff" : "#ffffff",
});

export default App;
