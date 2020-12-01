package model.access

class StringLiteral(val string: String): Expression<String> {
    override fun evaluate(): String {
        return string
    }
}