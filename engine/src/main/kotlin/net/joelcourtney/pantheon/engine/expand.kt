package net.joelcourtney.pantheon.engine

fun <C: Any> expand(plugins: Collection<Plugin<C>>, ctor: () -> C): C {
    val c = ctor()

    for (plugin in plugins) {
        with(plugin) { c.apply() }
    }

    return c
}
