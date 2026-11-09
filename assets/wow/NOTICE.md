# UI artwork

These are original Blizzard Entertainment World of Warcraft interface textures,
obtained from [Gethe’s UI texture mirror](https://github.com/Gethe/wow-ui-textures),
`live` branch, on 2026-11-09. They are not original artwork created for this project
and are not covered by the Rust source’s MIT license. No permission to redistribute
or relicense Blizzard assets is granted by this notice. The project is an
independent fan work with no Blizzard affiliation or endorsement.

The frame uses the dialog border atlas as eight CSS sprites, stone/marble
backgrounds, a dialog title plaque and the original button states. Button sprites
use an SVG viewport matching the game’s 80 × 22 region of the 128 × 32 atlas.
The title plaque uses a viewport around the visible part of its atlas.
The source PNG pixels are unchanged. CSS scales the dialog border to 48-pixel
atlas tiles and the rock texture to a 512-pixel repeat.

| File | Source directory | SHA-256 |
| --- | --- | --- |
| UI-Tooltip-Border.PNG | Tooltips | 7085e721db3dc8ecba96449c4b1a4f9afb3a732671504e7084694f36a9a29ce6 |
| UI-DialogBox-Background.PNG | DialogFrame | 9caced46c675e43fc465e3b58714b5119e775b33e5f3eb39d80fe2353ea81a47 |
| UI-DialogBox-Border.PNG | DialogFrame | d8500f9a50b78e73223ffa445eb6908e180b0b641d7ae0dd4f826db25c9df3af |
| UI-DialogBox-Header.PNG | DialogFrame | 0c259c2ce62e2923dfacec8105f373fe2fcc0dfcdaff0479ef8d1cc2d3a25644 |
| UI-Background-Marble.PNG | FrameGeneral | 2e7c499394bf6d14af68addf5a8961debfd6729e43bb27c9a6714c7b3e4f61ca |
| UI-Background-Rock.PNG | FrameGeneral | 226df00ae2fc4419c7c3cf2965324e7cc6f459bc77e3bdeb75b06b7c9c0ee212 |
| UI-Panel-Button-Up.PNG | Buttons | 0a8dc74976052b1f08550f271aaa1431bfa7a075b08c65becd43e221bc52d168 |
| UI-Panel-Button-Down.PNG | Buttons | 31ddfdc18f5f23132a08f7de0a4e8918b46d67008f0ee32d9fe411c58fa0e687 |
| UI-Panel-MinimizeButton-Up.PNG | Buttons | a8128c505fde277e167d8f8482e51f119ec23740867e147b76c32eb3feb8c509 |
| UI-Panel-MinimizeButton-Down.PNG | Buttons | fa4702d25bb025c9f47f3e72c5235a7491e98048e6bd493ac07a3be0b8419ae6 |

## UI and chat font

`BlizzQuadrata.woff2` is Blizzard’s Quadrata web font, downloaded from the
[official World of Warcraft asset host](https://assets-bwa.worldofwarcraft.blizzard.com/5bc893f6937a3ee903ac.woff2).
It is bundled as `WoW UI` for local rendering, rather than assuming Friz Quadrata
is installed and silently falling back to Georgia. It is not the game’s
`FRIZQT__.TTF` binary and is not covered by this project’s source-code license.
The game’s [font definitions](https://github.com/Gethe/wow-ui-source/blob/live/Interface/AddOns/Blizzard_Fonts_Shared/Shared/Fonts.xml)
were used to check the UI/chat font roles. For readability at this panel size,
messages and the composer use the bundled Quadrata font at 18px, with Arial as
their fallback, rather than the game's condensed chat-font treatment.
