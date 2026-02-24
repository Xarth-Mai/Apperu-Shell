(() => {
  const emit = (payload) => {
    if (window.__TAURI__?.event?.emit) {
      window.__TAURI__.event.emit("player_state", payload);
    }
  };

  const readFromMediaSession = () => {
    const metadata = navigator.mediaSession?.metadata;
    if (!metadata) return null;
    const artwork = metadata.artwork?.[0]?.src;
    return {
      playing: navigator.mediaSession.playbackState === "playing",
      title: metadata.title || "",
      artist: metadata.artist || "",
      album: metadata.album || "",
      artwork_url: artwork || null
    };
  };

  const readFromDomFallback = () => {
    const title = document.querySelector("[data-testid='now-playing-title'], .web-chrome-playback-lcd__song-name")?.textContent?.trim() || "";
    const artist = document.querySelector("[data-testid='now-playing-artist'], .web-chrome-playback-lcd__sub-copy")?.textContent?.trim() || "";
    const playButton = document.querySelector("button[aria-label*='Pause'], button[aria-label*='Play']");
    const isPlaying = !!playButton?.getAttribute("aria-label")?.toLowerCase().includes("pause");
    return {
      playing: isPlaying,
      title,
      artist,
      album: "",
      artwork_url: null
    };
  };

  const getPlayerState = () => readFromMediaSession() ?? readFromDomFallback();

  const control = (action) => {
    const map = {
      playpause: "button[aria-label*='Play'], button[aria-label*='Pause']",
      next: "button[aria-label*='Next']",
      prev: "button[aria-label*='Back'], button[aria-label*='Previous']"
    };
    const btn = document.querySelector(map[action]);
    if (btn) btn.click();
  };

  const probeDrm = async () => {
    try {
      if (!navigator.requestMediaKeySystemAccess) {
        emit({ drm_supported: false, reason: "EME API unavailable in WebView" });
        return;
      }
      emit({ drm_supported: true });
    } catch (err) {
      emit({ drm_supported: false, reason: String(err) });
    }
  };

  window.apperuPlayer = { getPlayerState, control };

  let lastKey = "";
  const tick = () => {
    const state = getPlayerState();
    const key = `${state.playing}|${state.title}|${state.artist}|${state.album}`;
    if (key !== lastKey) {
      lastKey = key;
      emit(state);
    }
  };

  probeDrm();
  tick();
  const observer = new MutationObserver(() => tick());
  observer.observe(document.body, { subtree: true, childList: true, characterData: true });
  setInterval(tick, 800);
})();
