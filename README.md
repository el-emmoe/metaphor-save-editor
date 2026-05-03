# Metaphor: ReFantazio save editor
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
  stats         Edit general stats like money, mag or virtues
    
              --mag <MAG>                  amount of mag to set
              --money <MONEY>              amount of money to set
          -c, --courage <COURAGE>          amount of courage to set (max 240)
          -w, --wisdom <WISDOM>            amount of wisdom to set (max 280)
          -t, --tolerance <TOLERANCE>      amount of tolerance to set (max 210)
          -e, --eloquence <ELOQUENCE>      amount of eloquence to set (max 170)
          -i, --imagination <IMAGINATION>  amount of imagination to set (max 280)
    
  party         Edit party stats like HP, MP, level

          -c, --character <CHARACTER>      party member starting from 1
                                           (Will is 1, Strohl 2 ...)
  
              --hp <HP>                    amount of HP to set
              --mp <MP>                    amount of MP to set
              --lvl <LVL>                  level to set
              --exp <EXP>                  amount of exp to set
          -s, --strength <STRENGTH>        amount of strength to set
          -m, --magic <MAGIC>              amount of magic to set
          -e, --endurance <ENDURANCE>      amount of endurance to set
          -a, --agility <AGILITY>          amount of agility to set
          -l, --luck <LUCK>                amount of luck to set
  
  help          Print this message or the help of the given subcommand(s)

Options:
  -s, --show    Show all stats (default)
  -h, --help    Print help
```
## Example
If you wanted to change your current mag and money amount to 99999 and courage to max you would write the command like

```
metaphor-save-edit -f DATA.DAT stats --mag 99999 --money 99999 --courage 240
```

If you wanted to change the HP and MP of Will to 999 and also max out his magic stat you would write the command like

```
metaphor-save-edit -f DATA.DAT party --character 1 --hp 999 --mp 999 --magic 99
```
## Notes
- Not 100% tested with all different values you can set. May or may not break
something so always keep the auto-generated backup save file somewhere safe.
- Need to run command multiple times for editing different stuff currently (cannot run stats and party commands at the same time)
- I haven't completed the game myself yet lol 

