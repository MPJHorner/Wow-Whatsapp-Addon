(() => {
  'use strict';
  // Wry currently injects into subframes on Windows too. Explicitly scope the
  // stylesheet to the official top-level page on both platforms.
  if (window.top !== window || location.origin !== 'https://web.whatsapp.com') return;
  const css = __WHISPER_CSS_JSON__;
  const id = 'wow-whisper-theme';
  // Load bundled bytes directly: no remote font requests or dependency on fonts
  // installed on this machine. This also works with the site's font-src policy.
  const font = new FontFace('WoW UI', Uint8Array.from(atob('__WHISPER_FONT_BASE64__'), c => c.charCodeAt(0)).buffer);
  font.load().then(loaded => document.fonts.add(loaded)).catch(() => {});
  function install() {
    if (!document.documentElement) return false;
    if (document.getElementById(id)) return true;
    const style = document.createElement('style');
    style.id = id;
    style.textContent = css;
    document.documentElement.appendChild(style);
    return true;
  }
  function classifyButton(button) {
    const width = button.offsetWidth;
    const height = button.offsetHeight;
    const iconOnly = width >= 18 && width <= 52 && height >= 18 && height <= 52 &&
      button.querySelector('svg, [data-icon]') && !button.querySelector('img') &&
      !button.closest('.message-in, .message-out, [data-pre-plain-text]');
    button.toggleAttribute('data-wow-control', !!iconOnly);
  }
  const buttonSizes = new ResizeObserver(entries => entries.forEach(entry => classifyButton(entry.target)));
  const seen = new WeakSet();
  function watchButton(button) {
    if (seen.has(button)) return;
    seen.add(button);
    buttonSizes.observe(button);
  }
  function decorate(root) {
    if (root.nodeType !== Node.ELEMENT_NODE) return;
    if (root.matches('button, [role="button"]')) watchButton(root);
    root.querySelectorAll('button, [role="button"]').forEach(watchButton);
  }
  if (!install()) {
    const observer = new MutationObserver(() => {
      if (install()) observer.disconnect();
    });
    observer.observe(document, { childList: true });
  }
  if (document.documentElement) decorate(document.documentElement);
  const controls = new MutationObserver(records => {
    for (const record of records) record.addedNodes.forEach(decorate);
  });
  controls.observe(document, { childList: true, subtree: true });
  // All presentation stays within this page. No messaging API, storage access,
  // network requests, message-body reads, or native bridge.
})();
