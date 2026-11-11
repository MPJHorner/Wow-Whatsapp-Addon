// Local design preview. All names, avatars and messages are fictional sample data.
// Uses the production frame and theme; never connects to a WhatsApp account.
const fs = require('node:fs');
const path = require('node:path');
const root = path.resolve(__dirname, '..');
const args = process.argv.slice(2);
if (args.length && (args.length !== 2 || args[0] !== '--scenario')) {
  throw new Error('Usage: node scripts/preview.cjs [--scenario <id>]');
}
const standalone = args.length > 0;
const demos = JSON.parse(fs.readFileSync(path.join(root, 'site/demos.json'), 'utf8'));
const scenario = demos.find(demo => demo.id === (args[1] || 'date-night'));
if (!scenario) throw new Error(`Unknown scenario. Choose: ${demos.map(demo => demo.id).join(', ')}`);
function textures(source) {
  source = source.replaceAll('{{UI_FONT}}', 'data:font/woff2;base64,' + fs.readFileSync(path.join(root, 'assets/wow/BlizzQuadrata.woff2')).toString('base64'));
  for (const [token, file] of Object.entries({
    TOOLTIP_BORDER: 'UI-Tooltip-Border.PNG',
    DIALOG_BORDER: 'UI-DialogBox-Border.PNG',
    DIALOG_HEADER: 'UI-DialogBox-Header.PNG',
    MARBLE: 'UI-Background-Marble.PNG',
    ROCK: 'UI-Background-Rock.PNG',
    BUTTON_UP: 'UI-Panel-Button-Up.PNG',
    BUTTON_DOWN: 'UI-Panel-Button-Down.PNG',
    DIALOG_BACKGROUND: 'UI-DialogBox-Background.PNG',
    CLOSE_UP: 'UI-Panel-MinimizeButton-Up.PNG',
    CLOSE_DOWN: 'UI-Panel-MinimizeButton-Down.PNG',
  })) {
    let data = 'data:image/png;base64,' + fs.readFileSync(path.join(root, 'assets/wow', file)).toString('base64');
    if (token === 'BUTTON_UP' || token === 'BUTTON_DOWN') {
      data = 'data:image/svg+xml;base64,' + Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 80 22" width="80" height="22"><image width="128" height="32" href="${data}"/></svg>`).toString('base64');
    } else if (token === 'DIALOG_HEADER') {
      data = 'data:image/svg+xml;base64,' + Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" viewBox="120 6 272 72" width="272" height="72"><image width="512" height="128" href="${data}"/></svg>`).toString('base64');
    }
    source = source.replaceAll('{{' + token + '}}', data);
  }
  return source;
}
function escapeAttribute(text) {
  return text.replaceAll('&', '&amp;').replaceAll('"', '&quot;').replaceAll('<', '&lt;');
}
function escapeText(text) {
  return text.replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;');
}
const icons = {
  more: '<circle cx="12" cy="5" r="1"/><circle cx="12" cy="12" r="1"/><circle cx="12" cy="19" r="1"/>',
  search: '<circle cx="10" cy="10" r="6"/><path d="m15 15 5 5"/>',
  chat: '<path d="M20 11a8 8 0 0 1-8 8H4l1-5a8 8 0 1 1 15-3Z"/>',
  emoji: '<circle cx="12" cy="12" r="8"/><path d="M8 14q4 5 8 0M8 9h1m6 0h1"/>',
  plus: '<path d="M12 5v14M5 12h14"/>',
  mic: '<rect x="9" y="3" width="6" height="12" rx="3"/><path d="M6 11v1a6 6 0 0 0 12 0v-1M12 18v3M9 21h6"/>',
  phone: '<path d="M7 3H3v4a14 14 0 0 0 14 14h4v-5l-5-2-2 3a12 12 0 0 1-7-7l3-2Z"/>',
};
function icon(name) { return '<svg viewBox="0 0 24 24" aria-hidden="true">' + icons[name] + '</svg>'; }
function button(name, label) { return `<button data-wow-control aria-label="${label}">${icon(name)}</button>`; }
function avatar(index, name) { return `<span class="avatar portrait-${index}" role="img" aria-label="Fictional portrait: ${name}"></span>`; }
const contacts = [
  ['Anya', 'A better loot table. ❤️', '20:08'],
  ['Katya', 'You owe me dessert.', '19:54'],
  ['Natasha', 'Is “one more pull” a unit of time?', '18:42'],
  ['Irina', 'Coffee > cooldowns.', '17:16'],
];
const messages = scenario.messages;
if (standalone) {
  const lastMessage = messages.at(-1);
  contacts[scenario.portrait] = [scenario.contact, lastMessage.text, lastMessage.time];
}
const sprite = 'data:image/png;base64,' + fs.readFileSync(path.join(root, 'docs/assets/demo-portraits.png')).toString('base64');
const theme = textures(fs.readFileSync(path.join(root, 'assets/whatsapp-theme.css'), 'utf8'));
const chat = textures(`<!doctype html><html lang="en"><head><meta charset="utf-8"><style>
@font-face{font-family:'WoW UI';src:url('{{UI_FONT}}') format('woff2');font-weight:400}
*{box-sizing:border-box}html,body{margin:0;height:100%;overflow:hidden;font:15px 'WoW UI',serif}button{cursor:default}svg{width:20px;height:20px;fill:none;stroke:currentColor;stroke-width:1.5;stroke-linecap:round;stroke-linejoin:round;vertical-align:middle}#app{display:flex;height:100%}.rail{width:48px;flex-shrink:0;background:#292720 url('{{ROCK}}') repeat;display:flex;flex-direction:column;align-items:center;gap:22px;padding:20px 0;color:#b7a779}.rail .active{color:#ffd100}.rail .bottom{margin-top:auto}.sidebar{width:275px;flex-shrink:0;display:flex;flex-direction:column}#side{flex:1;display:flex;flex-direction:column}header{height:64px;flex-shrink:0;display:flex;align-items:center;gap:9px;padding:10px 16px;border-bottom:1px solid #655b42}header h1{font-size:18px;margin:0;flex:1}button{padding:0;width:34px;height:34px;display:grid;place-items:center;flex-shrink:0}.avatar{width:43px;height:43px;flex-shrink:0;display:block;border-radius:50%;background-image:url('${sprite}');background-size:200% 200%;border:1px solid #85734b}.portrait-0{background-position:0 0}.portrait-1{background-position:100% 0}.portrait-2{background-position:0 100%}.portrait-3{background-position:100% 100%}.search{margin:13px 12px 9px;padding:9px 11px;display:flex;gap:9px;align-items:center;background:#292619;color:#aaa18c;border:1px solid #514b39;font:12px Arial,sans-serif}.search svg{width:15px;height:15px}.filters{display:flex;gap:6px;margin:0 12px 14px;color:#b1a58b;font:11px Arial,sans-serif}.filters span{padding:5px 9px;border:1px solid #494333;border-radius:2px}.filters .selected{color:#ffd100;background:#534223}#pane-side [role=row]{display:flex;align-items:center;gap:10px;min-height:86px;padding:15px 12px}.contact{flex:1;min-width:0}.contact p{margin:6px 0 0;color:#b6aa8e;font:12px/1.4 Arial,sans-serif;white-space:nowrap;overflow:hidden;text-overflow:ellipsis}.time{font:10px Arial,sans-serif;color:#a5997e;align-self:flex-start;margin-top:2px}.sidebar-note{margin-top:auto;padding:18px 14px;color:#8c816e;font:11px Arial,sans-serif;line-height:1.5;border-top:1px solid #393429}#main{flex:1;min-width:0;display:flex;flex-direction:column}#main header .name{flex:1}.name small{display:block;color:#aaa18c;font:11px Arial,sans-serif;margin-top:4px}.conversation{padding:18px 22px;display:flex;flex:1;flex-direction:column;gap:12px;overflow:auto}.day{align-self:center;font:11px Arial,sans-serif;color:#a3926c;letter-spacing:1.3px;margin-bottom:4px}.message-in,.message-out{max-width:87%;padding:6px 0;color:#efc3ea;line-height:1.5}.message-in{align-self:flex-start}.message-out{align-self:flex-end}.stamp{font:10px Arial,sans-serif;color:#a99d84;text-align:right;margin-top:3px}.ticks{color:#bdb08d;margin-left:4px}footer{height:64px;display:flex;gap:8px;align-items:center;padding:10px 14px;border-top:1px solid #655b42}.compose{flex:1;color:#a8997c;padding:7px 8px;font:15px 'WoW UI',serif;background:#0d0c0a;border:1px solid #494333}
</style><style>${theme}</style></head><body><div id="app">
<nav class="rail" aria-label="Sample navigation"><span class="active">${icon('chat')}</span>${icon('phone')}${icon('more')}<span class="bottom">${icon('emoji')}</span></nav>
<div class="sidebar"><header><h1>Chats</h1>${button('plus','New chat')}${button('more','Menu')}</header><div id="side"><div class="search">${icon('search')}<span>Search or start a new chat</span></div><div class="filters"><span class="selected">All</span><span>Unread</span><span>Favourites</span></div><div id="pane-side" role="grid">
${contacts.map(([name, snippet, time], index) => `<div role="row"${index === scenario.portrait ? ' aria-selected="true"' : ''}>${avatar(index, name)}<div class="contact"><span title="${escapeAttribute(name)}">${escapeText(name)}</span><p>${escapeText(snippet)}</p></div><span class="time">${escapeText(time)}</span></div>`).join('')}
</div><div class="sidebar-note">Ctrl+Shift+W · Show or hide</div></div></div>
<section id="main"><header>${avatar(scenario.portrait,scenario.contact)}<div class="name"><span title="${escapeAttribute(scenario.contact)}">${escapeText(scenario.contact)}</span><small>online</small></div>${button('phone','Call')}${button('search','Search conversation')}${button('more','Conversation menu')}</header>
<div class="conversation"><div class="day">TODAY</div>${messages.map(({direction,text,time})=>`<div class="message-${direction}"><span class="selectable-text">${escapeText(text)}</span><div class="stamp">${escapeText(time)}${direction==='out'?'<span class="ticks">✓✓</span>':''}</div></div>`).join('')}</div>
<footer>${button('plus','Attach')}${button('emoji','Emoji')}<div class="compose">Type a message</div>${button('mic','Voice message')}</footer></section>
</div></body></html>`);
let frame = textures(fs.readFileSync(path.join(root, 'assets/frame.html'), 'utf8')).replaceAll('{{HOTKEY}}', 'Ctrl+Shift+W');
frame = frame.replace("default-src 'none';", "default-src 'none'; frame-src 'self' about:;");
frame = frame.replace('<div class="well" aria-hidden="true"></div>', `<iframe title="Fictional demo conversation" sandbox="" class="well" style="width:calc(100% - 40px);height:calc(100% - 84px);border:0" srcdoc="${escapeAttribute(chat)}"></iframe>`);
const preview = textures(`<!doctype html><html lang="en"><head><meta charset="utf-8"><title>WoW Whisper · fictional demo</title><meta name="description" content="A fictional preview of the open-source WoW WhatsApp desktop companion, with Warcraft styling and a raid-night conversation."><style>@font-face{font-family:'WoW UI';src:url('{{UI_FONT}}') format('woff2');font-weight:400}*{box-sizing:border-box}html,body{margin:0;min-height:100%;background:#0d0e10}body{color:#d4c5a7;font:12px Arial,sans-serif}#preview{width:1120px;padding:24px 60px;background:radial-gradient(ellipse at top,#312a1d 0, #141414 55%,#0d0e10 100%)}.masthead{display:flex;align-items:baseline;justify-content:space-between;margin:0 0 18px}.masthead strong{font:21px 'WoW UI',serif;color:#e6c16c}.masthead span{color:#b2a58c;font-size:12px}iframe{display:block;width:1000px;height:730px;border:0;box-shadow:0 15px 32px #0008}p{margin:18px 0 0;text-align:center;color:#9a8e79;font-size:11px;line-height:1.5}</style></head><body><main id="preview"><div class="masthead"><strong>WoW Whisper</strong><span>One more pull. One less missed message.</span></div><iframe title="WoW Whisper production frame with fictional content" srcdoc="${escapeAttribute(frame)}"></iframe><p>Fictional demo · Sample chat layout · Generated adult portraits · No real messages</p></main></body></html>`);
fs.mkdirSync(path.join(root, 'dist'), {recursive:true});
const output = path.join(root, standalone ? `dist/demo-${scenario.id}.html` : 'dist/preview.html');
fs.writeFileSync(output, standalone ? frame : preview);
console.log(output);
