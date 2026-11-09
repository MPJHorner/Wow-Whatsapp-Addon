# Security

Report vulnerabilities through [GitHub private vulnerability reporting](https://github.com/MPJHorner/Wow-Whatsapp-Addon/security/advisories/new).
Do not open a public issue containing tokens, QR codes, account data or message
history. Include the app version, operating system and a minimal reproduction
using fictional data. There is no promised response time or third-party audit.

The app embeds WhatsApp Web in the OS WebView. WhatsApp handles authentication
and messaging. The remote page has no native command bridge; the offline frame
accepts a fixed set of window-control messages. External navigation, popups and
downloads are blocked. The theme script does not read chat bodies or account
storage. See [the architecture and limits](docs/DEVELOPMENT.md).

Keep your OS and WebView2 runtime updated. An unlocked device, compromised web
engine or screen recording can expose chats regardless of transport encryption.
Hide the panel before streaming. Revoke its session using WhatsApp on your phone:
**Settings → Linked devices**.

Developer screenshot flags capture visible chat content to local files. They are
off by default. Never attach those captures without reviewing and redacting them.
Use the fictional demo for public screenshots.
