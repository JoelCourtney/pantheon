# Content Syntax

All object content is supplied in YAML files. If you aren't familiar with YAML, you're going to want to fix that before moving forward.

## Basic Types

Most attributes are Strings, Integers, or Enums. For example, the template files include things like:

```yaml
name: STRING
description: STRING

weight: INTEGER

rarity: ENUM{common,uncommon,rare,very rare,legendary}
```

Strings can be any text you want. You can do multiline strings, and that is often necessary for the description. All strings can include Markdown stuff, if you like; so if you put something dumb like an H1 tag in the `name` attribute, DnFree will try to render it. It just won't look good.

You will also be able include Markdown links to other content packs or rules in Strings. THIS IS NOT YET SUPPORTED THOUGH IM SORRY AAAA

Integers are exactly what you think they are.

Enums list each of the possible values for the attribute. If you don't put something that exactly matches one of the choices (case sensitive because I'm evil), then the tests will fail and you will be very sad.

## Identifiers

Identifiers are used as attributes for Effects, or inside Amounts. (Both have sections below.) They consist of an object, and a series of access parameters, with the following syntax: `object[accessor][another accessor][etc]`.

The root can only be one of `character`, `parent`, or `root`. `character` allows you to access information about the character, and `root` allows you to access information about the object in the file you are writing.

The possible accessors for the character and for root objects are in `IDENTIFIERS.md`. This merits its own file because there are a lot of them.

There can be any number of accessors, as long as there is at least one. For example `character[walking speed]` is a valid identifier, as is `character[right hand item][rarity]`.

In unusual cases, the accessor can be another identifier. This is nasty, but sometimes there's no other way. A common example is `character[parent[choice]]` used for ASI's. I plan to make an ASI shortcut though, so that example might go away.

## Amounts and Quantities

Sometimes integers aren't enough to specify numeric things. If you need to do something more complicated, a template will say something like:

```yaml
damage: DAMAGE
range: DISTANCE
casting time: TIME

some possible example that might exist in the future: AMOUNT
```

Amounts are mathematical expressions that can contain integers, dice, and identifiers. The operations can be addition `+`, subtraction `-`, multiplication `*`, division rounded down `/`, and division rounded up `/^`.

Dice are written as `AdB` where `A` is the number of dice to roll, and `B` is the number of sides on the dice. `B` doesn't strictly have to be one of {4, 6, 8, 10, 12, 20, 100}, but if it isn't, its probably wrong. Examples: `2d6`, `3d8`.

Identifiers can be dropped in the middle of an expression, such as `5 + root[weight]`.

Amounts have no units. But some quantities require units, such as Damage, Distance, and Time. A damage quantity for example, could be something like `2d6 thunder`. Damages can have units of:

- acid
- bludgeoning
- cold
- fire
- force
- lightning
- necrotic
- piercing
- poison
- psychic
- radiant
- slashing
- thunder
- melee
- ranged

Distances can have units of:

- ft (also foot, feet)
- mi (also mile, miles)
- sp (also space, spaces)

Times can have units of

- action (also actions)
- bonus action (also bonus actions, boneless action)
- reaction (also reactions)
- hr (also hour, hours, hrs)
- min (also minute, mins, minutes)
- sec (also secs, second, seconds)
- day (also days)
- long rest (also LR, long rests)
- short rest (also SR, short rests)

To make composite quantities with muliple units of the same kind, use semicolons. For example, an attack that does 1d8 piercing and 1d4 thunder would be written as `1d8 piercing; 1d4 thunder`. PLUS SIGNS ARE ONLY FOR AMOUNTS. Attempting to add together two quantities that already have units (even if they are the same unit) will only work with semicolons. So, the following are valid and equivalent:

```
2d6 piercing; 5 piercing
2d6 + 5 piercing
```

But this is not valid:

```
2d6 piercing + 5 piercing
```

## Effects

Effects are a totally different beast. See `EFFECTS.md` for details.