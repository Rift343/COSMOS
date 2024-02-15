use syntaxic_parser;
pub fn main() {
    syntaxic_parser::parseur_syntaxique("SELECT id FROM Personne;".to_string());
}