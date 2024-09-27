package net.joelcourtney.pantheon.engine

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

class StagedTest {
    @Test
    fun testStaged() {
        val myString = Staged("hi")
        myString mod {
            "$it World!"
        }
        myString init { "Hello" }

        assertEquals("Hello World!", myString())
    }

    @Test
    fun testDependency() {
        val v1 = Staged<Int>()
        val v2 = Staged(5)

        v1 init { v2() + 1 }

        assertEquals(6, v1())
    }

    @Test
    fun testCircularDependency() {
        val v1 = Staged<Int>()
        val v2 = Staged<Int>()

        v1 init { v2() }
        v2 init { v1() }

        assertThrows<IllegalStateException>(v1::invoke)
    }

    @Test
    fun testCaching() {
        val v1 = Staged(0)

        var evalCounter = 0

        v1 mod {
            evalCounter++
            evalCounter
        }

        assertEquals(1, v1())
        assertEquals(1, v1())
    }

    @Test
    fun testSpoiling() {
        val v1 = Staged<Int>()

        v1 init { 1 }

        assertEquals(1, v1())

        v1 mod { it + 2 }

        assertThrows<IllegalStateException>(v1::invoke)
    }

    @Test
    fun testOverwrite() {
        val v1 = Staged(0)

        v1 mod { it + 1 }
        v1 overwrite { 10 }

        assertEquals(10, v1())
    }
}
