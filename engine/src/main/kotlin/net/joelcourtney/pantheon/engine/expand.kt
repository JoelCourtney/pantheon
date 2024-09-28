package net.joelcourtney.pantheon.engine

fun <SHEET: Any> expand(plugins: Collection<Plugin<SHEET>>, ctor: () -> SHEET): SHEET {
    val c = ctor()

    for (plugin in plugins) {
        with(plugin) { c.apply() }
    }

    return c
}
