use std::borrow::Cow;

pub struct NormalizeConfig {
    pub classic_english_contractions: bool,
    pub american_english: bool,
    pub classic_english_contractions_unstrict: bool,
    pub strict_ponctuation: bool,
}

impl Default for NormalizeConfig {
    fn default() -> Self {
        Self {
            classic_english_contractions: true,
            american_english: true,
            classic_english_contractions_unstrict: true,
            strict_ponctuation: false,
        }
    }
}

pub fn normalize_string(string: &str, config: &NormalizeConfig) -> String {
    // To lowercase and remove useless ponctuation

    let mut string: String = string.to_lowercase();

    if !config.strict_ponctuation {
        string = string
            .chars()
            .map(|x| match x {
                ',' | '.' | '?' | '!' | ';' | ':' => ' ',
                _ => x,
            })
            .collect();
    }

    if config.classic_english_contractions {
        string = classic_english(&string);
    }

    if config.classic_english_contractions_unstrict {
        string = classic_english_without_unstrict(&string);
    }

    if config.american_english {
        string = american_english(&string);
    }

    // Replace all groups of more than two spaces by one space
    let mut string = Cow::Borrowed(string.trim());
    while string.contains("  ") {
        string = Cow::Owned(string.replace("  ", " "));
    }

    string.into_owned()
}

#[inline]
fn classic_english(string: &str) -> String {
    string
        .replace("don't", "do not")
        .replace("doesn't", "does not")
        .replace("hasn't", "has not")
        .replace("haven't", "have not")
        .replace("i've", "i have")
        .replace("we've", "we have")
        .replace("you've", "you have")
        .replace("they've", "they have")
        .replace("can't", "cannot")
        .replace("couldn't", "could not")
        .replace("mustn't", "must not")
        .replace("'ll", " will")
        .replace("won't", " will not")
        .replace("weren't", "were not")
        .replace("let's", " let us")
        .replace("wouldn't", " would not")
        .replace("hadn't", "had not")
        .replace("who's", "who is")
        .replace("they're", "they are")
        .replace("we're", "we are")
        .replace("i'm", "i am")
        .replace("you're", "you are")
        .replace("shouldn't", "should not")
        .replace("wasn't", "was not")
        .replace("aren't", "are not")
        .replace("isn't", "is not")
        .replace("didn't", "did not")
        .replace("what's", "what is")
        .replace("shan't", "shall not")
}

#[inline]
fn american_english(string: &str) -> String {
    string
        .replace("gonna", "going to")
        .replace("wanna", "want to")
        .replace("gotta", "have got to")
        .replace("gona", "going to")
        .replace("wana", "want to")
        .replace("gota", "have got to")
}

#[inline]
fn classic_english_without_unstrict(string: &str) -> String {
    string
        .replace("dont", "do not")
        .replace("doesnt", "does not")
        .replace("hasnt", "has not")
        .replace("havent", "have not")
        .replace("ive", "i have")
        .replace("youve", "you have")
        .replace("weve", "we have")
        .replace("theyve", "they have")
        .replace("cant", "cannot")
        .replace("couldnt", "could not")
        .replace("mustnt", "must not")
        .replace("shant", "shall not")
        .replace("wont", " will not")
        .replace("hadnt", "had not")
        .replace("werent", "were not")
        .replace("whats", "what is")
        .replace("whos", "who is")
        .replace("im", "i am")
        .replace("theyre", "they are")
        .replace("theyre", "they are")
        .replace("shouldnt", "should not")
        .replace("youre", "you are")
        .replace("isnt", "is not")
        .replace("wasnt", "was not")
        .replace("didnt", "did not")
        .replace("arent", "are not")
}
