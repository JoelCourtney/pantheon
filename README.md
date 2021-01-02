# DnDCent

Its like DnDBeyond, but worse, and probably illegal.

## Building from source

You will need the newest versions of [gradle](https://gradle.org) and [yarn](https://yarnpkg.com/) installed. You will also need [python3](https://python.org) and [Java](https://www.java.com). If you don't have gradle, it might be possible to use the `./gradlew` script instead of the `gradle` command, but I make no guarantees and I haven't tested it.

In the root of the repo, run the following:

```bash
$ ./setup.py
$ gradle build
```

There is some non-generated documentation in `server/doc`, and you can also build JavaDoc-style documentation from the server-side code into the same directory using `gradle dokka`.

I'll make this README a little more useable and professional once the code itself is useable and professional ;)
