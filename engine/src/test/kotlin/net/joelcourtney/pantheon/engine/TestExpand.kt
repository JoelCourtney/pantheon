package net.joelcourtney.pantheon.engine

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class TestExpand {
    data class BasicRules(val i: Int): Plugin<Result>() {
        override fun Result.apply() {
            v1 init { i }

            v2 init { "Test says:" }
            v2 finish {
                val numHellos = it.split("hello").size - 1
                "$it " + "world!".repeat(numHellos)
            }
        }
    }

    object ExtraPlugin: Plugin<Result>() {
        override fun Result.apply() {
            v2 mod { "$it " + "hello".repeat(v1()) }
        }
    }

    class Result {
        val v1 = Staged<Int>()
        val v2 = Staged("")
    }

    @Test
    fun testExpand() {
        val result = expand(listOf(BasicRules(2), ExtraPlugin), ::Result)

        assertEquals(2, result.v1())
        assertEquals("Test says: hellohello world!world!", result.v2())
    }
}
