package model.identity

data class IdentifierKey(val key: String): IdentifierComponent {
    override fun evaluateObject(): Accessible {
        TODO("Not yet implemented")
    }

    override fun evaluateKey(): String {
        return key
    }
}
