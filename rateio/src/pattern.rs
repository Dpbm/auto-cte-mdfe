
pub mod xml{
    use regex::Regex;

    pub fn load() -> Regex{
        Regex::new(r"carga *:* *([0-9]+)").unwrap()
    }

    pub fn cubicage() -> Regex{
        Regex::new(r"cubicagem *:* *([0-9]+,[0-9]+) *m3").unwrap()
    }
}

pub mod text{
    use regex::Regex;

    pub fn email_text() -> Regex{
        Regex::new(r"carga *:* *([0-9]{6}) *placa *:* *([0-9a-z]{3,4}-* *[0-9a-z]{3,4}) *frete *:* *([0-9]\.[0-9]{3},[0-9]{2})").unwrap()
    }
}
