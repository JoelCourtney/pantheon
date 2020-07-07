package io

import model.quantities.amounts.Dice
import model.quantities.amounts.NumberLiteral
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import model.quantities.Identifier
import model.quantities.amounts.AmountBinary
import model.quantities.amounts.AmountBinaryOp
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.EnumSource

class TestAmountIO {

    @Test
    fun testNumber() {
        assertEquals(NumberLiteral(30), ANTLRWrapper.parseAmount("30"))
        assertEquals(NumberLiteral(1), ANTLRWrapper.parseAmount("1"))
        assertEquals(NumberLiteral(0), ANTLRWrapper.parseAmount("0"))
    }

    @Test
    fun testDice() {
        assertEquals(Dice(23, 34), ANTLRWrapper.parseAmount("23d34"))
        assertEquals(Dice(0, 0), ANTLRWrapper.parseAmount("0d0"))
        assertEquals(Dice(3, 1), ANTLRWrapper.parseAmount("3d1"))
    }

    @Test
    fun testIdentifier() {
        assertEquals(Identifier("hello"), ANTLRWrapper.parseAmount(("hello")))
        assertEquals(
            Identifier("hello", "world"), ANTLRWrapper.parseAmount("hello${'$'}world")
        )
    }

    @ParameterizedTest(name = "test binary op {0}")
    @EnumSource(AmountBinaryOp::class)
    fun testBinary(op: AmountBinaryOp) {
        assertEquals(
            AmountBinary(
                op,
                NumberLiteral(10),
                Dice(2, 3)
            ),
            ANTLRWrapper.parseAmount("10" + op.symbol + "2d3")
        )
    }

    @Test
    fun `test binary op divide`() {
        assertEquals(
            AmountBinary(
                AmountBinaryOp.DIVIDE_DOWN,
                NumberLiteral(10),
                Dice(2, 3)
            ),
            ANTLRWrapper.parseAmount("10/2d3")
        )
    }
}