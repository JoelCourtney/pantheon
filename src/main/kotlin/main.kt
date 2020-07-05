import model.*
import org.antlr.runtime.ANTLRInputStream
import org.antlr.v4.runtime.*
import quantities.*
import quantities.Unit
import java.lang.Exception

fun main() {
    println(parseTime("1 + 1d4 sec "))
}

fun parseTime(s: String): Quantity<Time> {
    val input = CharStreams.fromString(s)
    val lexer = DnFreeLexer(input)
    val tokens = CommonTokenStream(lexer)
    val parser = DnFreeParser(tokens)
    return parser.time_quantity().result
}