package net.joelcourtney.pantheon.engine

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class HelloWorldTest {
    @Test
    fun testHelloWorld() {
        assertEquals(HelloWorld.helloWorld(), 5)
    }
}
