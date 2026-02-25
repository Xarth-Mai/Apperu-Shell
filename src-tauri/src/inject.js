(() => {
  try {
    if (window.top !== window) {
      return;
    }
  } catch (_err) {
    return;
  }

  if (window.__apperuCleanup) {
    window.__apperuCleanup();
  }

  const invokeUpdate = async (payload) => {
    if (window.__TAURI__?.core?.invoke) {
      try {
        await window.__TAURI__.core.invoke("update_player_state", { state: payload });
      } catch (err) {
        console.warn("[apperu] failed to invoke update_player_state", err);
      }
    }
  };

  const emitPlayerState = async (payload) => {
    await invokeUpdate(payload);
  };

  let lastDrmReason = "";
  const emitDrmState = (payload) => {
    if (payload?.drm_supported) {
      lastDrmReason = "";
      return;
    }

    const reason = payload?.reason || "unknown reason";
    if (reason !== lastDrmReason) {
      lastDrmReason = reason;
      console.warn("[apperu] DRM unavailable:", reason);
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

  const HIDE_NATIVE_CTA_STYLE_ID = "apperu-hide-native-cta-style";
  let nativeCtaStyleEl = null;

  const hideNativeCta = () => {
    if (!nativeCtaStyleEl || !nativeCtaStyleEl.isConnected) {
      nativeCtaStyleEl = document.getElementById(HIDE_NATIVE_CTA_STYLE_ID);
      if (!nativeCtaStyleEl) {
        nativeCtaStyleEl = document.createElement("style");
        nativeCtaStyleEl.id = HIDE_NATIVE_CTA_STYLE_ID;
        nativeCtaStyleEl.textContent = `
          #navigation > .navigation__native-cta {
            display: none !important;
          }
        `;
        const target = document.head || document.documentElement;
        if (target) {
          target.appendChild(nativeCtaStyleEl);
        }
      }
    }

    for (const node of document.querySelectorAll("#navigation > .navigation__native-cta")) {
      node.style.setProperty("display", "none", "important");
    }
  };

  const probeDrm = async () => {
    try {
      if (!navigator.requestMediaKeySystemAccess) {
        emitDrmState({
          drm_supported: false,
          reason: "EME API unavailable in WebView (navigator.requestMediaKeySystemAccess is missing)"
        });
        return;
      }

      const keySystem = "com.widevine.alpha";
      const configs = [{
        initDataTypes: ["cenc"],
        audioCapabilities: [{ contentType: "audio/mp4; codecs=\"mp4a.40.2\"" }],
        videoCapabilities: [{ contentType: "video/mp4; codecs=\"avc1.42E01E\"" }]
      }];
      await navigator.requestMediaKeySystemAccess(keySystem, configs);
      emitDrmState({ drm_supported: true });
    } catch (err) {
      const reason = err instanceof Error ? err.message : String(err);
      emitDrmState({
        drm_supported: false,
        reason: `requestMediaKeySystemAccess rejected: ${reason}`
      });
    }
  };

  window.apperuPlayer = { getPlayerState, control };

  let lastKey = "";
  const tick = () => {
    const state = getPlayerState();
    const key = `${state.playing}|${state.title}|${state.artist}|${state.album}`;
    if (key !== lastKey) {
      lastKey = key;
      emitPlayerState(state);
    }
  };

  probeDrm();
  hideNativeCta();
  tick();

  const observer = new MutationObserver(() => {
    hideNativeCta();
    tick();
  });
  if (document.body) {
    observer.observe(document.body, { subtree: true, childList: true, characterData: true });
  }
  const intervalId = setInterval(tick, 800);

  window.__apperuCleanup = () => {
    observer.disconnect();
    clearInterval(intervalId);
    if (nativeCtaStyleEl?.isConnected) {
      nativeCtaStyleEl.remove();
    }
    delete window.__apperuCleanup;
  };
})();
