package io

import model.quantities.amounts.Dice
import model.quantities.amounts.NumberLiteral
import model.quantities.DistanceKeyword
import model.quantities.DistanceUnit
import model.quantities.Quantity
import model.quantities.QuantityComponent
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.Arguments
import org.junit.jupiter.params.provider.EnumSource
import org.junit.jupiter.params.provider.MethodSource
import java.util.stream.Stream
import model.quantities.QuantityType.Distance

class TestDistanceIO {
    @Test
    fun singleComponent() {
        assertEquals(
            Quantity<Distance>(arrayListOf(QuantityComponent(NumberLiteral(1), DistanceUnit.FOOT))),
            ANTLRWrapper.parseDistance("1 feet")
        )
    }

    @Test
    fun sum() {
        assertEquals(
            Quantity(
                arrayListOf(
                    QuantityComponent(Dice(2,4), DistanceUnit.SPACE),
                    QuantityComponent(NumberLiteral(10), DistanceUnit.MILE),
                    QuantityComponent(NumberLiteral(2), DistanceUnit.FOOT),
                    DistanceKeyword.TOUCH
                )
            ),
            ANTLRWrapper.parseDistance("2d4 spaces; 10 mi; 2 feet; touch")
        )
    }

    @ParameterizedTest(name = "parse unit literal {0}")
    @EnumSource(DistanceUnit::class)
    fun parseUnit(unit: DistanceUnit) {
        assertEquals(unit, ANTLRWrapper.parseDistanceUnit(unit.identity))
    }

    @ParameterizedTest(name = "parse unit literal {1}")
    @MethodSource
    fun parseAlternateUnitLiteral(s: String, unit: DistanceUnit) {
        assertEquals(
            unit,
            ANTLRWrapper.parseDistanceUnit(s)
        )
    }

    companion object {
        @JvmStatic
        fun parseAlternateUnitLiteral(): Stream<Arguments> {
            return Stream.of(
                Arguments.of("foot", DistanceUnit.FOOT),
                Arguments.of("feet", DistanceUnit.FOOT),
                Arguments.of("mile", DistanceUnit.MILE),
                Arguments.of("miles", DistanceUnit.MILE),
                Arguments.of("space", DistanceUnit.SPACE),
                Arguments.of("spaces", DistanceUnit.SPACE)
            )
        }
    }

    @ParameterizedTest(name = "parse quantity keyword {0}")
    @EnumSource(DistanceKeyword::class)
    fun parseKeyword(kw: DistanceKeyword) {
        assertEquals(Quantity<Distance>(arrayListOf(kw)), ANTLRWrapper.parseDistance(kw.name))
    }
}
