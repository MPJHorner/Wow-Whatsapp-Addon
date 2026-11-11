"use strict";
const toggle = document.querySelector(".menu-toggle");
const nav = document.querySelector("#main-nav");
if (toggle && nav) {
  const closeMenu = () => {
    toggle.setAttribute("aria-expanded", "false");
    nav.classList.remove("is-open");
  };
  toggle.addEventListener("click", () => {
    const open = toggle.getAttribute("aria-expanded") !== "true";
    toggle.setAttribute("aria-expanded", String(open));
    nav.classList.toggle("is-open", open);
  });
  nav
    .querySelectorAll("a")
    .forEach((link) => link.addEventListener("click", closeMenu));
  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && nav.classList.contains("is-open")) {
      closeMenu();
      toggle.focus();
    }
  });
}
const tabs = [...document.querySelectorAll('[role="tab"]')];
function selectTab(tab) {
  for (const item of tabs) {
    const selected = item === tab;
    item.setAttribute("aria-selected", String(selected));
    item.tabIndex = selected ? 0 : -1;
    document.getElementById(item.getAttribute("aria-controls")).hidden =
      !selected;
  }
}
tabs.forEach((tab, index) => {
  tab.addEventListener("click", () => selectTab(tab));
  tab.addEventListener("keydown", (event) => {
    let next;
    if (event.key === "ArrowRight") next = (index + 1) % tabs.length;
    if (event.key === "ArrowLeft")
      next = (index + tabs.length - 1) % tabs.length;
    if (event.key === "Home") next = 0;
    if (event.key === "End") next = tabs.length - 1;
    if (next !== undefined) {
      event.preventDefault();
      selectTab(tabs[next]);
      tabs[next].focus();
    }
  });
});
const dialog = document.querySelector("#screenshot-dialog");
if (dialog && typeof dialog.showModal === "function") {
  let opener;
  document.querySelectorAll("[data-zoom]").forEach((link) =>
    link.addEventListener("click", (event) => {
      event.preventDefault();
      opener = link;
      const panel = document.getElementById("demo-" + link.dataset.zoom);
      const title =
        panel?.querySelector("h3")?.textContent || "Fictional chat demo";
      const source =
        panel?.querySelector(".demo-figure img") || link.querySelector("img");
      const image = dialog.querySelector("img");
      image.src = link.href;
      image.alt = source?.alt || title;
      dialog.querySelector("h2").textContent = title + " · fictional demo";
      dialog.showModal();
      document.body.classList.add("dialog-open");
    }),
  );
  dialog
    .querySelector(".dialog-close")
    .addEventListener("click", () => dialog.close());
  dialog.addEventListener("click", (event) => {
    if (event.target === dialog) {
      const r = dialog.getBoundingClientRect();
      if (
        event.clientX < r.left ||
        event.clientX > r.right ||
        event.clientY < r.top ||
        event.clientY > r.bottom
      )
        dialog.close();
    }
  });
  dialog.addEventListener("close", () => {
    document.body.classList.remove("dialog-open");
    opener?.focus();
  });
}
// Public release metadata only. No cookies, visitor counters, or analytics events.
const releaseStatus = document.getElementById("release-status");
if (releaseStatus) {
  fetch(
    "https://api.github.com/repos/MPJHorner/Wow-Whatsapp-Addon/releases/latest",
    {
      credentials: "omit",
      referrerPolicy: "no-referrer",
      signal:
        typeof AbortSignal.timeout === "function"
          ? AbortSignal.timeout(7000)
          : undefined,
    },
  )
    .then((response) => {
      if (!response.ok) throw Error("Release metadata unavailable");
      return response.json();
    })
    .then((release) => {
      if (
        typeof release.tag_name !== "string" ||
        !Array.isArray(release.assets)
      )
        return;
      const link = document.createElement("a");
      link.href =
        "https://github.com/MPJHorner/Wow-Whatsapp-Addon/releases/latest";
      link.textContent = release.tag_name + " · Release notes ↗";
      const count = release.assets
        .filter((asset) => /^WoW-Whisper-.*\.zip$/.test(asset.name))
        .reduce(
          (total, asset) =>
            total +
            (Number.isSafeInteger(asset.download_count)
              ? asset.download_count
              : 0),
          0,
        );
      const note = document.createElement("span");
      note.textContent =
        " · " + count.toLocaleString() + " package downloads this release";
      note.title =
        "File downloads, not unique users. Includes repeated and verification downloads.";
      releaseStatus.replaceChildren(link, note);
    })
    .catch(() => {
      /* The normal release link remains usable offline or when rate-limited. */
    });
}
