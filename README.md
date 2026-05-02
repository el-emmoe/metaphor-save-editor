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
  
         --mag <MAG>                      amount of mag to set
         --money <MONEY>                  amount of money to set
         -c, --courage <COURAGE>          amount of courage to set
         -w, --wisdom <WISDOM>            amount of wisdom to set
         -t, --tolerance <TOLERANCE>      amount of tolerance to set
         -e, --eloquence <ELOQUENCE>      amount of eloquence to set
         -i, --imagination <IMAGINATION>  amount of imagination to set
  
  party  Edit party stats like HP, MP, level
  help   Print this message or the help of the given subcommand(s)

Options:
  -s, --show         Show all stats (default)
  -h, --help         Print help
```
## Notes 
Can only use on general stats (MAG, Money, Virtues) currently.

