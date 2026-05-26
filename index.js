// ABOUTME: Frontend logic for the song downloader.
// ABOUTME: Handles URL submission, clipboard paste, and song list rendering.

/**
 * @param {string} msg
 * @param {number} [ttl]
 */
function setStatus(msg, ttl = 0) {
  const el = /** @type {HTMLElement} */ (document.getElementById("status"));
  el.textContent = msg;
  if (ttl) setTimeout(() => { if (el.textContent === msg) el.textContent = ""; }, ttl);
}

/** @param {string} filename */
function songLabel(filename) {
  try {
    return decodeURIComponent(filename).replace(/\.mp3$/i, "");
  } catch {
    return filename.replace(/\.mp3$/i, "");
  }
}

async function loadSongs() {
  const response = await fetch("/api/songs");
  if (!response.ok) return;
  const { songs } = /** @type {{ songs: string[] }} */ (await response.json());
  const list = /** @type {HTMLElement} */ (document.getElementById("list"));
  list.innerHTML = songs
    .map((f) => `<li><a href="/songs/${encodeURIComponent(f)}" download>${songLabel(f)}</a></li>`)
    .join("");
}

async function downloadSong() {
  const input = /** @type {HTMLInputElement} */ (document.getElementById("urlInput"));
  const url = input.value.trim();
  if (!url) { setStatus("Enter a YouTube URL."); return; }

  setStatus("Downloading…");
  const response = await fetch("/api/download", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ url }),
  });
  const result = await response.json();
  setStatus(result.message, 4000);
  if (result.success) input.value = "";
}

async function pasteAndDownload() {
  if (!navigator.clipboard) {
    setStatus("Clipboard API unavailable — paste manually and hit Submit.");
    return;
  }
  try {
    const text = await navigator.clipboard.readText();
    const input = /** @type {HTMLInputElement} */ (document.getElementById("urlInput"));
    input.value = text.trim();
    await downloadSong();
  } catch (err) {
    setStatus(`Clipboard blocked (${err instanceof Error ? err.message : err}) — paste manually.`);
  }
}

window.addEventListener("load", loadSongs);
