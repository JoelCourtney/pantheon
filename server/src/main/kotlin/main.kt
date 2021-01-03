import io.JacksonWrapper
import model.gameObjects.items.Item

fun main() {
    val dagger = JacksonWrapper.readFile<Item>(
        "content-packs/official/Player's Handbook/Items/",
        "Dagger.yaml"
    )
    println(dagger.name)
//    val input = CharStreams.fromString("character[health] min ; 2d3 meh[erty]")
//    val lexer = DnLexer(input)
//    val tokens = CommonTokenStream(lexer) 
//    var token = lexer.nextToken()
//    while (token.getType() != Token.EOF) {
//        println("${token.type}: ${token.text}")
//        token = lexer.nextToken()
//    }
//    val parser = DnF(tokens)
//    println(parser.identifier().result)
//    val input = "root[level 1+]"
//    println(ANTLRWrapper.parseIdentifier(input))
}
