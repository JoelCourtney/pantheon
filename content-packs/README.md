# Content Packs

All content, including Official, Playtest, and Homebrew content, will be inside this directory. Homebrew content is currently unsupported, but when it is added, this README will be updated with instructions on how to use Homebrew content.

## Official and Playtest content

Official DnD book content goes in the `content-packs/official/<BOOKNAME>` directories. WotC Playtest content (like Unearthed-Arcana) goes in the `content-packs/playtest/<BOOKNAME>` directories.

The organization for each book's content is identical and is discussed below.

## Book organization

Each book directory contains an identical list of content type directories, which are:

- Armor
- Classes
- Feats
- Items
- Languages
- Races
- Spells
- Weapons

DnD does not usually differentiate between Weapons, Armor, and other Items, but this tool does. The distinction is needed because the information supplied for Weapons, Armor, and other Items are different, and I don't want to make the item deserialization system robust enough to deal with that :).

Each content type directory contains only `.yaml` files, each with a specific list of attributes; some attributes like `name` are always required, and some are not. The list of each type's required and optional attributes can be found in `content-packs/.templates/TYPE.yaml`.

### Content Syntax

This is a large enough topic to merit its own tutorial file, `CONTENT-SYNTAX.md`.

### Adding new content

To add more official or playtest content (or to add homebrew stuff once it is supported), there are some criteria:

- The name of the file must exactly match the content of the `name` attribute; and if it is official/playtest content the name must exactly match the name in the DnD book, including spaces, capitalization, and punctuation. (with the '.yaml' extension of course.)
- Any new book directories must be named exactly the same as the book, including spaces, capitalization, and punctuation.
- Organization must follow the *Book Organization* section above.
- The file must parse correctly when running `gradle test`.
- For official and playtest material, the content must behave as accurately as possible given the limitations of this software.
- For official and playtest material, `description` attribute strings must be nicely formatted in Markdown.
- Put your GitHub name at the top of the file in a comment of the form `# File by <YOUR NAME>`