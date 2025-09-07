use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

pub const COMMON_WORDS: &[&str] = &[
    "the", "be", "to", "of", "and", "a", "in", "that", "have", "i",
    "it", "for", "not", "on", "with", "he", "as", "you", "do", "at",
    "this", "but", "his", "by", "from", "they", "we", "say", "her", "she",
    "or", "an", "will", "my", "one", "all", "would", "there", "their",
    "what", "so", "up", "out", "if", "about", "who", "get", "which", "go",
    "me", "when", "make", "can", "like", "time", "no", "just", "him", "know",
    "take", "people", "into", "year", "your", "good", "some", "could", "them",
    "see", "other", "than", "then", "now", "look", "only", "come", "its", "over",
    "think", "also", "back", "after", "use", "two", "how", "our", "work", "first",
    "well", "way", "even", "new", "want", "because", "any", "these", "give", "day",
    "most", "us", "is", "water", "long", "very", "after", "called", "just", "where",
    "much", "before", "right", "too", "means", "old", "any", "same", "tell", "boy",
    "follow", "came", "want", "show", "also", "around", "farm", "three", "small",
    "set", "put", "end", "why", "again", "turn", "here", "off", "went", "old",
    "number", "great", "tell", "men", "say", "small", "every", "found", "still",
    "between", "name", "should", "home", "big", "give", "air", "line", "set",
    "own", "under", "read", "last", "never", "us", "left", "end", "along", "while",
    "might", "next", "sound", "below", "saw", "something", "thought", "both", "few",
    "those", "always", "looked", "show", "large", "often", "together", "asked",
    "house", "don't", "world", "going", "want", "school", "important", "until",
    "form", "food", "keep", "children", "feet", "land", "side", "without", "boy",
    "once", "animal", "life", "enough", "took", "sometimes", "four", "head", "above",
    "kind", "began", "almost", "live", "page", "got", "earth", "need", "far", "hand",
    "high", "year", "mother", "light", "country", "father", "let", "night", "picture",
    "being", "study", "second", "soon", "story", "since", "white", "ever", "paper",
    "hard", "near", "sentence", "better", "best", "across", "during", "today",
    "however", "sure", "knew", "it's", "try", "told", "young", "sun", "thing",
    "whole", "hear", "example", "heard", "several", "change", "answer", "room",
    "sea", "against", "top", "turned", "learn", "point", "city", "play", "toward",
    "five", "himself", "usually", "money", "seen", "didn't", "car", "morning",
    "i'm", "body", "upon", "family", "later", "turn", "move", "face", "door",
    "cut", "done", "group", "true", "leave", "color", "red", "friends", "easy",
    "become", "walk", "place", "turn", "such", "start", "lot", "eye", "ask", "late",
    "run", "move", "live", "believe", "feel", "week", "hand", "high", "government",
    "person", "plant", "cover", "court", "produce", "help", "far", "pull", "church",
    "small", "book", "include", "water", "follow", "act", "program", "close", "human",
    "community", "name", "run", "business", "increase", "problem", "service"
];

pub const CHALLENGING_WORDS: &[&str] = &[
    "accommodate", "achievement", "acknowledgment", "acquaintance", "acquire", "aggressive",
    "analysis", "appreciate", "argument", "beautiful", "beginning", "believe", "business",
    "calendar", "category", "cemetery", "changeable", "colleague", "commitment", "committee",
    "competition", "completely", "conscious", "convenience", "definitely", "desperate",
    "development", "difference", "disappoint", "discipline", "embarrass", "environment",
    "equipment", "especially", "exaggerate", "excellent", "exercise", "existence",
    "experience", "explanation", "facility", "familiar", "February", "foreign",
    "fourth", "frequently", "government", "grammar", "guarantee", "harassment",
    "height", "hierarchy", "humorous", "immediately", "independent", "intelligence",
    "interesting", "interrupt", "knowledge", "laboratory", "library", "license",
    "maintenance", "manageable", "maneuver", "millennium", "miniature", "mischievous",
    "misspell", "necessary", "occasion", "occurred", "occurrence", "opportunity",
    "parallel", "particular", "personnel", "pneumonia", "possession", "possible",
    "practically", "preferred", "privilege", "probably", "procedure", "professor",
    "pronunciation", "psychology", "publicly", "questionnaire", "receive", "recommend",
    "referred", "religious", "restaurant", "rhythm", "schedule", "secretary",
    "separate", "similar", "sincerely", "strength", "successful", "surprise",
    "thorough", "thought", "throughout", "tomorrow", "truly", "unfortunately",
    "unnecessary", "until", "vacuum", "valuable", "vegetable", "Wednesday",
    "weight", "whether", "which", "writing", "written"
];

pub const PROGRAMMING_WORDS: &[&str] = &[
    "function", "variable", "array", "object", "string", "boolean", "integer", "float",
    "class", "method", "return", "import", "export", "module", "package", "library",
    "framework", "algorithm", "data", "structure", "loop", "condition", "statement",
    "expression", "operator", "assignment", "comparison", "logical", "arithmetic",
    "syntax", "semantic", "compile", "execute", "debug", "error", "exception",
    "try", "catch", "finally", "throw", "async", "await", "promise", "callback",
    "event", "listener", "handler", "interface", "abstract", "inherit", "extend",
    "implement", "override", "public", "private", "protected", "static", "final",
    "const", "let", "var", "null", "undefined", "true", "false", "if", "else",
    "switch", "case", "default", "for", "while", "do", "break", "continue",
    "foreach", "map", "filter", "reduce", "find", "sort", "push", "pop", "shift",
    "unshift", "splice", "slice", "join", "split", "replace", "match", "search",
    "index", "length", "size", "count", "add", "remove", "delete", "update",
    "create", "read", "write", "open", "close", "save", "load", "parse", "stringify"
];

#[derive(Debug, Clone, PartialEq)]
pub enum WordDifficulty {
    Easy,
    Medium,
    Hard,
    Programming,
}

pub fn generate_random_words(count: usize) -> String {
    generate_words_with_difficulty(count, WordDifficulty::Easy)
}

pub fn generate_words_with_difficulty(count: usize, difficulty: WordDifficulty) -> String {
    let word_list = match difficulty {
        WordDifficulty::Easy => COMMON_WORDS,
        WordDifficulty::Medium => &[COMMON_WORDS, CHALLENGING_WORDS].concat(),
        WordDifficulty::Hard => CHALLENGING_WORDS,
        WordDifficulty::Programming => PROGRAMMING_WORDS,
    };

    let mut rng = thread_rng();
    let mut words = Vec::new();
    
    for _ in 0..count {
        if let Some(&word) = word_list.choose(&mut rng) {
            words.push(word.to_string());
        }
    }
    
    words.join(" ")
}

pub fn generate_focused_practice(target_chars: &[char], word_count: usize) -> String {
    let target_set: HashSet<char> = target_chars.iter().cloned().collect();
    
    let filtered_words: Vec<&str> = COMMON_WORDS.iter().filter(|word| {
            word.chars().any(|c| target_set.contains(&c))
        }).cloned().collect();

    if filtered_words.is_empty() {
        return generate_random_words(word_count);
    }

    let mut rng = thread_rng();
    let mut words = Vec::new();
    
    for _ in 0..word_count {
        if let Some(&word) = filtered_words.choose(&mut rng) {
            words.push(word.to_string());
        }
    }
    
    words.join(" ")
}

pub fn generate_pangram_text(repeat_count: usize) -> String {
    let pangrams = [
        "The quick brown fox jumps over the lazy dog.",
        "Pack my box with five dozen liquor jugs.",
        "Waltz, bad nymph, for quick jigs vex.",
        "Sphinx of black quartz, judge my vow.",
        "How vexingly quick daft zebras jump!",
        "Bright vixens jump; dozy fowl quack.",
        "Quick zephyrs blow, vexing daft Jim."
    ];

    let mut rng = thread_rng();
    let mut result = Vec::new();
    
    for _ in 0..repeat_count {
        if let Some(&pangram) = pangrams.choose(&mut rng) {
            result.push(pangram.to_string());
        }
    }
    
    result.join(" ")
}

pub fn generate_number_practice(count: usize) -> String {
    let mut rng = thread_rng();
    let mut parts = Vec::new();
    
    for _ in 0..count {
        let num = rand::random::<u32>() % 10000;
        parts.push(num.to_string());
    }
    
    parts.join(" ")
}

pub fn generate_symbol_practice(count: usize) -> String {
    let symbols = "!@#$%^&*()_+-=[]{}|;':\",./<>?`~";
    let symbol_chars: Vec<char> = symbols.chars().collect();
    let mut rng = thread_rng();
    let mut result = String::new();
    
    for i in 0..count {
        if i > 0 {
            result.push(' ');
        }
        let seq_length = 3 + (rand::random::<usize>() % 5);
        for _ in 0..seq_length {
            if let Some(&symbol) = symbol_chars.choose(&mut rng) {
                result.push(symbol);
            }
        }
    }
    
    result
}

pub fn generate_mixed_content(word_count: usize, include_numbers: bool, include_symbols: bool) -> String {
    let mut parts = Vec::new();
    let word_portion = word_count * 70 / 100; // 70% words
    let number_portion = if include_numbers { word_count * 20 / 100 } else { 0 }; // 20% numbers
    let symbol_portion = if include_symbols { word_count * 10 / 100 } else { 0 }; // 10% symbols
    
    if word_portion > 0 {
        parts.push(generate_random_words(word_portion));
    }
    
    if number_portion > 0 {
        parts.push(generate_number_practice(number_portion));
    }
    
    if symbol_portion > 0 {
        parts.push(generate_symbol_practice(symbol_portion));
    }
    
    let mut rng = thread_rng();
    parts.shuffle(&mut rng);
    parts.join(" ")
}

pub fn calculate_text_difficulty(text: &str) -> f64 {
    let total_chars = text.len();
    if total_chars == 0 {
        return 0.0;
    }
    
    let mut difficulty_score = 0.0;
    let mut char_count = 0;
    
    for ch in text.chars() {
        char_count += 1;

        difficulty_score += match ch {
            'a'..='z' | 'A'..='Z' | ' ' => 1.0,
            '0'..='9' => 1.5,
            '.' | ',' | ';' | ':' | '!' | '?' => 2.0,
            '\'' | '"' | '-' | '_' => 2.5,
            _ => 3.0,
        };
    }
    
    let base_difficulty = difficulty_score / char_count as f64;
    let words: Vec<&str> = text.split_whitespace().collect();
    let avg_word_length = if words.is_empty() {
        0.0
    } else {
        words.iter().map(|w| w.len()).sum::<usize>() as f64 / words.len() as f64
    };
    
    let length_multiplier = if avg_word_length > 6.0 {
        1.2
    } else if avg_word_length < 4.0 {
        0.9
    } else {
        1.0
    };
    
    (base_difficulty * length_multiplier).min(5.0)
}

pub fn get_difficulty_description(score: f64) -> &'static str {
    match score {
        s if s < 1.5 => "Very Easy",
        s if s < 2.0 => "Easy", 
        s if s < 2.5 => "Medium",
        s if s < 3.5 => "Hard",
        _ => "Very Hard",
    }
}

pub fn wrap_text_to_width(text: &str, width: usize) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();
    
    for word in words {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + 1 + word.len() <= width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }
    
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    lines
}

pub fn estimate_typing_time(text: &str, wpm: f64) -> std::time::Duration {
    let word_count = text.split_whitespace().count();
    let minutes = word_count as f64 / wpm;
    std::time::Duration::from_secs((minutes * 60.0) as u64)
}

pub fn sanitize_text(text: &str) -> String {
    text.chars().filter(|c| c.is_ascii() && (*c == ' ' || *c == '\n' || c.is_ascii_graphic())).collect::<String>().trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_words() {
        let text = generate_random_words(10);
        let words: Vec<&str> = text.split_whitespace().collect();
        assert_eq!(words.len(), 10);
        for word in words {
            assert!(COMMON_WORDS.contains(&word));
        }
    }

    #[test]
    fn test_generate_words_with_difficulty() {
        let easy_text = generate_words_with_difficulty(5, WordDifficulty::Easy);
        let hard_text = generate_words_with_difficulty(5, WordDifficulty::Hard);
        let prog_text = generate_words_with_difficulty(5, WordDifficulty::Programming);
        assert_eq!(easy_text.split_whitespace().count(), 5);
        assert_eq!(hard_text.split_whitespace().count(), 5);
        assert_eq!(prog_text.split_whitespace().count(), 5);
    }

    #[test]
    fn test_focused_practice() {
        let target_chars = vec!['a', 'e', 'i', 'o', 'u'];
        let text = generate_focused_practice(&target_chars, 10);
        let words: Vec<&str> = text.split_whitespace().collect();
        
        assert_eq!(words.len(), 10);
        for word in words {
            assert!(word.chars().any(|c| target_chars.contains(&c)));
        }
    }

    #[test]
    fn test_calculate_text_difficulty() {
        let easy_text = "the cat sat on the mat";
        let hard_text = "The qu!ck br@wn f0x jump$ 0v3r th3 l@zy d0g!!!";
        
        let easy_score = calculate_text_difficulty(easy_text);
        let hard_score = calculate_text_difficulty(hard_text);
        
        assert!(hard_score > easy_score);
        assert!(easy_score >= 1.0);
        assert!(hard_score <= 5.0);
    }

    #[test]
    fn test_wrap_text_to_width() {
        let text = "This is a long sentence that should be wrapped to multiple lines";
        let lines = wrap_text_to_width(text, 20);
        
        assert!(lines.len() > 1);
        for line in &lines {
            assert!(line.len() <= 20);
        }
    }

    #[test]
    fn test_sanitize_text() {
        let dirty_text = "Hello\tWorld\x00\x01\x02!!!";
        let clean_text = sanitize_text(dirty_text);
        
        assert_eq!(clean_text, "HelloWorld!!!");
    }

    #[test]
    fn test_estimate_typing_time() {
        let text = "hello world test typing";
        let duration = estimate_typing_time(text, 60.0); // 60 WPM
        assert_eq!(duration.as_secs(), 4);
    }

    #[test]
    fn test_difficulty_descriptions() {
        assert_eq!(get_difficulty_description(1.0), "Very Easy");
        assert_eq!(get_difficulty_description(1.8), "Easy");
        assert_eq!(get_difficulty_description(2.2), "Medium");
        assert_eq!(get_difficulty_description(3.0), "Hard");
        assert_eq!(get_difficulty_description(4.0), "Very Hard");
    }
}