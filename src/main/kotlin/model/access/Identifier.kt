package model.access

import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.IdentifierDeserializer


@JsonDeserialize(using = IdentifierDeserializer::class)
data class Identifier<out T>(
    val obj: IdentifierComponent,
    val key: IdentifierComponent
): Expression<T>, IdentifierComponent {
    fun set(value: String) {
        obj.evaluateObject()[key.evaluateKey()] = value
    }
    
    fun get(): T {
        TODO("holup")
    }
    
    override fun evaluateObject(): Accessible {
        TODO("Not yet implemented")
    }

    override fun evaluateKey(): String {
        TODO("Not yet implemented")
    }

    override fun evaluate(): T {
        TODO("Not yet implemented")
    }
}
