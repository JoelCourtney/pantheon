package net.joelcourtney.simpleRPG

import net.joelcourtney.pantheon.engine.Plugin

object SayHello: Plugin<StringCell>() {
    override fun StringCell.apply() {
        s = "Hello World!"
    }
}