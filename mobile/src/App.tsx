import React, { useState } from "react";

const API_BASE = "http://localhost:3001/api/v1";

function App() {
  const [token, setToken] = useState(localStorage.getItem("cicada_token") || "");
  const [userName, setUserName] = useState(localStorage.getItem("cicada_user") || "");
  const [page, setPage] = useState<"login" | "publish" | "list">("login");
  const [title, setTitle] = useState("");
  const [content, setContent] = useState("");
  const [announcementType, setAnnType] = useState("normal");
  const [publisher, setPublisher] = useState(userName);
  const [msg, setMsg] = useState("");

  const handleLogin = async () => {
    const authUrl = `https://appwrite.sectl.cn/oauth/authorize?client_id=${prompt("请输入 Client ID:") || ""}&redirect_uri=${window.location.origin}/callback&response_type=code&code_challenge=test&code_challenge_method=S256&state=test`;
    window.location.href = authUrl;
  };

  const handlePublish = async () => {
    if (!title.trim()) {
      setMsg("标题不能为空");
      return;
    }
    try {
      const res = await fetch(`${API_BASE}/announcements`, {
        method: "POST",
        headers: { "Content-Type": "application/json", "Authorization": `Bearer ${token}` },
        body: JSON.stringify({
          title: title.trim(),
          content_html: content || `<p>${title}</p>`,
          announcement_type: announcementType,
          publisher_name: publisher || "匿名",
        }),
      });
      if (res.ok) {
        setMsg("发布成功!");
        setTitle("");
        setContent("");
      } else {
        setMsg("发布失败: " + res.status);
      }
    } catch (e) {
      setMsg("网络错误: " + String(e));
    }
  };

  return (
    <div style={{ maxWidth: "500px", margin: "0 auto", padding: "1rem", fontFamily: "sans-serif" }}>
      <header style={{ textAlign: "center", padding: "1rem 0", borderBottom: "1px solid #eee", marginBottom: "1rem" }}>
        <h1 style={{ margin: 0, fontSize: "1.5rem" }}>知了发布</h1>
        {userName && <p style={{ color: "#666", fontSize: "0.9rem" }}>{userName}</p>}
      </header>

      <div style={{ display: "flex", gap: "8px", marginBottom: "1rem" }}>
        <button onClick={() => setPage("publish")} style={btnStyle(page === "publish")}>发布公告</button>
        <button onClick={() => setPage("list")} style={btnStyle(page === "list")}>公告列表</button>
      </div>

      {page === "login" && (
        <div style={{ textAlign: "center", padding: "2rem" }}>
          <button onClick={handleLogin} style={{ padding: "12px 32px", fontSize: "18px", background: "#1890ff", color: "white", border: "none", borderRadius: "8px" }}>
            登录 SECTL
          </button>
        </div>
      )}

      {page === "publish" && (
        <div>
          <input type="text" value={title} onChange={(e) => setTitle(e.target.value)} placeholder="公告标题 *" style={inputStyle} />
          <select value={announcementType} onChange={(e) => setAnnType(e.target.value)} style={inputStyle}>
            <option value="normal">普通公告</option>
            <option value="emergency">紧急通知</option>
            <option value="notice">一般通知</option>
          </select>
          <input type="text" value={publisher} onChange={(e) => setPublisher(e.target.value)} placeholder="发布者" style={inputStyle} />
          <textarea value={content} onChange={(e) => setContent(e.target.value)} placeholder="公告内容 (支持 HTML)" style={{ ...inputStyle, minHeight: "150px" }} />
          <button onClick={handlePublish} style={{ width: "100%", padding: "12px", background: "#1890ff", color: "white", border: "none", borderRadius: "8px", fontSize: "16px", marginTop: "0.5rem" }}>
            发布
          </button>
          {msg && <p style={{ color: msg.includes("成功") ? "green" : "red", marginTop: "0.5rem" }}>{msg}</p>}
        </div>
      )}

      {page === "list" && (
        <div>
          <p style={{ color: "#999" }}>连接到 Mock 服务器后显示公告列表</p>
        </div>
      )}
    </div>
  );
}

const inputStyle: React.CSSProperties = {
  width: "100%",
  padding: "10px",
  fontSize: "16px",
  marginBottom: "0.8rem",
  border: "1px solid #ddd",
  borderRadius: "6px",
  boxSizing: "border-box",
};

const btnStyle = (active: boolean): React.CSSProperties => ({
  flex: 1,
  padding: "8px",
  background: active ? "#1890ff" : "#f0f0f0",
  color: active ? "white" : "#333",
  border: "none",
  borderRadius: "6px",
  fontSize: "14px",
  cursor: "pointer",
});

export default App;
