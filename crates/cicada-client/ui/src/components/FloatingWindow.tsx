import { useEffect, useState, useCallback } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "../utils/tauri";

interface Announcement {
  id: string;
  title: string;
  content_html: string;
  announcement_type: "normal" | "emergency" | "notice";
  publisher_name: string;
  created_at: string;
}

interface DisplayConfig {
  font_size: number;
  font_color: string;
  font_family: string;
  bg_color: string;
  bg_opacity: number;
}

const FloatingWindow: React.FC = () => {
  const [announcement, setAnnouncement] = useState<Announcement | null>(null);
  const [contextMenu, setContextMenu] = useState<{ x: number; y: number } | null>(null);
  const [queue, setQueue] = useState<Announcement[]>([]);
  const [queueIndex, setQueueIndex] = useState(0);
  const [display, setDisplay] = useState<DisplayConfig>({
    font_size: 24,
    font_color: "#ffffff",
    font_family: "sans-serif",
    bg_color: "#000000",
    bg_opacity: 0.8,
  });

  const loadDisplayConfig = useCallback(async () => {
    try {
      const cfg = await invoke<{ display: DisplayConfig }>("get_config");
      if (cfg?.display) {
        setDisplay(cfg.display);
      }
    } catch {
      // use defaults
    }
  }, []);

  useEffect(() => {
    loadDisplayConfig();
  }, [loadDisplayConfig]);

  useEffect(() => {
    const unlistenConfig = listen("config-updated", () => {
      loadDisplayConfig();
    });

    const unlistenAnnouncement = listen<{ announcement: Announcement }>(
      "announcement",
      (event) => {
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
      }
    );

    return () => {
      unlistenConfig.then((fn) => fn());
      unlistenAnnouncement.then((fn) => fn());
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

  const inlineStyle: React.CSSProperties = {
    fontFamily: display.font_family,
    color: display.font_color,
    backgroundColor: display.bg_color,
    opacity: display.bg_opacity,
  };

  if (!announcement) {
    return (
      <div className="floating-window empty" style={inlineStyle} data-tauri-drag-region>
        <p className="empty-text" style={{ fontSize: `${display.font_size}px` }}>
          暂无公告
        </p>
      </div>
    );
  }

  const typeClass = `type-${announcement.announcement_type}`;

  return (
    <div
      className={`floating-window ${typeClass}`}
      style={inlineStyle}
      onContextMenu={handleContextMenu}
      onClick={closeContextMenu}
    >
      <div className="announcement-header" data-tauri-drag-region>
        {announcement.announcement_type === "emergency" && (
          <span className="emergency-badge">紧急通知</span>
        )}
        <h2 className="announcement-title" style={{ fontSize: `${display.font_size}px` }}>
          {announcement.title}
        </h2>
      </div>
      <div
        className="announcement-body"
        style={{ fontSize: `${Math.max(12, display.font_size - 4)}px` }}
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
            上一条
          </button>
          <span>
            {queueIndex + 1} / {queue.length}
          </span>
          <button onClick={nextAnnouncement} disabled={queueIndex >= queue.length - 1}>
            下一条
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
