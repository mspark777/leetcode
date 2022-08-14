use std::collections::{HashMap, HashSet, VecDeque};

struct FindLadders {
    begin_word: String,
    end_word: String,
    word_set: HashSet<String>,
    distance_map: HashMap<String, u8>,
    word_map: HashMap<String, Vec<String>>,
}

impl FindLadders {
    pub fn new(begin_word: String, end_word: String, word_list: Vec<String>) -> Self {
        let mut word_set = HashSet::with_capacity(word_list.len() + 1);
        word_set.insert(begin_word.clone());
        for word in word_list.iter() {
            word_set.insert(word.clone());
        }

        FindLadders {
            begin_word,
            end_word,
            word_set,
            distance_map: HashMap::new(),
            word_map: HashMap::new(),
        }
    }

    pub fn handle(&mut self) -> Vec<Vec<String>> {
        if !self.word_set.contains(&self.end_word) {
            return vec![];
        }

        let reached = self.bfs();
        if !reached {
            return vec![];
        }

        let mut result = Vec::<Vec<String>>::new();
        self.dfs(
            &mut result,
            &mut vec![self.begin_word.clone()],
            &self.begin_word,
        );
        result
    }

    fn bfs(&mut self) -> bool {
        let begin_word = &self.begin_word;
        let end_word = &self.end_word;
        let distance_map = &mut self.distance_map;
        let word_map = &mut self.word_map;
        let word_set = &self.word_set;

        let mut queue = VecDeque::<String>::new();
        let mut visited = HashSet::<String>::new();
        let mut reached = false;
        let mut distance = 0u8;

        queue.push_back(end_word.clone());
        visited.insert(end_word.clone());
        distance_map.insert(end_word.clone(), distance);

        while !queue.is_empty() {
            let qcount = queue.len();
            distance += 1;
            for _ in 0..qcount {
                let word = queue.pop_front().unwrap();
                let next_words = Self::get_next_word(word_set, &word);
                for next_word in next_words.iter() {
                    if let Some(words) = word_map.get_mut(next_word) {
                        words.push(word.clone());
                    } else {
                        word_map.insert(next_word.clone(), vec![word.clone()]);
                    }

                    if visited.contains(next_word) {
                        continue;
                    }

                    if next_word == begin_word {
                        reached = true;
                    }

                    distance_map.insert(next_word.clone(), distance);
                    queue.push_back(next_word.clone());
                    visited.insert(next_word.clone());
                }
            }
        }

        reached
    }

    fn dfs(&self, result: &mut Vec<Vec<String>>, temp_paths: &mut Vec<String>, word: &String) {
        if word == &self.end_word {
            result.push(temp_paths.clone());
            return;
        }

        let distance_map = &self.distance_map;
        if let Some(next_words) = self.word_map.get(word) {
            for next_word in next_words.iter() {
                let owdt = distance_map.get(word);
                let ondt = distance_map.get(next_word);
                match (owdt, ondt) {
                    (Some(wdt), Some(ndt)) => {
                        if *wdt == (*ndt + 1) {
                            temp_paths.push(next_word.clone());
                            self.dfs(result, temp_paths, next_word);
                            temp_paths.pop();
                        }
                    }
                    _ => continue,
                }
            }
        }
    }

    fn get_next_word(word_set: &HashSet<String>, word: &String) -> Vec<String> {
        let mut result = Vec::<String>::new();
        let bytes = word.as_bytes();
        for i in 0..bytes.len() {
            let code = bytes[i];
            for ch in b'a'..=b'z' {
                if code == ch {
                    continue;
                }

                let mut chars: Vec<u8> = Vec::from(bytes);
                chars[i] = ch;
                let new_word = String::from_utf8(chars).expect("NO");
                if word_set.contains(&new_word) {
                    result.push(new_word);
                }
            }
        }

        result
    }
}
struct Solution {}
impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        FindLadders::new(begin_word, end_word, word_list).handle()
    }
}

struct Input {
    begin_word: &'static str,
    end_word: &'static str,
    word_list: Vec<&'static str>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            begin_word: "hit",
            end_word: "cog",
            word_list: vec!["hot", "dot", "dog", "lot", "log", "cog"],
        },
        Input {
            begin_word: "hit",
            end_word: "cog",
            word_list: vec!["hot", "dot", "dog", "lot", "log"],
        },
        Input {
            begin_word: "aaaaa",
            end_word: "ggggg",
            word_list: vec![
                "aaaaa", "caaaa", "cbaaa", "daaaa", "dbaaa", "eaaaa", "ebaaa", "faaaa", "fbaaa",
                "gaaaa", "gbaaa", "haaaa", "hbaaa", "iaaaa", "ibaaa", "jaaaa", "jbaaa", "kaaaa",
                "kbaaa", "laaaa", "lbaaa", "maaaa", "mbaaa", "naaaa", "nbaaa", "oaaaa", "obaaa",
                "paaaa", "pbaaa", "bbaaa", "bbcaa", "bbcba", "bbdaa", "bbdba", "bbeaa", "bbeba",
                "bbfaa", "bbfba", "bbgaa", "bbgba", "bbhaa", "bbhba", "bbiaa", "bbiba", "bbjaa",
                "bbjba", "bbkaa", "bbkba", "bblaa", "bblba", "bbmaa", "bbmba", "bbnaa", "bbnba",
                "bboaa", "bboba", "bbpaa", "bbpba", "bbbba", "abbba", "acbba", "dbbba", "dcbba",
                "ebbba", "ecbba", "fbbba", "fcbba", "gbbba", "gcbba", "hbbba", "hcbba", "ibbba",
                "icbba", "jbbba", "jcbba", "kbbba", "kcbba", "lbbba", "lcbba", "mbbba", "mcbba",
                "nbbba", "ncbba", "obbba", "ocbba", "pbbba", "pcbba", "ccbba", "ccaba", "ccaca",
                "ccdba", "ccdca", "cceba", "cceca", "ccfba", "ccfca", "ccgba", "ccgca", "cchba",
                "cchca", "cciba", "ccica", "ccjba", "ccjca", "cckba", "cckca", "cclba", "cclca",
                "ccmba", "ccmca", "ccnba", "ccnca", "ccoba", "ccoca", "ccpba", "ccpca", "cccca",
                "accca", "adcca", "bccca", "bdcca", "eccca", "edcca", "fccca", "fdcca", "gccca",
                "gdcca", "hccca", "hdcca", "iccca", "idcca", "jccca", "jdcca", "kccca", "kdcca",
                "lccca", "ldcca", "mccca", "mdcca", "nccca", "ndcca", "occca", "odcca", "pccca",
                "pdcca", "ddcca", "ddaca", "ddada", "ddbca", "ddbda", "ddeca", "ddeda", "ddfca",
                "ddfda", "ddgca", "ddgda", "ddhca", "ddhda", "ddica", "ddida", "ddjca", "ddjda",
                "ddkca", "ddkda", "ddlca", "ddlda", "ddmca", "ddmda", "ddnca", "ddnda", "ddoca",
                "ddoda", "ddpca", "ddpda", "dddda", "addda", "aedda", "bddda", "bedda", "cddda",
                "cedda", "fddda", "fedda", "gddda", "gedda", "hddda", "hedda", "iddda", "iedda",
                "jddda", "jedda", "kddda", "kedda", "lddda", "ledda", "mddda", "medda", "nddda",
                "nedda", "oddda", "oedda", "pddda", "pedda", "eedda", "eeada", "eeaea", "eebda",
                "eebea", "eecda", "eecea", "eefda", "eefea", "eegda", "eegea", "eehda", "eehea",
                "eeida", "eeiea", "eejda", "eejea", "eekda", "eekea", "eelda", "eelea", "eemda",
                "eemea", "eenda", "eenea", "eeoda", "eeoea", "eepda", "eepea", "eeeea", "ggggg",
                "agggg", "ahggg", "bgggg", "bhggg", "cgggg", "chggg", "dgggg", "dhggg", "egggg",
                "ehggg", "fgggg", "fhggg", "igggg", "ihggg", "jgggg", "jhggg", "kgggg", "khggg",
                "lgggg", "lhggg", "mgggg", "mhggg", "ngggg", "nhggg", "ogggg", "ohggg", "pgggg",
                "phggg", "hhggg", "hhagg", "hhahg", "hhbgg", "hhbhg", "hhcgg", "hhchg", "hhdgg",
                "hhdhg", "hhegg", "hhehg", "hhfgg", "hhfhg", "hhigg", "hhihg", "hhjgg", "hhjhg",
                "hhkgg", "hhkhg", "hhlgg", "hhlhg", "hhmgg", "hhmhg", "hhngg", "hhnhg", "hhogg",
                "hhohg", "hhpgg", "hhphg", "hhhhg", "ahhhg", "aihhg", "bhhhg", "bihhg", "chhhg",
                "cihhg", "dhhhg", "dihhg", "ehhhg", "eihhg", "fhhhg", "fihhg", "ghhhg", "gihhg",
                "jhhhg", "jihhg", "khhhg", "kihhg", "lhhhg", "lihhg", "mhhhg", "mihhg", "nhhhg",
                "nihhg", "ohhhg", "oihhg", "phhhg", "pihhg", "iihhg", "iiahg", "iiaig", "iibhg",
                "iibig", "iichg", "iicig", "iidhg", "iidig", "iiehg", "iieig", "iifhg", "iifig",
                "iighg", "iigig", "iijhg", "iijig", "iikhg", "iikig", "iilhg", "iilig", "iimhg",
                "iimig", "iinhg", "iinig", "iiohg", "iioig", "iiphg", "iipig", "iiiig", "aiiig",
                "ajiig", "biiig", "bjiig", "ciiig", "cjiig", "diiig", "djiig", "eiiig", "ejiig",
                "fiiig", "fjiig", "giiig", "gjiig", "hiiig", "hjiig", "kiiig", "kjiig", "liiig",
                "ljiig", "miiig", "mjiig", "niiig", "njiig", "oiiig", "ojiig", "piiig", "pjiig",
                "jjiig", "jjaig", "jjajg", "jjbig", "jjbjg", "jjcig", "jjcjg", "jjdig", "jjdjg",
                "jjeig", "jjejg", "jjfig", "jjfjg", "jjgig", "jjgjg", "jjhig", "jjhjg", "jjkig",
                "jjkjg", "jjlig", "jjljg", "jjmig", "jjmjg", "jjnig", "jjnjg", "jjoig", "jjojg",
                "jjpig", "jjpjg", "jjjjg", "ajjjg", "akjjg", "bjjjg", "bkjjg", "cjjjg", "ckjjg",
                "djjjg", "dkjjg", "ejjjg", "ekjjg", "fjjjg", "fkjjg", "gjjjg", "gkjjg", "hjjjg",
                "hkjjg", "ijjjg", "ikjjg", "ljjjg", "lkjjg", "mjjjg", "mkjjg", "njjjg", "nkjjg",
                "ojjjg", "okjjg", "pjjjg", "pkjjg", "kkjjg", "kkajg", "kkakg", "kkbjg", "kkbkg",
                "kkcjg", "kkckg", "kkdjg", "kkdkg", "kkejg", "kkekg", "kkfjg", "kkfkg", "kkgjg",
                "kkgkg", "kkhjg", "kkhkg", "kkijg", "kkikg", "kkljg", "kklkg", "kkmjg", "kkmkg",
                "kknjg", "kknkg", "kkojg", "kkokg", "kkpjg", "kkpkg", "kkkkg", "ggggx", "gggxx",
                "ggxxx", "gxxxx", "xxxxx", "xxxxy", "xxxyy", "xxyyy", "xyyyy", "yyyyy", "yyyyw",
                "yyyww", "yywww", "ywwww", "wwwww", "wwvww", "wvvww", "vvvww", "vvvwz", "avvwz",
                "aavwz", "aaawz", "aaaaz",
            ],
        },
    ];

    for input in inputs {
        let begin_word = input.begin_word.to_string();
        let end_word = input.end_word.to_string();
        let word_list: Vec<String> = input.word_list.iter().map(|s| s.to_string()).collect();
        let result = Solution::find_ladders(begin_word, end_word, word_list);
        println!("{:?}", result);
    }
}
