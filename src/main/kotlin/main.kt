import io.ANTLRWrapper
import io.JacksonWrapper
import model.gameObjects.prototypes.Class
import model.gameObjects.prototypes.Race

fun main() {
    val rogue = JacksonWrapper.readFile<Class>(
        "content-packs/official/Player's Handbook/Classes/",
        "Rogue.yaml"
    )
    println(rogue.name)
    println(rogue)
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
