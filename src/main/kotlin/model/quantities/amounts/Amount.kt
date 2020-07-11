package model.quantities.amounts

import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.AmountDeserializer
import model.access.Expression

@JsonDeserialize(using = AmountDeserializer::class)
interface Amount: Expression<Amount> {
    override fun evaluate(): Amount {
        TODO("Not yet implemented")
    }
}
