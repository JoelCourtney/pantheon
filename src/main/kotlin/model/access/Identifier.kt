package model.access

import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.IdentifierDeserializer

@JsonDeserialize(using = IdentifierDeserializer::class)
data class Identifier<out T>(
    val obj: String,
    val key: String
): Expression<T> {
    val characterAttribute: String?
        get() = if (obj == "character") key else null

    fun set(env: Environment, value: String) {
        val obj = env[obj]
        obj[key] = value
    }

    @Suppress("Unchecked_Cast")
    fun get(env: Environment): T {
        val obj = env[obj]
        return obj[key] as T
    }

    override fun evaluate(): T {
        TODO("Not yet implemented")
    }
}
