import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream
import model.quantities.*

class ParseWrapper {
    companion object {
        private fun makeParser(s: String): DnFParser {
            val input = CharStreams.fromString(s)
            val lexer = DnFLexer(input)
            val tokens = CommonTokenStream(lexer)
            return DnFParser(tokens)
        }

        fun parseTime(s: String): Time {
            return makeParser(s).time_quantity().result
        }

        fun parseDistance(s: String): Distance {
            return makeParser(s).distance_quantity().result
        }

        fun parseDamage(s: String): Damage {
            return makeParser(s).damage_quantity().result
        }
    }
}