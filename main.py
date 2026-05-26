# ABOUTME: FastAPI backend for the song downloader.
# ABOUTME: Accepts YouTube URLs, downloads as MP3 via yt-dlp, and serves the files.

import asyncio
import os

from fastapi import FastAPI, HTTPException
from fastapi.responses import FileResponse
from fastapi.staticfiles import StaticFiles
from pydantic import BaseModel

app = FastAPI()

SONGS_DIR = "songs"
os.makedirs(SONGS_DIR, exist_ok=True)


class DownloadRequest(BaseModel):
    url: str


@app.get("/api/songs")
async def get_songs():
    songs = sorted(
        os.listdir(SONGS_DIR),
        key=lambda f: os.path.getmtime(os.path.join(SONGS_DIR, f)),
        reverse=True,
    )
    return {"songs": songs}


@app.post("/api/download", status_code=202)
async def start_download(req: DownloadRequest):
    asyncio.create_task(_download(req.url))
    return {"success": True, "message": "Download started in the background"}


async def _download(url: str):
    proc = await asyncio.create_subprocess_exec(
        "./yt-dlp", "-f", "bestaudio", "--extract-audio",
        "--audio-format", "mp3", "--output", f"{SONGS_DIR}/%(title)s.%(ext)s", url,
    )
    await proc.wait()
    print(f"{'downloaded' if proc.returncode == 0 else 'failed'}: {url}")


@app.get("/songs/{filename}")
async def serve_song(filename: str):
    if ".." in filename or "/" in filename:
        raise HTTPException(status_code=400, detail="Invalid filename")
    path = os.path.join(SONGS_DIR, filename)
    if not os.path.exists(path):
        raise HTTPException(status_code=404, detail="File not found")
    return FileResponse(path, media_type="audio/mpeg", filename=filename)


# Must be last — StaticFiles is a catch-all
app.mount("/", StaticFiles(directory=".", html=True), name="static")
