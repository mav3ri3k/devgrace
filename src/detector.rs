use rayon::prelude::*;
use regex::RegexBuilder;
use std::collections::HashMap;

pub struct PolitenessMatch {
    pub word: String,
    pub count: usize,
}

pub fn politeness_words() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        // Gratitude
        (
            "thank you",
            vec![
                "thank you",
                "thank u",
                "thank ya",
                "thx",
                "thnx",
                "ty",
                "tyvm",
                "tysm",
                "tyty",
                "tq",
                "tqsm",
            ],
        ),
        (
            "thanks",
            vec!["thanks", "thanx", "thnx", "thanq", "thankies"],
        ),
        (
            "much appreciated",
            vec!["much appreciated", "greatly appreciated"],
        ),
        ("grateful", vec!["grateful", "so grateful", "very grateful"]),
        (
            "appreciate",
            vec![
                "appreciate it",
                "appreciate you",
                "appreciated",
                "i appreciate",
                "really appreciate",
                "truly appreciate",
                "deeply appreciate",
            ],
        ),
        ("cheers", vec!["cheers"]),
        ("kudos", vec!["kudos"]),
        ("obliged", vec!["obliged", "much obliged"]),
        ("thankful", vec!["thankful", "so thankful", "very thankful"]),
        ("gratitude", vec!["gratitude", "with gratitude"]),
        ("thanks again", vec!["thanks again", "thank you again"]),
        // Requests
        (
            "please",
            vec![
                "please",
                "pls",
                "plz",
                "plis",
                "plox",
                "plspls",
                "plzplz",
                "pretty please",
            ],
        ),
        ("kindly", vec!["kindly"]),
        // Compliments - positive adjectives
        ("awesome", vec!["awesome"]),
        ("amazing", vec!["amazing"]),
        ("brilliant", vec!["brilliant"]),
        ("excellent", vec!["excellent"]),
        ("fantastic", vec!["fantastic"]),
        ("perfect", vec!["perfect"]),
        ("great", vec!["great"]),
        ("wonderful", vec!["wonderful"]),
        ("incredible", vec!["incredible"]),
        ("outstanding", vec!["outstanding"]),
        ("superb", vec!["superb"]),
        ("phenomenal", vec!["phenomenal"]),
        ("exceptional", vec!["exceptional"]),
        ("magnificent", vec!["magnificent"]),
        ("splendid", vec!["splendid"]),
        ("terrific", vec!["terrific"]),
        ("marvelous", vec!["marvelous"]),
        ("remarkable", vec!["remarkable"]),
        ("tremendous", vec!["tremendous"]),
        ("delightful", vec!["delightful"]),
        ("fabulous", vec!["fabulous"]),
        ("lovely", vec!["lovely"]),
        ("beautiful", vec!["beautiful"]),
        ("impressive", vec!["impressive"]),
        ("flawless", vec!["flawless"]),
        ("seamless", vec!["seamless"]),
        ("elegant", vec!["elegant"]),
        ("stellar", vec!["stellar"]),
        ("polished", vec!["polished"]),
        ("thoughtful", vec!["thoughtful"]),
        ("insightful", vec!["insightful"]),
        // Compliments - casual/modern
        ("nice", vec!["nice"]),
        ("cool", vec!["cool"]),
        ("neat", vec!["neat"]),
        ("wow", vec!["wow"]),
        ("sweet", vec!["sweet"]),
        ("dope", vec!["dope"]),
        ("sick", vec![" sick "]),
        ("fire", vec![" fire ", "🔥"]),
        ("goat", vec!["goat", "the goat"]),
        ("goated", vec!["goated"]),
        ("rad", vec![" rad "]),
        ("mint", vec![" mint "]),
        ("clutch", vec!["clutch"]),
        ("lit", vec![" lit "]),
        ("slay", vec!["slay", "slayed"]),
        ("ate", vec!["you ate", "ate this", "ate that"]),
        ("peak", vec!["peak"]),
        ("banger", vec!["banger"]),
        ("vibes", vec!["vibes", "good vibes"]),
        // Praise phrases
        ("good job", vec!["good job"]),
        ("great job", vec!["great job"]),
        ("nice job", vec!["nice job"]),
        ("awesome job", vec!["awesome job"]),
        ("excellent job", vec!["excellent job"]),
        ("fantastic job", vec!["fantastic job"]),
        ("brilliant job", vec!["brilliant job"]),
        ("perfect job", vec!["perfect job"]),
        ("good work", vec!["good work"]),
        ("great work", vec!["great work"]),
        ("nice work", vec!["nice work"]),
        ("awesome work", vec!["awesome work"]),
        ("excellent work", vec!["excellent work"]),
        ("fantastic work", vec!["fantastic work"]),
        ("brilliant work", vec!["brilliant work"]),
        ("perfect work", vec!["perfect work"]),
        ("impressive work", vec!["impressive work"]),
        ("solid work", vec!["solid work"]),
        ("stellar work", vec!["stellar work"]),
        ("beautiful work", vec!["beautiful work"]),
        ("thoughtful work", vec!["thoughtful work"]),
        ("careful work", vec!["careful work"]),
        ("well done", vec!["well done"]),
        ("well played", vec!["well played"]),
        ("well said", vec!["well said"]),
        ("well written", vec!["well written"]),
        ("well explained", vec!["well explained"]),
        ("well spotted", vec!["well spotted"]),
        ("well reasoned", vec!["well reasoned"]),
        ("well handled", vec!["well handled"]),
        ("well put", vec!["well put"]),
        ("nicely done", vec!["nicely done"]),
        ("beautifully done", vec!["beautifully done"]),
        ("expertly done", vec!["expertly done"]),
        ("good catch", vec!["good catch"]),
        ("nice catch", vec!["nice catch"]),
        ("great catch", vec!["great catch"]),
        ("excellent catch", vec!["excellent catch"]),
        ("sharp catch", vec!["sharp catch"]),
        ("smart catch", vec!["smart catch"]),
        ("good point", vec!["good point"]),
        ("nice point", vec!["nice point"]),
        ("great point", vec!["great point"]),
        ("excellent point", vec!["excellent point"]),
        ("fair point", vec!["fair point"]),
        ("valid point", vec!["valid point"]),
        ("good call", vec!["good call"]),
        ("nice call", vec!["nice call"]),
        ("great call", vec!["great call"]),
        ("good explanation", vec!["good explanation"]),
        ("great explanation", vec!["great explanation"]),
        ("nice explanation", vec!["nice explanation"]),
        ("clear explanation", vec!["clear explanation"]),
        ("perfect explanation", vec!["perfect explanation"]),
        ("helpful explanation", vec!["helpful explanation"]),
        ("thorough explanation", vec!["thorough explanation"]),
        ("clear answer", vec!["clear answer"]),
        ("helpful answer", vec!["helpful answer"]),
        ("great answer", vec!["great answer"]),
        ("good solution", vec!["good solution"]),
        ("great solution", vec!["great solution"]),
        ("nice solution", vec!["nice solution"]),
        ("perfect solution", vec!["perfect solution"]),
        ("brilliant solution", vec!["brilliant solution"]),
        ("clever solution", vec!["clever solution"]),
        ("elegant solution", vec!["elegant solution"]),
        ("smart solution", vec!["smart solution"]),
        ("robust solution", vec!["robust solution"]),
        ("practical solution", vec!["practical solution"]),
        ("thoughtful solution", vec!["thoughtful solution"]),
        ("great idea", vec!["great idea"]),
        ("good idea", vec!["good idea"]),
        ("nice idea", vec!["nice idea"]),
        ("smart idea", vec!["smart idea"]),
        ("clever idea", vec!["clever idea"]),
        ("good stuff", vec!["good stuff"]),
        ("nice one", vec!["nice one"]),
        ("big w", vec!["big w", "huge w", "massive w"]),
        ("w take", vec!["w take", "w idea", "w solution", "w fix"]),
        ("no notes", vec!["no notes"]),
        ("say less", vec!["say less"]),
        ("this is it", vec!["this is it"]),
        ("this slaps", vec!["this slaps", "that slaps"]),
        ("this goes hard", vec!["this goes hard", "that goes hard"]),
        (
            "love the",
            vec!["love the approach", "love the idea", "love the solution"],
        ),
        (
            "i like",
            vec!["i like this", "i like that", "i like the approach"],
        ),
        // Success phrases
        ("nailed it", vec!["nailed it"]),
        ("crushed it", vec!["crushed it"]),
        ("killed it", vec!["killed it"]),
        ("aced it", vec!["aced it"]),
        ("works perfectly", vec!["works perfectly"]),
        ("works great", vec!["works great"]),
        ("works like a charm", vec!["works like a charm"]),
        ("runs perfectly", vec!["runs perfectly"]),
        ("runs great", vec!["runs great"]),
        ("that fixed it", vec!["that fixed it"]),
        ("this fixed it", vec!["this fixed it"]),
        ("that did it", vec!["that did it"]),
        ("this did it", vec!["this did it"]),
        ("that works", vec!["that works"]),
        ("this works", vec!["this works"]),
        ("that helps", vec!["that helps"]),
        ("this helps", vec!["this helps"]),
        ("fixed it", vec!["fixed it"]),
        ("it works", vec!["it works"]),
        ("works now", vec!["works now", "it works now"]),
        ("problem solved", vec!["problem solved"]),
        ("issue solved", vec!["issue solved"]),
        ("all set", vec!["all set"]),
        ("nice fix", vec!["nice fix"]),
        ("good fix", vec!["good fix"]),
        ("great fix", vec!["great fix"]),
        ("perfect fix", vec!["perfect fix"]),
        ("w fix", vec!["w fix"]),
        ("fixed fr", vec!["fixed fr"]),
        ("works fr", vec!["works fr"]),
        // Agreement/confirmation
        ("exactly", vec!["exactly"]),
        ("precisely", vec!["precisely"]),
        ("spot on", vec!["spot on"]),
        ("bang on", vec!["bang on"]),
        ("on point", vec!["on point"]),
        ("bingo", vec!["bingo"]),
        ("absolutely", vec!["absolutely"]),
        ("indeed", vec!["indeed"]),
        ("totally", vec!["totally"]),
        ("agreed", vec!["agreed"]),
        ("yes exactly", vec!["yes exactly"]),
        ("right on", vec!["right on"]),
        ("fr", vec!["fr", "frfr"]),
        ("ngl good", vec!["ngl good", "ngl this is good"]),
        (
            "lowkey good",
            vec!["lowkey good", "lowkey great", "lowkey perfect"],
        ),
        (
            "highkey good",
            vec!["highkey good", "highkey great", "highkey perfect"],
        ),
        ("facts", vec!["facts"]),
        ("fax", vec!["fax"]),
        ("bet", vec!["bet"]),
        ("perfect that", vec!["that's perfect", "thats perfect"]),
        ("great that", vec!["that's great", "thats great"]),
        ("awesome that", vec!["that's awesome", "thats awesome"]),
        ("exactly what i needed", vec!["exactly what i needed"]),
        ("exactly what i wanted", vec!["exactly what i wanted"]),
        ("exactly right", vec!["exactly right"]),
        ("that's it", vec!["that's it", "thats it"]),
        ("makes sense", vec!["makes sense", "that makes sense"]),
        ("got it", vec!["got it"]),
        ("understood", vec!["understood"]),
        ("fair enough", vec!["fair enough"]),
        ("good to know", vec!["good to know"]),
        ("clear now", vec!["clear now"]),
        // Compliments about the AI
        ("you rock", vec!["you rock"]),
        ("you're the best", vec!["you're the best", "youre the best"]),
        ("you're amazing", vec!["you're amazing", "youre amazing"]),
        ("you're awesome", vec!["you're awesome", "youre awesome"]),
        ("you're a genius", vec!["you're a genius", "youre a genius"]),
        (
            "you're a lifesaver",
            vec!["you're a lifesaver", "youre a lifesaver"],
        ),
        ("you're a legend", vec!["you're a legend", "youre a legend"]),
        ("you're a star", vec!["you're a star", "youre a star"]),
        ("you're a hero", vec!["you're a hero", "youre a hero"]),
        ("you're a gem", vec!["you're a gem", "youre a gem"]),
        (
            "you're incredible",
            vec!["you're incredible", "youre incredible"],
        ),
        (
            "you're brilliant",
            vec!["you're brilliant", "youre brilliant"],
        ),
        ("you nailed it", vec!["you nailed it"]),
        ("you crushed it", vec!["you crushed it"]),
        ("you saved me", vec!["you saved me"]),
        ("you helped", vec!["you helped a lot", "you helped me"]),
        ("legend", vec!["legend", "absolute legend"]),
        ("hero", vec!["hero"]),
        ("genius", vec!["genius"]),
        ("champion", vec!["champion"]),
        ("star", vec!["star"]),
        ("lifesaver", vec!["lifesaver"]),
        ("savior", vec!["savior", "saviour"]),
        ("mvp", vec!["mvp"]),
        (
            "real one",
            vec!["real one", "you're a real one", "youre a real one"],
        ),
        ("absolute unit", vec!["absolute unit"]),
        // Encouragement
        (
            "love it",
            vec!["love it", "love this", "love that", "i love it"],
        ),
        ("chef's kiss", vec!["chef's kiss", "chefs kiss"]),
        ("bravo", vec!["bravo"]),
        ("hooray", vec!["hooray", "hurrah", "hurray"]),
        ("excellent stuff", vec!["excellent stuff"]),
        ("great stuff", vec!["great stuff"]),
        ("clap", vec!["👏"]),
        ("heart", vec!["❤️", "♥️"]),
        ("100", vec!["💯"]),
        // Quality descriptors
        ("top notch", vec!["top notch"]),
        ("first class", vec!["first class"]),
        ("world class", vec!["world class"]),
        ("magic", vec!["magic"]),
        ("magical", vec!["magical"]),
        ("high quality", vec!["high quality"]),
        ("production ready", vec!["production ready"]),
        ("battle tested", vec!["battle tested"]),
        ("bulletproof", vec!["bulletproof"]),
        // Informal politeness
        ("buddy", vec!["buddy"]),
        ("mate", vec!["mate"]),
        ("friend", vec!["friend"]),
        ("dear", vec!["dear"]),
        ("sir", vec!["sir"]),
        ("ma'am", vec!["ma'am", "maam"]),
        // Satisfaction
        (
            "happy with",
            vec![
                "happy with this",
                "happy with that",
                "glad it works",
                "glad that works",
                "glad this works",
                "happy now",
            ],
        ),
        ("satisfied", vec!["satisfied"]),
        ("pleased", vec!["pleased"]),
        ("im happy", vec!["i'm happy", "im happy"]),
        ("delighted", vec!["delighted"]),
        ("relieved", vec!["relieved"]),
        ("glad", vec!["glad"]),
        // Relief/success
        ("finally", vec!["finally works", "finally fixed", "at last"]),
        ("woohoo", vec!["woohoo"]),
        ("yay", vec!["yay"]),
        ("yess", vec!["yess", "yesss", "yessss"]),
        ("lets go", vec!["let's go", "lets go"]),
        ("lfg", vec!["lfg"]),
        ("gg", vec!["gg", "ggs"]),
        ("ez", vec!["ez"]),
        // Big thanks
        (
            "thanks so much",
            vec!["thanks so much", "thank you so much"],
        ),
        ("thanks a lot", vec!["thanks a lot"]),
        ("thanks a million", vec!["thanks a million"]),
        ("thanks a bunch", vec!["thanks a bunch"]),
        (
            "big thanks",
            vec!["big thanks", "huge thanks", "many thanks"],
        ),
        ("endless thanks", vec!["endless thanks"]),
        ("thanks heaps", vec!["thanks heaps"]),
        ("thanks kindly", vec!["thanks kindly"]),
        ("thank you kindly", vec!["thank you kindly"]),
        ("ily", vec!["ily", "ilysm"]),
        (
            "thank you very much",
            vec!["thank you very much", "thanks very much"],
        ),
        ("thx!!", vec!["thx!!", "ty!!", "thanks!!", "thank you!!"]),
        // "That's" compliments
        (
            "that's brilliant",
            vec!["that's brilliant", "thats brilliant"],
        ),
        ("that's amazing", vec!["that's amazing", "thats amazing"]),
        (
            "that's wonderful",
            vec!["that's wonderful", "thats wonderful"],
        ),
        (
            "that's fantastic",
            vec!["that's fantastic", "thats fantastic"],
        ),
        (
            "that's incredible",
            vec!["that's incredible", "thats incredible"],
        ),
        ("that's perfect", vec!["that's perfect", "thats perfect"]),
        ("that's exactly", vec!["that's exactly", "thats exactly"]),
        ("that's helpful", vec!["that's helpful", "thats helpful"]),
        ("that's useful", vec!["that's useful", "thats useful"]),
        ("that's clear", vec!["that's clear", "thats clear"]),
        ("that's smart", vec!["that's smart", "thats smart"]),
        ("that's clever", vec!["that's clever", "thats clever"]),
        ("that's elegant", vec!["that's elegant", "thats elegant"]),
        // Code-specific praise
        ("nice code", vec!["nice code"]),
        ("good code", vec!["good code"]),
        ("beautiful code", vec!["beautiful code"]),
        ("elegant code", vec!["elegant code"]),
        ("great code", vec!["great code"]),
        ("nice diff", vec!["nice diff"]),
        ("good diff", vec!["good diff"]),
        ("great diff", vec!["great diff"]),
        ("nice patch", vec!["nice patch"]),
        ("good patch", vec!["good patch"]),
        ("great patch", vec!["great patch"]),
        ("nice implementation", vec!["nice implementation"]),
        ("good implementation", vec!["good implementation"]),
        ("great implementation", vec!["great implementation"]),
        ("elegant implementation", vec!["elegant implementation"]),
        ("nice refactor", vec!["nice refactor"]),
        ("good refactor", vec!["good refactor"]),
        ("great refactor", vec!["great refactor"]),
        ("good test", vec!["good test"]),
        ("great test", vec!["great test"]),
        ("nice test", vec!["nice test"]),
        ("good tests", vec!["good tests"]),
        ("great tests", vec!["great tests"]),
        ("nice tests", vec!["nice tests"]),
        // Common dev politeness
        (
            "could you please",
            vec!["could you please", "can you please"],
        ),
        ("would you mind", vec!["would you mind"]),
        ("if you could", vec!["if you could"]),
        ("i'd appreciate", vec!["i'd appreciate", "id appreciate"]),
        ("when you get a chance", vec!["when you get a chance"]),
        ("if possible", vec!["if possible"]),
        ("if you have time", vec!["if you have time"]),
        ("at your convenience", vec!["at your convenience"]),
        ("whenever you can", vec!["whenever you can"]),
        ("no rush", vec!["no rush"]),
        ("take your time", vec!["take your time"]),
        ("rn please", vec!["rn please", "rn pls", "rn plz"]),
        ("real quick", vec!["real quick"]),
        ("quick q", vec!["quick q", "quick question"]),
        ("sorry", vec!["sorry", "apologies", "my apologies"]),
        ("excuse me", vec!["excuse me"]),
        ("pardon", vec!["pardon me"]),
        ("no worries", vec!["no worries"]),
        ("no problem", vec!["no problem"]),
        ("all good", vec!["all good"]),
        ("you're right", vec!["you're right", "youre right"]),
        ("sounds good", vec!["sounds good"]),
        ("sounds great", vec!["sounds great"]),
        ("sounds perfect", vec!["sounds perfect"]),
        ("sounds right", vec!["sounds right"]),
        ("sounds reasonable", vec!["sounds reasonable"]),
        ("looks good", vec!["looks good"]),
        ("looks great", vec!["looks great"]),
        ("looks perfect", vec!["looks perfect"]),
        ("looks right", vec!["looks right"]),
        ("looks clean", vec!["looks clean"]),
        ("looks solid", vec!["looks solid"]),
        ("looks better", vec!["looks better"]),
        ("seems right", vec!["seems right"]),
        ("seems good", vec!["seems good"]),
        ("seems correct", vec!["seems correct"]),
        ("seems solid", vec!["seems solid"]),
        // Emoji appreciation
        (
            "emoji praise",
            vec![
                "🙏", "👍", "🙌", "✨", "⭐", "🌟", "💎", "🏆", "🎉", "🎊", "👏", "❤️", "♥️", "💚",
                "💙", "💜", "🤝", "✅",
            ],
        ),
    ]
}

pub fn count_politeness(messages: &[String]) -> (usize, HashMap<String, usize>) {
    let words: Vec<(&'static str, Vec<regex::Regex>)> = politeness_words()
        .into_iter()
        .map(|(label, variants)| {
            (
                label,
                variants
                    .into_iter()
                    .filter_map(|variant| politeness_regex(variant).ok())
                    .collect(),
            )
        })
        .collect();

    let results: Vec<(usize, HashMap<String, usize>)> = messages
        .par_iter()
        .map(|msg| {
            let mut local_total = 0usize;
            let mut local_counts: HashMap<String, usize> = HashMap::new();
            for (label, variants) in &words {
                for variant in variants {
                    let count = variant.find_iter(msg).count();
                    if count > 0 {
                        local_total += count;
                        *local_counts.entry(label.to_string()).or_insert(0) += count;
                    }
                }
            }
            (local_total, local_counts)
        })
        .collect();

    let mut total = 0usize;
    let mut counts: HashMap<String, usize> = HashMap::new();
    for (t, c) in results {
        total += t;
        for (k, v) in c {
            *counts.entry(k).or_insert(0) += v;
        }
    }

    (total, counts)
}

fn politeness_regex(variant: &str) -> Result<regex::Regex, regex::Error> {
    let variant = variant.trim();
    let escaped = regex::escape(variant);
    let starts_with_word = variant
        .chars()
        .next()
        .map(|c| c.is_alphanumeric() || c == '_')
        .unwrap_or(false);
    let ends_with_word = variant
        .chars()
        .next_back()
        .map(|c| c.is_alphanumeric() || c == '_')
        .unwrap_or(false);

    let pattern = match (starts_with_word, ends_with_word) {
        (true, true) => format!(r"\b{}\b", escaped),
        (true, false) => format!(r"\b{}", escaped),
        (false, true) => format!(r"{}\b", escaped),
        (false, false) => escaped,
    };

    RegexBuilder::new(&pattern).case_insensitive(true).build()
}

pub fn top_words(counts: &HashMap<String, usize>, limit: usize) -> Vec<PolitenessMatch> {
    let mut items: Vec<PolitenessMatch> = counts
        .iter()
        .map(|(word, count)| PolitenessMatch {
            word: word.clone(),
            count: *count,
        })
        .collect();
    items.sort_by(|a, b| b.count.cmp(&a.count));
    items.truncate(limit);
    items
}

#[cfg(test)]
mod tests {
    use super::count_politeness;

    #[test]
    fn counts_politeness_case_insensitively() {
        let messages = vec![
            "THANK YOU, this is PERFECT.".to_string(),
            "I Really Appreciate the GREAT FIX.".to_string(),
        ];

        let (total, counts) = count_politeness(&messages);

        assert!(total >= 5);
        assert_eq!(counts.get("thank you"), Some(&1));
        assert_eq!(counts.get("perfect"), Some(&1));
        assert_eq!(counts.get("appreciate"), Some(&1));
        assert_eq!(counts.get("great"), Some(&1));
        assert_eq!(counts.get("great fix"), Some(&1));
    }

    #[test]
    fn does_not_count_polite_words_inside_larger_words() {
        let messages = vec!["Start the T5-Gemma training estimate.".to_string()];

        let (total, counts) = count_politeness(&messages);

        assert_eq!(total, 0);
        assert!(counts.is_empty());
    }

    #[test]
    fn does_not_count_standalone_technical_descriptors() {
        let messages = vec![
            "The correct clean solid implementation has 100 test cases.".to_string(),
            "This useful robust practical function is clear and maintainable.".to_string(),
        ];

        let (total, counts) = count_politeness(&messages);

        assert_eq!(total, 0);
        assert!(counts.is_empty());
    }

    #[test]
    fn counts_genz_shorthand_and_abbreviations() {
        let messages = vec![
            "plz fix this, tyty".to_string(),
            "big W, this slaps fr".to_string(),
            "lfg, you are goated".to_string(),
        ];

        let (total, counts) = count_politeness(&messages);

        assert!(total >= 6);
        assert_eq!(counts.get("please"), Some(&1));
        assert_eq!(counts.get("thank you"), Some(&1));
        assert_eq!(counts.get("big w"), Some(&1));
        assert_eq!(counts.get("this slaps"), Some(&1));
        assert_eq!(counts.get("fr"), Some(&1));
        assert_eq!(counts.get("lfg"), Some(&1));
        assert_eq!(counts.get("goated"), Some(&1));
    }
}
