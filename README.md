# Metaphor: ReFantazio save editor (WIP)
## Description
Very basic save editor for Metaphor: ReFantazio. Tested with PS4 save files but the format seems to be the same for all consoles.

Includes editing of MAG, Money, Virtues and party member stats.
## Usage
Use your save file as an argument to the program.

- PS4: `DATA.DAT`
- Steam: `saveXXXX.sav`

```
metaphor-save-edit [OPTIONS] --file <FILE> [COMMAND]

Commands:
  stats  Edit general stats like money, mag or virtues
  party  Edit party stats like HP, MP, level
  help   Print this message or the help of the given subcommand(s)

Options:
  -s, --show         Show all stats
  -h, --help         Print help
  -V, --version      Print version
```
## Notes 
Can only use on general stats (MAG, Money, Virtues) currently.

