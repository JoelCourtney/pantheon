package io

import model.identity.Identifier
import model.identity.IdentifierKey
import model.quantities.Quantity
import model.quantities.QuantityComponent
import model.quantities.amounts.Dice
import model.quantities.TimeKeyword
import model.quantities.TimeUnit
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.Arguments
import org.junit.jupiter.params.provider.EnumSource
import org.junit.jupiter.params.provider.MethodSource
import java.util.stream.Stream
import model.quantities.QuantityType.Time
import model.quantities.amounts.NumberLiteral

class TestTimeIO {
    @Test
    fun singleComponent() {
        assertEquals(
            Quantity<Time>(
                arrayListOf(
                    QuantityComponent(
                        NumberLiteral(1),
                        TimeUnit.ACTION
                    )
                )
            ),
            ANTLRWrapper.parseTime("1 action")
        )
    }

    @Test
    fun sum() {
        assertEquals(
            Quantity<Time>(
                arrayListOf(
                    QuantityComponent(
                        Identifier(
                            IdentifierKey("character"), 
                            IdentifierKey("health")
                        ),
                        TimeUnit.MINUTE
                    ),
                    QuantityComponent(
                        Dice(2, 3),
                        Identifier(
                            IdentifierKey("meh"),
                            IdentifierKey("erty")
                        )
                    )
                )
            ),
            ANTLRWrapper.parseTime("character[health] min; 2d3 meh[erty]")
        )
    }

    @ParameterizedTest(name = "parse unit literal {0}")
    @EnumSource(TimeUnit::class)
    fun parseUnitLiteral(unit: TimeUnit) {
        assertEquals(unit, ANTLRWrapper.parseTimeUnit(unit.identity))
    }

    @ParameterizedTest(name = "parse unit literal {1}")
    @MethodSource
    fun parseAlternateUnitLiteral(s: String, unit: TimeUnit) {
        assertEquals(
            unit,
            ANTLRWrapper.parseTimeUnit(s)
        )
    }

    companion object {
        @JvmStatic
        fun parseAlternateUnitLiteral(): Stream<Arguments> {
            return Stream.of(
                Arguments.of("actions", TimeUnit.ACTION),

                Arguments.of("bonus actions", TimeUnit.BONUS_ACTION),
                Arguments.of("boneless action", TimeUnit.BONUS_ACTION),
                Arguments.of("boneless actions", TimeUnit.BONUS_ACTION),

                Arguments.of("reactions", TimeUnit.REACTION),

                Arguments.of("rounds", TimeUnit.ROUND),

                Arguments.of("sec", TimeUnit.SECOND),
                Arguments.of("secs", TimeUnit.SECOND),
                Arguments.of("seconds", TimeUnit.SECOND),

                Arguments.of("min", TimeUnit.MINUTE),
                Arguments.of("mins", TimeUnit.MINUTE),
                Arguments.of("minutes", TimeUnit.MINUTE),

                Arguments.of("hours", TimeUnit.HOUR),
                Arguments.of("hrs", TimeUnit.HOUR),
                Arguments.of("hr", TimeUnit.HOUR),

                Arguments.of("days", TimeUnit.DAY),

                Arguments.of("short rests", TimeUnit.SHORT_REST),
                Arguments.of("sr", TimeUnit.SHORT_REST),

                Arguments.of("long rests", TimeUnit.LONG_REST),
                Arguments.of("lr", TimeUnit.LONG_REST)
            )
        }
    }

    @ParameterizedTest(name = "parse quantity keyword {0}")
    @EnumSource(TimeKeyword::class)
    fun parseKeyword(kw: TimeKeyword) {
        assertEquals(Quantity<Time>(arrayListOf(kw)), ANTLRWrapper.parseTime(kw.name))
    }
}
