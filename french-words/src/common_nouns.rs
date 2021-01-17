use crate::structs::{StaticLemma, StaticCommonNoun, NounGender};
pub const COMMON_NOUNS: [StaticCommonNoun; 121] = [
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "achard", phonetic: Some("a S a R") },
        plural: Some(StaticLemma { content: "achars", phonetic: Some("a S a R") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "adieu", phonetic: Some("a d j 2") },
        plural: Some(StaticLemma { content: "adieus", phonetic: Some("a d j 2") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "affure", phonetic: Some("a f y R @") },
        plural: Some(StaticLemma { content: "affurs", phonetic: Some("a f y R") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "ail", phonetic: Some("a j") },
        plural: Some(StaticLemma { content: "aulx", phonetic: Some("o") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "alcatraz", phonetic: Some("a l k a t R a z") },
        plural: Some(StaticLemma { content: "alcatrazs", phonetic: Some("a l k a t R a z") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "amenokal", phonetic: Some("a m e n O k a l") },
        plural: Some(StaticLemma { content: "amenokals", phonetic: Some("a m e n O k a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "andalou", phonetic: Some("a~ d a l u") },
        plural: Some(StaticLemma { content: "andaloux", phonetic: Some("a~ d a l u") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "archal", phonetic: Some("a R k a l") },
        plural: Some(StaticLemma { content: "archals", phonetic: Some("a R k a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "argenton", phonetic: Some("a R Z a~ t o~") },
        plural: Some(StaticLemma { content: "argenton", phonetic: Some("a R Z a~ t o~") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "aspalathe", phonetic: Some("a s p a l a t @") },
        plural: Some(StaticLemma { content: "aspalaths", phonetic: Some("a s p a l a t") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "azalaïe", phonetic: Some("a z a l a j") },
        plural: Some(StaticLemma { content: "azalaïs", phonetic: Some("a z a l a j") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "azédarach", phonetic: Some("a z e d a R a k") },
        plural: Some(StaticLemma { content: "azédaracs", phonetic: Some("a z e d a R a k") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "baal", phonetic: Some("b a l") },
        plural: Some(StaticLemma { content: "baals", phonetic: Some("b a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "baby", phonetic: Some("b e b i") },
        plural: Some(StaticLemma { content: "babys", phonetic: Some("b e b i") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "bagout", phonetic: Some("b a g u") },
        plural: Some(StaticLemma { content: "bagous", phonetic: Some("b a g u") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "banal", phonetic: Some("b a n a l") },
        plural: Some(StaticLemma { content: "banaux", phonetic: Some("b a n o") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "baroufe", phonetic: Some("b a R u f @") },
        plural: Some(StaticLemma { content: "baroufs", phonetic: Some("b a R u f") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "besant", phonetic: Some("b @ z a~") },
        plural: Some(StaticLemma { content: "besans", phonetic: Some("b @ z a~") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "bestiau", phonetic: Some("b E s t j o") },
        plural: Some(StaticLemma { content: "bestiau", phonetic: Some("b E s t j o") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "bischoff", phonetic: Some("b i S O f") },
        plural: Some(StaticLemma { content: "bischofs", phonetic: Some("b i S O f") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "bizuth", phonetic: Some("b i z y t") },
        plural: Some(StaticLemma { content: "bizuts", phonetic: Some("b i z y") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "boscot", phonetic: Some("b O s k o") },
        plural: Some(StaticLemma { content: "boscos", phonetic: Some("b O s k o") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "bosselard", phonetic: Some("b O s @ l a R") },
        plural: Some(StaticLemma { content: "bosselars", phonetic: Some("b O s @ l a R") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "bougnat", phonetic: Some("b u J a") },
        plural: Some(StaticLemma { content: "bougnas", phonetic: Some("b u J a") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "brandy", phonetic: Some("b R a~ d i") },
        plural: Some(StaticLemma { content: "brandies", phonetic: Some("b R a~ d i") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "burgau", phonetic: Some("b y R g o") },
        plural: Some(StaticLemma { content: "burgaux", phonetic: Some("b y R g o") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "capital", phonetic: Some("k a p i t a l") },
        plural: Some(StaticLemma { content: "capital", phonetic: Some("k a p i t a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "captal", phonetic: Some("k a p t a l") },
        plural: Some(StaticLemma { content: "captaux", phonetic: Some("k a p t o") })
    },
    StaticCommonNoun {
        gender: NounGender::Feminine,
        singular: StaticLemma { content: "catau", phonetic: Some("k a t o") },
        plural: Some(StaticLemma { content: "cataus", phonetic: Some("k a t o") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "cavagnole", phonetic: Some("k a v a J O l @") },
        plural: Some(StaticLemma { content: "cavagnols", phonetic: Some("k a v a J O l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "cendal", phonetic: Some("s a~ d a l") },
        plural: Some(StaticLemma { content: "cendals", phonetic: Some("s a~ d a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "champart", phonetic: Some("S a~ p a R") },
        plural: Some(StaticLemma { content: "champart", phonetic: Some("S a~ p a R") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "chaparral", phonetic: Some("S a p a R a l") },
        plural: Some(StaticLemma { content: "chaparrals", phonetic: Some("S a p a R a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "charabiat", phonetic: Some("S a R a b j a") },
        plural: Some(StaticLemma { content: "charabias", phonetic: Some("S a R a b j a") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "charre", phonetic: Some("S a R @") },
        plural: Some(StaticLemma { content: "charrs", phonetic: Some("S a R") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "choral", phonetic: Some("k O R a l") },
        plural: Some(StaticLemma { content: "choraux", phonetic: Some("k O R o") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "chouquet", phonetic: Some("S u k E") },
        plural: Some(StaticLemma { content: "chouques", phonetic: Some("S u k @") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "cipal", phonetic: Some("s i p a l") },
        plural: Some(StaticLemma { content: "cipals", phonetic: Some("s i p a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "clédal", phonetic: Some("k l e d a l") },
        plural: Some(StaticLemma { content: "clédals", phonetic: Some("k l e d a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "coli", phonetic: Some("k O l i") },
        plural: Some(StaticLemma { content: "colirs", phonetic: Some("k O l i R") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "colombard", phonetic: Some("k O l o~ b a R") },
        plural: Some(StaticLemma { content: "colombars", phonetic: Some("k O l o~ b a R") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "concasseur", phonetic: Some("k o~ k a s 9 R") },
        plural: Some(StaticLemma { content: "concasseur", phonetic: Some("k o~ k a s 9 R") })
    },
    StaticCommonNoun {
        gender: NounGender::Feminine,
        singular: StaticLemma { content: "corroborie", phonetic: Some("k O R O b O R i") },
        plural: Some(StaticLemma { content: "corroboris", phonetic: Some("k O R O b O R i") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "cosson", phonetic: Some("k O s o~") },
        plural: Some(StaticLemma { content: "cosson", phonetic: Some("k O s o~") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "courson", phonetic: Some("k u R s o~") },
        plural: Some(StaticLemma { content: "courson", phonetic: Some("k u R s o~") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "dabe", phonetic: Some("d a b @") },
        plural: Some(StaticLemma { content: "dabs", phonetic: Some("d a b") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "dandy", phonetic: Some("d a~ d i") },
        plural: Some(StaticLemma { content: "dandies", phonetic: Some("d a~ d i") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "desiderata", phonetic: Some("d e z i d e R a t a") },
        plural: Some(StaticLemma { content: "desiderata", phonetic: Some("d e z i d e R a t a") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "différentiel", phonetic: Some("d i f e R a~ s j E l") },
        plural: Some(StaticLemma { content: "différentiel", phonetic: Some("d i f e R a~ s j E l") })
    },
    StaticCommonNoun {
        gender: NounGender::Feminine,
        singular: StaticLemma { content: "dourah", phonetic: Some("d u R a") },
        plural: Some(StaticLemma { content: "douras", phonetic: Some("d u R a") })
    },
    StaticCommonNoun {
        gender: NounGender::Feminine,
        singular: StaticLemma { content: "durandal", phonetic: Some("d y R a~ d a l") },
        plural: Some(StaticLemma { content: "durandals", phonetic: Some("d y R a~ d a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "durion", phonetic: Some("d y R j o~") },
        plural: Some(StaticLemma { content: "durion", phonetic: Some("d y R j o~") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "dégagé", phonetic: Some("d e g a Z e") },
        plural: Some(StaticLemma { content: "dégagé", phonetic: Some("d e g a Z e") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "empannon", phonetic: Some("a~ p a n o~") },
        plural: Some(StaticLemma { content: "empannon", phonetic: Some("a~ p a n o~") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "enfeu", phonetic: Some("a~ f 2") },
        plural: Some(StaticLemma { content: "enfeux", phonetic: Some("a~ f 2") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "entravon", phonetic: Some("a~ t R a v o~") },
        plural: Some(StaticLemma { content: "entravon", phonetic: Some("a~ t R a v o~") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "essart", phonetic: Some("e s a R") },
        plural: Some(StaticLemma { content: "essart", phonetic: Some("e s a R") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "fantoche", phonetic: Some("f a~ t O S @") },
        plural: Some(StaticLemma { content: "fantoche", phonetic: Some("f a~ t O S @") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "final", phonetic: Some("f i n a l") },
        plural: Some(StaticLemma { content: "finals", phonetic: Some("f i n a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Invariable,
        singular: StaticLemma { content: "foutah", phonetic: Some("f u t a") },
        plural: Some(StaticLemma { content: "foutas", phonetic: Some("f u t a") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "frontal", phonetic: Some("f R o~ t a l") },
        plural: Some(StaticLemma { content: "frontal", phonetic: Some("f R o~ t a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "galiléen", phonetic: Some("g a l i l e e~") },
        plural: Some(StaticLemma { content: "galiléen", phonetic: Some("g a l i l e e~") })
    },
    StaticCommonNoun {
        gender: NounGender::Feminine,
        singular: StaticLemma { content: "gandourah", phonetic: Some("g a~ d u R a") },
        plural: Some(StaticLemma { content: "gandouras", phonetic: Some("g a~ d u R a") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "garingal", phonetic: Some("g a R e~ g a l") },
        plural: Some(StaticLemma { content: "garingals", phonetic: Some("g a R e~ g a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "germinal", phonetic: Some("Z E R m i n a l") },
        plural: Some(StaticLemma { content: "germinaux", phonetic: Some("Z E R m i n o") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "giraumont", phonetic: Some("Z i R o m o~") },
        plural: Some(StaticLemma { content: "giraumons", phonetic: Some("Z i R o m o~") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "granite", phonetic: Some("g R a n i t @") },
        plural: Some(StaticLemma { content: "granits", phonetic: Some("g R a n i t") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "grau", phonetic: Some("g R o") },
        plural: Some(StaticLemma { content: "graux", phonetic: Some("g R o") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "grésillon", phonetic: Some("g R e z i j o~") },
        plural: Some(StaticLemma { content: "grésillon", phonetic: Some("g R e z i j o~") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "hamal", phonetic: Some("a m a l") },
        plural: Some(StaticLemma { content: "hamals", phonetic: Some("a m a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "hammal", phonetic: Some("a m a l") },
        plural: Some(StaticLemma { content: "hammals", phonetic: Some("a m a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "hemlock", phonetic: Some("E m l O k") },
        plural: Some(StaticLemma { content: "hemlocs", phonetic: Some("E m l O k") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "hobby", phonetic: Some("O b i") },
        plural: Some(StaticLemma { content: "hobbys", phonetic: Some("O b i") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "hosannah", phonetic: Some("O z a n a") },
        plural: Some(StaticLemma { content: "hosannas", phonetic: Some("O z a n a") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "humain", phonetic: Some("y m e~") },
        plural: Some(StaticLemma { content: "humain", phonetic: Some("y m e~") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "idéal", phonetic: Some("i d e a l") },
        plural: Some(StaticLemma { content: "idéals", phonetic: Some("i d e a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "impedimenta", phonetic: Some("e~ p e d i m e~ t a") },
        plural: Some(StaticLemma { content: "impedimenta", phonetic: Some("e~ p e d i m e~ t a") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "incarnat", phonetic: Some("e~ k a R n a") },
        plural: Some(StaticLemma { content: "incarnat", phonetic: Some("e~ k a R n a") })
    },
    StaticCommonNoun {
        gender: NounGender::Invariable,
        singular: StaticLemma { content: "jaina", phonetic: Some("Z E n a") },
        plural: Some(StaticLemma { content: "jains", phonetic: Some("Z e~") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "journal", phonetic: Some("Z u R n a l") },
        plural: Some(StaticLemma { content: "journals", phonetic: Some("Z u R n a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "juvenilia", phonetic: Some("Z y v @ n i l j a") },
        plural: Some(StaticLemma { content: "juvenilia", phonetic: Some("Z y v @ n i l j a") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "karik", phonetic: Some("k a R i k") },
        plural: Some(StaticLemma { content: "karis", phonetic: Some("k a R i") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "kirghiz", phonetic: Some("k i R g i z") },
        plural: Some(StaticLemma { content: "kirghizs", phonetic: Some("k i R g i z") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "kolkhoz", phonetic: Some("k O l k o z") },
        plural: Some(StaticLemma { content: "kolkhozs", phonetic: Some("k O l k o z") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "lacertien", phonetic: Some("l a s E R t j e~") },
        plural: Some(StaticLemma { content: "lacertien", phonetic: Some("l a s E R t j e~") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "lacertilien", phonetic: Some("l a s E R t i l j e~") },
        plural: Some(StaticLemma { content: "lacertilien", phonetic: Some("l a s E R t i l j e~") })
    },
    StaticCommonNoun {
        gender: NounGender::Feminine,
        singular: StaticLemma { content: "lady", phonetic: Some("l e d i") },
        plural: Some(StaticLemma { content: "ladys", phonetic: Some("l e d i") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "lapilli", phonetic: Some("l a p i l i") },
        plural: Some(StaticLemma { content: "lapilli", phonetic: Some("l a p i l i") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "lieu", phonetic: Some("l j 2") },
        plural: Some(StaticLemma { content: "lieus", phonetic: Some("l j 2") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "lissier", phonetic: Some("l i s j e") },
        plural: Some(StaticLemma { content: "lissier", phonetic: Some("l i s j e") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "malin", phonetic: Some("m a l e~") },
        plural: Some(StaticLemma { content: "malin", phonetic: Some("m a l e~") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "malin", phonetic: Some("m a l e~") },
        plural: Some(StaticLemma { content: "malin", phonetic: Some("m a l e~") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "match", phonetic: Some("m a t S") },
        plural: Some(StaticLemma { content: "matches", phonetic: Some("m a t S @") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "maton", phonetic: Some("m a t o~") },
        plural: Some(StaticLemma { content: "maton", phonetic: Some("m a t o~") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "maximum", phonetic: Some("m a k s i m O m") },
        plural: Some(StaticLemma { content: "maxima", phonetic: Some("m a k s i m a") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "minimum", phonetic: Some("m i n i m O m") },
        plural: Some(StaticLemma { content: "minima", phonetic: Some("m i n i m a") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "minéral", phonetic: Some("m i n e R a l") },
        plural: Some(StaticLemma { content: "minéral", phonetic: Some("m i n e R a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "morfal", phonetic: Some("m O R f a l") },
        plural: Some(StaticLemma { content: "morfals", phonetic: Some("m O R f a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "naevus", phonetic: Some("n e v y s") },
        plural: Some(StaticLemma { content: "naevi", phonetic: Some("n e @ v i") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "nilgau", phonetic: Some("n i l g o") },
        plural: Some(StaticLemma { content: "nilgaus", phonetic: Some("n i l g o") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "nominatif", phonetic: Some("n O m i n a t i f") },
        plural: Some(StaticLemma { content: "nominatif", phonetic: Some("n O m i n a t i f") })
    },
    StaticCommonNoun {
        gender: NounGender::Feminine,
        singular: StaticLemma { content: "novae", phonetic: Some("n O v e") },
        plural: Some(StaticLemma { content: "novas", phonetic: Some("n O v a") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "néolithique", phonetic: Some("n e O l i t i k @") },
        plural: Some(StaticLemma { content: "néolithique", phonetic: Some("n e O l i t i k @") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "orbe", phonetic: Some("O R b @") },
        plural: Some(StaticLemma { content: "orbe", phonetic: Some("O R b @") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "orthoptère", phonetic: Some("O R t O p t E R @") },
        plural: Some(StaticLemma { content: "orthoptère", phonetic: Some("O R t O p t E R @") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "palliatif", phonetic: Some("p a l j a t i f") },
        plural: Some(StaticLemma { content: "palliatif", phonetic: Some("p a l j a t i f") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "pascal", phonetic: Some("p a s k a l") },
        plural: Some(StaticLemma { content: "pascaux", phonetic: Some("p a s k o") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "pence", phonetic: Some("p E n s @") },
        plural: Some(StaticLemma { content: "pence", phonetic: Some("p E n s @") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "penny", phonetic: Some("p E n i") },
        plural: Some(StaticLemma { content: "pennies", phonetic: Some("p E n i") })
    },
    StaticCommonNoun {
        gender: NounGender::Feminine,
        singular: StaticLemma { content: "piau", phonetic: Some("p j o") },
        plural: Some(StaticLemma { content: "piaus", phonetic: Some("p j o") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "picaillon", phonetic: Some("p i k a j o~") },
        plural: Some(StaticLemma { content: "picaillon", phonetic: Some("p i k a j o~") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "picardant", phonetic: Some("p i k a R d a~") },
        plural: Some(StaticLemma { content: "picardans", phonetic: Some("p i k a R d a~") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "piffre", phonetic: Some("p i f R @") },
        plural: Some(StaticLemma { content: "piffre", phonetic: Some("p i f R @") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "pogrome", phonetic: Some("p O g R O m @") },
        plural: Some(StaticLemma { content: "pogroms", phonetic: Some("p o g R O m") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "pointillé", phonetic: Some("p w e~ t i j e") },
        plural: Some(StaticLemma { content: "pointillé", phonetic: Some("p w e~ t i j e") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "poussah", phonetic: Some("p u s a") },
        plural: Some(StaticLemma { content: "poussas", phonetic: Some("p u s a") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "pronominal", phonetic: Some("p R O n O m i n a l") },
        plural: Some(StaticLemma { content: "pronominal", phonetic: Some("p R O n O m i n a l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "précambrien", phonetic: Some("p R e k a~ b R j e~") },
        plural: Some(StaticLemma { content: "précambrien", phonetic: Some("p R e k a~ b R j e~") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "pudenda", phonetic: Some("p y d a~ d a") },
        plural: Some(StaticLemma { content: "pudenda", phonetic: Some("p y d a~ d a") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "pyrrole", phonetic: Some("p i R O l @") },
        plural: Some(StaticLemma { content: "pyrrols", phonetic: Some("p i R O l") })
    },
    StaticCommonNoun {
        gender: NounGender::Masculine,
        singular: StaticLemma { content: "pâturon", phonetic: Some("p a t y R o~") },
        plural: Some(StaticLemma { content: "pâturon", phonetic: Some("p a t y R o~") })
    }
];