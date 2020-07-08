package io

import DnFLexer
import DnFParser
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream
import model.quantities.amounts.Amount
import model.quantities.damage.Damage
import model.quantities.damage.DamageUnit
import model.quantities.distance.Distance
import model.quantities.distance.DistanceUnit
import model.quantities.time.Time
import model.quantities.time.TimeUnit
import model.quantities.Identifier

object ANTLRWrapper {
    private fun makeParser(s: String): DnFParser {
        val input = CharStreams.fromString(s)
        val lexer = DnFLexer(input)
        val tokens = CommonTokenStream(lexer)
        return DnFParser(tokens)
    }

    fun parseTime(s: String): Time {
        return makeParser(s).time().result
    }

    fun parseDistance(s: String): Distance {
        return makeParser(s).distance().result
    }

    fun parseDamage(s: String): Damage {
        return makeParser(s).damage().result
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

    fun parseAmount(s: String): Amount {
        return makeParser(s).amount().result
    }

    fun parseIdentifier(s: String): Identifier {
        return makeParser(s).identifier().result
    }
}