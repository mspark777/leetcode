package main

import (
	"fmt"
)

type solution struct {
	beginWord   string
	endWord     string
	wordSet     map[string]bool
	distanceMap map[string]int
	wordMap     map[string][]string
}

func (self *solution) handle() [][]string {
	if _, ok := self.wordSet[self.endWord]; !ok {
		return [][]string{}
	}

	reached := self.bfs()
	if !reached {
		return [][]string{}
	}

	return self.dfs([][]string{}, []string{self.beginWord}, self.beginWord)
}

func (self *solution) bfs() bool {
	beginWord := self.beginWord
	endWord := self.endWord
	distanceMap := self.distanceMap
	wordMap := self.wordMap

	queue := []string{endWord}
	visited := make(map[string]bool)
	reached := false
	distance := 0

	visited[endWord] = true
	distanceMap[endWord] = distance

	for {
		qcount := len(queue)
		if qcount < 1 {
			break
		}
		distance += 1

		for i := 0; i < qcount; i += 1 {
			word := queue[0]
			queue = queue[1:]
			for _, nextWord := range self.getNextWord(word) {
				if nextWords, ok := wordMap[nextWord]; ok {
					nextWords = append(nextWords, word)
					wordMap[nextWord] = nextWords
				} else {
					wordMap[nextWord] = []string{word}
				}

				if _, ok := visited[nextWord]; ok {
					continue
				}

				if nextWord == beginWord {
					reached = true
				}

				distanceMap[nextWord] = distance
				queue = append(queue, nextWord)
				visited[nextWord] = true
			}
		}
	}

	return reached
}

func (self *solution) dfs(result [][]string, tempPaths []string, word string) [][]string {
	if word == self.endWord {
		temp := make([]string, len(tempPaths))
		copy(temp, tempPaths)
		return append(result, temp)
	}

	distanceMap := self.distanceMap
	for _, nextWord := range self.wordMap[word] {
		if distanceMap[word] == (distanceMap[nextWord] + 1) {
			tempPaths = append(tempPaths, nextWord)
			result = self.dfs(result, tempPaths, nextWord)
			tempPaths = tempPaths[:len(tempPaths)-1]
		}
	}

	return result
}

func (self *solution) getNextWord(word string) []string {
	ACODE := 'a'
	ZCODE := 'z'
	result := []string{}

	runes := []rune(word)
	for i := range runes {
		code := runes[i]
		for ch := ACODE; ch <= ZCODE; ch += 1 {
			if code == ch {
				continue
			}

			chars := []rune(word)
			chars[i] = ch
			newWord := string(chars)
			if _, ok := self.wordSet[newWord]; ok {
				result = append(result, newWord)
			}
		}
	}

	return result
}

func findLadders(beginWord string, endWord string, wordList []string) [][]string {
	wordSet := make(map[string]bool)
	wordSet[beginWord] = true
	for _, word := range wordList {
		wordSet[word] = true
	}

	sol := solution{
		beginWord:   beginWord,
		endWord:     endWord,
		wordSet:     wordSet,
		distanceMap: make(map[string]int),
		wordMap:     make(map[string][]string),
	}

	return sol.handle()
}

type input struct {
	beginWord string
	endWord   string
	wordList  []string
}

func main() {
	inputs := []*input{
		{
			beginWord: "hit",
			endWord:   "cog",
			wordList:  []string{"hot", "dot", "dog", "lot", "log", "cog"},
		},
		{
			beginWord: "hit",
			endWord:   "cog",
			wordList:  []string{"hot", "dot", "dog", "lot", "log"},
		},
		{
			beginWord: "aaaaa",
			endWord:   "ggggg",
			wordList:  []string{"aaaaa", "caaaa", "cbaaa", "daaaa", "dbaaa", "eaaaa", "ebaaa", "faaaa", "fbaaa", "gaaaa", "gbaaa", "haaaa", "hbaaa", "iaaaa", "ibaaa", "jaaaa", "jbaaa", "kaaaa", "kbaaa", "laaaa", "lbaaa", "maaaa", "mbaaa", "naaaa", "nbaaa", "oaaaa", "obaaa", "paaaa", "pbaaa", "bbaaa", "bbcaa", "bbcba", "bbdaa", "bbdba", "bbeaa", "bbeba", "bbfaa", "bbfba", "bbgaa", "bbgba", "bbhaa", "bbhba", "bbiaa", "bbiba", "bbjaa", "bbjba", "bbkaa", "bbkba", "bblaa", "bblba", "bbmaa", "bbmba", "bbnaa", "bbnba", "bboaa", "bboba", "bbpaa", "bbpba", "bbbba", "abbba", "acbba", "dbbba", "dcbba", "ebbba", "ecbba", "fbbba", "fcbba", "gbbba", "gcbba", "hbbba", "hcbba", "ibbba", "icbba", "jbbba", "jcbba", "kbbba", "kcbba", "lbbba", "lcbba", "mbbba", "mcbba", "nbbba", "ncbba", "obbba", "ocbba", "pbbba", "pcbba", "ccbba", "ccaba", "ccaca", "ccdba", "ccdca", "cceba", "cceca", "ccfba", "ccfca", "ccgba", "ccgca", "cchba", "cchca", "cciba", "ccica", "ccjba", "ccjca", "cckba", "cckca", "cclba", "cclca", "ccmba", "ccmca", "ccnba", "ccnca", "ccoba", "ccoca", "ccpba", "ccpca", "cccca", "accca", "adcca", "bccca", "bdcca", "eccca", "edcca", "fccca", "fdcca", "gccca", "gdcca", "hccca", "hdcca", "iccca", "idcca", "jccca", "jdcca", "kccca", "kdcca", "lccca", "ldcca", "mccca", "mdcca", "nccca", "ndcca", "occca", "odcca", "pccca", "pdcca", "ddcca", "ddaca", "ddada", "ddbca", "ddbda", "ddeca", "ddeda", "ddfca", "ddfda", "ddgca", "ddgda", "ddhca", "ddhda", "ddica", "ddida", "ddjca", "ddjda", "ddkca", "ddkda", "ddlca", "ddlda", "ddmca", "ddmda", "ddnca", "ddnda", "ddoca", "ddoda", "ddpca", "ddpda", "dddda", "addda", "aedda", "bddda", "bedda", "cddda", "cedda", "fddda", "fedda", "gddda", "gedda", "hddda", "hedda", "iddda", "iedda", "jddda", "jedda", "kddda", "kedda", "lddda", "ledda", "mddda", "medda", "nddda", "nedda", "oddda", "oedda", "pddda", "pedda", "eedda", "eeada", "eeaea", "eebda", "eebea", "eecda", "eecea", "eefda", "eefea", "eegda", "eegea", "eehda", "eehea", "eeida", "eeiea", "eejda", "eejea", "eekda", "eekea", "eelda", "eelea", "eemda", "eemea", "eenda", "eenea", "eeoda", "eeoea", "eepda", "eepea", "eeeea", "ggggg", "agggg", "ahggg", "bgggg", "bhggg", "cgggg", "chggg", "dgggg", "dhggg", "egggg", "ehggg", "fgggg", "fhggg", "igggg", "ihggg", "jgggg", "jhggg", "kgggg", "khggg", "lgggg", "lhggg", "mgggg", "mhggg", "ngggg", "nhggg", "ogggg", "ohggg", "pgggg", "phggg", "hhggg", "hhagg", "hhahg", "hhbgg", "hhbhg", "hhcgg", "hhchg", "hhdgg", "hhdhg", "hhegg", "hhehg", "hhfgg", "hhfhg", "hhigg", "hhihg", "hhjgg", "hhjhg", "hhkgg", "hhkhg", "hhlgg", "hhlhg", "hhmgg", "hhmhg", "hhngg", "hhnhg", "hhogg", "hhohg", "hhpgg", "hhphg", "hhhhg", "ahhhg", "aihhg", "bhhhg", "bihhg", "chhhg", "cihhg", "dhhhg", "dihhg", "ehhhg", "eihhg", "fhhhg", "fihhg", "ghhhg", "gihhg", "jhhhg", "jihhg", "khhhg", "kihhg", "lhhhg", "lihhg", "mhhhg", "mihhg", "nhhhg", "nihhg", "ohhhg", "oihhg", "phhhg", "pihhg", "iihhg", "iiahg", "iiaig", "iibhg", "iibig", "iichg", "iicig", "iidhg", "iidig", "iiehg", "iieig", "iifhg", "iifig", "iighg", "iigig", "iijhg", "iijig", "iikhg", "iikig", "iilhg", "iilig", "iimhg", "iimig", "iinhg", "iinig", "iiohg", "iioig", "iiphg", "iipig", "iiiig", "aiiig", "ajiig", "biiig", "bjiig", "ciiig", "cjiig", "diiig", "djiig", "eiiig", "ejiig", "fiiig", "fjiig", "giiig", "gjiig", "hiiig", "hjiig", "kiiig", "kjiig", "liiig", "ljiig", "miiig", "mjiig", "niiig", "njiig", "oiiig", "ojiig", "piiig", "pjiig", "jjiig", "jjaig", "jjajg", "jjbig", "jjbjg", "jjcig", "jjcjg", "jjdig", "jjdjg", "jjeig", "jjejg", "jjfig", "jjfjg", "jjgig", "jjgjg", "jjhig", "jjhjg", "jjkig", "jjkjg", "jjlig", "jjljg", "jjmig", "jjmjg", "jjnig", "jjnjg", "jjoig", "jjojg", "jjpig", "jjpjg", "jjjjg", "ajjjg", "akjjg", "bjjjg", "bkjjg", "cjjjg", "ckjjg", "djjjg", "dkjjg", "ejjjg", "ekjjg", "fjjjg", "fkjjg", "gjjjg", "gkjjg", "hjjjg", "hkjjg", "ijjjg", "ikjjg", "ljjjg", "lkjjg", "mjjjg", "mkjjg", "njjjg", "nkjjg", "ojjjg", "okjjg", "pjjjg", "pkjjg", "kkjjg", "kkajg", "kkakg", "kkbjg", "kkbkg", "kkcjg", "kkckg", "kkdjg", "kkdkg", "kkejg", "kkekg", "kkfjg", "kkfkg", "kkgjg", "kkgkg", "kkhjg", "kkhkg", "kkijg", "kkikg", "kkljg", "kklkg", "kkmjg", "kkmkg", "kknjg", "kknkg", "kkojg", "kkokg", "kkpjg", "kkpkg", "kkkkg", "ggggx", "gggxx", "ggxxx", "gxxxx", "xxxxx", "xxxxy", "xxxyy", "xxyyy", "xyyyy", "yyyyy", "yyyyw", "yyyww", "yywww", "ywwww", "wwwww", "wwvww", "wvvww", "vvvww", "vvvwz", "avvwz", "aavwz", "aaawz", "aaaaz"},
		},
	}

	for _, input := range inputs {
		result := findLadders(input.beginWord, input.endWord, input.wordList)
		fmt.Println(result)
	}
}
