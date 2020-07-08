package model.quantities.amounts

import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.AmountDeserializer

@JsonDeserialize(using = AmountDeserializer::class)
interface Amount {
    override fun toString(): String
}
