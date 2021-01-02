package io

import model.access.Identifier
import model.access.StringLiteral
import model.quantities.amounts.Amount
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test

class TestIdentifier {
    @Test
    fun testIdentifier() {
        Assertions.assertEquals(
                Identifier<Amount>(
                        "hello",
                        listOf(StringLiteral("world"))
                ),
                ANTLRWrapper.parseAmount("hello[world]")
        )
    }
}