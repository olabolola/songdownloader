/**
 * Get songs from endpoint
 * @returns {Promise<{songs: string[]}>}
 */
async function getSongs() {
  const url = "/api/songs";
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`Response status: ${response.status}`);
  }

  const result = await response.json();
  console.log(result);
  return result;
}

/**
 * download a song async from a youtube URL
 */
async function downloadSong() {
  const input = /** @type {HTMLInputElement} */ (
    document.getElementById("urlInput")
  );
  const url = input.value;

  if (!url) {
    alert("enter a youtube url");
    return;
  }

  const response = await fetch("/api/download", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ url: url }),
  });
  const result = await response.json();
  alert(result.message);

  if (result.success) {
    input.value = "";
  }
}
window.onload = async () => {
  const result = await getSongs();
  const songs = result.songs; // Extract the songs array
  const list = document.getElementById("list");
  list.innerHTML = songs
    .map((i) => `<li><a href="/songs/${i}" download>${i}</a></li>`)
    .join("");
};
