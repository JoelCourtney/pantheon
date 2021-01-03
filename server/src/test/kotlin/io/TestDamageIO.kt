package io

import model.access.Identifier
import model.access.StringLiteral
import model.quantities.*
import model.quantities.QuantityType.Damage
import model.quantities.amounts.AmountBinary
import model.quantities.amounts.AmountBinaryOp
import model.quantities.amounts.Dice
import model.quantities.amounts.NumberLiteral
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.EnumSource

class TestDamageIO {
    @Test
    fun singleComponent() {
        assertEquals(
            Quantity<QuantityType.Damage>(
                arrayListOf(
                    QuantityComponent(NumberLiteral(1), DamageUnit.COLD)
                )
            ),
            ANTLRWrapper.parseDamage("1 cold")
        )
    }

    @Test
    fun sum() {
        assertEquals(
            Quantity<Damage>(
                arrayListOf(
                    QuantityComponent(
                        Identifier(
                            "character",
                            listOf(StringLiteral("health"))
                        ),
                        DamageUnit.MELEE
                    ),
                    QuantityComponent(
                        AmountBinary(
                            AmountBinaryOp.DIVIDE_DOWN,
                            Dice(2, 3),
                            NumberLiteral(3)
                        ),
                        Identifier(
                            "this",
                            listOf(StringLiteral("thang"))
                        )
                    )
                )
            ),
            ANTLRWrapper.parseDamage("character[health] melee; 2d3/3 this[thang]")
        )
    }

    @ParameterizedTest(name = "parse unit literal {0}")
    @EnumSource(DamageUnit::class)
    fun parseUnit(unit: DamageUnit) {
        assertEquals(unit, ANTLRWrapper.parseDamageUnit(unit.identity))
    }

    @ParameterizedTest(name = "parse quantity keyword {0}")
    @EnumSource(DamageKeyword::class)
    fun parseKeyword(kw: DamageKeyword) {
        assertEquals(Quantity<Damage>(arrayListOf(kw)), ANTLRWrapper.parseDamage(kw.name))
    }
}
