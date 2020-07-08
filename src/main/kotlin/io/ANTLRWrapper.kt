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
import model.quantities.Quantity

/**
 * Parses Strings into [Quantity]'s and [Identifier]'s using ANTLR.
 *
 * Singleton object; no not attempt to instantiate.
 */
object ANTLRWrapper {
    private fun makeParser(s: String): DnFParser {
        val input = CharStreams.fromString(s)
        val lexer = DnFLexer(input)
        val tokens = CommonTokenStream(lexer)
        return DnFParser(tokens)
    }

    /**
     * Parses a [String] as a [Time] object.
     * 
     * @param [s] String to parse.
     * @return [Time] object containing the same data.
     */
    fun parseTime(s: String): Time {
        return makeParser(s).time().result
    }


    /**
     * Parses a [String] as a [Distance] object.
     *
     * @param [s] String to parse.
     * @return [Distance] object containing the same data.
     */
    fun parseDistance(s: String): Distance {
        return makeParser(s).distance().result
    }


    /**
     * Parses a [String] as a [Damage] object.
     *
     * @param [s] String to parse.
     * @return [Damage] object containing the same data.
     */
    fun parseDamage(s: String): Damage {
        return makeParser(s).damage().result
    }


    /**
     * Parses a [String] as a [DamageUnit] object.
     *
     * @param [s] String to parse.
     * @return [DamageUnit] object containing the same data.
     */
    fun parseDamageUnit(s: String): DamageUnit {
        return makeParser(s).damage_unit().result
    }


    /**
     * Parses a [String] as a [DistanceUnit] object.
     *
     * @param [s] String to parse.
     * @return [DistanceUnit] object containing the same data.
     */
    fun parseDistanceUnit(s: String): DistanceUnit {
        return makeParser(s).distance_unit().result
    }


    /**
     * Parses a [String] as a [TimeUnit] object.
     *
     * @param [s] String to parse.
     * @return [TimeUnit] object containing the same data.
     */
    fun parseTimeUnit(s: String): TimeUnit {
        return makeParser(s).time_unit().result
    }


    /**
     * Parses a [String] as a [Amount] object.
     *
     * @param [s] String to parse.
     * @return [Amount] object containing the same data.
     */
    fun parseAmount(s: String): Amount {
        return makeParser(s).amount().result
    }


    /**
     * Parses a [String] as a [Identifier] object.
     *
     * @param [s] String to parse.
     * @return [Identifier] object containing the same data.
     */
    fun parseIdentifier(s: String): Identifier {
        return makeParser(s).identifier().result
    }
}
