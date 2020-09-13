package io

import model.access.Identifier
import model.quantities.amounts.Amount
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test

class TestIdentifier {
    @Test
    fun testIdentifier() {
        Assertions.assertEquals(
                Identifier<Amount>(
                        "hello",
                        "world"
                ),
                ANTLRWrapper.parseAmount("hello[world]")
        )
    }
}