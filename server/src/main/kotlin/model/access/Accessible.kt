package model.access

interface Accessible {
    operator fun get(id: String): Any
    operator fun set(id: String, value: String)
}
