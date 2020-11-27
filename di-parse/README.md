#di parse

## Overview

- crate responsibe for taking a string or file of natural language (english) text and parsing it into a syntax tree consisting of some rough structure:
- The c:**string** has one or more (in order of precedence):
    - c:**sentence** in _{,
      exclamation: (content) ~ ("!"),
      question,
      ellipsis,
      ambig_eoi,
      ambig_newl,
      ambig_other
      }_
      has one or more (in order of precedence):
        - > c:**** in _{
          ,
          colonterm: (words) ~ (":"),
          parenwrapped,
          quotewrapped,
          sentenceterm,
          bracketterm,
          periodterm,
          eoiterm,
          newlterm,
          ambigterm
          }_
          has one or more (in order of precedence):
            - c:**expression** in _(
              <!-- non-trivial -->
              nounverbnoun,
              nounverbadj,
              nounverb: ""
              <!-- trivial -->
              date,
              time,
              verb,
              noun,
              adj,
              number,
              preposition,
              adv),

            -

- Basic expression (non trivial )rules:
        - verb can always be preceded/succeeded by (consuming) 0 or more adverbs.
        - parts of speeech (nouns, etc.) in expressions at their most general as such are always lower in precedence than specific subtypes:
        -

- Basic word kinds:
    - nounn: pronoun, time, date, literal
    - adjective: ownership, literal

2. General parts of speech then mapped to more specific identifiers, i.e. subject object relations are inferred from atomic nounverbnoun sentence items, and contextualized with localized information (tense, term type, capitalization, tone, etc, some of which can be gathered from the initial parsing run, such as a match for "by HH:SS" providing the immediate result of a due date, which may require more info) before being sent to the interpreter crate for a full in-context analysis which tags each sentence / sentence item with entity metadata: subject / object if applicable, chronological context, the usual sentiment and important words extraction, etc. Only information mappings with a very high degree of confidence (i.e. match to a specific PEG grammar rule provides nearly most specific information) are then stored as "facts". Other sentence items which have been tagged with interpreter metadata but have been diagnosed with some type of significant ambiguity will be stored alongside other "questions" which will be stored with a query priority/category corresponding both to the aggregate metadata already collected for it combined with its specific and separate ambiguity heuristic score. This will all eventually map to the record-item-field-value data model
... to be continued
