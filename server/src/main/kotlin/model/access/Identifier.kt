package model.access

import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.IdentifierDeserializer

@JsonDeserialize(using = IdentifierDeserializer::class)
data class Identifier<out T>(
    val obj: String,
    val keys: List<Expression<String>>,
    val height: Int
): Expression<T> {
    constructor(obj: String, keys: List<Expression<String>>): this(obj, keys, 1)

    val characterAttribute: String?
        get() = if (obj == "character") keys[0].evaluate() else null

    fun set(env: Environment, value: String) {
        var acc: Any = env[obj]
        for (key in keys.subList(0,keys.size-1)) {
            acc = (acc as Accessible)[key.evaluate()]
        }
        (acc as Accessible)[keys.last().evaluate()] = value
    }

    @Suppress("Unchecked_Cast")
    fun get(env: Environment): T {
        var acc: Any = env[obj]
        for (key in keys) {
            acc = (acc as Accessible)[key.evaluate()]
        }
        return acc as T
    }

    override fun evaluate(): T {
        TODO("Not yet implemented")
    }
}
