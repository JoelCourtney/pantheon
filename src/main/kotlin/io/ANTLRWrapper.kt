package io

import DnLexer
import DnF
import model.access.Expression
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream
import model.quantities.amounts.Amount
import model.access.Identifier
import model.quantities.*
import model.quantities.QuantityType.*
import model.modifications.results.SavingThrowType

/**
 * Parses Strings into [Quantity]'s and [Identifier]'s using ANTLR.
 *
 * Singleton object; no not attempt to instantiate.
 */
object ANTLRWrapper {
    private fun makeParser(s: String): DnF {
        val input = CharStreams.fromString(s)
        val lexer = DnLexer(input)
        val tokens = CommonTokenStream(lexer)
        return DnF(tokens)
    }

    /**
     * Parses a [String] as a [Time] object.
     * 
     * @param [s] String to parse.
     * @return [Time] object containing the same data.
     */
    fun parseTime(s: String): Expression<Quantity<Time>> {
        return makeParser(s).time().result
    }


    /**
     * Parses a [String] as a [Distance] object.
     *
     * @param [s] String to parse.
     * @return [Distance] object containing the same data.
     */
    fun parseDistance(s: String): Expression<Quantity<Distance>> {
        return makeParser(s).distance().result
    }


    /**
     * Parses a [String] as a [Damage] object.
     *
     * @param [s] String to parse.
     * @return [Damage] object containing the same data.
     */
    fun parseDamage(s: String): Expression<Quantity<Damage>> {
        return makeParser(s).damage().result
    }


    /**
     * Parses a [String] as a [DamageUnit] object.
     *
     * @param [s] String to parse.
     * @return [DamageUnit] object containing the same data.
     */
    fun parseDamageUnit(s: String): Expression<QuantityUnit<Damage>> {
        return makeParser(s).damage_unit().result
    }


    /**
     * Parses a [String] as a [DistanceUnit] object.
     *
     * @param [s] String to parse.
     * @return [DistanceUnit] object containing the same data.
     */
    fun parseDistanceUnit(s: String): Expression<QuantityUnit<Distance>> {
        return makeParser(s).distance_unit().result
    }


    /**
     * Parses a [String] as a [TimeUnit] object.
     *
     * @param [s] String to parse.
     * @return [TimeUnit] object containing the same data.
     */
    fun parseTimeUnit(s: String): Expression<QuantityUnit<Time>> {
        return makeParser(s).time_unit().result
    }


    /**
     * Parses a [String] as a [Amount] object.
     *
     * @param [s] String to parse.
     * @return [Amount] object containing the same data.
     */
    fun parseAmount(s: String): Expression<Amount> {
        return makeParser(s).amount().result
    }

    fun parseIdentifier(s: String): Identifier<*> {
        return makeParser(s).identifier().result
    }
    
    fun parseSavingThrowType(s: String): Expression<SavingThrowType> {
        return makeParser(s).saving_throw_type().result
    }
    
    fun parseBoolean(s: String): Expression<Boolean> {
        return makeParser(s).bool().result
    }
}
