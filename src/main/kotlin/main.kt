import model.*
import org.antlr.runtime.ANTLRInputStream
import org.antlr.v4.runtime.*
import quantities.*
import quantities.Unit
import java.lang.Exception

fun main() {
    println(ParseWrapper.parseTime("infinite"))
    println(ParseWrapper.parseDamage(
        "(8d6 + (spell_cast_level - spell_level) * 1d6) /- 2 fire"
    ))
}

