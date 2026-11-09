(() => {
  if (location.origin !== 'https://web.whatsapp.com') return { originAllowed: false };
  function describe(element) {
    const rect = element.getBoundingClientRect();
    const css = getComputedStyle(element);
    return {
      tag: element.tagName, classes: typeof element.className === 'string' ? element.className : '',
      attributes: element.getAttributeNames(), role: element.getAttribute('role'),
      box: [rect.x, rect.y, rect.width, rect.height].map(Math.round),
      style: Object.fromEntries(['fontFamily', 'fontSize', 'color', 'backgroundColor', 'display', 'alignItems', 'justifyContent', 'marginLeft', 'marginRight'].map(key => [key, css[key]])),
    };
  }
  const selectors = ['#app', '#side', '#pane-side', '#main', '[role="main"]', '[role="grid"]', '[data-pre-plain-text]', '.selectable-text', '.message-in', '.message-out', '#main [data-id]', '#main [data-pre-plain-text]', '#main .selectable-text', '[data-wow-whisper]', '[data-wow-control]'];
  const counts = Object.fromEntries(selectors.map(selector => [selector, document.querySelectorAll(selector).length]));
  const chains = [...document.querySelectorAll('[data-pre-plain-text], .selectable-text')].slice(0, 3).map(node => {
    const chain = [];
    for (let i = 0; node && i < 7; i++, node = node.parentElement) chain.push(describe(node));
    return chain;
  });
  const lightSurfaces = [...document.querySelectorAll('#app div')].slice(0, 2000).filter(node => {
    const rect = node.getBoundingClientRect();
    if (rect.width < 180 || rect.height < 40 || rect.height > 170) return false;
    const rgb = getComputedStyle(node).backgroundColor.match(/^rgb\((\d+), (\d+), (\d+)\)$/);
    return rgb && Math.min(+rgb[1], +rgb[2], +rgb[3]) > 160;
  }).slice(0, 8).map(node => ({ ...describe(node), icons: [...node.querySelectorAll('[data-icon]')].map(icon => icon.getAttribute('data-icon')).slice(0, 10) }));
  return {
    ready: document.querySelectorAll('#app button, #app [role="button"]').length > 5 || !!document.querySelector('#app canvas'),
    originAllowed: true, counts, viewport: [innerWidth, innerHeight],
    fontLoaded: [...document.fonts].some(font => font.family.replaceAll('"', '') === 'WoW UI' && font.status === 'loaded'),
    panels: [...document.querySelectorAll('#side, #main, #main header, #main footer')].map(describe), chains, lightSurfaces,
  };
  // No message text, names, attribute values, image URLs, IDs or account storage.
})();
