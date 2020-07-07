package io

import model.quantities.Identifier
import model.quantities.QuantityBinary
import model.quantities.amounts.Dice
import model.quantities.amounts.NumberLiteral
import model.quantities.time.TimeComponent
import model.quantities.time.TimeKeyword
import model.quantities.time.TimeUnit
import model.quantities.time.TimeUnitLiteral
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.*
import java.util.stream.Stream

class TestTimeIO {
    @Test
    fun singleComponent() {
        assertEquals(
            TimeComponent(NumberLiteral(1), TimeUnitLiteral.ACTION),
            ANTLRWrapper.parseTime("1 action")
        )
    }

    @Test
    fun sum() {
        assertEquals(
            QuantityBinary(
                TimeComponent(Identifier("character", "health"), TimeUnitLiteral.MINUTE),
                TimeComponent(Dice(2, 3), Identifier("meh"))
            ),
            ANTLRWrapper.parseTime("character${'$'}health min; 2d3 meh")
        )
    }

    @ParameterizedTest(name = "parse unit literal {0}")
    @EnumSource(TimeUnitLiteral::class)
    fun parseUnitLiteral(unit: TimeUnitLiteral) {
        assertEquals(unit, ANTLRWrapper.parseTimeUnit(unit.symbol))
    }

    @ParameterizedTest(name = "parse unit literal {0}")
    @MethodSource
    fun parseAlternateUnitLiteral(s: String, unit: TimeUnitLiteral) {
        assertEquals(
            unit,
            ANTLRWrapper.parseTimeUnit(s)
        )
    }

    @ParameterizedTest(name = "parse quantity keyword {0}")
    @EnumSource(TimeKeyword::class)
    fun parseKeyword(kw: TimeKeyword) {
        assertEquals(kw, ANTLRWrapper.parseTime(kw.name))
    }

    companion object {
        @JvmStatic
        fun parseAlternateUnitLiteral(): Stream<Arguments> {
            return Stream.of(
                Arguments.of("actions", TimeUnitLiteral.ACTION),

                Arguments.of("bonus actions", TimeUnitLiteral.BONUS_ACTION),
                Arguments.of("boneless action", TimeUnitLiteral.BONUS_ACTION),
                Arguments.of("boneless actions", TimeUnitLiteral.BONUS_ACTION),

                Arguments.of("reactions", TimeUnitLiteral.REACTION),

                Arguments.of("rounds", TimeUnitLiteral.ROUND),

                Arguments.of("sec", TimeUnitLiteral.SECOND),
                Arguments.of("secs", TimeUnitLiteral.SECOND),
                Arguments.of("seconds", TimeUnitLiteral.SECOND),

                Arguments.of("min", TimeUnitLiteral.MINUTE),
                Arguments.of("mins", TimeUnitLiteral.MINUTE),
                Arguments.of("minutes", TimeUnitLiteral.MINUTE),

                Arguments.of("hours", TimeUnitLiteral.HOUR),
                Arguments.of("hrs", TimeUnitLiteral.HOUR),
                Arguments.of("hr", TimeUnitLiteral.HOUR),

                Arguments.of("days", TimeUnitLiteral.DAY),

                Arguments.of("short rests", TimeUnitLiteral.SHORT_REST),
                Arguments.of("sr", TimeUnitLiteral.SHORT_REST),

                Arguments.of("long rests", TimeUnitLiteral.LONG_REST),
                Arguments.of("lr", TimeUnitLiteral.LONG_REST)
            )
        }
    }
}
