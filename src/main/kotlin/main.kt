import model.*
import org.antlr.runtime.ANTLRInputStream
import org.antlr.v4.runtime.*
import quantities.*
import quantities.Unit
import java.lang.Exception

fun main() {
    println(ParseWrapper.parseDamage("1d345 -2 fire + 3 cold"))
}

