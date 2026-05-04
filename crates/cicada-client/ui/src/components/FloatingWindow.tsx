import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

interface Announcement {
  id: string;
  title: string;
  content_html: string;
  announcement_type: "normal" | "emergency" | "notice";
  publisher_name: string;
  created_at: string;
}

const FloatingWindow: React.FC = () => {
  const [announcement, setAnnouncement] = useState<Announcement | null>(null);
  const [contextMenu, setContextMenu] = useState<{ x: number; y: number } | null>(null);
  const [queue, setQueue] = useState<Announcement[]>([]);
  const [queueIndex, setQueueIndex] = useState(0);

  useEffect(() => {
    const unlisten = listen<{ announcement: Announcement }>("announcement", (event) => {
      const ann = event.payload.announcement;
      if (ann.announcement_type === "emergency") {
        setAnnouncement(ann);
        setQueue((prev) => [ann, ...prev]);
      } else {
        setQueue((prev) => {
          const updated = [...prev, ann];
          if (!announcement) {
            setAnnouncement(ann);
          }
          return updated;
        });
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const handleContextMenu = (e: React.MouseEvent) => {
    e.preventDefault();
    setContextMenu({ x: e.clientX, y: e.clientY });
  };

  const closeContextMenu = () => setContextMenu(null);

  const nextAnnouncement = () => {
    const nextIdx = queueIndex + 1;
    if (nextIdx < queue.length) {
      setQueueIndex(nextIdx);
      setAnnouncement(queue[nextIdx]);
    }
  };

  const prevAnnouncement = () => {
    const prevIdx = queueIndex - 1;
    if (prevIdx >= 0) {
      setQueueIndex(prevIdx);
      setAnnouncement(queue[prevIdx]);
    }
  };

  if (!announcement) {
    return (
      <div className="floating-window empty" data-tauri-drag-region>
        <p className="empty-text">暂无公告</p>
      </div>
    );
  }

  const typeClass = `type-${announcement.announcement_type}`;

  return (
    <div
      className={`floating-window ${typeClass}`}
      onContextMenu={handleContextMenu}
      onClick={closeContextMenu}
    >
      <div className="announcement-header" data-tauri-drag-region>
        {announcement.announcement_type === "emergency" && (
          <span className="emergency-badge">⚠️ 紧急通知</span>
        )}
        <h2 className="announcement-title">{announcement.title}</h2>
      </div>
      <div
        className="announcement-body"
        dangerouslySetInnerHTML={{ __html: announcement.content_html }}
      />
      <div className="announcement-footer">
        <span className="publisher">{announcement.publisher_name}</span>
        <span className="time">
          {new Date(announcement.created_at).toLocaleString()}
        </span>
      </div>

      {queue.length > 1 && (
        <div className="queue-nav">
          <button onClick={prevAnnouncement} disabled={queueIndex === 0}>
            ← 上一条
          </button>
          <span>{queueIndex + 1} / {queue.length}</span>
          <button onClick={nextAnnouncement} disabled={queueIndex >= queue.length - 1}>
            下一条 →
          </button>
        </div>
      )}

      {contextMenu && (
        <div
          className="context-menu"
          style={{ left: contextMenu.x, top: contextMenu.y }}
          onClick={closeContextMenu}
        >
          <div className="menu-item">置顶 / 取消置顶</div>
          <div className="menu-item">隐藏</div>
          <div className="menu-item">设置...</div>
        </div>
      )}
    </div>
  );
};

export default FloatingWindow;
