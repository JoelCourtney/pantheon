package io

import model.quantities.Identifier
import model.quantities.QuantityBinary
import model.quantities.amounts.AmountBinary
import model.quantities.amounts.AmountBinaryOp
import model.quantities.amounts.Dice
import model.quantities.amounts.NumberLiteral
import model.quantities.distance.DistanceComponent
import model.quantities.distance.DistanceKeyword
import model.quantities.distance.DistanceUnitLiteral
import model.quantities.time.TimeComponent
import model.quantities.time.TimeKeyword
import model.quantities.time.TimeUnitLiteral
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.EnumSource

class TestDistanceIO {
    @Test
    fun singleComponent() {
        assertEquals(
            DistanceComponent(NumberLiteral(1), DistanceUnitLiteral.FOOT),
            ANTLRWrapper.parseDistance("1 feet")
        )
    }

    @Test
    fun sum() {
        assertEquals(
            QuantityBinary(
                QuantityBinary(
                    QuantityBinary(
                        DistanceComponent(
                            Dice(2, 4),
                            DistanceUnitLiteral.SPACE
                        ),
                        DistanceComponent(
                            NumberLiteral(10),
                            DistanceUnitLiteral.MILE
                        )
                    ),
                    DistanceComponent(
                        NumberLiteral(2),
                        DistanceUnitLiteral.FOOT
                    )
                ),
                DistanceKeyword.TOUCH
            ),
            ANTLRWrapper.parseDistance("2d4 spaces; 10 mi; 2 feet; touch")
        )
    }

    @ParameterizedTest(name = "parse unit literal {0}")
    @EnumSource(DistanceUnitLiteral::class)
    fun parseUnit(unit: DistanceUnitLiteral) {
        assertEquals(unit, ANTLRWrapper.parseDistanceUnit(unit.symbol))
    }

    @ParameterizedTest(name = "parse quantity keyword {0}")
    @EnumSource(DistanceKeyword::class)
    fun parseKeyword(kw: DistanceKeyword) {
        assertEquals(kw, ANTLRWrapper.parseDistance(kw.name))
    }
}
