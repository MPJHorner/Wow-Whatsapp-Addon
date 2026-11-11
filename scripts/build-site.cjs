// Static GitHub Pages output. No server, credentials, or runtime build dependency.
const fs = require("node:fs");
const path = require("node:path");
const root = path.resolve(__dirname, "..");
const source = path.join(root, "site");
const output = path.join(root, "dist/site");
const origin = "https://mpjhorner.github.io/Wow-Whatsapp-Addon/";
const repository = "https://github.com/MPJHorner/Wow-Whatsapp-Addon";
const escape = (value) =>
  String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
const demos = JSON.parse(
  fs.readFileSync(path.join(source, "demos.json"), "utf8"),
);
const gallery = demos
  .map(
    (
      demo,
      index,
    ) => `<article class="demo-panel" id="demo-${escape(demo.id)}" role="tabpanel" aria-labelledby="tab-${escape(demo.id)}" ${index ? "hidden" : ""}>
  <div class="demo-story"><span class="eyebrow">${escape(demo.kicker)}</span><h3>${escape(demo.title)}</h3><p>${escape(demo.description)}</p><a class="text-link" href="assets/${escape(path.basename(demo.image))}" data-zoom="${escape(demo.id)}">Inspect the encounter <span aria-hidden="true">↗</span></a></div>
  <figure class="demo-figure"><a href="assets/${escape(path.basename(demo.image))}" data-zoom="${escape(demo.id)}" aria-label="Enlarge ${escape(demo.title)} demo"><img src="assets/${escape(path.basename(demo.image))}" width="1000" height="730" loading="lazy" decoding="async" alt="WoW Whisper ${escape(demo.title)}: fictional WhatsApp conversation with ${escape(demo.contact)} in the Warcraft-styled desktop panel"></a><figcaption>Fictional conversation · Production skin, sample chat layout</figcaption></figure>
  <details class="transcript"><summary>Read the conversation</summary><dl>${demo.messages.map((message) => `<div><dt>${message.direction === "out" ? "You" : escape(demo.contact)}</dt><dd>${escape(message.text)}</dd></div>`).join("")}</dl></details>
</article>`,
  )
  .join("\n");
const tabLabels = {
  "date-night": "Date night",
  "dinner-aggro": "Dinner aggro",
  "keyboard-cat": "Keyboard cat",
};
const tabs = demos
  .map(
    (demo, index) =>
      `<button type="button" role="tab" id="tab-${escape(demo.id)}" aria-controls="demo-${escape(demo.id)}" aria-selected="${!index}" tabindex="${index ? -1 : 0}"><span class="tab-number">0${index + 1}</span>${escape(tabLabels[demo.id] || demo.title)}</button>`,
  )
  .join("");
const schema = {
  "@context": "https://schema.org",
  "@graph": [
    {
      "@type": "WebSite",
      "@id": origin + "#website",
      url: origin,
      name: "WoW Whisper",
      alternateName: "WoW WhatsApp Addon",
    },
    {
      "@type": "SoftwareApplication",
      "@id": origin + "#app",
      name: "WoW Whisper",
      alternateName: "WoW WhatsApp Addon",
      url: origin,
      description:
        "A free Warcraft-styled desktop companion that brings official WhatsApp Web beside World of Warcraft on macOS and Windows. A separate desktop panel, with no game modification.",
      applicationCategory: "CommunicationApplication",
      operatingSystem: [
        "macOS 14 or later (Apple Silicon and Intel)",
        "Windows 10/11 (x64)",
      ],
      offers: {
        "@type": "Offer",
        price: "0",
        priceCurrency: "USD",
        url: origin + "#download",
      },
      downloadUrl: repository + "/releases/latest",
      screenshot: origin + "assets/demo-date-night.png",
      author: {
        "@type": "Person",
        name: "Matt Horner",
        url: "https://github.com/MPJHorner",
      },
      sameAs: repository,
    },
    {
      "@type": "WebPage",
      "@id": origin + "#page",
      url: origin,
      name: "WoW WhatsApp Addon for Mac & Windows | WoW Whisper",
      isPartOf: { "@id": origin + "#website" },
      about: { "@id": origin + "#app" },
    },
  ],
};
fs.mkdirSync(output, { recursive: true });
for (const name of ["styles.css", "app.js"])
  fs.copyFileSync(path.join(source, name), path.join(output, name));
fs.cpSync(path.join(source, "assets"), path.join(output, "assets"), {
  recursive: true,
});
fs.mkdirSync(path.join(output, "assets/wow"), { recursive: true });
for (const name of [
  "BlizzQuadrata.woff2",
  "UI-Background-Rock.PNG",
  "UI-Background-Marble.PNG",
  "UI-DialogBox-Border.PNG",
])
  fs.copyFileSync(
    path.join(root, "assets/wow", name),
    path.join(output, "assets/wow", name),
  );
for (const [token, file] of [
  ["button-up", "UI-Panel-Button-Up.PNG"],
  ["button-down", "UI-Panel-Button-Down.PNG"],
]) {
  const data = fs
    .readFileSync(path.join(root, "assets/wow", file))
    .toString("base64");
  fs.writeFileSync(
    path.join(output, "assets/wow", token + ".svg"),
    `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 80 22" width="80" height="22"><image width="128" height="32" href="data:image/png;base64,${data}"/></svg>`,
  );
}
let home = fs
  .readFileSync(path.join(source, "index.html"), "utf8")
  .replace("{{GALLERY}}", gallery)
  .replace("{{TABS}}", tabs)
  .replace("{{APP_SCHEMA}}", JSON.stringify(schema).replaceAll("<", "\\u003c"));
if (/\{\{[A-Z_]+\}\}/.test(home)) throw Error("Unresolved template token");
fs.writeFileSync(path.join(output, "index.html"), home);
fs.mkdirSync(path.join(output, "install"), { recursive: true });
fs.copyFileSync(
  path.join(source, "install/index.html"),
  path.join(output, "install/index.html"),
);
fs.copyFileSync(path.join(source, "404.html"), path.join(output, "404.html"));
fs.writeFileSync(path.join(output, ".nojekyll"), "");
fs.writeFileSync(
  path.join(output, "sitemap.xml"),
  `<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"><url><loc>${origin}</loc></url><url><loc>${origin}install/</loc></url></urlset>\n`,
);
console.log("Built static site:", output);
