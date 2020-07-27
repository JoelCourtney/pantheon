import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.Token
import io.JacksonWrapper
import model.gameObjects.Race

fun main() {
    val human = JacksonWrapper.readFile<Race>(
        "content-packs/official/Player's Handbook/Races/",
        "Human.yaml"
    )
    println(human.identity)
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
}
