package io

import model.quantities.Identifier
import model.quantities.QuantityBinary
import model.quantities.amounts.AmountBinary
import model.quantities.amounts.AmountBinaryOp
import model.quantities.amounts.Dice
import model.quantities.amounts.NumberLiteral
import model.quantities.damage.DamageComponent
import model.quantities.damage.DamageKeyword
import model.quantities.damage.DamageUnitLiteral
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.EnumSource

class TestDamageIO {
    @Test
    fun singleComponent() {
        assertEquals(
            DamageComponent(NumberLiteral(1), DamageUnitLiteral.COLD),
            ANTLRWrapper.parseDamage("1 cold")
        )
    }

    @Test
    fun sum() {
        assertEquals(
            QuantityBinary(
                DamageComponent(Identifier("character", "health"), DamageUnitLiteral.MELEE),
                DamageComponent(
                    AmountBinary(
                        AmountBinaryOp.DIVIDE_DOWN,
                        Dice(2, 3),
                        NumberLiteral(3)
                    ),
                    Identifier("this")
                )
            ),
            ANTLRWrapper.parseDamage("character${'$'}health melee; 2d3/3 this")
        )
    }

    @ParameterizedTest(name = "parse unit literal {0}")
    @EnumSource(DamageUnitLiteral::class)
    fun parseUnit(unit: DamageUnitLiteral) {
        assertEquals(unit, ANTLRWrapper.parseDamageUnit(unit.symbol))
    }

    @ParameterizedTest(name = "parse quantity keyword {0}")
    @EnumSource(DamageKeyword::class)
    fun parseKeyword(kw: DamageKeyword) {
        assertEquals(kw, ANTLRWrapper.parseDamage(kw.name))
    }
}