package model.logic

data class Property<T>(
    val key: String,
    val ask: String,
    val value: T?
)
