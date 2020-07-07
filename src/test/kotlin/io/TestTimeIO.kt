package io

import model.quantities.Identifier
import model.quantities.QuantityBinary
import model.quantities.amounts.Dice
import model.quantities.amounts.NumberLiteral
import model.quantities.time.TimeComponent
import model.quantities.time.TimeUnitLiteral
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.EnumSource

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
    fun parseUnit(unit: TimeUnitLiteral) {
        assertEquals(unit, ANTLRWrapper.parseTimeUnit(unit.symbol))
    }
}
