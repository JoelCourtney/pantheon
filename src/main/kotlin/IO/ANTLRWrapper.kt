package IO

import DnFLexer
import DnFParser
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream
import model.quantities.*
import model.quantities.damage.Damage
import model.quantities.damage.DamageUnit
import model.quantities.distance.Distance
import model.quantities.distance.DistanceUnit
import model.quantities.time.Time
import model.quantities.time.TimeUnit

class ANTLRWrapper {
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

        fun parseDamageUnit(s: String): DamageUnit {
            return makeParser(s).damage_unit().result
        }

        fun parseDistanceUnit(s: String): DistanceUnit {
            return makeParser(s).distance_unit().result
        }

        fun parseTimeUnit(s: String): TimeUnit {
            return makeParser(s).time_unit().result
        }

        fun parseExpression(s: String): Expression {
            return makeParser(s).expression().result
        }
    }
}