package net.joelcourtney.pantheon.server

import kotlinx.serialization.Serializable
import net.joelcourtney.pantheon.engine.Plugin
import net.joelcourtney.pantheon.engine.Staged
import org.junit.jupiter.api.Test

class TestSystem {
    @Serializable
    class MySheet {
        val name = Staged<String>()
    }

    class MySystem: System<MySheet> {
        override val name = "My System"

        override fun newCharacterSheet() = MySheet()

        override fun newCharacterSource() = listOf<Plugin<MySheet>>()

        override val sheetSerializer = MySheet.serializer()
    }

    @Test
    fun dummy() {}
}
