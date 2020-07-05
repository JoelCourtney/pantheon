import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream
import quantities.*

class ParseWrapper {
    companion object {
        private fun makeParser(s: String): DnFreeParser {
            val input = CharStreams.fromString(s)
            val lexer = DnFreeLexer(input)
            val tokens = CommonTokenStream(lexer)
            return DnFreeParser(tokens)
        }

        fun parseTime(s: String): Quantity<Time> {
            return makeParser(s).time_quantity().result
        }

        fun parseDistance(s: String): Quantity<Distance> {
            return makeParser(s).distance_quantity().result
        }

        fun parseDamage(s: String): Quantity<Damage> {
            return makeParser(s).damage_quantity().result
        }
    }
}